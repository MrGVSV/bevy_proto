use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::Font;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::sprite::TextureAtlas;
use bevy::text::{
    BreakLineOn, GlyphAtlasInfo, PositionedGlyph, Text, Text2dBounds, TextAlignment,
    TextLayoutInfo, TextSection, TextStyle,
};

use crate::impls::macros::{from_to_default, from_to_input, register_schematic};
use crate::proto::ProtoColor;
use crate::schematics::{FromSchematicInput, SchematicContext, SchematicId};

use crate::assets::ProtoAsset;
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, Text, Text2dBounds);

    // Can be removed if https://github.com/bevyengine/bevy/pull/5781 is ever merged
    app.register_type::<TextSectionInput>()
        .register_type::<Vec<TextSectionInput>>()
        .register_type::<TextStyleInput>();
}

impl_external_schematic! {
    #[schematic(from = TextInput)]
    struct Text {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct TextInput {
        pub sections: Vec<TextSectionInput>,
        pub alignment: TextAlignment,
        pub linebreak_behavior: BreakLineOn,
    }
    from_to_input! {
        Text,
        TextInput,
        move |input: Input, id: SchematicId, context: &mut SchematicContext| {
            let mut sections = Vec::with_capacity(input.sections.len());
            for section in input.sections {
                sections.push(FromSchematicInput::from_input(
                    section,
                    id.next(bevy::utils::Uuid::from_u128(0x529d5df317584b1dbede8eb45d2e2144)),
                    &mut *context
                ));
            }

            Self {
                sections,
                alignment: input.alignment,
                linebreak_behavior: input.linebreak_behavior,
            }
        }
    }
    impl Default for TextInput {
        fn default() -> Self {
            let base = Text::default();
            Self {
                sections: Vec::new(),
                alignment: base.alignment,
                linebreak_behavior: base.linebreak_behavior,
            }
        }
    }

    #[derive(Reflect)]
    pub struct TextSectionInput {
        pub value: String,
        pub style: TextStyleInput,
    }
    from_to_input! {
        TextSection,
        TextSectionInput,
        |input: Input, id: SchematicId, context| Self {
            value: input.value,
            style: FromSchematicInput::from_input(
                input.style,
                id.next(bevy::utils::Uuid::from_u128(0x1526927b34b34337b4141868f98d52cd)),
                context
            )
        }
    }

    #[derive(Reflect)]
    pub struct TextStyleInput {
        pub font: ProtoAsset<Font>,
        pub font_size: f32,
        pub color: ProtoColor,
    }
    from_to_input! {
        TextStyle,
        TextStyleInput,
        |input: Input, id: SchematicId, context| Self {
            font: FromSchematicInput::from_input(
                input.font,
                id.next(bevy::utils::Uuid::from_u128(0xeae7ac5b229c4f58883dfc11bb02031b)),
                context
            ),
            font_size: input.font_size,
            color: input.color.into(),
        }
    }
}

impl_external_schematic! {
    #[schematic(from = Text2dBoundsInput)]
    struct Text2dBounds {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct Text2dBoundsInput {
        pub size: Vec2,
    }
    from_to_default! {
        Text2dBounds,
        Text2dBoundsInput,
        |value: Input| Self {
            size: value.size,
        }
    }
}

#[derive(Reflect, Default)]
#[reflect(Default)]
pub struct TextLayoutInfoInput {
    pub glyphs: Vec<PositionedGlyphInput>,
    pub size: Vec2,
}

impl FromSchematicInput<TextLayoutInfoInput> for TextLayoutInfo {
    fn from_input(
        input: TextLayoutInfoInput,
        id: SchematicId,
        context: &mut SchematicContext,
    ) -> Self {
        Self {
            glyphs: input
                .glyphs
                .into_iter()
                .enumerate()
                .map(|(index, glyph)| {
                    FromSchematicInput::from_input(glyph, id.next(index), context)
                })
                .collect(),
            size: input.size,
        }
    }
}

#[derive(Reflect)]
pub struct PositionedGlyphInput {
    pub position: Vec2,
    pub size: Vec2,
    pub atlas_info: GlyphAtlasInfoInput,
    pub section_index: usize,
    pub byte_index: usize,
}

impl FromSchematicInput<PositionedGlyphInput> for PositionedGlyph {
    fn from_input(
        input: PositionedGlyphInput,
        id: SchematicId,
        context: &mut SchematicContext,
    ) -> Self {
        Self {
            position: input.position,
            size: input.size,
            atlas_info: FromSchematicInput::from_input(input.atlas_info, id, context),
            section_index: input.section_index,
            byte_index: input.byte_index,
        }
    }
}

#[derive(Reflect)]
pub struct GlyphAtlasInfoInput {
    pub texture_atlas: ProtoAsset<TextureAtlas>,
    pub glyph_index: usize,
}

impl FromSchematicInput<GlyphAtlasInfoInput> for GlyphAtlasInfo {
    fn from_input(
        input: GlyphAtlasInfoInput,
        id: SchematicId,
        context: &mut SchematicContext,
    ) -> Self {
        Self {
            texture_atlas: FromSchematicInput::from_input(input.texture_atlas, id, context),
            glyph_index: input.glyph_index,
        }
    }
}
