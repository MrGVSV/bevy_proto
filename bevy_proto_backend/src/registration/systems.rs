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
            AssetEvent::Created { handle } => {
                if let Err(err) = manager.register(handle) {
                    error!("could not register prototype: {}", err);
                }
            }
            AssetEvent::Modified { handle } => {
                if let Err(err) = manager.reload(handle) {
                    error!("could not reload modified prototype: {}", err);
                }
            }
            AssetEvent::Removed { handle } => {
                manager.unregister(handle);
            }
        }
    }
}
