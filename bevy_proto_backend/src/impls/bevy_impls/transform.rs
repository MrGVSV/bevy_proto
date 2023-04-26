use bevy::app::App;
use bevy::transform::components::{GlobalTransform, Transform};

use crate::impls::macros::register_schematic;
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, Transform, GlobalTransform);
}

// FIXME: `TransformBundle` does not impl `Reflect`
// impl_external_schematic! {
//     #[schematic(from = TransformBundleInput)]
//     struct TransformBundle {}
//     // ---
//     #[derive(Reflect, FromReflect)]
//     pub struct TransformBundleInput {
//         local: Transform,
//         global: GlobalTransform
//     }
//     impl From<TransformBundleInput> for TransformBundle {
//         fn from(value: TransformBundleInput) -> Self {
//             Self {
//                 local: value.local,
//                 global: value.global
//             }
//         }
//     }
// }

impl_external_schematic! {
    struct Transform {}
}

impl_external_schematic! {
    struct GlobalTransform {}
}
