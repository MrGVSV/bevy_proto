use bevy::app::App;
use bevy::transform::components::{GlobalTransform, Transform};

use crate::impls::macros::register_schematic;
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, Transform, GlobalTransform);
}

impl_external_schematic! {
    struct Transform {}
}

impl_external_schematic! {
    struct GlobalTransform {}
}
