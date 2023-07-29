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
    app.register_type::<BorderColorInput>()
        .register_type::<OverflowInput>()
        .register_type::<UiRectInput>();
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
    struct Button;
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
    enum FocusPolicy {}
}

impl_external_schematic! {
    enum Interaction {}
}

impl_external_schematic! {
    struct Label;
}

impl_external_schematic! {
    struct Node {}
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
        pub display: Display,
        pub position_type: PositionType,
        pub overflow: OverflowInput,
        pub direction: Direction,
        pub flex_direction: FlexDirection,
        pub flex_wrap: FlexWrap,
        pub align_items: AlignItems,
        pub align_self: AlignSelf,
        pub align_content: AlignContent,
        pub justify_content: JustifyContent,
        pub justify_self: JustifySelf,
        pub justify_items: JustifyItems,
        pub margin: UiRectInput,
        pub padding: UiRectInput,
        pub border: UiRectInput,
        pub flex_grow: f32,
        pub flex_shrink: f32,
        pub flex_basis: Val,
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
            display: value.display,
            position_type: value.position_type,
            overflow: value.overflow.into(),
            direction: value.direction,
            flex_direction: value.flex_direction,
            flex_wrap: value.flex_wrap,
            align_items: value.align_items,
            align_self: value.align_self,
            align_content: value.align_content,
            justify_content: value.justify_content,
            justify_self: value.justify_self,
            justify_items: value.justify_items,
            margin: value.margin.into(),
            padding: value.padding.into(),
            border: value.border.into(),
            flex_grow: value.flex_grow,
            flex_shrink: value.flex_shrink,
            flex_basis: value.flex_basis,
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
    pub struct UiRectInput {
        pub left: Val,
        pub right: Val,
        pub top: Val,
        pub bottom: Val,
    }
    from_to_default! {
        UiRect,
        UiRectInput,
        |value: Input| Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
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
    enum ZIndex {}
}
