use std::error::Error;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::path::Path;

use bevy::asset::{Asset, AssetIo, AssetPath, HandleId, LoadContext, LoadedAsset};
use bevy::prelude::Handle;
use bevy::reflect::TypeRegistryInternal;

use crate::children::ProtoChildBuilder;
use crate::deps::DependenciesBuilder;
use crate::load::{Loader, ProtoLoadMeta};
use crate::path::ProtoPathContext;
use crate::proto::Prototypical;

/// The context when loading a [prototype].
///
/// [prototype]: Prototypical
pub struct ProtoLoadContext<'a, 'ctx, T: Prototypical, L: Loader<T>> {
    registry: &'a TypeRegistryInternal,
    loader: &'a L,
    load_context: Option<&'a mut LoadContext<'ctx>>,
    child_paths: Vec<AssetPath<'static>>,
    index_path: IndexPath,
    _phantom: PhantomData<T>,
}

impl<'a, 'ctx, T: Prototypical, L: Loader<T>> ProtoLoadContext<'a, 'ctx, T, L> {
    pub(crate) fn new(
        registry: &'a TypeRegistryInternal,
        loader: &'a L,
        load_context: &'a mut LoadContext<'ctx>,
    ) -> Self {
        Self {
            registry,
            loader,
            load_context: Some(load_context),
            child_paths: Vec::new(),
            index_path: IndexPath::default(),
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
        self.index_path.depth()
    }

    /// Creates a [`ProtoChildBuilder`] to allow for proper child processing.
    pub fn with_children<E: Error, F: FnOnce(&mut ProtoChildBuilder<T, L>) -> Result<(), E>>(
        &mut self,
        f: F,
    ) -> Result<(), E> {
        self.index_path.push();

        let mut ctx = Self {
            registry: self.registry,
            loader: self.loader,
            load_context: self.load_context.take(),
            child_paths: Vec::new(),
            index_path: IndexPath::default(),
            _phantom: Default::default(),
        };

        std::mem::swap(&mut ctx.child_paths, &mut self.child_paths);
        std::mem::swap(&mut ctx.index_path, &mut self.index_path);

        let mut builder = ProtoChildBuilder::new(ctx);
        let result = f(&mut builder);

        self.load_context = builder.context.load_context;
        self.child_paths = builder.context.child_paths;
        self.index_path = builder.context.index_path;

        self.index_path.pop();

        result
    }

    /// The loader used to load the prototype.
    pub fn loader(&self) -> &'a L {
        self.loader
    }

    pub(crate) fn increment_index(&mut self) {
        self.index_path.increment();
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

    pub(crate) fn meta(&self) -> ProtoLoadMeta<T> {
        let label = if self.index_path.is_root() {
            // Root prototype
            None
        } else {
            // Descendant prototype
            Some(self.index_path.to_string())
        };

        let path = AssetPath::new(self.base_path().to_owned(), label);
        let handle = self.get_handle(path.get_id());

        ProtoLoadMeta {
            path,
            handle,
            depth: self.depth(),
        }
    }

    pub(crate) fn preprocess_proto(
        &mut self,
        prototype: T,
    ) -> Result<(T, ProtoLoadMeta<T>, Vec<AssetPath<'static>>), L::Error> {
        let meta = self.meta();
        let mut prototype = self.loader.on_load_prototype(prototype, &meta)?;

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

        Ok((prototype, meta, dependency_paths))
    }
}

impl<'a, 'ctx, T: Prototypical, L: Loader<T>> ProtoPathContext
    for ProtoLoadContext<'a, 'ctx, T, L>
{
    fn base_path(&self) -> &Path {
        self.load_context.as_ref().unwrap().path()
    }

    fn asset_io(&self) -> &dyn AssetIo {
        self.load_context.as_ref().unwrap().asset_io()
    }

    fn extensions(&self) -> &[&'static str] {
        self.loader.extensions()
    }
}

/// Helper struct for tracking the depth and index of a prototype being processed.
///
/// Each entry in the index path represents the index of the prototype,
/// and the index of that entry represents the depth of the prototype.
///
/// The root prototype is represented as an empty index path.
#[derive(Default, Debug)]
struct IndexPath(Vec<usize>);

impl IndexPath {
    pub fn push(&mut self) {
        self.0.push(0);
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn increment(&mut self) {
        *self.0.last_mut().expect("index path should not be empty") += 1;
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for IndexPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut is_first = true;
        for index in self.0.iter() {
            if is_first {
                write!(f, "Child: ")?;
                is_first = false;
            } else {
                write!(f, "--")?;
            }

            write!(f, "{:0>4}", index)?;
        }

        Ok(())
    }
}
