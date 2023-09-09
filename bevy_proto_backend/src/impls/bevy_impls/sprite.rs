use bevy::app::App;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{Color, Image};
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::prelude::Mesh;
use bevy::sprite::{Anchor, ColorMaterial, Mesh2dHandle, Sprite, TextureAtlas, TextureAtlasSprite};
use bevy::utils::HashMap;

use crate::assets::{AssetSchematicAppExt, InlinableProtoAsset, ProtoAsset};
use crate::deps::DependenciesBuilder;
use crate::impls::macros::{from_to_default, register_schematic};
use crate::proto::ProtoColor;
use crate::schematics::{
    FromSchematicInput, FromSchematicPreloadInput, SchematicContext, SchematicId,
};
use bevy_proto_derive::{impl_external_asset_schematic, impl_external_schematic};

pub(super) fn register(app: &mut App) {
    register_schematic!(app, Anchor, Mesh2dHandle, Sprite, TextureAtlasSprite);

    app.register_asset_schematic::<ColorMaterial>()
        .register_asset_schematic::<TextureAtlas>();
}

impl_external_schematic! {
    enum Anchor {}
}

impl_external_schematic! {
    #[schematic(input(vis = pub))]
    struct Mesh2dHandle(#[schematic(asset(inline))] pub Handle<Mesh>);
    // ---
    #[allow(clippy::derivable_impls)]
    impl Default for Mesh2dHandleInput {
        fn default() -> Self {
            Self(InlinableProtoAsset::default())
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

// === Assets === //

impl_external_asset_schematic! {
    #[asset_schematic(from = ColorMaterialInput)]
    struct ColorMaterial {}
}

/// The schematic input type for [`ColorMaterial`].
#[derive(Reflect)]
#[reflect(Default)]
pub struct ColorMaterialInput {
    pub color: Color,
    pub texture: Option<ProtoAsset<Image>>,
}

impl FromSchematicInput<ColorMaterialInput> for ColorMaterial {
    fn from_input(
        input: ColorMaterialInput,
        id: SchematicId,
        context: &mut SchematicContext,
    ) -> Self {
        Self {
            color: input.color,
            texture: input.texture.map(|value| {
                FromSchematicInput::from_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x6df96d568e7642c8bc7b9274ef732591,
                    )),
                    context,
                )
            }),
        }
    }
}

impl FromSchematicPreloadInput<ColorMaterialInput> for ColorMaterial {
    fn from_preload_input(
        input: ColorMaterialInput,
        id: SchematicId,
        dependencies: &mut DependenciesBuilder,
    ) -> Self {
        Self {
            color: input.color,
            texture: input.texture.map(|value| {
                FromSchematicPreloadInput::from_preload_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x6df96d568e7642c8bc7b9274ef732591,
                    )),
                    dependencies,
                )
            }),
        }
    }
}

impl Default for ColorMaterialInput {
    fn default() -> Self {
        let base = ColorMaterial::default();
        Self {
            color: base.color,
            texture: base.texture.map(ProtoAsset::from),
        }
    }
}

impl_external_asset_schematic! {
    #[asset_schematic(from = TextureAtlasInput)]
    struct TextureAtlas {}
}

/// The schematic input type for [`TextureAtlas`].
#[derive(Reflect)]
pub struct TextureAtlasInput {
    pub texture: ProtoAsset<Image>,
    pub size: Vec2,
    pub textures: Vec<Rect>,
    pub texture_handles: Option<HashMap<ProtoAsset<Image>, usize>>,
}

impl FromSchematicInput<TextureAtlasInput> for TextureAtlas {
    fn from_input(
        input: TextureAtlasInput,
        id: SchematicId,
        context: &mut SchematicContext,
    ) -> Self {
        Self {
            texture: FromSchematicInput::from_input(
                input.texture,
                id.next(bevy::utils::Uuid::from_u128(
                    0x9d0bb563b0fe45d689404b63567c597b,
                )),
                context,
            ),
            size: input.size,
            textures: input.textures,
            texture_handles: input.texture_handles.map(|value| {
                value
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            FromSchematicInput::from_input(
                                key,
                                id.next(bevy::utils::Uuid::from_u128(
                                    0xccb7edfefe6e4a92ab42c7c7e87e3afa,
                                )),
                                context,
                            ),
                            value,
                        )
                    })
                    .collect()
            }),
        }
    }
}

impl FromSchematicPreloadInput<TextureAtlasInput> for TextureAtlas {
    fn from_preload_input(
        input: TextureAtlasInput,
        id: SchematicId,
        dependencies: &mut DependenciesBuilder,
    ) -> Self {
        Self {
            texture: FromSchematicPreloadInput::from_preload_input(
                input.texture,
                id.next(bevy::utils::Uuid::from_u128(
                    0x9d0bb563b0fe45d689404b63567c597b,
                )),
                dependencies,
            ),
            size: input.size,
            textures: input.textures,
            texture_handles: input.texture_handles.map(|value| {
                value
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            FromSchematicPreloadInput::from_preload_input(
                                key,
                                id.next(bevy::utils::Uuid::from_u128(
                                    0xccb7edfefe6e4a92ab42c7c7e87e3afa,
                                )),
                                dependencies,
                            ),
                            value,
                        )
                    })
                    .collect()
            }),
        }
    }
}
