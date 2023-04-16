use std::fmt::{Debug, Formatter};

use bevy::asset::{Asset, AssetPath, Handle, HandleUntyped, LoadContext};
use bevy::utils::hashbrown::hash_map::Iter;
use bevy::utils::HashMap;

/// A collection of dependencies for a [prototype].
///
/// [prototype]: crate::proto::Prototypical
#[derive(Default)]
pub struct Dependencies {
    deps: HashMap<AssetPath<'static>, HandleUntyped>,
}

impl Dependencies {
    /// Get a reference to a dependency's strong handle.
    pub fn get<P: Into<AssetPath<'static>>>(&self, path: P) -> Option<&HandleUntyped> {
        let path = path.into();
        self.deps.get(&path)
    }

    /// Insert a new dependency.
    ///
    /// # Panics
    ///
    /// This method panics if the given handle is weak.
    pub fn insert<P: Into<AssetPath<'static>>, H: Into<HandleUntyped>>(
        &mut self,
        path: P,
        handle: H,
    ) -> Option<HandleUntyped> {
        let handle = handle.into();
        debug_assert!(handle.is_strong(), "dependency inserted with weak handle");

        self.deps.insert(path.into(), handle)
    }

    /// Merge this [`Dependencies`] with another.
    pub fn combine(&mut self, other: Dependencies) {
        self.deps.extend(other.deps);
    }

    /// Returns an iterator over the dependencies.
    pub fn iter(&self) -> Iter<'_, AssetPath<'static>, HandleUntyped> {
        self.deps.iter()
    }

    /// Returns the number of dependencies.
    pub fn len(&self) -> usize {
        self.deps.len()
    }

    /// Returns true if there are no dependencies.
    pub fn is_empty(&self) -> bool {
        self.deps.is_empty()
    }
}

impl Debug for Dependencies {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dependencies(")?;
        f.debug_map().entries(self.deps.iter()).finish()?;
        write!(f, ")")
    }
}

/// Builder used to construct [`Dependencies`].
pub struct DependenciesBuilder<'a, 'ctx> {
    deps: HashMap<AssetPath<'static>, HandleUntyped>,
    ctx: &'a mut LoadContext<'ctx>,
}

impl<'a, 'ctx> DependenciesBuilder<'a, 'ctx> {
    pub fn new(ctx: &'a mut LoadContext<'ctx>) -> Self {
        Self {
            deps: HashMap::new(),
            ctx,
        }
    }

    /// Finalize this builder into a new [`Dependencies`] struct.
    pub fn build(self) -> Dependencies {
        Dependencies { deps: self.deps }
    }

    /// Add a new dependency at the given asset path.
    pub fn add_dependency<T: Asset, P: Into<AssetPath<'static>>>(&mut self, path: P) -> Handle<T> {
        let (path, handle) = self.get_handle(path);
        self.deps.insert(path, handle.clone_untyped());
        handle
    }

    fn get_handle<T: Asset, P: Into<AssetPath<'static>>>(
        &mut self,
        path: P,
    ) -> (AssetPath<'static>, Handle<T>) {
        let path = path.into();
        let handle = self.ctx.get_handle(path.get_id());
        (path, handle)
    }
}
