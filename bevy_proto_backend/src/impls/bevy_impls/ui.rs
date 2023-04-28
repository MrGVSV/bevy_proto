use bevy::app::App;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{BackgroundColor, Button, Color, Label};
use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};
use bevy::ui::{
    AlignContent, AlignItems, AlignSelf, CalculatedClip, CalculatedSize, Direction, Display,
    FlexDirection, FlexWrap, FocusPolicy, Interaction, JustifyContent, Node, Overflow,
    PositionType, RelativeCursorPosition, Size, Style, UiImage, UiRect, Val, ZIndex,
};

use crate::impls::macros::{from_to_default, register_schematic};
use crate::proto::ProtoAsset;
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(
        app,
        BackgroundColor,
        Button,
        CalculatedClip,
        CalculatedSize,
        FocusPolicy,
        Interaction,
        Label,
        Node,
        RelativeCursorPosition,
        Style,
        UiImage,
    );
}

impl_external_schematic! {
    #[schematic(from = BackgroundColorInput)]
    struct BackgroundColor();
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub struct BackgroundColorInput(pub Color);
    from_to_default! {
        BackgroundColor,
        BackgroundColorInput,
        |value: Input| Self(value.0)
    }
}

impl_external_schematic! {
    #[schematic(from = ButtonInput)]
    struct Button;
    // ---
    #[derive(Reflect, FromReflect)]
    pub struct ButtonInput;
    from_to_default!(
        Button,
        ButtonInput,
        |_: Input| Self
    );
}

impl_external_schematic! {
    #[schematic(from = CalculatedClipInput)]
    struct CalculatedClip {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub struct CalculatedClipInput {
        pub clip: Rect,
    }
    from_to_default! {
        CalculatedClip,
        CalculatedClipInput,
        |value: Input| Self {
            clip: value.clip,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = CalculatedSizeInput)]
    struct CalculatedSize {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub struct CalculatedSizeInput {
        pub size: Vec2,
        pub preserve_aspect_ratio: bool,
    }
    from_to_default! {
        CalculatedSize,
        CalculatedSizeInput,
        |value: Input| Self {
            size: value.size,
            preserve_aspect_ratio: value.preserve_aspect_ratio,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = FocusPolicyInput)]
    enum FocusPolicy {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum FocusPolicyInput {
        Block,
        Pass,
    }
    from_to_default! {
        FocusPolicy,
        FocusPolicyInput,
        |value: Input| match value {
            Input::Block => Self::Block,
            Input::Pass => Self::Pass,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = InteractionInput)]
    enum Interaction {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum InteractionInput {
        Clicked,
        Hovered,
        None,
    }
    from_to_default! {
        Interaction,
        InteractionInput,
        |value: Input| match value {
            Input::Clicked => Self::Clicked,
            Input::Hovered => Self::Hovered,
            Input::None => Self::None,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = LabelInput)]
    struct Label;
    // ---
    #[derive(Reflect, FromReflect)]
    pub struct LabelInput;
    impl From<LabelInput> for Label {
        fn from(_: LabelInput) -> Self {
            Self
        }
    }
}

impl_external_schematic! {
    #[schematic(from = NodeInput)]
    struct Node {}
    // ---
    #[derive(Reflect, FromReflect)]
    pub struct NodeInput;
    from_to_default!(
        Node,
        NodeInput,
        |_: Input| Self::default()
    );
}

impl_external_schematic! {
    #[schematic(from = RelativeCursorPositionInput)]
    struct RelativeCursorPosition {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub struct RelativeCursorPositionInput {
        pub normalized: Option<Vec2>,
    }
    from_to_default! {
        RelativeCursorPosition,
        RelativeCursorPositionInput,
        |value: Input| Self {
            normalized: value.normalized,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = StyleInput)]
    struct Style {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub struct StyleInput {
        pub display: DisplayInput,
        pub position_type: PositionTypeInput,
        pub direction: DirectionInput,
        pub flex_direction: FlexDirectionInput,
        pub flex_wrap: FlexWrapInput,
        pub align_items: AlignItemsInput,
        pub align_self: AlignSelfInput,
        pub align_content: AlignContentInput,
        pub justify_content: JustifyContentInput,
        pub position: UiRectInput,
        pub margin: UiRectInput,
        pub padding: UiRectInput,
        pub border: UiRectInput,
        pub flex_grow: f32,
        pub flex_shrink: f32,
        pub flex_basis: ValInput,
        pub size: SizeInput,
        pub min_size: SizeInput,
        pub max_size: SizeInput,
        pub aspect_ratio: Option<f32>,
        pub overflow: OverflowInput,
        pub gap: SizeInput,
    }
    from_to_default! {
        Style,
        StyleInput,
        |value: Input| Self {
            display: value.display.into(),
            position_type: value.position_type.into(),
            direction: value.direction.into(),
            flex_direction: value.flex_direction.into(),
            flex_wrap: value.flex_wrap.into(),
            align_items: value.align_items.into(),
            align_self: value.align_self.into(),
            align_content: value.align_content.into(),
            justify_content: value.justify_content.into(),
            position: value.position.into(),
            margin: value.margin.into(),
            padding: value.padding.into(),
            border: value.border.into(),
            flex_grow: value.flex_grow,
            flex_shrink: value.flex_shrink,
            flex_basis: value.flex_basis.into(),
            size: value.size.into(),
            min_size: value.min_size.into(),
            max_size: value.max_size.into(),
            aspect_ratio: value.aspect_ratio,
            overflow: value.overflow.into(),
            gap: value.gap.into(),
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum AlignContentInput {
        Start,
        End,
        FlexStart,
        FlexEnd,
        Center,
        Stretch,
        SpaceBetween,
        SpaceEvenly,
        SpaceAround,
    }
    from_to_default! {
        AlignContent,
        AlignContentInput,
        |value: Input| match value {
            Input::Start => Self::Start,
            Input::End => Self::End,
            Input::FlexStart => Self::FlexStart,
            Input::FlexEnd => Self::FlexEnd,
            Input::Center => Self::Center,
            Input::Stretch => Self::Stretch,
            Input::SpaceBetween => Self::SpaceBetween,
            Input::SpaceEvenly => Self::SpaceEvenly,
            Input::SpaceAround => Self::SpaceAround,
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum AlignItemsInput {
        Start,
        End,
        FlexStart,
        FlexEnd,
        Center,
        Baseline,
        Stretch,
    }
    from_to_default! {
        AlignItems,
        AlignItemsInput,
        |value: Input| match value {
            Input::Start => Self::Start,
            Input::End => Self::End,
            Input::FlexStart => Self::FlexStart,
            Input::FlexEnd => Self::FlexEnd,
            Input::Center => Self::Center,
            Input::Baseline => Self::Baseline,
            Input::Stretch => Self::Stretch,
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum AlignSelfInput {
        Auto,
        Start,
        End,
        FlexStart,
        FlexEnd,
        Center,
        Baseline,
        Stretch,
    }
    from_to_default! {
        AlignSelf,
        AlignSelfInput,
        |value: Input| match value {
            Input::Auto => Self::Auto,
            Input::Start => Self::Start,
            Input::End => Self::End,
            Input::FlexStart => Self::FlexStart,
            Input::FlexEnd => Self::FlexEnd,
            Input::Center => Self::Center,
            Input::Baseline => Self::Baseline,
            Input::Stretch => Self::Stretch,
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum DirectionInput {
        Inherit,
        LeftToRight,
        RightToLeft,
    }
    from_to_default! {
        Direction,
        DirectionInput,
        |value: Input| match value {
            Input::Inherit => Self::Inherit,
            Input::LeftToRight => Self::LeftToRight,
            Input::RightToLeft => Self::RightToLeft,
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum DisplayInput {
        None,
        Flex,
    }
    from_to_default! {
        Display,
        DisplayInput,
        |value: Input| match value {
            Input::None => Self::None,
            Input::Flex => Self::Flex,
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum FlexWrapInput {
        NoWrap,
        Wrap,
        WrapReverse,
    }
    from_to_default! {
        FlexWrap,
        FlexWrapInput,
        |value: Input| match value {
            Input::NoWrap => Self::NoWrap,
            Input::Wrap => Self::Wrap,
            Input::WrapReverse => Self::WrapReverse,
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum FlexDirectionInput {
        Row,
        Column,
        RowReverse,
        ColumnReverse,
    }
    from_to_default! {
        FlexDirection,
        FlexDirectionInput,
        |value: Input| match value {
            Input::Row => Self::Row,
            Input::Column => Self::Column,
            Input::RowReverse => Self::RowReverse,
            Input::ColumnReverse => Self::ColumnReverse,
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum JustifyContentInput {
        Start,
        End,
        FlexStart,
        FlexEnd,
        Center,
        SpaceBetween,
        SpaceAround,
        SpaceEvenly,
    }
    from_to_default! {
        JustifyContent,
        JustifyContentInput,
        |value: Input| match value {
            Input::Start => Self::Start,
            Input::End => Self::End,
            Input::FlexStart => Self::FlexStart,
            Input::FlexEnd => Self::FlexEnd,
            Input::Center => Self::Center,
            Input::SpaceBetween => Self::SpaceBetween,
            Input::SpaceEvenly => Self::SpaceEvenly,
            Input::SpaceAround => Self::SpaceAround,
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum OverflowInput {
        Visible,
        Hidden,
    }
    from_to_default! {
        Overflow,
        OverflowInput,
        |value: Input| match value {
            Input::Visible => Self::Visible,
            Input::Hidden => Self::Hidden,
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum PositionTypeInput {
        Relative,
        Absolute,
    }
    from_to_default! {
        PositionType,
        PositionTypeInput,
        |value: Input| match value {
            Input::Relative => Self::Relative,
            Input::Absolute => Self::Absolute,
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub struct SizeInput {
        pub width: ValInput,
        pub height: ValInput,
    }
    from_to_default! {
        Size,
        SizeInput,
        |value: Input| Self {
            width: value.width.into(),
            height: value.height.into(),
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub struct UiRectInput {
        pub left: ValInput,
        pub right: ValInput,
        pub top: ValInput,
        pub bottom: ValInput,
    }
    from_to_default! {
        UiRect,
        UiRectInput,
        |value: Input| Self {
            left: value.left.into(),
            right: value.right.into(),
            top: value.top.into(),
            bottom: value.bottom.into(),
        }
    }

    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum ValInput {
        Undefined,
        Auto,
        Px(f32),
        Percent(f32),
    }
    from_to_default! {
        Val,
        ValInput,
        |value: Input| match value {
            Input::Undefined => Self::Undefined,
            Input::Auto => Self::Auto,
            Input::Px(value) => Self::Px(value),
            Input::Percent(value) => Self::Percent(value),
        }
    }
}

impl_external_schematic! {
    #[schematic(input(vis = pub))]
    pub struct UiImage {
        #[schematic(asset(lazy))]
        pub texture: Handle<Image>,
        #[reflect(default)]
        pub flip_x: bool,
        #[reflect(default)]
        pub flip_y: bool,
    }

    impl Default for UiImageInput {
        fn default() -> Self {
            let base = UiImage::default();
            Self {
                texture: ProtoAsset::HandleId(base.texture.id()),
                flip_x: base.flip_x,
                flip_y: base.flip_y,
            }
        }
    }
}

impl_external_schematic! {
    #[schematic(from = ZIndexInput)]
    enum ZIndex {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum ZIndexInput {
        Local(i32),
        Global(i32),
    }
    from_to_default! {
        ZIndex,
        ZIndexInput,
        |value: Input| match value {
            Input::Local(z) => Self::Local(z),
            Input::Global(z) => Self::Global(z),
        }
    }
}
