use bevy::app::App;
use bevy::gltf::GltfExtras;

use crate::impls::macros::register_schematic;
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, GltfExtras);
}

impl_external_schematic! {
    struct GltfExtras {}
}
