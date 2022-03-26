use crate::command::ProtoCommand;
use crate::prelude::Prototypical;
use crate::utils::analyze_deps;
use bevy::asset::{Asset, AssetEvent, AssetServer, Assets, HandleId, LoadState};
use bevy::prelude::{Commands, EventReader, Handle, Res, ResMut};
use bevy::utils::hashbrown::hash_map::Iter;
use bevy::utils::HashMap;

pub struct ProtoManager<T: Prototypical + Asset> {
    name_to_handle: HashMap<String, Handle<T>>,
    handle_to_name: HashMap<HandleId, String>,
    handles: HashMap<HandleId, Handle<T>>,
    spawn_queue: Vec<Handle<T>>,
}

impl<T: Prototypical + Asset> Default for ProtoManager<T> {
    fn default() -> Self {
        Self {
            name_to_handle: HashMap::new(),
            handle_to_name: HashMap::new(),
            handles: HashMap::new(),
            spawn_queue: Vec::new(),
        }
    }
}

impl<T: Prototypical + Asset> ProtoManager<T> {
    /// Queues a handle to a [prototypical] asset to be spawned.
    ///
    /// # Arguments
    ///
    /// * `handle`: A strong handle to the [prototypical] asset
    ///
    /// # Panics
    ///
    /// Will panic if the given handle is not strong.
    ///
    /// [prototypical]: crate::Prototypical
    pub fn spawn(&mut self, handle: Handle<T>) {
        assert!(handle.is_strong());
        self.spawn_queue.push(handle);
    }

    pub fn is_loaded(&self, name: &str) -> bool {
        self.name_to_handle.contains_key(name)
    }

    /// Saves a handle to a [prototypical] asset so that it will stay loaded.
    ///
    /// The saved handle can be removed using the (`unsave`)[Self::unsave] method.
    ///
    /// # Arguments
    ///
    /// * `handle`: A strong handle to the [prototypical] asset
    ///
    /// # Panics
    ///
    /// Will panic if the given handle is not strong.
    ///
    /// [prototypical]: crate::Prototypical
    pub fn save(&mut self, handle: Handle<T>) {
        assert!(handle.is_strong());
        self.handles.insert(handle.id, handle);
    }

    /// Removes a saved handle to a [prototypical] asset.
    ///
    /// This allows the asset to be unloaded if there are no other strong handles
    /// referencing it.
    ///
    /// # Arguments
    ///
    /// * `handle`: A handle to the [prototypical] asset
    ///
    /// [prototypical]: crate::Prototypical
    pub fn unsave<H: Into<HandleId>>(&mut self, handle: H) -> Option<Handle<T>> {
        self.handles.remove(&handle.into())
    }

    pub fn get_handle(&self, name: &str) -> Option<&Handle<T>> {
        self.name_to_handle.get(name)
    }

    pub fn get_name<H: Into<HandleId>>(&self, handle: H) -> Option<&String> {
        self.handle_to_name.get(&handle.into())
    }

    pub(crate) fn iter(&self) -> Iter<'_, String, Handle<T>> {
        self.name_to_handle.iter()
    }

    pub(crate) fn add_name<N: Into<String>>(&mut self, name: N, handle: Handle<T>) {
        let handle = handle.as_weak::<T>();
        let name = name.into();

        self.handle_to_name.insert(handle.id, name.clone());
        self.name_to_handle.insert(name, handle);
    }

    pub(crate) fn remove_name(&mut self, name: &str) {
        if let Some(handle) = self.name_to_handle.remove(name) {
            self.handle_to_name.remove(&handle.id);
        }
    }

    pub(crate) fn remove_name_by_handle<H: Into<HandleId>>(&mut self, handle: H) {
        if let Some(name) = self.handle_to_name.remove(&handle.into()) {
            self.name_to_handle.remove(&name);
        }
    }
}

/// System for spawning queued prototypes
pub(crate) fn spawn_proto<T: Prototypical + Asset>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut manager: ResMut<ProtoManager<T>>,
) {
    let spawn_queue = std::mem::take(&mut manager.spawn_queue);
    for handle in spawn_queue {
        match asset_server.get_load_state(handle.id) {
            LoadState::Loaded if manager.handle_to_name.contains_key(&handle.id) => {
                let entity = commands.spawn().id();
                commands.add(ProtoCommand::<T>::from_handle(entity, handle.id));
            }
            _ => manager.spawn_queue.push(handle),
        }
    }
}

/// System for updating the tracked prototypes.
pub(crate) fn update_tracked<T: Prototypical + Asset>(
    mut reader: EventReader<AssetEvent<T>>,
    assets: Res<Assets<T>>,
    mut manager: ResMut<ProtoManager<T>>,
) {
    for evt in reader.iter() {
        match evt {
            AssetEvent::Created { ref handle } => {
                if let Some(proto) = assets.get(handle) {
                    manager.add_name(proto.name(), handle.clone_weak());

                    #[cfg(feature = "analysis")]
                    analyze_deps(proto, &assets);
                }
                // TODO: Add "prune" feature to allow the removal of duplicates from lower priority templates
            }
            AssetEvent::Modified { ref handle } => {
                if let Some(proto) = assets.get(handle) {
                    manager.add_name(proto.name(), handle.clone_weak());
                }
            }
            AssetEvent::Removed { ref handle } => {
                manager.remove_name_by_handle(handle);
            }
        }
    }
}
