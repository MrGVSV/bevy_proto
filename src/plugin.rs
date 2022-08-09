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
                #[cfg(feature = "hot_reloading")]
                hot_reload: false,
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
                #[cfg(feature = "hot_reloading")]
                hot_reload: false,
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
                #[cfg(feature = "hot_reloading")]
                hot_reload: false,
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
                #[cfg(feature = "hot_reloading")]
                hot_reload: false,
            }),
        }
    }
}

impl Plugin for ProtoPlugin {
    fn build(&self, app: &mut App) {
        if let Some(opts) = &self.options {
            // Insert custom prototype options
            app.insert_resource(opts.clone());
            #[cfg(feature = "hot_reloading")]
            if opts.hot_reload {
                app.add_startup_system(begin_watch);
                app.add_system(watch_for_changes);
            }
        } else {
            // Insert default options
            app.insert_resource(ProtoDataOptions {
                directories: vec![String::from("assets/prototypes")],
                recursive_loading: false,
                deserializer: Box::new(DefaultProtoDeserializer),
                extensions: Some(vec!["yaml", "json"]),
                #[cfg(feature = "hot_reloading")]
                hot_reload: false,
            });
        }

        // Initialize prototypes
        app.init_resource::<ProtoData>();
    }
}

fn begin_watch(
    mut data: bevy::prelude::ResMut<ProtoData>,
    opts: bevy::prelude::Res<ProtoDataOptions>,
) {
    data.watcher.watch(opts.directories[0].clone()).unwrap();
}

// Copied from bevy_asset's filesystem watching implementation:
// https://github.com/bevyengine/bevy/blob/main/crates/bevy_asset/src/io/file_asset_io.rs#L167-L199
fn watch_for_changes(
    mut proto_data: bevy::prelude::ResMut<ProtoData>,
    options: bevy::prelude::Res<ProtoDataOptions>,
) {
    let mut changed = bevy::utils::HashSet::default();
    loop {
        let event = match proto_data.watcher.receiver.try_recv() {
            Ok(result) => result.unwrap(),
            Err(crossbeam_channel::TryRecvError::Empty) => break,
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                panic!("FilesystemWatcher disconnected.")
            }
        };
        if let notify::event::Event {
            kind: notify::event::EventKind::Modify(_),
            paths,
            ..
        } = event
        {
            for path in &paths {
                if !changed.contains(path) {
                    if let Ok(data) = std::fs::read_to_string(path) {
                        if let Some(proto) = options.deserializer.deserialize(&data) {
                            proto_data
                                .prototypes
                                .insert(proto.name().to_string(), proto);
                        }
                    }
                }
            }
            changed.extend(paths);
        }
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
