use std::sync::Mutex;

use bevy::app::{App, Plugin};
use bevy_proto_backend::load::Loader;

use bevy_proto_backend::ProtoBackendPlugin;

use crate::config::ProtoConfig;
use crate::loader::ProtoLoader;
use crate::proto::Prototype;

/// Adds support for [`Prototype`] assets.
pub struct ProtoPlugin<L: Loader<Prototype> = ProtoLoader> {
    config: Mutex<Option<ProtoConfig>>,
    loader: Mutex<Option<L>>,
}

impl ProtoPlugin {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(None),
            loader: Mutex::new(None),
        }
    }
}

impl<L: Loader<Prototype>> ProtoPlugin<L> {
    pub fn new_with_loader(loader: L) -> Self {
        Self {
            config: Mutex::new(None),
            loader: Mutex::new(Some(loader)),
        }
    }

    pub fn with_config(mut self, config: ProtoConfig) -> Self {
        self.config = Mutex::new(Some(config));
        self
    }
}

impl<L: Loader<Prototype>> Plugin for ProtoPlugin<L> {
    fn build(&self, app: &mut App) {
        let mut plugin = ProtoBackendPlugin::<Prototype, L, ProtoConfig>::new();

        if let Ok(Some(config)) = self.config.lock().map(|mut config| config.take()) {
            plugin = plugin.with_config(config);
        }

        if let Ok(Some(loader)) = self.loader.lock().map(|mut loader| loader.take()) {
            plugin = plugin.with_loader(loader);
        }

        app.add_plugin(plugin);

        #[cfg(feature = "custom_schematics")]
        crate::custom::register_custom_schematics(app);
    }
}

impl Default for ProtoPlugin {
    fn default() -> Self {
        Self::new()
    }
}
