use std::marker::PhantomData;

use bevy::app::{App, Plugin};
use bevy::asset::AddAsset;
use parking_lot::Mutex;

use crate::impls;
use crate::load::ProtoLoader;
use crate::proto::{ProtoAsset, ProtoAssetEvent, ProtoStorage, Prototypical};
use crate::registration::{on_proto_asset_event, ProtoRegistry};
use crate::tree::{AccessOp, ChildAccess, EntityAccess, ProtoEntity};

/// Plugin to add support for the given [prototype] `T`.
///
/// [prototype]: Prototypical
pub struct ProtoBackendPlugin<T: Prototypical> {
    config: Mutex<Option<T::Config>>,
    _phantom: PhantomData<T>,
}

impl<T: Prototypical> ProtoBackendPlugin<T> {
    pub fn with_config(mut self, config: T::Config) -> Self {
        self.config = Mutex::new(Some(config));
        self
    }
}

impl<T: Prototypical> Default for ProtoBackendPlugin<T> {
    fn default() -> Self {
        Self {
            config: Mutex::new(None),
            _phantom: Default::default(),
        }
    }
}

impl<T: Prototypical> Plugin for ProtoBackendPlugin<T> {
    fn build(&self, app: &mut App) {
        if let Some(config) = self.config.try_lock().and_then(|mut config| config.take()) {
            app.insert_resource(config);
        } else {
            app.init_resource::<T::Config>();
        }

        app.register_type::<ProtoAsset>()
            .register_type::<ProtoEntity>()
            .register_type::<EntityAccess>()
            .register_type::<AccessOp>()
            .register_type::<ChildAccess>()
            .init_resource::<ProtoRegistry<T>>()
            .init_resource::<ProtoStorage<T>>()
            .init_asset_loader::<ProtoLoader<T>>()
            .add_asset::<T>()
            .add_event::<ProtoAssetEvent<T>>()
            .add_system(on_proto_asset_event::<T>);

        impls::register_impls(app);
    }
}
