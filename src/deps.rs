use bevy::asset::{HandleId, HandleUntyped};
use bevy::utils::HashMap;
use std::path::PathBuf;

/// A map of assets depended upon by a [prototypical] asset.
///
/// Assets are broken into two categories: templates and general assets.
/// Both are kept loaded as long as the prototype remains loaded.
///
/// [prototypical]: crate::Prototypical
#[derive(Default, Debug, PartialEq)]
pub struct DependencyMap {
    /// A mapping of a template to a _strong_ handle of its asset.
    templates: HashMap<PathBuf, HandleUntyped>,
    /// A collection of general assets this template depends upon
    assets: HashMap<HandleId, HandleUntyped>,
}

impl DependencyMap {
    /// Add an asset dependency.
    ///
    /// # Panics
    ///
    /// Panics if the given handle is not _strong_.
    pub fn add_dependency(&mut self, handle: HandleUntyped) -> Option<HandleUntyped> {
        assert!(handle.is_strong(), "Dependency handle must be strong");
        self.assets.insert(handle.id, handle)
    }

    /// Add a template dependency.
    ///
    /// The given path should be relative to the prototype and point to the file
    /// containing the template prototype.
    ///
    /// # Panics
    ///
    /// Panics if the given handle is not _strong_.
    pub fn add_template<P: Into<PathBuf>>(
        &mut self,
        path: P,
        handle: HandleUntyped,
    ) -> Option<HandleUntyped> {
        assert!(handle.is_strong(), "Dependency handle must be strong");
        self.templates.insert(path.into(), handle)
    }

    /// Get the template with the given path (relative to this prototype).
    pub fn get_template(&self, path: &PathBuf) -> Option<&HandleUntyped> {
        self.templates.get(path)
    }

    /// Remove the template with the given path (relative to this prototype).
    pub fn remove_template<P: Into<String>>(&mut self, path: &PathBuf) -> Option<HandleUntyped> {
        self.templates.remove(path)
    }

    /// Remove the asset dependency with the given handle.
    pub fn remove_dependency<H: Into<HandleId>>(&mut self, handle: H) -> Option<HandleUntyped> {
        self.assets.remove(&handle.into())
    }
}
