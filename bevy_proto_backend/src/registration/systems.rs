use bevy::asset::AssetEvent;
use bevy::prelude::{error, EventReader};

use crate::proto::Prototypical;
use crate::registration::ProtoManager;

/// Handles the registration of loaded, modified, and removed prototypes.
pub(crate) fn on_proto_asset_event<T: Prototypical>(
    mut events: EventReader<AssetEvent<T>>,
    mut manager: ProtoManager<T>,
) {
    for event in events.iter() {
        match event {
            AssetEvent::Created { handle } => match manager.register(handle) {
                Err(err) => error!("could not register prototype: {}", err),
                _ => {}
            },
            AssetEvent::Modified { handle } => match manager.reload(handle) {
                Err(err) => error!("could not reload modified prototype: {}", err),
                _ => {}
            },
            AssetEvent::Removed { handle } => {
                manager.unregister(handle);
            }
        }
    }
}
