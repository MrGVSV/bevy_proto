use bevy::prelude::{App, Plugin, Res, ResMut, Resource};
use crossbeam_channel::Receiver;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Result, Watcher};

use crate::prelude::{ProtoData, ProtoDataOptions};

// Copied from bevy_asset's implementation
// https://github.com/bevyengine/bevy/blob/main/crates/bevy_asset/src/filesystem_watcher.rs
#[derive(Resource)]
struct FilesystemWatcher {
    watcher: RecommendedWatcher,
    receiver: Receiver<Result<Event>>,
}

impl FilesystemWatcher {
    /// Watch for changes recursively at the provided path.
    fn watch<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<()> {
        self.watcher.watch(path.as_ref(), RecursiveMode::Recursive)
    }
}

impl Default for FilesystemWatcher {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let watcher: RecommendedWatcher = RecommendedWatcher::new(
            move |res| {
                sender.send(res).expect("Watch event send failure.");
            },
            Config::default(),
        )
        .expect("Failed to create filesystem watcher.");
        FilesystemWatcher { watcher, receiver }
    }
}

// Copied from bevy_asset's filesystem watching implementation:
// https://github.com/bevyengine/bevy/blob/main/crates/bevy_asset/src/io/file_asset_io.rs#L167-L199
fn watch_for_changes(
    watcher: Res<FilesystemWatcher>,
    mut proto_data: ResMut<ProtoData>,
    options: Res<ProtoDataOptions>,
) {
    let mut changed = bevy::utils::HashSet::default();
    loop {
        let event = match watcher.receiver.try_recv() {
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

pub(crate) struct HotReloadPlugin {
    pub(crate) path: String,
}

impl Plugin for HotReloadPlugin {
    fn build(&self, app: &mut App) {
        let mut watcher = FilesystemWatcher::default();
        watcher.watch(self.path.clone()).unwrap();

        app.insert_resource(watcher).add_system(watch_for_changes);
    }
}
