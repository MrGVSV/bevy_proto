use bevy::app::App;
use bevy::math::Vec2;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::text::{BreakLineOn, Text, Text2dBounds, TextAlignment, TextSection, TextStyle};

use crate::impls::macros::{from_to, from_to_default, from_to_input, register_schematic};
use crate::proto::{ProtoAsset, ProtoColor};
use crate::schematics::{FromSchematicInput, SchematicContext};
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, Text, Text2dBounds);

    // Can be removed if https://github.com/bevyengine/bevy/pull/5781 is ever merged
    app.register_type::<BreakLineOnInput>()
        .register_type::<TextAlignmentInput>()
        .register_type::<TextSectionInput>()
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
        pub alignment: TextAlignmentInput,
        pub linebreak_behaviour: BreakLineOnInput,
    }
    from_to_input! {
        Text,
        TextInput,
        move |input: Input, context: &mut SchematicContext| {
            let mut sections = Vec::with_capacity(input.sections.len());
            for section in input.sections {
                sections.push(FromSchematicInput::from_input(section, &mut *context));
            }

            Self {
                sections,
                alignment: input.alignment.into(),
                linebreak_behaviour: input.linebreak_behaviour.into(),
            }
        }
    }
    impl Default for TextInput {
        fn default() -> Self {
            let base = Text::default();
            Self {
                sections: Vec::new(),
                alignment: base.alignment.into(),
                linebreak_behaviour: base.linebreak_behaviour.into(),
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
        |input: Input, context| Self {
            value: input.value,
            style: FromSchematicInput::from_input(input.style, context)
        }
    }

    #[derive(Reflect)]
    pub struct TextStyleInput {
        pub font: ProtoAsset,
        pub font_size: f32,
        pub color: ProtoColor,
    }
    from_to_input! {
        TextStyle,
        TextStyleInput,
        |input: Input, context| Self {
            font: FromSchematicInput::from_input(input.font, context),
            font_size: input.font_size,
            color: input.color.into(),
        }
    }

    #[derive(Reflect)]
    pub enum TextAlignmentInput {
        Left,
        Center,
        Right,
    }
    from_to! {
        TextAlignment,
        TextAlignmentInput,
        |value: Input| match value {
            Input::Left => Self::Left,
            Input::Center => Self::Center,
            Input::Right => Self::Right,
        }
    }

    #[derive(Reflect)]
    pub enum BreakLineOnInput {
        WordBoundary,
        AnyCharacter,
    }
    from_to! {
        BreakLineOn,
        BreakLineOnInput,
        |value: Input| match value {
            Input::WordBoundary => Self::WordBoundary,
            Input::AnyCharacter => Self::AnyCharacter,
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
