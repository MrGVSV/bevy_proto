use std::marker::PhantomData;

use bevy::app::{App, Plugin};
use bevy::asset::AddAsset;
use bevy::prelude::FromWorld;
use parking_lot::Mutex;

use crate::impls;
use crate::load::{Loader, ProtoAssetLoader};
use crate::proto::{Config, ProtoAsset, ProtoAssetEvent, ProtoStorage, Prototypical};
use crate::registration::{on_proto_asset_event, ProtoRegistry};
use crate::tree::{AccessOp, ChildAccess, EntityAccess, ProtoEntity};

/// Plugin to add support for the given [prototype] `P`.
///
/// [prototype]: Prototypical
pub struct ProtoBackendPlugin<T: Prototypical, L: Loader<T>, C: Config<T>> {
    config: Mutex<Option<C>>,
    loader: Mutex<Option<L>>,
    _phantom: PhantomData<T>,
}

impl<T: Prototypical, L: Loader<T>, C: Config<T>> ProtoBackendPlugin<T, L, C> {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(None),
            loader: Mutex::new(None),
            _phantom: Default::default(),
        }
    }

    /// Add a custom [`Config`] to the plugin.
    pub fn with_config(mut self, config: C) -> Self {
        self.config = Mutex::new(Some(config));
        self
    }

    /// Add a custom [`Loader`] to the plugin.
    pub fn with_loader(mut self, loader: L) -> Self {
        self.loader = Mutex::new(Some(loader));
        self
    }
}

impl<T: Prototypical, L: Loader<T>, C: Config<T>> Plugin for ProtoBackendPlugin<T, L, C> {
    fn build(&self, app: &mut App) {
        // === Types === //
        #[cfg(feature = "bevy_render")]
        app.register_type::<crate::proto::ProtoColor>();

        app.register_type::<ProtoAsset>()
            .register_type::<ProtoEntity>()
            .register_type::<EntityAccess>()
            .register_type::<AccessOp>()
            .register_type::<ChildAccess>();
        impls::register_impls(app);

        // === Resources === //
        if let Some(config) = self.config.lock().take() {
            app.insert_resource(config);
        } else {
            app.init_resource::<C>();
        }

        app.init_resource::<ProtoRegistry<T, C>>()
            .init_resource::<ProtoStorage<T>>();

        // === Assets === //
        let loader = self
            .loader
            .lock()
            .take()
            .unwrap_or_else(|| <L as FromWorld>::from_world(&mut app.world));
        let asset_loader = ProtoAssetLoader::<T, L, C>::new(loader, &mut app.world);

        app.add_asset_loader(asset_loader).add_asset::<T>();

        // === Events === //
        app.add_event::<ProtoAssetEvent<T>>();

        // === Systems === //
        app.add_system(on_proto_asset_event::<T, C>);
    }
}

impl<T: Prototypical, L: Loader<T>, C: Config<T>> Default for ProtoBackendPlugin<T, L, C> {
    fn default() -> Self {
        Self::new()
    }
}
