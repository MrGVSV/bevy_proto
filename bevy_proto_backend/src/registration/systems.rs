use bevy::asset::AssetEvent;
use bevy::prelude::{error, EventReader, EventWriter};

use crate::proto::{ProtoAssetEvent, Prototypical};
use crate::registration::ProtoManager;

/// Handles the registration of loaded, modified, and removed prototypes.
pub(crate) fn on_proto_asset_event<T: Prototypical>(
    mut events: EventReader<AssetEvent<T>>,
    mut manager: ProtoManager<T>,
    mut proto_events: EventWriter<ProtoAssetEvent<T>>,
) {
    for event in events.iter() {
        match event {
            AssetEvent::Created { handle } => match manager.register(handle) {
                Ok(proto) => proto_events.send(ProtoAssetEvent::Created {
                    id: proto.id().clone(),
                    handle: handle.clone_weak(),
                }),
                Err(err) => error!("could not register prototype: {}", err),
            },
            AssetEvent::Modified { handle } => match manager.register(handle) {
                Ok(proto) => proto_events.send(ProtoAssetEvent::Modified {
                    id: proto.id().clone(),
                    handle: handle.clone_weak(),
                }),
                Err(err) => error!("could not re-register modified prototype: {}", err),
            },
            AssetEvent::Removed { handle } => proto_events.send(ProtoAssetEvent::Removed {
                id: manager.unregister(handle),
                handle: handle.clone_weak(),
            }),
        }
    }
}
