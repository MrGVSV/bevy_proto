//! A collection of custom schematics (requires the `custom_schematics` feature).
//!
//! The types in this module are meant to provide [`Schematic`] implementations of
//! common types.
//! For example, many bundles in Bevy do not meet the requirements to implement
//! `Schematic` themselves, so they have equivalent types defined here as a stopgap.

use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{Component, GlobalTransform, Transform};
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::ui::widget::UiImageSize;
use bevy_proto_backend::assets::{AssetSchematic, InlinableProtoAsset};
use bevy_proto_backend::impls::bevy_impls;
use bevy_proto_backend::proto::ProtoColor;
use bevy_proto_backend::{from, from_to_default, register_schematic};

use bevy_proto_backend::schematics::{
    FromSchematicInput, ReflectSchematic, Schematic, SchematicContext, SchematicId,
};

pub(crate) fn register_custom_schematics(app: &mut App) {
    register_schematic!(app, TransformBundle);
    #[cfg(feature = "bevy_core_pipeline")]
    register_schematic!(app, Camera2dBundle, Camera3dBundle);
    #[cfg(feature = "bevy_pbr")]
    register_schematic!(
        app,
        DirectionalLightBundle,
        PointLightBundle,
        SpotLightBundle,
        MaterialMeshBundle<bevy::pbr::StandardMaterial>
    );
    #[cfg(feature = "bevy_render")]
    register_schematic!(app, VisibilityBundle, SpatialBundle);
    #[cfg(feature = "bevy_sprite")]
    register_schematic!(app, DynamicSceneBundle, SceneBundle);
    #[cfg(feature = "bevy_sprite")]
    register_schematic!(
        app,
        SpriteBundle,
        SpriteSheetBundle,
        MaterialMesh2dBundle<bevy::sprite::ColorMaterial>
    );
    #[cfg(feature = "bevy_text")]
    register_schematic!(app, Text2dBundle);
    #[cfg(feature = "bevy_ui")]
    register_schematic!(app, ButtonBundle, ImageBundle, NodeBundle, TextBundle);
}

fn transparent_background_color() -> bevy_impls::ui::BackgroundColorInput {
    bevy_impls::ui::BackgroundColorInput(ProtoColor::None)
}

fn transparent_border_color() -> bevy_impls::ui::BorderColorInput {
    bevy_impls::ui::BorderColorInput(ProtoColor::None)
}

/// A [`Schematic`] implementation of [`TransformBundle`].
///
/// [`TransformBundle`]: bevy::prelude::TransformBundle
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic, Default)]
#[schematic(into = bevy::prelude::TransformBundle)]
pub struct TransformBundle {
    pub local: Transform,
    pub global: GlobalTransform,
}

from_to_default! {
    bevy::transform::TransformBundle,
    TransformBundle,
    |value: Input| Self {
        local: value.local,
        global: value.global,
    }
}

/// A [`Schematic`] implementation of [`SpatialBundle`].
///
/// [`SpatialBundle`]: bevy::prelude::SpatialBundle
#[cfg(feature = "bevy_render")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic, Default)]
#[schematic(into = bevy::render::prelude::SpatialBundle)]
pub struct SpatialBundle {
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed: bevy::render::view::ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[cfg(feature = "bevy_render")]
from_to_default! {
    bevy::render::prelude::SpatialBundle,
    SpatialBundle,
    |value: Input| Self {
        visibility: value.visibility,
        computed: value.computed,
        transform: value.transform,
        global_transform: value.global_transform,
    }
}

/// A [`Schematic`] implementation of [`VisibilityBundle`].
///
/// [`VisibilityBundle`]: bevy::prelude::VisibilityBundle
#[cfg(feature = "bevy_render")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic, Default)]
#[schematic(into = bevy::prelude::VisibilityBundle)]
pub struct VisibilityBundle {
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed: bevy::render::view::ComputedVisibility,
}

#[cfg(feature = "bevy_render")]
from_to_default! {
    bevy::render::view::VisibilityBundle,
    VisibilityBundle,
    |value: Input| Self {
        visibility: value.visibility,
        computed: value.computed,
    }
}

/// A [`Schematic`] implementation of [`SpriteBundle`].
///
/// [`SpriteBundle`]: bevy::prelude::SpriteBundle
#[cfg(feature = "bevy_sprite")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::prelude::SpriteBundle)]
pub struct SpriteBundle {
    #[reflect(default)]
    pub sprite: bevy::prelude::Sprite,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[schematic(asset)]
    pub texture: Handle<bevy::prelude::Image>,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
}

#[cfg(feature = "bevy_sprite")]
from!(bevy::sprite::SpriteBundle, SpriteBundle, |value: Input| {
    Self {
        sprite: value.sprite,
        transform: value.transform,
        global_transform: value.global_transform,
        texture: value.texture,
        visibility: value.visibility,
        computed_visibility: value.computed_visibility,
    }
});

/// A [`Schematic`] implementation of [`SpriteSheetBundle`].
///
/// [`SpriteSheetBundle`]: bevy::prelude::SpriteSheetBundle
#[cfg(feature = "bevy_sprite")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::sprite::SpriteSheetBundle)]
pub struct SpriteSheetBundle {
    #[reflect(default)]
    pub sprite: bevy_impls::sprite::TextureAtlasSpriteInput,
    #[schematic(asset(inline))]
    pub texture_atlas: Handle<bevy::prelude::TextureAtlas>,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
}

#[cfg(feature = "bevy_sprite")]
from!(
    bevy::sprite::SpriteSheetBundle,
    SpriteSheetBundle,
    |value: Input| {
        Self {
            sprite: value.sprite.into(),
            texture_atlas: value.texture_atlas,
            transform: value.transform,
            global_transform: value.global_transform,
            visibility: value.visibility,
            computed_visibility: value.computed_visibility,
        }
    }
);

/// A [`Schematic`] implementation of [`Camera2dBundle`].
///
/// [`Camera2dBundle`]: bevy::core_pipeline::core_2d::Camera2dBundle
#[cfg(feature = "bevy_core_pipeline")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic, Default)]
#[schematic(into = bevy::prelude::Camera2dBundle)]
pub struct Camera2dBundle {
    pub camera: bevy::render::camera::Camera,
    camera_render_graph: bevy::render::camera::CameraRenderGraph,
    pub projection: bevy::render::prelude::OrthographicProjection,
    #[reflect(ignore)]
    pub visible_entities: bevy::render::view::VisibleEntities,
    #[reflect(ignore)]
    pub frustum: bevy::render::primitives::Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    camera_2d: bevy_impls::core_pipeline::Camera2dInput,
    pub tonemapping: bevy::core_pipeline::tonemapping::Tonemapping,
    pub deband_dither: bevy::core_pipeline::tonemapping::DebandDither,
}

#[cfg(feature = "bevy_core_pipeline")]
from_to_default! {
    bevy::core_pipeline::core_2d::Camera2dBundle,
    Camera2dBundle,
    |value: Input| Self {
        camera: value.camera,
        camera_render_graph: value.camera_render_graph,
        projection: value.projection,
        visible_entities: value.visible_entities,
        frustum: value.frustum,
        transform: value.transform,
        global_transform: value.global_transform,
        camera_2d: value.camera_2d.into(),
        tonemapping: value.tonemapping,
        deband_dither: value.deband_dither,
    }
}

/// A [`Schematic`] implementation of [`Camera3dBundle`].
///
/// [`Camera3dBundle`]: bevy::core_pipeline::core_3d::Camera3dBundle
#[cfg(feature = "bevy_core_pipeline")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic, Default)]
#[schematic(into = bevy::prelude::Camera3dBundle)]
pub struct Camera3dBundle {
    pub camera: bevy::render::camera::Camera,
    camera_render_graph: bevy::render::camera::CameraRenderGraph,
    pub projection: bevy_impls::render::ProjectionInput,
    #[reflect(ignore)]
    pub visible_entities: bevy::render::view::VisibleEntities,
    #[reflect(ignore)]
    pub frustum: bevy::render::primitives::Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    camera_3d: bevy_impls::core_pipeline::Camera3dInput,
    pub tonemapping: bevy::core_pipeline::tonemapping::Tonemapping,
    pub dither: bevy::core_pipeline::tonemapping::DebandDither,
    pub color_grading: bevy_impls::render::ColorGradingInput,
}

#[cfg(feature = "bevy_core_pipeline")]
from_to_default! {
    bevy::core_pipeline::core_3d::Camera3dBundle,
    Camera3dBundle,
    |value: Input| Self {
        camera: value.camera,
        camera_render_graph: value.camera_render_graph,
        projection: value.projection.into(),
        visible_entities: value.visible_entities,
        frustum: value.frustum,
        transform: value.transform,
        global_transform: value.global_transform,
        camera_3d: value.camera_3d.into(),
        tonemapping: value.tonemapping,
        dither: value.dither,
        color_grading: value.color_grading.into(),
    }
}

/// A [`Schematic`] implementation of [`DirectionalLightBundle`].
///
/// [`DirectionalLightBundle`]: bevy::pbr::DirectionalLightBundle
#[cfg(feature = "bevy_pbr")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic, Default)]
#[schematic(into = bevy::pbr::DirectionalLightBundle)]
pub struct DirectionalLightBundle {
    pub directional_light: bevy::pbr::DirectionalLight,
    #[reflect(ignore)]
    pub frusta: bevy::render::primitives::CascadesFrusta,
    #[reflect(ignore)]
    pub cascades: bevy::pbr::Cascades,
    pub cascade_shadow_config: bevy_impls::pbr::CascadeShadowConfigInput,
    #[reflect(ignore)]
    pub visible_entities: bevy::pbr::CascadesVisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
}

#[cfg(feature = "bevy_pbr")]
from!(
    bevy::pbr::DirectionalLightBundle,
    DirectionalLightBundle,
    |value: Input| Self {
        directional_light: value.directional_light,
        frusta: value.frusta,
        cascades: value.cascades,
        cascade_shadow_config: value.cascade_shadow_config.into(),
        visible_entities: value.visible_entities,
        transform: value.transform,
        global_transform: value.global_transform,
        visibility: value.visibility,
        computed_visibility: value.computed_visibility,
    }
);

#[cfg(feature = "bevy_pbr")]
impl Default for DirectionalLightBundle {
    fn default() -> Self {
        let base = bevy::pbr::DirectionalLightBundle::default();
        Self {
            directional_light: base.directional_light,
            frusta: base.frusta,
            cascades: base.cascades,
            cascade_shadow_config: bevy_impls::pbr::CascadeShadowConfigInput::default(),
            visible_entities: base.visible_entities,
            transform: base.transform,
            global_transform: base.global_transform,
            visibility: base.visibility,
            computed_visibility: base.computed_visibility,
        }
    }
}

/// A [`Schematic`] implementation of [`PointLightBundle`].
///
/// [`PointLightBundle`]: bevy::pbr::PointLightBundle
#[cfg(feature = "bevy_pbr")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic, Default)]
#[schematic(into = bevy::pbr::PointLightBundle)]
pub struct PointLightBundle {
    pub point_light: bevy::pbr::PointLight,
    #[reflect(ignore)]
    pub cubemap_frusta: bevy::render::primitives::CubemapFrusta,
    #[reflect(ignore)]
    pub cubemap_visible_entities: bevy::pbr::CubemapVisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
}

#[cfg(feature = "bevy_pbr")]
from_to_default!(
    bevy::pbr::PointLightBundle,
    PointLightBundle,
    |value: Input| Self {
        point_light: value.point_light,
        cubemap_frusta: value.cubemap_frusta,
        cubemap_visible_entities: value.cubemap_visible_entities,
        transform: value.transform,
        global_transform: value.global_transform,
        visibility: value.visibility,
        computed_visibility: value.computed_visibility,
    }
);

/// A [`Schematic`] implementation of [`SpotLightBundle`].
///
/// [`SpotLightBundle`]: bevy::pbr::SpotLightBundle
#[cfg(feature = "bevy_pbr")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic, Default)]
#[schematic(into = bevy::pbr::SpotLightBundle)]
pub struct SpotLightBundle {
    pub spot_light: bevy::pbr::SpotLight,
    #[reflect(ignore)]
    pub visible_entities: bevy::render::view::VisibleEntities,
    #[reflect(ignore)]
    pub frustum: bevy::render::primitives::Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
}

#[cfg(feature = "bevy_pbr")]
from_to_default!(
    bevy::pbr::SpotLightBundle,
    SpotLightBundle,
    |value: Input| Self {
        spot_light: value.spot_light,
        visible_entities: value.visible_entities,
        frustum: value.frustum,
        transform: value.transform,
        global_transform: value.global_transform,
        visibility: value.visibility,
        computed_visibility: value.computed_visibility,
    }
);

/// A [`Schematic`] implementation of [`DynamicSceneBundle`].
///
/// [`DynamicSceneBundle`]: bevy::scene::DynamicSceneBundle
#[cfg(feature = "bevy_scene")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::scene::DynamicSceneBundle)]
pub struct DynamicSceneBundle {
    #[schematic(asset)]
    pub scene: Handle<bevy::scene::DynamicScene>,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
}

#[cfg(feature = "bevy_scene")]
from!(
    bevy::scene::DynamicSceneBundle,
    DynamicSceneBundle,
    |value: Input| Self {
        scene: value.scene,
        transform: value.transform,
        global_transform: value.global_transform,
        visibility: value.visibility,
        computed_visibility: value.computed_visibility,
    }
);

/// A [`Schematic`] implementation of [`SceneBundle`].
///
/// [`SceneBundle`]: bevy::scene::SceneBundle
#[cfg(feature = "bevy_scene")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::scene::SceneBundle)]
pub struct SceneBundle {
    #[schematic(asset)]
    pub scene: Handle<bevy::scene::Scene>,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
}

#[cfg(feature = "bevy_scene")]
from!(bevy::scene::SceneBundle, SceneBundle, |value: Input| Self {
    scene: value.scene,
    transform: value.transform,
    global_transform: value.global_transform,
    visibility: value.visibility,
    computed_visibility: value.computed_visibility,
});

/// A [`Schematic`] implementation of [`Text2dBundle`].
///
/// [`Text2dBundle`]: bevy::text::Text2dBundle
#[cfg(feature = "bevy_text")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::text::Text2dBundle)]
pub struct Text2dBundle {
    #[reflect(default)]
    pub text: bevy_impls::text::TextInput,
    #[reflect(default)]
    pub text_anchor: bevy::sprite::Anchor,
    #[reflect(default)]
    pub text_2d_bounds: bevy_impls::text::Text2dBoundsInput,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
    #[reflect(default)]
    pub text_layout_info: bevy_impls::text::TextLayoutInfoInput,
}

#[cfg(feature = "bevy_text")]
impl FromSchematicInput<Text2dBundle> for bevy::text::Text2dBundle {
    fn from_input(input: Text2dBundle, id: SchematicId, context: &mut SchematicContext) -> Self {
        Self {
            text: FromSchematicInput::from_input(
                input.text,
                id.next(bevy::utils::Uuid::from_u128(
                    0x14c512589e954232b77ee264aecebe56,
                )),
                context,
            ),
            text_anchor: input.text_anchor,
            text_2d_bounds: FromSchematicInput::from_input(
                input.text_2d_bounds,
                id.next(bevy::utils::Uuid::from_u128(
                    0x73693c0f150b44389da54b25360886b2,
                )),
                context,
            ),
            transform: input.transform,
            global_transform: input.global_transform,
            visibility: input.visibility,
            computed_visibility: input.computed_visibility,
            text_layout_info: FromSchematicInput::from_input(
                input.text_layout_info,
                id.next(bevy::utils::Uuid::from_u128(
                    0xa28731f3a3f445d69b85c438111c9949,
                )),
                context,
            ),
        }
    }
}

/// A [`Schematic`] implementation of [`ButtonBundle`].
///
/// [`ButtonBundle`]: bevy::ui::node_bundles::ButtonBundle
#[cfg(feature = "bevy_ui")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::ui::node_bundles::ButtonBundle)]
pub struct ButtonBundle {
    #[reflect(default)]
    pub node: bevy::ui::Node,
    #[reflect(default)]
    pub button: bevy::ui::widget::Button,
    #[reflect(default)]
    pub style: bevy_impls::ui::StyleInput,
    #[reflect(default)]
    pub interaction: bevy::ui::Interaction,
    #[reflect(default)]
    pub focus_policy: bevy::ui::FocusPolicy,
    #[reflect(default)]
    pub background_color: bevy_impls::ui::BackgroundColorInput,
    #[reflect(default = "transparent_border_color")]
    pub border_color: bevy_impls::ui::BorderColorInput,
    #[reflect(default)]
    pub image: bevy_impls::ui::UiImageInput,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
    #[reflect(default)]
    pub z_index: bevy::ui::ZIndex,
}

#[cfg(feature = "bevy_ui")]
impl FromSchematicInput<ButtonBundle> for bevy::ui::node_bundles::ButtonBundle {
    fn from_input(input: ButtonBundle, id: SchematicId, context: &mut SchematicContext) -> Self {
        Self {
            node: input.node,
            button: input.button,
            style: input.style.into(),
            interaction: input.interaction,
            focus_policy: input.focus_policy,
            background_color: input.background_color.into(),
            border_color: input.border_color.into(),
            image: bevy::ui::UiImage::from_input(
                input.image,
                id.next(bevy::utils::Uuid::from_u128(
                    0x1d002cb9c29f40cf97a71a341abe855f,
                )),
                context,
            ),
            transform: input.transform,
            global_transform: input.global_transform,
            visibility: input.visibility,
            computed_visibility: input.computed_visibility,
            z_index: input.z_index,
        }
    }
}

/// A [`Schematic`] implementation of [`ImageBundle`].
///
/// [`ImageBundle`]: bevy::ui::node_bundles::ImageBundle
#[cfg(feature = "bevy_ui")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::ui::node_bundles::ImageBundle)]
pub struct ImageBundle {
    #[reflect(default)]
    pub node: bevy::ui::Node,
    #[reflect(default)]
    pub style: bevy_impls::ui::StyleInput,
    #[reflect(default)]
    pub background_color: bevy_impls::ui::BackgroundColorInput,
    #[reflect(default)]
    pub image: bevy_impls::ui::UiImageInput,
    #[reflect(default)]
    pub focus_policy: bevy::ui::FocusPolicy,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
    #[reflect(default)]
    pub z_index: bevy::ui::ZIndex,
    #[reflect(ignore)]
    pub image_size: UiImageSize,
}

#[cfg(feature = "bevy_ui")]
impl FromSchematicInput<ImageBundle> for bevy::ui::node_bundles::ImageBundle {
    fn from_input(input: ImageBundle, id: SchematicId, context: &mut SchematicContext) -> Self {
        Self {
            node: input.node,
            style: input.style.into(),
            calculated_size: Default::default(),
            background_color: input.background_color.into(),
            image: bevy::ui::UiImage::from_input(
                input.image,
                id.next(bevy::utils::Uuid::from_u128(
                    0x5f3dc1f3b56d49e99f2a5978aea6b745,
                )),
                context,
            ),
            focus_policy: input.focus_policy,
            transform: input.transform,
            global_transform: input.global_transform,
            visibility: input.visibility,
            computed_visibility: input.computed_visibility,
            z_index: input.z_index,
            image_size: UiImageSize::default(), // this field is set automatically by Bevy normally
        }
    }
}

/// A [`Schematic`] implementation of [`NodeBundle`].
///
/// [`NodeBundle`]: bevy::ui::node_bundles::NodeBundle
#[cfg(feature = "bevy_ui")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic, Default)]
#[schematic(into = bevy::ui::node_bundles::NodeBundle)]
pub struct NodeBundle {
    pub node: bevy::ui::Node,
    pub style: bevy_impls::ui::StyleInput,
    pub background_color: bevy_impls::ui::BackgroundColorInput,
    pub border_color: bevy_impls::ui::BorderColorInput,
    pub focus_policy: bevy::ui::FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
    pub z_index: bevy::ui::ZIndex,
}

#[cfg(feature = "bevy_ui")]
from_to_default!(
    bevy::ui::node_bundles::NodeBundle,
    NodeBundle,
    |value: Input| Self {
        node: value.node,
        style: value.style.into(),
        background_color: value.background_color.into(),
        border_color: value.border_color.into(),
        focus_policy: value.focus_policy,
        transform: value.transform,
        global_transform: value.global_transform,
        visibility: value.visibility,
        computed_visibility: value.computed_visibility,
        z_index: value.z_index,
    }
);

/// A [`Schematic`] implementation of [`TextBundle`].
///
/// [`TextBundle`]: bevy::ui::node_bundles::TextBundle
#[cfg(feature = "bevy_ui")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::ui::node_bundles::TextBundle)]
pub struct TextBundle {
    #[reflect(default)]
    pub node: bevy::ui::Node,
    #[reflect(default)]
    pub style: bevy_impls::ui::StyleInput,
    #[reflect(default)]
    pub text: bevy_impls::text::TextInput,
    #[reflect(default)]
    pub text_layout_info: bevy_impls::text::TextLayoutInfoInput,
    #[reflect(default)]
    pub text_flags: bevy::ui::widget::TextFlags,
    #[reflect(ignore)]
    pub calculated_size: bevy::ui::ContentSize,
    #[reflect(default)]
    pub focus_policy: bevy::ui::FocusPolicy,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
    #[reflect(default)]
    pub z_index: bevy::ui::ZIndex,
    #[reflect(default = "transparent_background_color")]
    pub background_color: bevy_impls::ui::BackgroundColorInput,
}

#[cfg(feature = "bevy_ui")]
impl FromSchematicInput<TextBundle> for bevy::ui::node_bundles::TextBundle {
    fn from_input(input: TextBundle, id: SchematicId, context: &mut SchematicContext) -> Self {
        Self {
            node: input.node,
            style: FromSchematicInput::from_input(
                input.style,
                id.next(bevy::utils::Uuid::from_u128(
                    0x63ca849a99e44de2b6fcad4cbdd03640,
                )),
                context,
            ),
            text: FromSchematicInput::from_input(
                input.text,
                id.next(bevy::utils::Uuid::from_u128(
                    0xd3a21a2a19ab4a18b8a60f5ab95f299d,
                )),
                context,
            ),
            text_layout_info: FromSchematicInput::from_input(
                input.text_layout_info,
                id.next(bevy::utils::Uuid::from_u128(
                    0x3647343b704b40f2a789b890cf417c18,
                )),
                context,
            ),
            text_flags: input.text_flags,
            calculated_size: input.calculated_size,
            focus_policy: input.focus_policy,
            transform: input.transform,
            global_transform: input.global_transform,
            visibility: input.visibility,
            computed_visibility: input.computed_visibility,
            z_index: input.z_index,
            background_color: FromSchematicInput::from_input(
                input.background_color,
                id.next(bevy::utils::Uuid::from_u128(
                    0xc82dc5faf3ff4442878496f8d77187c3,
                )),
                context,
            ),
        }
    }
}

/// A [`Schematic`] implementation of [`MaterialMeshBundle`].
///
/// [`MaterialMeshBundle`]: bevy::pbr::MaterialMeshBundle
#[cfg(feature = "bevy_pbr")]
#[doc(alias = "PbrBundle")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::pbr::MaterialMeshBundle<M>)]
pub struct MaterialMeshBundle<M: bevy::pbr::Material + AssetSchematic>
where
    Handle<M>: FromSchematicInput<InlinableProtoAsset<M>>,
{
    #[reflect(default)]
    #[schematic(asset(inline))]
    pub mesh: Handle<bevy::render::mesh::Mesh>,
    #[reflect(default)]
    #[schematic(asset(inline))]
    pub material: Handle<M>,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
}

#[cfg(feature = "bevy_pbr")]
impl<M: bevy::pbr::Material + AssetSchematic> From<MaterialMeshBundle<M>>
    for bevy::pbr::MaterialMeshBundle<M>
where
    Handle<M>: FromSchematicInput<InlinableProtoAsset<M>>,
{
    fn from(value: MaterialMeshBundle<M>) -> Self {
        Self {
            mesh: value.mesh,
            material: value.material,
            transform: value.transform,
            global_transform: value.global_transform,
            visibility: value.visibility,
            computed_visibility: value.computed_visibility,
        }
    }
}

/// A [`Schematic`] implementation of [`MaterialMesh2dBundle`].
///
/// [`MaterialMesh2dBundle`]: bevy::sprite::MaterialMesh2dBundle
#[cfg(feature = "bevy_sprite")]
#[doc(alias = "ColorMesh2dBundle")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::sprite::MaterialMesh2dBundle<M>)]
pub struct MaterialMesh2dBundle<M: bevy::sprite::Material2d + AssetSchematic>
where
    Handle<M>: FromSchematicInput<InlinableProtoAsset<M>>,
{
    #[reflect(default)]
    pub mesh: bevy_impls::sprite::Mesh2dHandleInput,
    #[reflect(default)]
    #[schematic(asset(inline))]
    pub material: Handle<M>,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
}

#[cfg(feature = "bevy_sprite")]
impl<M: bevy::sprite::Material2d + AssetSchematic> FromSchematicInput<MaterialMesh2dBundle<M>>
    for bevy::sprite::MaterialMesh2dBundle<M>
where
    Handle<M>: FromSchematicInput<InlinableProtoAsset<M>>,
{
    fn from_input(
        input: MaterialMesh2dBundle<M>,
        id: SchematicId,
        context: &mut SchematicContext,
    ) -> Self {
        Self {
            mesh: bevy::sprite::Mesh2dHandle::from_input(
                input.mesh,
                id.next(bevy::utils::Uuid::from_u128(
                    0xc96384968f7f4143906ec8802addfac0,
                )),
                context,
            ),
            material: input.material,
            transform: input.transform,
            global_transform: input.global_transform,
            visibility: input.visibility,
            computed_visibility: input.computed_visibility,
        }
    }
}
