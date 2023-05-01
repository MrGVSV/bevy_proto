use std::path::{Path, PathBuf};

use bevy::asset::Handle;
use bevy::prelude::Resource;
use bevy::utils::HashMap;

use crate::proto::Prototypical;

/// General-purpose resource for storing [prototype] asset handles in order to keep them loaded.
///
/// [prototype]: Prototypical
#[derive(Resource)]
pub(crate) struct ProtoStorage<T: Prototypical> {
    path_to_handle: HashMap<PathBuf, Handle<T>>,
}

impl<T: Prototypical> ProtoStorage<T> {
    /// Returns true if a prototype with the given path is currently stored in this resource.
    pub fn contains<P: AsRef<Path>>(&self, path: P) -> bool {
        self.path_to_handle.contains_key(path.as_ref())
    }

    /// Get a reference to the strong handle for the prototype at the given path.
    ///
    /// Returns `None` if no matching prototype is currently stored in this resource.
    pub fn get<P: AsRef<Path>>(&self, path: P) -> Option<&Handle<T>> {
        self.path_to_handle.get(path.as_ref())
    }

    /// Insert a prototype handle into this resource for the given path.
    ///
    /// If a handle already existed for the path, the existing one is returned.
    ///
    /// # Panics
    ///
    /// Panics if the given handle is weak.
    pub fn insert<P: Into<PathBuf>>(&mut self, path: P, handle: Handle<T>) -> Option<Handle<T>> {
        debug_assert!(handle.is_strong(), "attempted to store weak handle");
        self.path_to_handle.insert(path.into(), handle)
    }

    /// Remove the handle with the given path.
    pub fn remove<P: AsRef<Path>>(&mut self, path: P) -> Option<Handle<T>> {
        self.path_to_handle.remove(path.as_ref())
    }

    /// Remove all handles.
    pub fn clear(&mut self) {
        self.path_to_handle.clear();
    }
}

impl<T: Prototypical> Default for ProtoStorage<T> {
    fn default() -> Self {
        Self {
            path_to_handle: HashMap::new(),
        }
    }
}
