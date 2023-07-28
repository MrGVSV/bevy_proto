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
use bevy_proto_backend::impls::bevy_impls;
use bevy_proto_backend::{from, from_to_default};

use bevy_proto_backend::schematics::{
    FromSchematicInput, ReflectSchematic, Schematic, SchematicContext,
};

pub(crate) fn register_custom_schematics(app: &mut App) {
    app.register_type::<TransformBundle>();
    #[cfg(feature = "bevy_core_pipeline")]
    app.register_type::<Camera2dBundle>()
        .register_type::<Camera3dBundle>();
    #[cfg(feature = "bevy_pbr")]
    app.register_type::<DirectionalLightBundle>()
        .register_type::<PointLightBundle>()
        .register_type::<SpotLightBundle>()
        .register_type::<MaterialMeshBundle<bevy::pbr::StandardMaterial>>();
    #[cfg(feature = "bevy_render")]
    app.register_type::<VisibilityBundle>()
        .register_type::<SpatialBundle>();
    #[cfg(feature = "bevy_sprite")]
    app.register_type::<DynamicSceneBundle>()
        .register_type::<SceneBundle>();
    #[cfg(feature = "bevy_sprite")]
    app.register_type::<SpriteBundle>()
        .register_type::<SpriteSheetBundle>()
        .register_type::<MaterialMesh2dBundle<bevy::sprite::ColorMaterial>>();
    #[cfg(feature = "bevy_text")]
    app.register_type::<Text2dBundle>();
    #[cfg(feature = "bevy_ui")]
    app.register_type::<ButtonBundle>()
        .register_type::<ImageBundle>()
        .register_type::<NodeBundle>()
        .register_type::<TextBundle>();
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
    #[schematic(asset(lazy))]
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
    #[schematic(asset(lazy))]
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
    camera_render_graph: bevy_impls::render::CameraRenderGraphInput,
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
        camera_render_graph: value.camera_render_graph.into(),
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
    camera_render_graph: bevy_impls::render::CameraRenderGraphInput,
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
        camera_render_graph: value.camera_render_graph.into(),
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
    pub directional_light: bevy_impls::pbr::DirectionalLightInput,
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
        directional_light: value.directional_light.into(),
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
            directional_light: base.directional_light.into(),
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
    pub point_light: bevy_impls::pbr::PointLightInput,
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
        point_light: value.point_light.into(),
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
    pub spot_light: bevy_impls::pbr::SpotLightInput,
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
        spot_light: value.spot_light.into(),
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
    #[schematic(asset(lazy))]
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
    #[schematic(asset(lazy))]
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
    fn from_input(input: Text2dBundle, context: &mut SchematicContext) -> Self {
        Self {
            text: FromSchematicInput::from_input(input.text, context),
            text_anchor: input.text_anchor,
            text_2d_bounds: FromSchematicInput::from_input(input.text_2d_bounds, context),
            transform: input.transform,
            global_transform: input.global_transform,
            visibility: input.visibility,
            computed_visibility: input.computed_visibility,
            text_layout_info: FromSchematicInput::from_input(input.text_layout_info, context),
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
    pub node: bevy_impls::ui::NodeInput,
    #[reflect(default)]
    pub button: bevy_impls::ui::ButtonInput,
    #[reflect(default)]
    pub style: bevy_impls::ui::StyleInput,
    #[reflect(default)]
    pub interaction: bevy_impls::ui::InteractionInput,
    #[reflect(default)]
    pub focus_policy: bevy_impls::ui::FocusPolicyInput,
    #[reflect(default)]
    pub background_color: bevy_impls::ui::BackgroundColorInput,
    #[reflect(default)]
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
    pub z_index: bevy_impls::ui::ZIndexInput,
}

#[cfg(feature = "bevy_ui")]
impl FromSchematicInput<ButtonBundle> for bevy::ui::node_bundles::ButtonBundle {
    fn from_input(input: ButtonBundle, context: &mut SchematicContext) -> Self {
        Self {
            node: input.node.into(),
            button: input.button.into(),
            style: input.style.into(),
            interaction: input.interaction.into(),
            focus_policy: input.focus_policy.into(),
            background_color: input.background_color.into(),
            border_color: input.border_color.into(),
            image: bevy::ui::UiImage::from_input(input.image, context),
            transform: input.transform,
            global_transform: input.global_transform,
            visibility: input.visibility,
            computed_visibility: input.computed_visibility,
            z_index: input.z_index.into(),
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
    pub node: bevy_impls::ui::NodeInput,
    #[reflect(default)]
    pub style: bevy_impls::ui::StyleInput,
    #[reflect(default)]
    pub background_color: bevy_impls::ui::BackgroundColorInput,
    #[reflect(default)]
    pub image: bevy_impls::ui::UiImageInput,
    #[reflect(default)]
    pub focus_policy: bevy_impls::ui::FocusPolicyInput,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
    #[reflect(default)]
    pub z_index: bevy_impls::ui::ZIndexInput,
    #[reflect(ignore)]
    pub image_size: UiImageSize,
}

#[cfg(feature = "bevy_ui")]
impl FromSchematicInput<ImageBundle> for bevy::ui::node_bundles::ImageBundle {
    fn from_input(input: ImageBundle, context: &mut SchematicContext) -> Self {
        Self {
            node: input.node.into(),
            style: input.style.into(),
            calculated_size: Default::default(),
            background_color: input.background_color.into(),
            image: bevy::ui::UiImage::from_input(input.image, context),
            focus_policy: input.focus_policy.into(),
            transform: input.transform,
            global_transform: input.global_transform,
            visibility: input.visibility,
            computed_visibility: input.computed_visibility,
            z_index: input.z_index.into(),
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
    pub node: bevy_impls::ui::NodeInput,
    pub style: bevy_impls::ui::StyleInput,
    pub background_color: bevy_impls::ui::BackgroundColorInput,
    pub border_color: bevy::ui::BorderColor,
    pub focus_policy: bevy_impls::ui::FocusPolicyInput,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
    pub z_index: bevy_impls::ui::ZIndexInput,
}

#[cfg(feature = "bevy_ui")]
from_to_default!(
    bevy::ui::node_bundles::NodeBundle,
    NodeBundle,
    |value: Input| Self {
        node: value.node.into(),
        style: value.style.into(),
        background_color: value.background_color.into(),
        border_color: value.border_color,
        focus_policy: value.focus_policy.into(),
        transform: value.transform,
        global_transform: value.global_transform,
        visibility: value.visibility,
        computed_visibility: value.computed_visibility,
        z_index: value.z_index.into(),
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
    pub node: bevy_impls::ui::NodeInput,
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
    pub focus_policy: bevy_impls::ui::FocusPolicyInput,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[reflect(default)]
    pub visibility: bevy::render::view::Visibility,
    #[reflect(ignore)]
    pub computed_visibility: bevy::render::view::ComputedVisibility,
    #[reflect(default)]
    pub z_index: bevy_impls::ui::ZIndexInput,
    #[reflect(default)]
    pub background_color: bevy_impls::ui::BackgroundColorInput,
}

#[cfg(feature = "bevy_ui")]
impl FromSchematicInput<TextBundle> for bevy::ui::node_bundles::TextBundle {
    fn from_input(input: TextBundle, context: &mut SchematicContext) -> Self {
        Self {
            node: FromSchematicInput::from_input(input.node, context),
            style: FromSchematicInput::from_input(input.style, context),
            text: FromSchematicInput::from_input(input.text, context),
            text_layout_info: FromSchematicInput::from_input(input.text_layout_info, context),
            text_flags: input.text_flags,
            calculated_size: input.calculated_size,
            focus_policy: FromSchematicInput::from_input(input.focus_policy, context),
            transform: input.transform,
            global_transform: input.global_transform,
            visibility: input.visibility,
            computed_visibility: input.computed_visibility,
            z_index: FromSchematicInput::from_input(input.z_index, context),
            background_color: FromSchematicInput::from_input(input.background_color, context),
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
pub struct MaterialMeshBundle<M: bevy::pbr::Material> {
    // #[reflect(
    //     default = "bevy_proto_backend::proto::ProtoAsset::default_handle_id::<bevy::render::mesh::Mesh>"
    // )]
    #[schematic(asset(lazy))]
    pub mesh: Handle<bevy::render::mesh::Mesh>,
    // #[reflect(default = "bevy_proto_backend::proto::ProtoAsset::default_handle_id::<M>")]
    #[schematic(asset(lazy))]
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
impl<M: bevy::pbr::Material> From<MaterialMeshBundle<M>> for bevy::pbr::MaterialMeshBundle<M> {
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
pub struct MaterialMesh2dBundle<M: bevy::sprite::Material2d> {
    #[reflect(default)]
    pub mesh: bevy_impls::sprite::Mesh2dHandleInput,
    #[schematic(asset(lazy))]
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
impl<M: bevy::sprite::Material2d> FromSchematicInput<MaterialMesh2dBundle<M>>
    for bevy::sprite::MaterialMesh2dBundle<M>
{
    fn from_input(input: MaterialMesh2dBundle<M>, context: &mut SchematicContext) -> Self {
        Self {
            mesh: bevy::sprite::Mesh2dHandle::from_input(input.mesh, context),
            material: input.material,
            transform: input.transform,
            global_transform: input.global_transform,
            visibility: input.visibility,
            computed_visibility: input.computed_visibility,
        }
    }
}
