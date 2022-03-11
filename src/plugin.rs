//! Contains [`ProtoPlugin`].
use crate::config::{ProtoConfig, ProtoConfigArc};
use crate::loader::ProtoAssetLoader;
use bevy::app::{App, Plugin};
use bevy::asset::{AddAsset, Asset};
use std::marker::PhantomData;

use crate::prelude::Prototype;
use crate::prototype::Prototypical;
use crate::serde::ProtoDeserializable;

/// Inserts resources for loading prototypes.
#[derive(Default)]
pub struct ProtoPlugin<T: Prototypical + ProtoDeserializable + Asset = Prototype> {
    config: ProtoConfigArc,
    phantom: PhantomData<T>,
}

impl<T: Prototypical + ProtoDeserializable + Asset> ProtoPlugin<T> {
    pub fn with_config(config: ProtoConfig) -> Self {
        Self {
            config: ProtoConfigArc::new(config),
            phantom: PhantomData::default(),
        }
    }
}

impl<T: Prototypical + ProtoDeserializable + Asset> Plugin for ProtoPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone())
            .init_asset_loader::<ProtoAssetLoader>()
            .add_asset::<T>();
    }
}
