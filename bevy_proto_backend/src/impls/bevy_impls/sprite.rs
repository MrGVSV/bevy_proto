use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::Color;
use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};
use bevy::sprite::{Anchor, Mesh2dHandle, Sprite, TextureAtlasSprite};

use crate::impls::macros::{from_to_default, register_schematic};
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, Anchor, Mesh2dHandle, Sprite, TextureAtlasSprite);
}

impl_external_schematic! {
    enum Anchor {}
}

impl_external_schematic! {
    struct Mesh2dHandle(#[schematic(asset)] pub Handle<Mesh>);
}

impl_external_schematic! {
    struct Sprite {}
}

impl_external_schematic! {
    #[schematic(from = TextureAtlasSpriteInput)]
    struct TextureAtlasSprite {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub struct TextureAtlasSpriteInput {
        pub color: Color,
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
            color: value.color,
            index: value.index,
            flip_x: value.flip_x,
            flip_y: value.flip_y,
            custom_size: value.custom_size,
            anchor: value.anchor,
        }
    }
}
