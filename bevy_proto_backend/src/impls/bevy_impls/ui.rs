use bevy::app::App;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{BackgroundColor, Button, Label};
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::ui::widget::TextFlags;
use bevy::ui::{
    AlignContent, AlignItems, AlignSelf, BorderColor, CalculatedClip, ContentSize, Direction,
    Display, FlexDirection, FlexWrap, FocusPolicy, GridAutoFlow, GridPlacement, GridTrack,
    Interaction, JustifyContent, JustifyItems, JustifySelf, Node, Overflow, OverflowAxis,
    PositionType, RelativeCursorPosition, RepeatedGridTrack, Style, UiImage, UiRect, Val, ZIndex,
};

use crate::impls::macros::{from_to_default, register_schematic};
use crate::proto::{ProtoAsset, ProtoColor};
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(
        app,
        BackgroundColor,
        Button,
        CalculatedClip,
        FocusPolicy,
        Interaction,
        Label,
        Node,
        RelativeCursorPosition,
        Style,
        TextFlags,
        UiImage,
    );

    // Can be removed if https://github.com/bevyengine/bevy/pull/5781 is ever merged
    app.register_type::<AlignContentInput>()
        .register_type::<AlignItemsInput>()
        .register_type::<AlignSelfInput>()
        .register_type::<BorderColorInput>()
        .register_type::<DirectionInput>()
        .register_type::<DisplayInput>()
        .register_type::<FlexDirectionInput>()
        .register_type::<FlexWrapInput>()
        .register_type::<JustifyContentInput>()
        .register_type::<OverflowInput>()
        .register_type::<PositionTypeInput>()
        .register_type::<UiRectInput>()
        .register_type::<ValInput>();
}

impl_external_schematic! {
    #[schematic(from = BackgroundColorInput)]
    struct BackgroundColor();
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct BackgroundColorInput(pub ProtoColor);
    from_to_default! {
        BackgroundColor,
        BackgroundColorInput,
        |value: Input| Self(value.0.into())
    }
}

impl_external_schematic! {
    #[schematic(from = BorderColorInput)]
    struct BorderColor();
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct BorderColorInput(pub ProtoColor);
    from_to_default! {
        BorderColor,
        BorderColorInput,
        |value: Input| Self(value.0.into())
    }
}

impl_external_schematic! {
    #[schematic(from = ButtonInput)]
    struct Button;
    // ---
    #[derive(Reflect)]
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
    #[derive(Reflect)]
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
    struct ContentSize {}
}

impl_external_schematic! {
    #[schematic(from = FocusPolicyInput)]
    enum FocusPolicy {}
    // ---
    #[derive(Reflect)]
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
    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum InteractionInput {
        Pressed,
        Hovered,
        None,
    }
    from_to_default! {
        Interaction,
        InteractionInput,
        |value: Input| match value {
            Input::Pressed => Self::Pressed,
            Input::Hovered => Self::Hovered,
            Input::None => Self::None,
        }
    }
}

impl_external_schematic! {
    #[schematic(from = LabelInput)]
    struct Label;
    // ---
    #[derive(Reflect)]
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
    #[derive(Reflect)]
    pub struct NodeInput;

    impl From<Node> for NodeInput {
        fn from(_: Node) -> Self {
            Self
        }
    }

    impl From<NodeInput> for Node {
        fn from(_: NodeInput) -> Self {
            Self::default()
        }
    }

    impl Default for NodeInput {
        fn default() -> Self {
            Self
        }
    }
}

impl_external_schematic! {
    #[schematic(from = RelativeCursorPositionInput)]
    struct RelativeCursorPosition {}
    // ---
    #[derive(Reflect)]
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
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct StyleInput {
        pub display: DisplayInput,
        pub position_type: PositionTypeInput,
        pub overflow: OverflowInput,
        pub direction: DirectionInput,
        pub flex_direction: FlexDirectionInput,
        pub flex_wrap: FlexWrapInput,
        pub align_items: AlignItemsInput,
        pub align_self: AlignSelfInput,
        pub align_content: AlignContentInput,
        pub justify_content: JustifyContentInput,
        pub justify_self: JustifySelf,
        pub justify_items: JustifyItems,
        pub margin: UiRectInput,
        pub padding: UiRectInput,
        pub border: UiRectInput,
        pub flex_grow: f32,
        pub flex_shrink: f32,
        pub flex_basis: ValInput,
        pub aspect_ratio: Option<f32>,
        pub left: Val,
        pub right: Val,
        pub top: Val,
        pub bottom: Val,
        pub width: Val,
        pub min_width: Val,
        pub max_width: Val,
        pub height: Val,
        pub min_height: Val,
        pub max_height: Val,
        pub row_gap: Val,
        pub column_gap: Val,
        pub grid_auto_flow: GridAutoFlow,
        pub grid_template_rows: Vec<RepeatedGridTrack>,
        pub grid_template_columns: Vec<RepeatedGridTrack>,
        pub grid_auto_rows: Vec<GridTrack>,
        pub grid_auto_columns: Vec<GridTrack>,
        pub grid_row: GridPlacement,
        pub grid_column: GridPlacement,
    }
    from_to_default! {
        Style,
        StyleInput,
        |value: Input| Self {
            display: value.display.into(),
            position_type: value.position_type.into(),
            overflow: value.overflow.into(),
            direction: value.direction.into(),
            flex_direction: value.flex_direction.into(),
            flex_wrap: value.flex_wrap.into(),
            align_items: value.align_items.into(),
            align_self: value.align_self.into(),
            align_content: value.align_content.into(),
            justify_content: value.justify_content.into(),
            justify_self: value.justify_self,
            justify_items: value.justify_items,
            margin: value.margin.into(),
            padding: value.padding.into(),
            border: value.border.into(),
            flex_grow: value.flex_grow,
            flex_shrink: value.flex_shrink,
            flex_basis: value.flex_basis.into(),
            aspect_ratio: value.aspect_ratio,
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
            width: value.width,
            min_width: value.min_width,
            max_width: value.max_width,
            height: value.height,
            min_height: value.min_height,
            max_height: value.max_height,
            row_gap: value.row_gap,
            column_gap: value.column_gap,
            grid_auto_flow: value.grid_auto_flow,
            grid_template_rows: value.grid_template_rows,
            grid_template_columns: value.grid_template_columns,
            grid_auto_rows: value.grid_auto_rows,
            grid_auto_columns: value.grid_auto_columns,
            grid_row: value.grid_row,
            grid_column: value.grid_column,
        }
    }

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum AlignContentInput {
        Default,
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
            Input::Default => Self::Default,
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

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum AlignItemsInput {
        Default,
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
            Input::Default => Self::Default,
            Input::Start => Self::Start,
            Input::End => Self::End,
            Input::FlexStart => Self::FlexStart,
            Input::FlexEnd => Self::FlexEnd,
            Input::Center => Self::Center,
            Input::Baseline => Self::Baseline,
            Input::Stretch => Self::Stretch,
        }
    }

    #[derive(Reflect)]
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

    #[derive(Reflect)]
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

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum DisplayInput {
        Flex,
        Grid,
        None,
    }
    from_to_default! {
        Display,
        DisplayInput,
        |value: Input| match value {
            Input::Flex => Self::Flex,
            Input::Grid => Self::Grid,
            Input::None => Self::None,
        }
    }

    #[derive(Reflect)]
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

    #[derive(Reflect)]
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

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum JustifyContentInput {
        Default,
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
            Input::Default => Self::Default,
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

    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct OverflowInput {
        pub x: OverflowAxis,
        pub y: OverflowAxis,
    }
    from_to_default! {
        Overflow,
        OverflowInput,
        |value: Input| Self {
            x: value.x,
            y: value.y,
        }
    }

    #[derive(Reflect)]
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


    #[derive(Reflect)]
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

    #[derive(Reflect)]
    #[reflect(Default)]
    pub enum ValInput {
        Auto,
        Px(f32),
        Percent(f32),
        Vw(f32),
        Vh(f32),
        VMin(f32),
        VMax(f32),
    }
    from_to_default! {
        Val,
        ValInput,
        |value: Input| match value {
            Input::Auto => Self::Auto,
            Input::Px(value) => Self::Px(value),
            Input::Percent(value) => Self::Percent(value),
            Input::Vw(value) => Self::Vw(value),
            Input::Vh(value) => Self::Vh(value),
            Input::VMin(value) => Self::VMin(value),
            Input::VMax(value) => Self::VMax(value),
        }
    }
}

impl_external_schematic! {
    pub struct TextFlags {}
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
    #[derive(Reflect)]
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
