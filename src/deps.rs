use bevy::asset::{HandleId, HandleUntyped};
use bevy::utils::HashMap;
use std::path::PathBuf;

#[derive(Default, Debug, PartialEq)]
pub struct DependencyMap {
    /// A mapping of a template to a _strong_ handle of its asset.
    templates: HashMap<PathBuf, HandleUntyped>,
    /// A collection of general assets this template depends upon
    assets: HashMap<HandleId, HandleUntyped>,
}

impl DependencyMap {
    pub fn add_dependency(&mut self, handle: HandleUntyped) -> Option<HandleUntyped> {
        assert!(handle.is_strong(), "Dependency handle must be strong");
        self.assets.insert(handle.id, handle)
    }

    pub fn add_template<P: Into<PathBuf>>(
        &mut self,
        path: P,
        handle: HandleUntyped,
    ) -> Option<HandleUntyped> {
        assert!(handle.is_strong(), "Dependency handle must be strong");
        self.templates.insert(path.into(), handle)
    }

    pub fn get_template(&self, path: &PathBuf) -> Option<&HandleUntyped> {
        self.templates.get(path)
    }

    pub fn remove_dependency<P: Into<String>>(&mut self, path: &PathBuf) -> Option<HandleUntyped> {
        self.templates.remove(path)
    }

    pub fn remove_template<P: Into<String>>(&mut self, handle: HandleId) -> Option<HandleUntyped> {
        self.assets.remove(&handle)
    }
}
