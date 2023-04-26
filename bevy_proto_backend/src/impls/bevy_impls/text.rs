use bevy::app::App;
use bevy::math::Vec2;
use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};
use bevy::text::{BreakLineOn, Text, Text2dBounds, TextAlignment, TextSection};

use crate::impls::macros::{from_to, from_to_default, register_schematic};
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, Text, Text2dBounds);
}

impl_external_schematic! {
    #[schematic(from = TextInput)]
    struct Text {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub struct TextInput {
        pub sections: Vec<TextSection>,
        pub alignment: TextAlignmentInput,
        pub linebreak_behaviour: BreakLineOnInput,
    }
    from_to_default! {
        Text,
        TextInput,
        |value: Input| Self {
            sections: value.sections,
            alignment: value.alignment.into(),
            linebreak_behaviour: value.linebreak_behaviour.into(),
        }
    }

    #[derive(Reflect, FromReflect)]
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

    #[derive(Reflect, FromReflect)]
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
    #[derive(Reflect, FromReflect)]
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
