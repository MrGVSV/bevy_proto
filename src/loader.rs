use std::marker::PhantomData;

use anyhow::Error;
use bevy::asset::{Asset, AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset};
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
                load_templates(load_context, templates)
            } else {
                Vec::new()
            };

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
    templates: &'a TemplateList,
) -> Vec<AssetPath<'static>> {
    let path = load_context.path();
    let ext = path.extension();
    templates
        .iter_inheritance_order()
        .map(|template| {
            let template_path = path.join(template);
            if let Some(ext) = ext {
                if template_path.extension().is_some() {
                    AssetPath::new(template_path, None)
                } else {
                    AssetPath::new(template_path.join(ext), None)
                }
            } else {
                AssetPath::new(template_path, None)
            }
        })
        .collect()
}
