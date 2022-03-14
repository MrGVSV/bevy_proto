//! Contains [`ProtoPlugin`].
use crate::config::{ProtoConfig, ProtoConfigArc};
use crate::loader::ProtoAssetLoader;
use crate::manager;
use crate::manager::ProtoManager;
use bevy::app::{App, Plugin};
use bevy::asset::{AddAsset, Asset};
use bevy::prelude::{ParallelSystemDescriptorCoercion, SystemLabel};
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
        app.insert_resource(self.config.clone())
            .init_asset_loader::<ProtoAssetLoader>()
            .add_asset::<T>()
            .init_resource::<ProtoManager<T>>()
            .add_system(manager::update_tracked::<T>.label(ProtoLabel::UpdateTracked))
            .add_system(manager::spawn_proto::<T>.after(ProtoLabel::UpdateTracked));
    }
}

#[derive(SystemLabel, Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum ProtoLabel {
    UpdateTracked,
}
