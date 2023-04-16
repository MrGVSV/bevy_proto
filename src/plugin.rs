use std::sync::Mutex;

use bevy::app::{App, Plugin};

use bevy_proto_backend::ProtoBackendPlugin;

use crate::config::ProtoConfig;
use crate::proto::Prototype;

/// Adds support for [`Prototype`] assets.
#[derive(Default)]
pub struct ProtoPlugin {
    config: Mutex<Option<ProtoConfig>>,
}

impl ProtoPlugin {
    pub fn new(config: ProtoConfig) -> Self {
        Self {
            config: Mutex::new(Some(config)),
        }
    }
}

impl Plugin for ProtoPlugin {
    fn build(&self, app: &mut App) {
        let mut plugin = ProtoBackendPlugin::<Prototype>::default();

        if let Ok(Some(config)) = self.config.lock().map(|mut config| config.take()) {
            plugin = plugin.with_config(config);
        }

        app.add_plugin(plugin);

        #[cfg(feature = "custom_schematics")]
        crate::custom::register_custom_schematics(app);
    }
}
