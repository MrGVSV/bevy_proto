use std::error::Error;
use std::marker::PhantomData;
use std::path::Path;

use bevy::asset::{Asset, AssetIo, AssetPath, HandleId, LoadContext, LoadedAsset};
use bevy::prelude::Handle;
use bevy::reflect::TypeRegistryInternal;

use crate::children::ProtoChildBuilder;
use crate::deps::DependenciesBuilder;
use crate::path::ProtoPathContext;
use crate::proto::Prototypical;

/// The context when loading a [prototype].
///
/// [prototype]: Prototypical
pub struct ProtoLoadContext<'a, 'ctx, T: Prototypical> {
    registry: &'a TypeRegistryInternal,
    load_context: Option<&'a mut LoadContext<'ctx>>,
    extensions: &'a [&'static str],
    child_paths: Vec<AssetPath<'static>>,
    depth: usize,
    _phantom: PhantomData<T>,
}

impl<'a, 'ctx, T: Prototypical> ProtoLoadContext<'a, 'ctx, T> {
    pub(crate) fn new(
        registry: &'a TypeRegistryInternal,
        load_context: &'a mut LoadContext<'ctx>,
        extensions: &'a [&'static str],
    ) -> Self {
        Self {
            registry,
            load_context: Some(load_context),
            extensions,
            child_paths: Vec::new(),
            depth: 0,
            _phantom: Default::default(),
        }
    }

    /// Bevy's type registry.
    pub fn registry(&self) -> &'a TypeRegistryInternal {
        self.registry
    }

    /// Get a strong [`Handle`] for a given asset.
    pub fn get_handle<I: Into<HandleId>, A: Asset>(&self, handle: I) -> Handle<A> {
        self.load_context.as_ref().unwrap().get_handle(handle)
    }

    /// The current hierarchical depth of the prototype being processed.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Creates a [`ProtoChildBuilder`] to allow for proper child processing.
    pub fn with_children<E: Error, F: FnOnce(&mut ProtoChildBuilder<T>) -> Result<(), E>>(
        &mut self,
        f: F,
    ) -> Result<(), E> {
        let mut ctx = Self {
            registry: self.registry,
            load_context: self.load_context.take(),
            extensions: self.extensions,
            child_paths: Vec::new(),
            depth: self.depth + 1,
            _phantom: Default::default(),
        };

        std::mem::swap(&mut ctx.child_paths, &mut self.child_paths);
        self.depth += 1;

        let mut builder = ProtoChildBuilder::new(ctx);
        let result = f(&mut builder);
        self.load_context = builder.context.load_context;
        self.child_paths = builder.context.child_paths;

        self.depth -= 1;

        result
    }

    pub(crate) fn set_labeled_asset<A: Asset>(
        &mut self,
        label: &str,
        asset: LoadedAsset<A>,
    ) -> Handle<A> {
        self.load_context
            .as_mut()
            .unwrap()
            .set_labeled_asset(label, asset)
    }

    pub(crate) fn load_context(&self) -> &&'a mut LoadContext<'ctx> {
        self.load_context.as_ref().unwrap()
    }

    pub(crate) fn child_paths_mut(&mut self) -> &mut Vec<AssetPath<'static>> {
        &mut self.child_paths
    }

    pub(crate) fn preprocess_proto(
        &mut self,
        prototype: &mut T,
    ) -> Result<Vec<AssetPath<'static>>, T::Error> {
        let mut deps = DependenciesBuilder::new(self.load_context.as_mut().unwrap());

        // 1. Track schematic dependencies
        for (_, schematic) in prototype.schematics_mut().iter_mut() {
            schematic.preload_dependencies(&mut deps)?;
        }

        prototype.dependencies_mut().combine(deps.build());

        // 2. Track prototype dependencies
        let mut dependency_paths = prototype
            .dependencies()
            .iter()
            .map(|(path, _)| path)
            .cloned()
            .collect::<Vec<_>>();

        // 3. Track template dependencies
        if let Some(templates) = prototype.templates() {
            dependency_paths.extend(templates.iter().map(|(path, _)| path.into()));
        }

        Ok(dependency_paths)
    }
}

impl<'a, 'ctx, T: Prototypical> ProtoPathContext for ProtoLoadContext<'a, 'ctx, T> {
    fn base_path(&self) -> &Path {
        self.load_context.as_ref().unwrap().path()
    }

    fn asset_io(&self) -> &dyn AssetIo {
        self.load_context.as_ref().unwrap().asset_io()
    }

    fn extensions(&self) -> &[&'static str] {
        self.extensions
    }
}
