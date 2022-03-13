use std::marker::PhantomData;
use std::path::Path;

use anyhow::Error;
use bevy::asset::{Asset, AssetLoader, AssetPath, BoxedFuture, Handle, LoadContext, LoadedAsset};
use bevy::prelude::{FromWorld, World};
use bevy::reflect::TypeRegistryArc;

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

            let templates = if let Some(templates) = proto.templates() {
                load_templates(load_context, self.extensions(), templates)
            } else {
                Vec::new()
            };

            if let Some(list) = proto.templates_mut() {
                list.set_handles(
                    templates
                        .iter()
                        .map(|template| {
                            let handle: Handle<T> = load_context.get_handle(template.clone());
                            handle.clone_untyped()
                        })
                        .collect::<Vec<_>>(),
                )
            }

            let asset = LoadedAsset::new(proto).with_dependencies(templates);
            load_context.set_default_asset(asset);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

fn load_templates<'a>(
    load_context: &'a LoadContext,
    extensions: &[&str],
    templates: &'a TemplateList,
) -> Vec<AssetPath<'static>> {
    let path = load_context.path();
    let path_str = path.to_str().unwrap_or_default();

    let ext = extensions
        .iter()
        .find(|ext| path_str.ends_with(**ext))
        .map(|ext| *ext);

    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    templates
        .iter_paths()
        .map(|template| {
            let template_path = parent.join(template);
            if let Some(ext) = ext {
                if template_path.extension().is_some() {
                    AssetPath::new(template_path, None)
                } else {
                    AssetPath::new(template_path.with_extension(ext), None)
                }
            } else {
                AssetPath::new(template_path, None)
            }
        })
        .collect()
}
