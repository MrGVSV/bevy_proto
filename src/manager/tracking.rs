use crate::manager::{HandleToName, NameToHandle};
use crate::utils::analyze_deps;
use crate::Prototypical;
use bevy::asset::{Asset, AssetEvent, Assets, Handle};
use bevy::prelude::{EventReader, Res, ResMut};

/// System for updating the tracked prototypes.
pub(crate) fn track_prototypes<T: Prototypical + Asset>(
    mut reader: EventReader<AssetEvent<T>>,
    assets: Res<Assets<T>>,
    handle_to_name: ResMut<HandleToName>,
    name_to_handle: ResMut<NameToHandle>,
) {
    for evt in reader.iter() {
        match evt {
            AssetEvent::Created { ref handle } => {
                if let Some(proto) = assets.get(handle) {
                    track(proto, handle, &handle_to_name, &name_to_handle);

                    #[cfg(feature = "analysis")]
                    analyze_deps(proto, &assets);
                }
                // TODO: Add "prune" feature to allow the removal of duplicates from lower priority templates
            }
            AssetEvent::Modified { ref handle } => {
                if let Some(proto) = assets.get(handle) {
                    track(proto, handle, &handle_to_name, &name_to_handle);
                }
            }
            AssetEvent::Removed { ref handle } => {
                if handle_to_name.read().contains_key(&handle.id) {
                    if let Some(name) = handle_to_name.write().remove(&handle.id) {
                        name_to_handle.write().remove(&name);
                    }
                }
            }
        }
    }
}

fn track<T: Prototypical + Asset>(
    proto: &T,
    handle: &Handle<T>,
    handle_to_name: &HandleToName,
    name_to_handle: &NameToHandle,
) {
    if !handle_to_name.read().contains_key(&handle.id) {
        handle_to_name
            .write()
            .insert(handle.id, proto.name().to_string());
    }
    if !name_to_handle.read().contains_key(proto.name()) {
        name_to_handle
            .write()
            .insert(proto.name().to_string(), handle.id);
    }
}
