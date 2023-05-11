use std::sync::Mutex;

use bevy::app::{App, Plugin};
use bevy_proto_backend::load::Loader;
use bevy_proto_backend::proto::Config;

use bevy_proto_backend::ProtoBackendPlugin;

use crate::config::ProtoConfig;
use crate::loader::ProtoLoader;
use crate::proto::Prototype;

/// Adds support for [`Prototype`] assets.
///
/// By default this sets up the plugin with the default [`ProtoLoader`] and [`ProtoConfig`].
/// However, this may be configured with user-defined [`Loaders`] and [`Configs`].
///
/// Note: If using a custom `Config` type, be sure to use that type as the generic
/// parameter for any system parameter from this crate that requires it
/// (e.g. [`Prototypes`], [`ProtoCommands`], etc.).
///
/// [`Loaders`]: Loader
/// [`Configs`]: Config
/// [`Prototypes`]: crate::prelude::Prototypes
/// [`ProtoCommands`]: crate::prelude::ProtoCommands
pub struct ProtoPlugin<L: Loader<Prototype> = ProtoLoader, C: Config<Prototype> = ProtoConfig> {
    loader: Mutex<Option<L>>,
    config: Mutex<Option<C>>,
}

impl ProtoPlugin {
    /// Create a new default plugin instance.
    pub fn new() -> Self {
        Self {
            loader: Mutex::new(None),
            config: Mutex::new(None),
        }
    }
}

impl<L: Loader<Prototype>> ProtoPlugin<L, ProtoConfig> {
    /// Create a new plugin instance with the given [`Loader`].
    pub fn new_with_loader(loader: L) -> Self {
        Self {
            loader: Mutex::new(Some(loader)),
            config: Mutex::new(None),
        }
    }

    /// Pass the given [`ProtoConfig`] instance to the plugin.
    pub fn with_config(mut self, config: ProtoConfig) -> Self {
        self.config = Mutex::new(Some(config));
        self
    }
}

impl<C: Config<Prototype>> ProtoPlugin<ProtoLoader, C> {
    /// Create a new plugin instance with the given [`Config`].
    ///
    /// Note: If using a custom `Config` type, be sure to use that type as the generic
    /// parameter for any system parameter from this crate that requires it
    /// (e.g. [`Prototypes`], [`ProtoCommands`], etc.).
    ///
    /// [`Prototypes`]: crate::prelude::Prototypes
    /// [`ProtoCommands`]: crate::prelude::ProtoCommands
    pub fn new_with_config(config: C) -> Self {
        Self {
            loader: Mutex::new(None),
            config: Mutex::new(Some(config)),
        }
    }
}

impl<L: Loader<Prototype>, C: Config<Prototype>> ProtoPlugin<L, C> {
    /// Create a new plugin instance with the given [`Loader`] and [`Config`].
    ///
    /// Note: If using a custom `Config` type, be sure to use that type as the generic
    /// parameter for any system parameter from this crate that requires it
    /// (e.g. [`Prototypes`], [`ProtoCommands`], etc.).
    ///
    /// [`Prototypes`]: crate::prelude::Prototypes
    /// [`ProtoCommands`]: crate::prelude::ProtoCommands
    pub fn new_with_loader_and_config(loader: L, config: C) -> Self {
        Self {
            loader: Mutex::new(Some(loader)),
            config: Mutex::new(Some(config)),
        }
    }
}

impl<L: Loader<Prototype>, C: Config<Prototype>> Plugin for ProtoPlugin<L, C> {
    fn build(&self, app: &mut App) {
        let mut plugin = ProtoBackendPlugin::<Prototype, L, C>::new();

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
