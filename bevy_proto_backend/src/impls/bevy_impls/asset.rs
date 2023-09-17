use bevy::app::App;
use bevy::asset::{Asset, Handle};

use crate::impls::macros::register_schematic;
use bevy_proto_derive::impl_external_schematic;

use crate::assets::ProtoAsset;

#[allow(unused_variables)]
pub(super) fn register(app: &mut App) {
    #[cfg(feature = "bevy_animation")]
    register_schematic!(app, Handle<bevy::prelude::AnimationClip>);

    #[cfg(feature = "bevy_audio")]
    register_schematic!(app, Handle<bevy::prelude::AudioSource>,);

    #[cfg(feature = "bevy_gltf")]
    register_schematic!(
        app,
        Handle<bevy::gltf::Gltf>,
        Handle<bevy::gltf::GltfMesh>,
        Handle<bevy::gltf::GltfPrimitive>,
        Handle<bevy::gltf::GltfNode>,
    );

    #[cfg(feature = "bevy_pbr")]
    register_schematic!(app, Handle<bevy::prelude::StandardMaterial>);

    #[cfg(feature = "bevy_render")]
    register_schematic!(
        app,
        Handle<bevy::prelude::Image>,
        Handle<bevy::render::mesh::skinning::SkinnedMeshInverseBindposes>,
        Handle<bevy::prelude::Mesh>,
        Handle<bevy::prelude::Shader>,
    );

    #[cfg(feature = "bevy_scene")]
    register_schematic!(
        app,
        Handle<bevy::prelude::DynamicScene>,
        Handle<bevy::prelude::Scene>,
    );

    #[cfg(feature = "bevy_sprite")]
    register_schematic!(
        app,
        Handle<bevy::prelude::ColorMaterial>,
        Handle<bevy::prelude::TextureAtlas>,
    );

    #[cfg(feature = "bevy_text")]
    register_schematic!(
        app,
        Handle<bevy::prelude::Font>,
        Handle<bevy::text::FontAtlasSet>,
    );
}

impl_external_schematic! {
    #[schematic(from = ProtoAsset<T>)]
    struct Handle<T: Asset> {}
}
