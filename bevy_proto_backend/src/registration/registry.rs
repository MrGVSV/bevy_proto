use std::borrow::Borrow;
use std::hash::Hash;
use std::sync::Arc;

use bevy::asset::{Assets, Handle, HandleId};
use bevy::prelude::Resource;
use bevy::utils::{HashMap, HashSet};
use parking_lot::RwLock;

use crate::proto::{Config, ProtoError, Prototypical};
use crate::tree::{ProtoTree, ProtoTreeBuilder};

/// Resource used to track load states, store mappings, and generate cached data.
#[derive(Resource)]
pub(crate) struct ProtoRegistry<T: Prototypical> {
    ids: HashMap<HandleId, T::Id>,
    handles: HashMap<T::Id, Handle<T>>,
    trees: HashMap<HandleId, ProtoTree<T>>,
    /// This contains a mapping of a prototype to a set of prototypes that
    /// depend on it.
    ///
    /// We can use this information to perform selective re-computations
    /// when the prototype is modified.
    dependents: HashMap<HandleId, HashSet<HandleId>>,
    /// Tracks the prototypes currently being loaded.
    load_queue: Arc<RwLock<LoadQueue<T>>>,
    /// Set of prototypes that failed to be registered.
    failed: HashSet<HandleId>,
}

impl<T: Prototypical> ProtoRegistry<T> {
    pub fn register<'w, H: Into<HandleId>>(
        &mut self,
        handle: H,
        prototypes: &'w Assets<T>,
        config: &'w mut T::Config,
    ) -> Result<&'w T, ProtoError> {
        let handle_id = handle.into();
        match self.register_internal(handle_id, prototypes, config) {
            Err(error) => {
                self.failed.insert(handle_id);
                Err(error)
            }
            ok => ok,
        }
    }

    pub fn unregister<'w, H: Into<HandleId>>(
        &mut self,
        handle: H,
        prototypes: &'w Assets<T>,
        config: &'w mut T::Config,
    ) -> bool {
        let handle_id = handle.into();

        if let Some(id) = self.ids.remove(&handle_id) {
            self.handles.remove(&id);
            self.failed.remove(&handle_id);
            self.trees.remove(&handle_id);
            if let Some(dependents) = self.dependents.remove(&handle_id) {
                for dependent in dependents {
                    // This will return an error when a dependent is missing.
                    // We allow it here because there are times when we expect a dependent to be missing.
                    // For example, if a parent prototype is dropped then its child might be dropped as well.
                    // If this happens, the child will unregister after the non-existent parent.
                    // In the future, we can add better diffing strategies to reduce unnecessary unregistrations.
                    self.register(dependent, prototypes, config).ok();
                }
            }
            true
        } else {
            false
        }
    }

    pub fn contains<I: Hash + Eq + ?Sized>(&self, id: &I) -> bool
    where
        T::Id: Borrow<I>,
    {
        self.handles.contains_key(id)
    }

    pub fn contains_handle<H: Into<HandleId>>(&self, handle: H) -> bool {
        self.ids.contains_key(&handle.into())
    }

    pub fn contains_failed_handle<H: Into<HandleId>>(&self, handle: H) -> bool {
        self.failed.contains(&handle.into())
    }

    pub fn add_dependent<H: Into<HandleId>>(&mut self, dependent: H, dependency: H) {
        let dependents = self.dependents.entry(dependent.into()).or_default();
        dependents.insert(dependency.into());
    }

    pub fn insert_tree<H: Into<HandleId>>(
        &mut self,
        handle: H,
        tree: ProtoTree<T>,
    ) -> Option<ProtoTree<T>> {
        self.trees.insert(handle.into(), tree)
    }

    pub fn get_tree<H: Into<HandleId>>(&self, handle: H) -> Option<&ProtoTree<T>> {
        self.trees.get(&handle.into())
    }

    pub fn get_tree_by_id<I: Borrow<T::Id>>(&self, id: I) -> Option<&ProtoTree<T>> {
        self.handles
            .get(id.borrow())
            .and_then(|handle| self.get_tree(handle))
    }

    pub fn load_queue(&self) -> &Arc<RwLock<LoadQueue<T>>> {
        &self.load_queue
    }

    fn register_internal<'w>(
        &mut self,
        handle: HandleId,
        prototypes: &'w Assets<T>,
        config: &'w mut <T as Prototypical>::Config,
    ) -> Result<&'w T, ProtoError> {
        let handle = prototypes.get_handle(handle);
        let prototype = prototypes
            .get(&handle)
            .ok_or_else(|| ProtoError::DoesNotExist(handle.clone_weak_untyped()))?;

        // Check if ID already exists
        if let Some(existing_handle) = self.handles.get(prototype.id()) {
            if existing_handle.id() != handle.id() {
                let exiting_prototype = prototypes
                    .get(&handle)
                    .ok_or_else(|| ProtoError::DoesNotExist(handle.clone_weak_untyped()))?;
                panic!(
                    "{}",
                    ProtoError::AlreadyExists {
                        id: prototype.id().to_string(),
                        path: prototype.path().into(),
                        existing: exiting_prototype.path().into(),
                    }
                );
            }
        }

        // If already registered -> unregister so we can update all the cached data
        if self.unregister(&handle, prototypes, config) {
            config.on_unregister_prototype(prototype, handle.clone());
        }

        config.on_register_prototype(prototype, handle.clone());

        ProtoTreeBuilder::new(self, prototypes, config).build(&handle)?;

        self.ids.insert(handle.id(), prototype.id().clone());
        self.handles
            .insert(prototype.id().clone(), handle.clone_weak());

        // Complete load
        self.load_queue().write().deque(prototype.id());

        Ok(prototype)
    }
}

impl<T: Prototypical> Default for ProtoRegistry<T> {
    fn default() -> Self {
        Self {
            ids: HashMap::new(),
            handles: HashMap::new(),
            trees: HashMap::new(),
            dependents: HashMap::new(),
            load_queue: Default::default(),
            failed: HashSet::new(),
        }
    }
}

pub(crate) struct LoadQueue<T: Prototypical> {
    handles: HashMap<T::Id, Handle<T>>,
    ids: HashMap<HandleId, T::Id>,
}

impl<T: Prototypical> LoadQueue<T> {
    pub fn queue<I: Into<T::Id>>(&mut self, id: I, handle: &Handle<T>) {
        let id = id.into();
        self.handles.insert(id.clone(), handle.clone_weak());
        self.ids.insert(handle.id(), id);
    }

    pub fn deque<I: Borrow<T::Id>>(&mut self, id: I) -> Option<Handle<T>> {
        if let Some(handle) = self.handles.remove(id.borrow()) {
            self.ids.remove(&handle.id());
            Some(handle)
        } else {
            None
        }
    }

    pub fn is_queued<I: Borrow<T::Id>>(&self, id: I) -> bool {
        self.handles.contains_key(id.borrow())
    }

    pub fn is_queued_handle<I: Borrow<HandleId>>(&self, id: I) -> bool {
        self.ids.contains_key(id.borrow())
    }
}

impl<T: Prototypical> Clone for LoadQueue<T> {
    fn clone(&self) -> Self {
        Self {
            handles: self.handles.clone(),
            ids: self.ids.clone(),
        }
    }
}

impl<T: Prototypical> Default for LoadQueue<T> {
    fn default() -> Self {
        Self {
            handles: Default::default(),
            ids: Default::default(),
        }
    }
}
