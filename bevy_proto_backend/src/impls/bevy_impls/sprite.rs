use bevy::app::App;
use bevy::math::Vec2;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::sprite::{Anchor, Mesh2dHandle, Sprite, TextureAtlasSprite};

use crate::impls::macros::{from_to_default, register_schematic};
use crate::proto::{ProtoAsset, ProtoColor};
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, Anchor, Mesh2dHandle, Sprite, TextureAtlasSprite);
}

impl_external_schematic! {
    enum Anchor {}
}

impl_external_schematic! {
    #[schematic(input(vis = pub))]
    struct Mesh2dHandle(#[schematic(asset)] pub Handle<Mesh>);
    // ---
    impl Default for Mesh2dHandleInput {
        fn default() -> Self {
            let base = <Mesh2dHandle as Default>::default();
            Self(ProtoAsset::HandleId(base.0.id()))
        }
    }
}

impl_external_schematic! {
    struct Sprite {}
}

impl_external_schematic! {
    #[schematic(from = TextureAtlasSpriteInput)]
    struct TextureAtlasSprite {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct TextureAtlasSpriteInput {
        pub color: ProtoColor,
        pub index: usize,
        pub flip_x: bool,
        pub flip_y: bool,
        pub custom_size: Option<Vec2>,
        pub anchor: Anchor,
    }
    from_to_default! {
        TextureAtlasSprite,
        TextureAtlasSpriteInput,
        |value: Input| Self {
            color: value.color.into(),
            index: value.index,
            flip_x: value.flip_x,
            flip_y: value.flip_y,
            custom_size: value.custom_size,
            anchor: value.anchor,
        }
    }
}
