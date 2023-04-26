use bevy::app::App;

pub(crate) fn register_impls(app: &mut App) {
    bevy_impls::asset::register(app);
    bevy_impls::core::register(app);
    bevy_impls::transform::register(app);
    bevy_impls::window::register(app);

    #[cfg(feature = "bevy_animation")]
    bevy_impls::animation::register(app);
    #[cfg(feature = "bevy_core_pipeline")]
    bevy_impls::core_pipeline::register(app);
    #[cfg(feature = "bevy_gltf")]
    bevy_impls::gltf::register(app);
    #[cfg(feature = "bevy_pbr")]
    bevy_impls::pbr::register(app);
    #[cfg(feature = "bevy_render")]
    bevy_impls::render::register(app);
    #[cfg(feature = "bevy_sprite")]
    bevy_impls::sprite::register(app);
    #[cfg(feature = "bevy_text")]
    bevy_impls::text::register(app);
    #[cfg(feature = "bevy_ui")]
    bevy_impls::ui::register(app);
}

macro_rules! register_schematic {
    ($app: ident, $($ty: ty),* $(,)?) => {{
        $(
            // Sanity check: ensure the type is actually registered
            // before actually registering the `ReflectSchematic` type data
            $app.register_type::<$ty>()
                .register_type_data::<$ty, crate::schematics::ReflectSchematic>();

        )*
    }};
}

mod bevy_impls {

    #[cfg(feature = "bevy_animation")]
    pub mod animation {
        use bevy::app::App;

        pub fn register(_app: &mut App) {}
    }

    pub mod asset {
        use bevy::app::App;
        use bevy::asset::{Asset, AssetServer, Handle};
        use bevy::ecs::world::EntityMut;

        use bevy_proto_derive::impl_external_schematic;

        use crate::proto::ProtoAsset;
        use crate::schematics::FromSchematicInput;
        use crate::tree::EntityTree;

        #[allow(unused_variables)]
        pub fn register(app: &mut App) {
            #[cfg(feature = "bevy_animation")]
            register_schematic!(app, Handle<bevy::prelude::AnimationClip>);

            #[cfg(feature = "bevy_audio")]
            register_schematic!(
                app,
                Handle<bevy::prelude::AudioSink>,
                Handle<bevy::prelude::AudioSource>,
                Handle<bevy::prelude::SpatialAudioSink>,
            );

            #[cfg(feature = "bevy_gltf")]
            register_schematic!(
                app,
                Handle<bevy::gltf::Gltf>,
                Handle<bevy::gltf::GltfMesh>,
                Handle<bevy::gltf::GltfPrimitive>,
                Handle<bevy::gltf::GltfNode>,
            );

            #[cfg(feature = "bevy_pbr")]
            register_schematic!(app, Handle<bevy::prelude::StandardMaterial>);

            #[cfg(feature = "bevy_render")]
            register_schematic!(
                app,
                Handle<bevy::prelude::Image>,
                Handle<bevy::render::mesh::skinning::SkinnedMeshInverseBindposes>,
                Handle<bevy::prelude::Mesh>,
                Handle<bevy::prelude::Shader>,
            );

            #[cfg(feature = "bevy_scene")]
            register_schematic!(
                app,
                Handle<bevy::prelude::DynamicScene>,
                Handle<bevy::prelude::Scene>,
            );

            #[cfg(feature = "bevy_sprite")]
            register_schematic!(
                app,
                Handle<bevy::prelude::ColorMaterial>,
                Handle<bevy::prelude::TextureAtlas>,
            );

            #[cfg(feature = "bevy_text")]
            register_schematic!(
                app,
                Handle<bevy::prelude::Font>,
                Handle<bevy::text::FontAtlasSet>,
            );
        }

        impl_external_schematic! {
            #[schematic(from = ProtoAsset)]
            struct Handle<T: Asset> {}
            // ---
            impl<T: Asset> FromSchematicInput<ProtoAsset> for Handle<T> {
                fn from_input(input: ProtoAsset, entity: &mut EntityMut, _: &EntityTree) -> Self {
                    match input {
                        ProtoAsset::AssetPath(path) => entity.world().resource::<AssetServer>().get_handle(path)
                    }
                }
            }
        }
    }

    pub mod core {
        use bevy::app::App;
        use bevy::core::Name;
        use bevy::reflect::{FromReflect, Reflect};

        use bevy_proto_derive::impl_external_schematic;

        pub fn register(app: &mut App) {
            register_schematic!(app, Name);
        }

        impl_external_schematic! {
            #[schematic(from = NameInput)]
            struct Name {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct NameInput(String);
            impl From<NameInput> for Name {
                fn from(input: NameInput) -> Self {
                    Name::new(input.0)
                }
            }
        }
    }

    #[cfg(feature = "bevy_core_pipeline")]
    pub mod core_pipeline {
        use bevy::app::App;
        use bevy::core_pipeline::bloom::{
            BloomCompositeMode, BloomPrefilterSettings, BloomSettings,
        };
        use bevy::core_pipeline::clear_color::ClearColorConfig;
        use bevy::core_pipeline::core_3d::Camera3dDepthLoadOp;
        use bevy::core_pipeline::fxaa::Fxaa;
        use bevy::core_pipeline::prepass::{DepthPrepass, NormalPrepass};
        use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
        use bevy::prelude::{Camera2d, Camera3d, Color};
        use bevy::reflect::{FromReflect, Reflect};

        use bevy_proto_derive::impl_external_schematic;

        pub fn register(app: &mut App) {
            register_schematic!(
                app,
                BloomSettings,
                Camera2d,
                Camera3d,
                DebandDither,
                DepthPrepass,
                Fxaa,
                NormalPrepass,
                Tonemapping
            );
        }

        impl_external_schematic! {
            #[schematic(from = BloomSettingsInput)]
            struct BloomSettings {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct BloomSettingsInput {
                pub intensity: f32,
                pub low_frequency_boost: f32,
                pub low_frequency_boost_curvature: f32,
                pub high_pass_frequency: f32,
                pub prefilter_settings: BloomPrefilterSettingsInput,
                pub composite_mode: BloomCompositeModeInput,
            }
            impl From<BloomSettingsInput> for BloomSettings {
                fn from(value: BloomSettingsInput) -> Self {
                    Self {
                        intensity: value.intensity,
                        low_frequency_boost: value.low_frequency_boost,
                        low_frequency_boost_curvature: value.low_frequency_boost_curvature,
                        high_pass_frequency: value.high_pass_frequency,
                        prefilter_settings: value.prefilter_settings.into(),
                        composite_mode: value.composite_mode.into(),
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub struct BloomPrefilterSettingsInput {
                pub threshold: f32,
                pub threshold_softness: f32,
            }
            impl From<BloomPrefilterSettingsInput> for BloomPrefilterSettings {
                fn from(value: BloomPrefilterSettingsInput) -> Self {
                    Self {
                        threshold: value.threshold,
                        threshold_softness: value.threshold_softness,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum BloomCompositeModeInput {
                EnergyConserving,
                Additive,
            }
            impl From<BloomCompositeModeInput> for BloomCompositeMode {
                fn from(value: BloomCompositeModeInput) -> Self {
                    match value {
                        BloomCompositeModeInput::EnergyConserving => Self::EnergyConserving,
                        BloomCompositeModeInput::Additive => Self::Additive,
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = Camera2dInput)]
            struct Camera2d {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct Camera2dInput {
                pub clear_color: ClearColorConfigInput,
            }
            impl From<Camera2dInput> for Camera2d {
                fn from(value: Camera2dInput) -> Self {
                    Self {
                        clear_color: value.clear_color.into(),
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = Camera3dInput)]
            struct Camera3d {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct Camera3dInput {
                pub clear_color: ClearColorConfigInput,
                pub depth_load_op: Camera3dDepthLoadOpInput,
            }
            impl From<Camera3dInput> for Camera3d {
                fn from(value: Camera3dInput) -> Self {
                    Self {
                        clear_color: value.clear_color.into(),
                        depth_load_op: value.depth_load_op.into()
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum Camera3dDepthLoadOpInput {
                Clear(f32),
                Load,
            }
            impl From<Camera3dDepthLoadOpInput> for Camera3dDepthLoadOp {
                fn from(value: Camera3dDepthLoadOpInput) -> Self {
                    match value {
                        Camera3dDepthLoadOpInput::Clear(value) => Self::Clear(value),
                        Camera3dDepthLoadOpInput::Load => Self::Load,
                    }
                }
            }
        }

        impl_external_schematic! {
            enum DebandDither {}
        }

        impl_external_schematic! {
            #[schematic(from = DepthPrepassInput)]
            struct DepthPrepass {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct DepthPrepassInput;
            impl From<DepthPrepassInput> for DepthPrepass {
                fn from(_: DepthPrepassInput) -> Self {
                    Self
                }
            }
        }

        impl_external_schematic! {
            struct Fxaa {}
        }

        impl_external_schematic! {
            #[schematic(from = NormalPrepassInput)]
            struct NormalPrepass {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct NormalPrepassInput;
            impl From<NormalPrepassInput> for NormalPrepass {
                fn from(_: NormalPrepassInput) -> Self {
                    Self
                }
            }
        }

        impl_external_schematic! {
            enum Tonemapping {}
        }

        #[derive(Reflect, FromReflect)]
        pub enum ClearColorConfigInput {
            Default,
            Custom(Color),
            None,
        }
        impl From<ClearColorConfigInput> for ClearColorConfig {
            fn from(value: ClearColorConfigInput) -> Self {
                match value {
                    ClearColorConfigInput::Default => Self::Default,
                    ClearColorConfigInput::Custom(color) => Self::Custom(color),
                    ClearColorConfigInput::None => Self::None,
                }
            }
        }
    }

    #[cfg(feature = "bevy_gltf")]
    pub mod gltf {
        use bevy::app::App;
        use bevy::gltf::GltfExtras;
        use bevy::reflect::{FromReflect, Reflect};

        use bevy_proto_derive::impl_external_schematic;

        pub fn register(app: &mut App) {
            register_schematic!(app, GltfExtras);
        }

        impl_external_schematic! {
            #[schematic(from = GltfExtrasInput)]
            struct GltfExtras {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct GltfExtrasInput{
                pub value: String,
            }
            impl From<GltfExtrasInput> for GltfExtras {
                fn from(value: GltfExtrasInput) -> Self {
                    Self {
                        value: value.value,
                    }
                }
            }
        }
    }

    #[cfg(feature = "bevy_pbr")]
    pub mod pbr {
        use bevy::app::App;
        use bevy::math::{UVec3, Vec3};
        use bevy::pbr::wireframe::Wireframe;
        use bevy::pbr::{
            AlphaMode, CascadeShadowConfig, CascadeShadowConfigBuilder, ClusterConfig,
            ClusterZConfig, DirectionalLight, EnvironmentMapLight, FogFalloff, FogSettings,
            NotShadowCaster, NotShadowReceiver, PointLight, SpotLight,
        };
        use bevy::prelude::Color;
        use bevy::reflect::{FromReflect, Reflect};

        use bevy_proto_derive::impl_external_schematic;

        pub fn register(app: &mut App) {
            register_schematic!(
                app,
                AlphaMode,
                CascadeShadowConfig,
                ClusterConfig,
                DirectionalLight,
                EnvironmentMapLight,
                FogSettings,
                NotShadowCaster,
                NotShadowReceiver,
                PointLight,
                SpotLight,
                Wireframe,
            );
        }

        impl_external_schematic! {
            enum AlphaMode {}
        }

        impl_external_schematic! {
            #[schematic(from = CascadeShadowConfigInput)]
            struct CascadeShadowConfig {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct CascadeShadowConfigInput {
                pub num_cascades: usize,
                pub minimum_distance: f32,
                pub maximum_distance: f32,
                pub first_cascade_far_bound: f32,
                pub overlap_proportion: f32,
            }
            impl From<CascadeShadowConfigInput> for CascadeShadowConfig {
                fn from(value: CascadeShadowConfigInput) -> Self {
                    CascadeShadowConfigBuilder {
                        num_cascades: value.num_cascades,
                        minimum_distance: value.minimum_distance,
                        maximum_distance: value.maximum_distance,
                        first_cascade_far_bound: value.first_cascade_far_bound,
                        overlap_proportion: value.overlap_proportion,
                    }.into()
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = ClusterConfigInput)]
            enum ClusterConfig {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub enum ClusterConfigInput {
                None,
                Single,
                XYZ {
                    dimensions: UVec3,
                    z_config: ClusterZConfig,
                    dynamic_resizing: bool,
                },
                FixedZ {
                    total: u32,
                    z_slices: u32,
                    z_config: ClusterZConfig,
                    dynamic_resizing: bool,
                },
            }
            impl From<ClusterConfigInput> for ClusterConfig {
                fn from(value: ClusterConfigInput) -> Self {
                    match value {
                        ClusterConfigInput::None => Self::None,
                        ClusterConfigInput::Single => Self::Single,
                        ClusterConfigInput::XYZ {
                            dimensions,
                            z_config,
                            dynamic_resizing,
                        } => Self::XYZ {dimensions, z_config, dynamic_resizing},
                        ClusterConfigInput::FixedZ {
                            total,
                            z_slices,
                            z_config,
                            dynamic_resizing,
                        } => Self::FixedZ {total, z_slices, z_config, dynamic_resizing},
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = DirectionalLightInput)]
            struct DirectionalLight {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct DirectionalLightInput {
                pub color: Color,
                pub illuminance: f32,
                pub shadows_enabled: bool,
                pub shadow_depth_bias: f32,
                pub shadow_normal_bias: f32,
            }
            impl From<DirectionalLightInput> for DirectionalLight {
                fn from(value: DirectionalLightInput) -> Self {
                    Self {
                        color: value.color,
                        illuminance: value.illuminance,
                        shadows_enabled: value.shadows_enabled,
                        shadow_depth_bias: value.shadow_depth_bias,
                        shadow_normal_bias: value.shadow_normal_bias,
                    }
                }
            }
        }

        impl_external_schematic! {
            pub struct EnvironmentMapLight {
                #[schematic(asset)]
                pub diffuse_map: Handle<Image>,
                #[schematic(asset)]
                pub specular_map: Handle<Image>,
            }
        }

        impl_external_schematic! {
            #[schematic(from = FogSettingsInput)]
            struct FogSettings {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct FogSettingsInput {
                pub color: Color,
                pub directional_light_color: Color,
                pub directional_light_exponent: f32,
                pub falloff: FogFalloffInput,
            }
            impl From<FogSettingsInput> for FogSettings {
                fn from(value: FogSettingsInput) -> Self {
                    Self {
                        color: value.color,
                        directional_light_color: value.directional_light_color,
                        directional_light_exponent: value.directional_light_exponent,
                        falloff: value.falloff.into(),
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum FogFalloffInput {
                Linear {
                    start: f32,
                    end: f32,
                },
                Exponential {
                    density: f32,
                },
                ExponentialSquared {
                    density: f32,
                },
                Atmospheric {
                    extinction: Vec3,
                    inscattering: Vec3,
                },
            }
            impl From<FogFalloffInput> for FogFalloff {
                fn from(value: FogFalloffInput) -> Self {
                    match value {
                        FogFalloffInput::Linear {start, end} => Self::Linear {start, end},
                        FogFalloffInput::Exponential {density} => Self::Exponential {density},
                        FogFalloffInput::ExponentialSquared {density} => Self::ExponentialSquared {density},
                        FogFalloffInput::Atmospheric {extinction, inscattering} => Self::Atmospheric {extinction, inscattering},
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = NotShadowCasterInput)]
            struct NotShadowCaster;
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct NotShadowCasterInput;
            impl From<NotShadowCasterInput> for NotShadowCaster {
                fn from(_: NotShadowCasterInput) -> Self {
                    Self
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = NotShadowReceiverInput)]
            struct NotShadowReceiver;
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct NotShadowReceiverInput;
            impl From<NotShadowReceiverInput> for NotShadowReceiver {
                fn from(_: NotShadowReceiverInput) -> Self {
                    Self
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = PointLightInput)]
            struct PointLight {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct PointLightInput {
                pub color: Color,
                pub intensity: f32,
                pub range: f32,
                pub radius: f32,
                pub shadows_enabled: bool,
                pub shadow_depth_bias: f32,
                pub shadow_normal_bias: f32,
            }
            impl From<PointLightInput> for PointLight {
                fn from(value: PointLightInput) -> Self {
                    Self {
                        color: value.color,
                        intensity: value.intensity,
                        range: value.range,
                        radius: value.radius,
                        shadows_enabled: value.shadows_enabled,
                        shadow_depth_bias: value.shadow_depth_bias,
                        shadow_normal_bias: value.shadow_normal_bias,
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = SpotLightInput)]
            struct SpotLight {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct SpotLightInput {
                pub color: Color,
                pub intensity: f32,
                pub range: f32,
                pub radius: f32,
                pub shadows_enabled: bool,
                pub shadow_depth_bias: f32,
                pub shadow_normal_bias: f32,
                pub outer_angle: f32,
                pub inner_angle: f32,
            }
            impl From<SpotLightInput> for SpotLight {
                fn from(value: SpotLightInput) -> Self {
                    Self {
                        color: value.color,
                        intensity: value.intensity,
                        range: value.range,
                        radius: value.radius,
                        shadows_enabled: value.shadows_enabled,
                        shadow_depth_bias: value.shadow_depth_bias,
                        shadow_normal_bias: value.shadow_normal_bias,
                        outer_angle: value.outer_angle,
                        inner_angle: value.inner_angle,
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = WireframeInput)]
            struct Wireframe;
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct WireframeInput;
            impl From<WireframeInput> for Wireframe {
                fn from(_: WireframeInput) -> Self {
                    Self
                }
            }
        }
    }

    #[cfg(feature = "bevy_render")]
    pub mod render {
        use bevy::app::App;
        use bevy::prelude::{
            Camera, Entity, OrthographicProjection, PerspectiveProjection, Projection,
        };
        use bevy::reflect::{FromReflect, Reflect};
        use bevy::render::camera::CameraRenderGraph;
        use bevy::render::mesh::skinning::SkinnedMesh;
        use bevy::render::primitives::Aabb;
        use bevy::render::view::{ColorGrading, RenderLayers, Visibility};

        use crate::tree::ProtoEntityList;
        use bevy_proto_derive::impl_external_schematic;

        pub fn register(app: &mut App) {
            register_schematic!(
                app,
                Aabb,
                Camera,
                CameraRenderGraph,
                ColorGrading,
                OrthographicProjection,
                PerspectiveProjection,
                Projection,
                RenderLayers,
                SkinnedMesh,
                Visibility,
            );
        }

        impl_external_schematic! {
            struct Aabb {}
        }

        impl_external_schematic! {
            struct Camera {}
        }

        impl_external_schematic! {
            #[schematic(from = CameraRenderGraphInput)]
            struct CameraRenderGraph();
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct CameraRenderGraphInput(String);
            impl From<CameraRenderGraphInput> for CameraRenderGraph {
                fn from(value: CameraRenderGraphInput) -> Self {
                    Self::new(value.0)
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = ColorGradingInput)]
            struct ColorGrading {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct ColorGradingInput {
                pub exposure: f32,
                pub gamma: f32,
                pub pre_saturation: f32,
                pub post_saturation: f32,
            }
            impl From<ColorGradingInput> for ColorGrading {
                fn from(value: ColorGradingInput) -> Self {
                    Self {
                        exposure: value.exposure,
                        gamma: value.gamma,
                        pre_saturation: value.pre_saturation,
                        post_saturation: value.post_saturation,
                    }
                }
            }
        }

        impl_external_schematic! {
            struct OrthographicProjection {}
        }

        impl_external_schematic! {
            struct PerspectiveProjection {}
        }

        impl_external_schematic! {
            #[schematic(from = ProjectionInput)]
            enum Projection {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub enum ProjectionInput {
                Perspective(PerspectiveProjection),
                Orthographic(OrthographicProjection),
            }
            impl From<ProjectionInput> for Projection {
                fn from(value: ProjectionInput) -> Self {
                    match value {
                        ProjectionInput::Perspective(projection) => Self::Perspective(projection),
                        ProjectionInput::Orthographic(projection) => Self::Orthographic(projection),
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = RenderLayersInput)]
            struct RenderLayers();
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct RenderLayersInput(u8);
            impl From<RenderLayersInput> for RenderLayers {
                fn from(value: RenderLayersInput) -> Self {
                    Self::layer(value.0)
                }
            }
        }

        impl_external_schematic! {
            pub struct SkinnedMesh {
                #[schematic(asset)]
                pub inverse_bindposes: Handle<SkinnedMeshInverseBindposes>,
                #[schematic(from = ProtoEntityList)]
                pub joints: Vec<Entity>,
            }
        }

        impl_external_schematic! {
            enum Visibility {}
        }
    }

    #[cfg(feature = "bevy_sprite")]
    pub mod sprite {
        use bevy::app::App;
        use bevy::math::Vec2;
        use bevy::prelude::Color;
        use bevy::reflect::{FromReflect, Reflect};
        use bevy::sprite::{Anchor, Mesh2dHandle, Sprite, TextureAtlasSprite};

        use bevy_proto_derive::impl_external_schematic;

        pub fn register(app: &mut App) {
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
            pub struct TextureAtlasSpriteInput {
                pub color: Color,
                pub index: usize,
                pub flip_x: bool,
                pub flip_y: bool,
                pub custom_size: Option<Vec2>,
                pub anchor: Anchor,
            }
            impl From<TextureAtlasSpriteInput> for TextureAtlasSprite {
                fn from(value: TextureAtlasSpriteInput) -> Self {
                    Self {
                        color: value.color,
                        index: value.index,
                        flip_x: value.flip_x,
                        flip_y: value.flip_y,
                        custom_size: value.custom_size,
                        anchor: value.anchor,
                    }
                }
            }
        }
    }

    #[cfg(feature = "bevy_text")]
    pub mod text {
        use bevy::app::App;
        use bevy::math::Vec2;
        use bevy::reflect::{FromReflect, Reflect};
        use bevy::text::{BreakLineOn, Text, Text2dBounds, TextAlignment, TextSection};

        use bevy_proto_derive::impl_external_schematic;

        pub fn register(app: &mut App) {
            register_schematic!(app, Text, Text2dBounds);
        }

        impl_external_schematic! {
            #[schematic(from = TextInput)]
            struct Text {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct TextInput {
                pub sections: Vec<TextSection>,
                pub alignment: TextAlignmentInput,
                pub linebreak_behaviour: BreakLineOnInput,
            }
            impl From<TextInput> for Text {
                fn from(value: TextInput) -> Self {
                    Self {
                        sections: value.sections,
                        alignment: value.alignment.into(),
                        linebreak_behaviour: value.linebreak_behaviour.into(),
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum TextAlignmentInput {
                Left,
                Center,
                Right,
            }
            impl From<TextAlignmentInput> for TextAlignment {
                fn from(value: TextAlignmentInput) -> Self {
                    match value {
                        TextAlignmentInput::Left => Self::Left,
                        TextAlignmentInput::Center => Self::Center,
                        TextAlignmentInput::Right => Self::Right,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum BreakLineOnInput {
                WordBoundary,
                AnyCharacter,
            }
            impl From<BreakLineOnInput> for BreakLineOn {
                fn from(value: BreakLineOnInput) -> Self {
                    match value {
                        BreakLineOnInput::WordBoundary => Self::WordBoundary,
                        BreakLineOnInput::AnyCharacter => Self::AnyCharacter,
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = Text2dBoundsInput)]
            struct Text2dBounds {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct Text2dBoundsInput {
                pub size: Vec2,
            }
            impl From<Text2dBoundsInput> for Text2dBounds {
                fn from(value: Text2dBoundsInput) -> Self {
                    Self {
                        size: value.size,
                    }
                }
            }
        }
    }

    pub mod transform {
        use bevy::app::App;
        use bevy::transform::components::{GlobalTransform, Transform};

        use bevy_proto_derive::impl_external_schematic;

        pub fn register(app: &mut App) {
            register_schematic!(app, Transform, GlobalTransform);
        }

        // FIXME: `TransformBundle` does not impl `Reflect`
        // impl_external_schematic! {
        //     #[schematic(from = TransformBundleInput)]
        //     struct TransformBundle {}
        //     // ---
        //     #[derive(Reflect, FromReflect)]
        //     pub struct TransformBundleInput {
        //         local: Transform,
        //         global: GlobalTransform
        //     }
        //     impl From<TransformBundleInput> for TransformBundle {
        //         fn from(value: TransformBundleInput) -> Self {
        //             Self {
        //                 local: value.local,
        //                 global: value.global
        //             }
        //         }
        //     }
        // }

        impl_external_schematic! {
            struct Transform {}
        }

        impl_external_schematic! {
            struct GlobalTransform {}
        }
    }

    #[cfg(feature = "bevy_ui")]
    pub mod ui {
        use bevy::app::App;
        use bevy::math::{Rect, Vec2};
        use bevy::prelude::{BackgroundColor, Button, Color, Label};
        use bevy::reflect::{FromReflect, Reflect};
        use bevy::ui::{
            AlignContent, AlignItems, AlignSelf, CalculatedClip, CalculatedSize, Direction,
            Display, FlexDirection, FlexWrap, FocusPolicy, Interaction, JustifyContent, Overflow,
            PositionType, RelativeCursorPosition, Size, Style, UiImage, UiRect, Val, ZIndex,
        };

        use bevy_proto_derive::impl_external_schematic;

        pub fn register(app: &mut App) {
            register_schematic!(
                app,
                BackgroundColor,
                Button,
                CalculatedClip,
                CalculatedSize,
                FocusPolicy,
                Interaction,
                Label,
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
            pub struct BackgroundColorInput(pub Color);
            impl From<BackgroundColorInput> for BackgroundColor {
                fn from(value: BackgroundColorInput) -> Self {
                    Self(value.0)
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = ButtonInput)]
            struct Button;
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct ButtonInput;
            impl From<ButtonInput> for Button {
                fn from(_: ButtonInput) -> Self {
                    Self
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = CalculatedClipInput)]
            struct CalculatedClip {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct CalculatedClipInput {
                pub clip: Rect,
            }
            impl From<CalculatedClipInput> for CalculatedClip {
                fn from(value: CalculatedClipInput) -> Self {
                    Self {
                        clip: value.clip,
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = CalculatedSizeInput)]
            struct CalculatedSize {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct CalculatedSizeInput {
                pub size: Vec2,
                pub preserve_aspect_ratio: bool,
            }
            impl From<CalculatedSizeInput> for CalculatedSize {
                fn from(value: CalculatedSizeInput) -> Self {
                    Self {
                        size: value.size,
                        preserve_aspect_ratio: value.preserve_aspect_ratio,
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = FocusPolicyInput)]
            enum FocusPolicy {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub enum FocusPolicyInput {
                Block,
                Pass,
            }
            impl From<FocusPolicyInput> for FocusPolicy {
                fn from(value: FocusPolicyInput) -> Self {
                    match value {
                        FocusPolicyInput::Block => Self::Block,
                        FocusPolicyInput::Pass => Self::Pass,
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = InteractionInput)]
            enum Interaction {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub enum InteractionInput {
                Clicked,
                Hovered,
                None,
            }
            impl From<InteractionInput> for Interaction {
                fn from(value: InteractionInput) -> Self {
                    match value {
                        InteractionInput::Clicked => Self::Clicked,
                        InteractionInput::Hovered => Self::Hovered,
                        InteractionInput::None => Self::None,
                    }
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
            #[schematic(from = RelativeCursorPositionInput)]
            struct RelativeCursorPosition {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct RelativeCursorPositionInput {
                pub normalized: Option<Vec2>,
            }
            impl From<RelativeCursorPositionInput> for RelativeCursorPosition {
                fn from(value: RelativeCursorPositionInput) -> Self {
                    Self {
                        normalized: value.normalized,
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = StyleInput)]
            struct Style {}
            // ---
            #[derive(Reflect, FromReflect)]
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
            impl From<StyleInput> for Style {
                fn from(value: StyleInput) -> Self {
                    Self {
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
            }
            #[derive(Reflect, FromReflect)]
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
            impl From<AlignContentInput> for AlignContent {
                fn from(value: AlignContentInput) -> Self {
                    match value {
                        AlignContentInput::Start => Self::Start,
                        AlignContentInput::End => Self::End,
                        AlignContentInput::FlexStart => Self::FlexStart,
                        AlignContentInput::FlexEnd => Self::FlexEnd,
                        AlignContentInput::Center => Self::Center,
                        AlignContentInput::Stretch => Self::Stretch,
                        AlignContentInput::SpaceBetween => Self::SpaceBetween,
                        AlignContentInput::SpaceEvenly => Self::SpaceEvenly,
                        AlignContentInput::SpaceAround => Self::SpaceAround,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum AlignItemsInput {
                Start,
                End,
                FlexStart,
                FlexEnd,
                Center,
                Baseline,
                Stretch,
            }
            impl From<AlignItemsInput> for AlignItems {
                fn from(value: AlignItemsInput) -> Self {
                    match value {
                        AlignItemsInput::Start => Self::Start,
                        AlignItemsInput::End => Self::End,
                        AlignItemsInput::FlexStart => Self::FlexStart,
                        AlignItemsInput::FlexEnd => Self::FlexEnd,
                        AlignItemsInput::Center => Self::Center,
                        AlignItemsInput::Baseline => Self::Baseline,
                        AlignItemsInput::Stretch => Self::Stretch,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
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
            impl From<AlignSelfInput> for AlignSelf {
                fn from(value: AlignSelfInput) -> Self {
                    match value {
                        AlignSelfInput::Auto => Self::Auto,
                        AlignSelfInput::Start => Self::Start,
                        AlignSelfInput::End => Self::End,
                        AlignSelfInput::FlexStart => Self::FlexStart,
                        AlignSelfInput::FlexEnd => Self::FlexEnd,
                        AlignSelfInput::Center => Self::Center,
                        AlignSelfInput::Baseline => Self::Baseline,
                        AlignSelfInput::Stretch => Self::Stretch,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum DirectionInput {
                Inherit,
                LeftToRight,
                RightToLeft,
            }
            impl From<DirectionInput> for Direction {
                fn from(value: DirectionInput) -> Self {
                    match value {
                        DirectionInput::Inherit => Self::Inherit,
                        DirectionInput::LeftToRight => Self::LeftToRight,
                        DirectionInput::RightToLeft => Self::RightToLeft,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum DisplayInput {
                None,
                Flex,
            }
            impl From<DisplayInput> for Display {
                fn from(value: DisplayInput) -> Self {
                    match value {
                        DisplayInput::None => Self::None,
                        DisplayInput::Flex => Self::Flex,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum FlexWrapInput {
                NoWrap,
                Wrap,
                WrapReverse,
            }
            impl From<FlexWrapInput> for FlexWrap {
                fn from(value: FlexWrapInput) -> Self {
                    match value {
                        FlexWrapInput::NoWrap => Self::NoWrap,
                        FlexWrapInput::Wrap => Self::Wrap,
                        FlexWrapInput::WrapReverse => Self::WrapReverse,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum FlexDirectionInput {
                Row,
                Column,
                RowReverse,
                ColumnReverse,
            }
            impl From<FlexDirectionInput> for FlexDirection {
                fn from(value: FlexDirectionInput) -> Self {
                    match value {
                        FlexDirectionInput::Row => Self::Row,
                        FlexDirectionInput::Column => Self::Column,
                        FlexDirectionInput::RowReverse => Self::RowReverse,
                        FlexDirectionInput::ColumnReverse => Self::ColumnReverse,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
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
            impl From<JustifyContentInput> for JustifyContent {
                fn from(value: JustifyContentInput) -> Self {
                    match value {
                        JustifyContentInput::Start => Self::Start,
                        JustifyContentInput::End => Self::End,
                        JustifyContentInput::FlexStart => Self::FlexStart,
                        JustifyContentInput::FlexEnd => Self::FlexEnd,
                        JustifyContentInput::Center => Self::Center,
                        JustifyContentInput::SpaceBetween => Self::SpaceBetween,
                        JustifyContentInput::SpaceEvenly => Self::SpaceEvenly,
                        JustifyContentInput::SpaceAround => Self::SpaceAround,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum OverflowInput {
                Visible,
                Hidden,
            }
            impl From<OverflowInput> for Overflow {
                fn from(value: OverflowInput) -> Self {
                    match value {
                        OverflowInput::Visible => Self::Visible,
                        OverflowInput::Hidden => Self::Hidden,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum PositionTypeInput {
                Relative,
                Absolute,
            }
            impl From<PositionTypeInput> for PositionType {
                fn from(value: PositionTypeInput) -> Self {
                    match value {
                        PositionTypeInput::Relative => Self::Relative,
                        PositionTypeInput::Absolute => Self::Absolute,
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub struct SizeInput {
                pub width: ValInput,
                pub height: ValInput,
            }
            impl From<SizeInput> for Size {
                fn from(value: SizeInput) -> Self {
                    Self {
                        width: value.width.into(),
                        height: value.height.into(),
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub struct UiRectInput {
                pub left: ValInput,
                pub right: ValInput,
                pub top: ValInput,
                pub bottom: ValInput,
            }
            impl From<UiRectInput> for UiRect {
                fn from(value: UiRectInput) -> Self {
                    Self {
                        left: value.left.into(),
                        right: value.right.into(),
                        top: value.top.into(),
                        bottom: value.bottom.into(),
                    }
                }
            }
            #[derive(Reflect, FromReflect)]
            pub enum ValInput {
                Undefined,
                Auto,
                Px(f32),
                Percent(f32),
            }
            impl From<ValInput> for Val {
                fn from(value: ValInput) -> Self {
                    match value {
                        ValInput::Undefined => Self::Undefined,
                        ValInput::Auto => Self::Auto,
                        ValInput::Px(value) => Self::Px(value),
                        ValInput::Percent(value) => Self::Percent(value),
                    }
                }
            }
        }

        impl_external_schematic! {
            pub struct UiImage {
                #[schematic(asset)]
                pub texture: Handle<Image>,
                pub flip_x: bool,
                pub flip_y: bool,
            }
        }

        impl_external_schematic! {
            #[schematic(from = ZIndexInput)]
            enum ZIndex {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub enum ZIndexInput {
                Local(i32),
                Global(i32),
            }
            impl From<ZIndexInput> for ZIndex {
                fn from(value: ZIndexInput) -> Self {
                    match value {
                        ZIndexInput::Local(z) => Self::Local(z),
                        ZIndexInput::Global(z) => Self::Global(z),
                    }
                }
            }
        }
    }

    pub mod window {
        use bevy::app::App;
        use bevy::reflect::{FromReflect, Reflect};
        use bevy::window::{PrimaryWindow, Window};

        use bevy_proto_derive::impl_external_schematic;

        pub fn register(app: &mut App) {
            register_schematic!(app, Window, PrimaryWindow);
        }

        impl_external_schematic! {
            struct Window {}
        }

        impl_external_schematic! {
            #[schematic(from = PrimaryWindowInput)]
            struct PrimaryWindow;
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct PrimaryWindowInput;
            impl From<PrimaryWindowInput> for PrimaryWindow {
                fn from(_: PrimaryWindowInput) -> Self {
                    Self
                }
            }
        }
    }
}
