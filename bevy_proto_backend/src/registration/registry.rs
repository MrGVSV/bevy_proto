use std::borrow::Borrow;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::registration::params::RegistryParams;
use bevy::asset::{Handle, HandleId};
use bevy::prelude::Resource;

use crate::assets::ProtoAssetEvent;
use bevy::utils::{HashMap, HashSet};
use parking_lot::RwLock;

use crate::proto::{Config, ProtoError, Prototypical};
use crate::tree::{ProtoTree, ProtoTreeBuilder};

/// Resource used to track load states, store mappings, and generate cached data.
#[derive(Resource)]
pub(crate) struct ProtoRegistry<T: Prototypical, C: Config<T>> {
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
    _phantom: PhantomData<C>,
}

impl<T: Prototypical, C: Config<T>> ProtoRegistry<T, C> {
    /// Registers a prototype.
    ///
    /// This will return an error if the prototype is already registered.
    pub(super) fn register<'w>(
        &mut self,
        handle: &Handle<T>,
        params: &mut RegistryParams<'w, T, C>,
    ) -> Result<&'w T, ProtoError> {
        let prototype = self.register_internal(handle, params, false)?;

        params
            .config_mut()
            .on_register_prototype(prototype, handle.clone());

        params.send_event(ProtoAssetEvent::Created {
            handle: handle.clone_weak(),
            id: prototype.id().clone(),
        });

        Ok(prototype)
    }

    /// Removes a prototype from the registry.
    ///
    /// Returns the ID of the prototype if it was registered.
    pub(super) fn unregister(
        &mut self,
        handle: &Handle<T>,
        params: &mut RegistryParams<T, C>,
    ) -> Option<T::Id> {
        let id = self.unregister_internal(handle, params)?;

        let strong_handle = params.get_strong_handle(handle);
        params
            .config_mut()
            .on_unregister_prototype(&id, strong_handle);

        params.send_event(ProtoAssetEvent::Removed {
            handle: handle.clone_weak(),
            id: id.clone(),
        });

        Some(id)
    }

    /// Reload a registered prototype.
    ///
    /// This will return an error if the prototype is not registered.
    pub(super) fn reload<'w>(
        &mut self,
        handle: &Handle<T>,
        params: &mut RegistryParams<'w, T, C>,
    ) -> Result<&'w T, ProtoError> {
        if self.unregister_internal(handle, params).is_some() {
            let prototype = self.register_internal(handle, params, true)?;
            let strong_handle = params.get_strong_handle(handle);
            params
                .config_mut()
                .on_reload_prototype(prototype, strong_handle);
            params.send_event(ProtoAssetEvent::Modified {
                handle: handle.clone_weak(),
                id: prototype.id().clone(),
            });
            Ok(prototype)
        } else {
            Err(ProtoError::NotRegistered(handle.clone_weak_untyped()))
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
        handle: &Handle<T>,
        params: &mut RegistryParams<'w, T, C>,
        is_reload: bool,
    ) -> Result<&'w T, ProtoError> {
        let handle = params.get_strong_handle(handle);
        let prototype = params.get_prototype(&handle)?;

        if !is_reload {
            // Check if handle already exists
            if self.ids.contains_key(&handle.id()) {
                return Err(ProtoError::AlreadyExists {
                    id: prototype.id().to_string(),
                    path: Box::new(prototype.path().into()),
                    existing: Box::new(prototype.path().into()),
                });
            }

            // Check if ID already exists
            if let Some(existing_handle) = self.handles.get(prototype.id()) {
                self.failed.insert(handle.id());
                if existing_handle.id() != handle.id() {
                    // Not the same asset!
                    let exiting_prototype = params.get_prototype(&handle)?;
                    return Err(ProtoError::AlreadyExists {
                        id: prototype.id().to_string(),
                        path: Box::new(prototype.path().into()),
                        existing: Box::new(exiting_prototype.path().into()),
                    });
                }
            }
        }

        ProtoTreeBuilder::new(self, params.prototypes(), params.config()).build(&handle)?;

        self.ids.insert(handle.id(), prototype.id().clone());
        self.handles
            .insert(prototype.id().clone(), handle.clone_weak());
        self.failed.remove(&handle.id());

        // Complete load
        self.load_queue().write().deque(prototype.id());

        Ok(prototype)
    }

    fn unregister_internal(
        &mut self,
        handle: &Handle<T>,
        params: &mut RegistryParams<T, C>,
    ) -> Option<T::Id> {
        let handle_id = handle.id();

        let id = self.ids.remove(&handle_id)?;
        self.handles.remove(&id);
        self.failed.remove(&handle_id);
        self.trees.remove(&handle_id);
        if let Some(dependents) = self.dependents.remove(&handle_id) {
            for dependent in dependents {
                let dependent_handle = Handle::weak(dependent);
                // This will return an error when a dependent is missing.
                // We allow it here because there are times when we expect a dependent to be missing.
                // For example, if a parent prototype is dropped then its child might be dropped as well.
                // If this happens, the child will unregister after the non-existent parent.
                // In the future, we can add better diffing strategies to reduce unnecessary unregistrations.
                self.reload(&dependent_handle, params).ok();
            }
        }

        Some(id)
    }
}

impl<T: Prototypical, C: Config<T>> Default for ProtoRegistry<T, C> {
    fn default() -> Self {
        Self {
            ids: HashMap::new(),
            handles: HashMap::new(),
            trees: HashMap::new(),
            dependents: HashMap::new(),
            load_queue: Default::default(),
            failed: HashSet::new(),
            _phantom: PhantomData,
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
