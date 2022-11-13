//! Contains [`ProtoPlugin`].
use bevy::app::{App, Plugin};

use crate::{
    data::{ProtoData, ProtoDataOptions, ProtoDeserializer},
    prototype::{Prototype, Prototypical},
};

/// Inserts resources for loading prototypes.
#[derive(Default)]
pub struct ProtoPlugin {
    /// Optional plugin configuration.
    pub options: Option<ProtoDataOptions>,
}

impl ProtoPlugin {
    /// Creates a [`ProtoPlugin`], using the given path to find prototype files.
    /// See also: [`with_dir_recursive`][`ProtoPlugin::with_dir_recursive`].
    ///
    /// # Arguments
    ///
    /// * `dir`: The directory path, relative to the project root.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_proto::ProtoPlugin;
    ///
    /// let plugin = ProtoPlugin::with_dir("assets/config");
    /// ```
    pub fn with_dir(dir: &str) -> Self {
        Self {
            options: Some(ProtoDataOptions {
                directories: vec![dir.to_string()],
                recursive_loading: false,
                deserializer: Box::new(DefaultProtoDeserializer),
                extensions: Some(vec!["yaml", "json"]),
            }),
        }
    }

    /// Creates a [`ProtoPlugin`], using the given path to recursively find prototype files.
    /// See also: [`with_dir`][`ProtoPlugin::with_dir`].
    ///
    /// # Arguments
    ///
    /// * `dir`: The directory path, relative to the project root.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_proto::ProtoPlugin;
    ///
    /// let plugin = ProtoPlugin::with_dir_recursive("assets/config");
    /// ```
    pub fn with_dir_recursive(dir: &str) -> Self {
        Self {
            options: Some(ProtoDataOptions {
                directories: vec![dir.to_string()],
                recursive_loading: true,
                deserializer: Box::new(DefaultProtoDeserializer),
                extensions: Some(vec!["yaml", "json"]),
            }),
        }
    }

    /// Creates a [`ProtoPlugin`], using the given vec of paths to find prototype files.
    ///
    /// # Arguments
    ///
    /// * `dirs`: The directory paths, relative to the project root.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_proto::ProtoPlugin;
    ///
    /// let plugin = ProtoPlugin::with_dirs(vec![
    ///   String::from("assets/config"),
    ///   String::from("assets/mods"),
    /// ]);
    /// ```
    pub fn with_dirs(dirs: Vec<String>) -> Self {
        Self {
            options: Some(ProtoDataOptions {
                directories: dirs,
                recursive_loading: false,
                deserializer: Box::new(DefaultProtoDeserializer),
                extensions: Some(vec!["yaml", "json"]),
            }),
        }
    }

    /// Creates a [`ProtoPlugin`], using the given vec of dirs to recursively find prototype files.
    ///
    /// # Arguments
    ///
    /// * `dirs`: The directory paths, relative to the project root
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_proto::ProtoPlugin;
    ///
    /// let plugin = ProtoPlugin::with_dirs(vec![
    ///   String::from("assets/config"),
    ///   String::from("assets/mods"),
    /// ]);
    /// ```
    pub fn with_dirs_recursive(dirs: Vec<String>) -> Self {
        Self {
            options: Some(ProtoDataOptions {
                directories: dirs,
                recursive_loading: true,
                deserializer: Box::new(DefaultProtoDeserializer),
                extensions: Some(vec!["yaml", "json"]),
            }),
        }
    }
}

impl Plugin for ProtoPlugin {
    fn build(&self, app: &mut App) {
        let opts = self.options.clone();
        let opts = opts.unwrap_or(ProtoDataOptions {
            directories: vec![String::from("assets/prototypes")],
            recursive_loading: false,
            deserializer: Box::new(DefaultProtoDeserializer),
            extensions: Some(vec!["yaml", "json"]),
        });

        #[cfg(feature = "hot_reloading")]
        app.add_plugin(crate::hot_reload::HotReloadPlugin {
            path: opts.directories[0].clone(),
        });

        app.insert_resource(opts);
        // Initialize prototypes
        app.init_resource::<ProtoData>();
    }
}

#[derive(Clone)]
pub(crate) struct DefaultProtoDeserializer;

impl ProtoDeserializer for DefaultProtoDeserializer {
    fn deserialize(&self, data: &str) -> Option<Box<dyn Prototypical>> {
        if let Ok(value) = serde_yaml::from_str::<Prototype>(data) {
            Some(Box::new(value))
        } else {
            None
        }
    }
}
