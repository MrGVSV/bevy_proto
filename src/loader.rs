use std::marker::PhantomData;
use std::path::Path;

use anyhow::Error;
use bevy::asset::{
    Asset, AssetLoader, AssetPath, BoxedFuture, Handle, HandleId, HandleUntyped, LoadContext,
    LoadedAsset,
};
use bevy::prelude::{FromWorld, World};
use bevy::reflect::TypeRegistryArc;
use bevy::utils::HashSet;

use crate::config::ProtoConfigArc;
use crate::prelude::{Prototype, Prototypical, TemplateList};
use crate::serde::extensions;
use crate::serde::ProtoDeserializable;

pub(crate) struct ProtoAssetLoader<T: Prototypical + ProtoDeserializable + Asset = Prototype> {
    pub config: ProtoConfigArc,
    pub registry: TypeRegistryArc,
    pub extensions: Vec<&'static str>,
    pub phantom: PhantomData<T>,
}

impl<T: Prototypical + ProtoDeserializable + Asset> FromWorld for ProtoAssetLoader<T> {
    fn from_world(world: &mut World) -> Self {
        let registry = world.resource::<TypeRegistryArc>();
        let config = world.resource::<ProtoConfigArc>();

        let mut exts = Vec::new();
        #[cfg(feature = "yaml")]
        exts.push(extensions::YAML_EXT);
        #[cfg(feature = "json")]
        exts.push(extensions::JSON_EXT);
        #[cfg(feature = "ron")]
        exts.push(extensions::RON_EXT);

        Self {
            registry: registry.clone(),
            config: config.clone(),
            extensions: exts,
            phantom: PhantomData::default(),
        }
    }
}

impl<T: Prototypical + ProtoDeserializable + Asset> AssetLoader for ProtoAssetLoader<T> {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async move {
            let config = &self.config.read();
            let registry = &self.registry;
            let mut proto = T::deserialize(bytes, load_context.path(), config, registry)?;
            config.call_on_register(&mut proto)?;

            let mut preloader = AssetPreloader::new(&load_context);
            for component in proto.components_mut() {
                component.preload_assets(&mut preloader);
            }

            let templates = if let Some(templates) = proto.templates() {
                templates
                    .iter_defined_order()
                    .map(|template| AssetPath::new(template.to_owned(), None))
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };

            let proto_deps = proto.dependencies_mut();
            for template in templates.iter() {
                let handle: Handle<T> = load_context.get_handle(template.clone());
                let handle = handle.clone_untyped();
                proto_deps.add_template(template.path(), handle);
            }

            for handle in preloader.dependency_handles {
                proto_deps.add_dependency(handle);
            }

            let mut asset_deps = templates;
            for path in preloader.preload_paths {
                asset_deps.push(path);
            }

            let asset = LoadedAsset::new(proto).with_dependencies(asset_deps);
            load_context.set_default_asset(asset);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

/// Controls how assets are preloaded for [prototypical] objects.
///
/// [prototypical]: crate::prelude::Prototypical
pub struct AssetPreloader<'a, 'ctx> {
    load_context: &'a LoadContext<'ctx>,
    preload_paths: Vec<AssetPath<'static>>,
    dependency_handles: HashSet<HandleUntyped>,
}

impl<'a, 'ctx> AssetPreloader<'a, 'ctx> {
    fn new(load_context: &'a LoadContext<'ctx>) -> Self {
        Self {
            load_context,
            preload_paths: Vec::new(),
            dependency_handles: HashSet::new(),
        }
    }

    /// Preload an asset.
    ///
    /// The asset at the given path will be loaded alongside the [prototypical] object,
    /// forcing it to wait for the asset before it itself is marked as loaded.
    ///
    /// The returned strong handle is not stored as a dependency on the prototype, so
    /// the asset may be unloaded if the handle is dropped. Either store it on the component
    /// or use the [`preload_dependency`] method to add it as a dependency.
    ///
    /// To just get a strong handle without loading the asset, use the [`get_handle`]
    /// method.
    ///
    /// [prototypical]: crate::prelude::Prototypical
    /// [`preload_dependency`]: Self::preload_dependency
    /// [`get_handle`]: Self::get_handle
    pub fn preload<I: Into<AssetPath<'static>>, T: Asset>(&mut self, path: I) -> Handle<T> {
        let path = path.into();
        self.preload_paths.push(path.clone());
        self.load_context.get_handle(path)
    }

    /// Preload an asset and mark it as a dependency of the [prototypical] object.
    ///
    /// The asset at the given path will be loaded alongside the [prototypical] object,
    /// forcing it to wait for the asset before it itself is marked as loaded.
    ///
    /// A copy of the returned strong handle is stored as a dependency on the prototype,
    /// forcing it stay loaded as long as the prototype asset itself. To instead load
    /// the asset only and just use the handle to manage the asset lifetime manually,
    /// use the [`preload`] method.
    ///
    /// To just get a strong handle without loading the asset, use the [`get_handle`]
    /// method.
    ///
    /// [prototypical]: crate::prelude::Prototypical
    /// [`preload`]: Self::preload
    /// [`get_handle`]: Self::get_handle
    pub fn preload_dependency<I: Into<AssetPath<'static>>, T: Asset>(
        &mut self,
        path: I,
    ) -> Handle<T> {
        let path = path.into();
        self.preload_paths.push(path.clone());
        let handle: Handle<T> = self.load_context.get_handle(path);
        self.dependency_handles.insert(handle.clone_untyped());
        handle
    }

    /// Get a strong handle to the asset with the given path or ID.
    ///
    /// This will _not_ automatically load the asset— it will have to be done manually
    /// at another point in time. If the asset is meant to be loaded alongside or as a
    /// dependency of the [prototypical] object, consider using the [`preload`] or
    /// [`preload_dependency`] methods.
    ///
    /// [`preload`]: Self::preload
    /// [`preload_dependency`]: Self::preload_dependency
    /// [prototypical]: crate::prelude::Prototypical
    pub fn get_handle<I: Into<HandleId>, T: Asset>(&self, id: I) -> Handle<T> {
        self.load_context.get_handle(id)
    }
}
