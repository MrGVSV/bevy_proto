use bevy::app::App;
use bevy::gltf::GltfExtras;
use bevy::reflect::{FromReflect, Reflect};

use crate::impls::macros::{from_to_default, register_schematic};
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, GltfExtras);
}

impl_external_schematic! {
    #[schematic(from = GltfExtrasInput)]
    struct GltfExtras {}
    // ---
    #[derive(Reflect, FromReflect)]
    pub struct GltfExtrasInput{
        pub value: String,
    }
    from_to_default!(
        GltfExtras,
        GltfExtrasInput,
        |value: Input| Self {
            value: value.value,
        }
    );
}
