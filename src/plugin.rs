//! Contains [`ProtoPlugin`].
use crate::config::{ProtoConfig, ProtoConfigArc};
use crate::loader::ProtoAssetLoader;
use crate::manager::ProtoManagerPlugin;
use bevy::app::{App, Plugin};
use bevy::asset::{AddAsset, Asset};
use std::marker::PhantomData;

use crate::prelude::{Prototype, Prototypical};
use crate::serde::ProtoDeserializable;

/// Inserts resources for loading prototypes.
pub struct ProtoPlugin<T: Prototypical + ProtoDeserializable + Asset = Prototype> {
    config: ProtoConfigArc,
    phantom: PhantomData<T>,
}

impl<T: Prototypical + ProtoDeserializable + Asset> Default for ProtoPlugin<T> {
    fn default() -> Self {
        Self {
            config: ProtoConfigArc::default(),
            phantom: PhantomData::default(),
        }
    }
}

impl<T: Prototypical + ProtoDeserializable + Asset> ProtoPlugin<T> {
    /// Create a new [`ProtoPlugin`] with the given configuration.
    pub fn with_config(config: ProtoConfig) -> Self {
        Self {
            config: ProtoConfigArc::new(config),
            phantom: PhantomData::default(),
        }
    }
}

impl<T: Prototypical + ProtoDeserializable + Asset> Plugin for ProtoPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(ProtoManagerPlugin::<T>::default())
            .insert_resource(self.config.clone())
            .init_asset_loader::<ProtoAssetLoader>()
            .add_asset::<T>();
    }
}
