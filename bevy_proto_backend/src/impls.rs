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

macro_rules! from_to {
    ($real: ty, $mock: ty, $body: expr) => {
        const _: () = {
            type Input = $real;

            impl From<Input> for $mock {
                fn from(value: Input) -> Self {
                    $body(value)
                }
            }
        };

        const _: () = {
            type Input = $mock;

            impl From<Input> for $real {
                fn from(value: Input) -> Self {
                    $body(value)
                }
            }
        };
    };
}

macro_rules! from_to_default {
    ($real: ty, $mock: ty, $body: expr) => {
        from_to!($real, $mock, $body);

        const _: () = {
            impl Default for $mock {
                fn default() -> Self {
                    <$real as Default>::default().into()
                }
            }
        };
    };
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
        use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};

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
            #[reflect(Default)]
            pub struct BloomSettingsInput {
                pub intensity: f32,
                pub low_frequency_boost: f32,
                pub low_frequency_boost_curvature: f32,
                pub high_pass_frequency: f32,
                pub prefilter_settings: BloomPrefilterSettingsInput,
                pub composite_mode: BloomCompositeModeInput,
            }
            from_to_default!(
                BloomSettings,
                BloomSettingsInput,
                |value: Input| Self {
                    intensity: value.intensity,
                    low_frequency_boost: value.low_frequency_boost,
                    low_frequency_boost_curvature: value.low_frequency_boost_curvature,
                    high_pass_frequency: value.high_pass_frequency,
                    prefilter_settings: value.prefilter_settings.into(),
                    composite_mode: value.composite_mode.into(),
                }
            );

            #[derive(Reflect, FromReflect)]
            #[reflect(Default)]
            pub struct BloomPrefilterSettingsInput {
                pub threshold: f32,
                pub threshold_softness: f32,
            }
            from_to_default!(
                BloomPrefilterSettings,
                BloomPrefilterSettingsInput,
                |value: Input| Self {
                    threshold: value.threshold,
                    threshold_softness: value.threshold_softness,
                }
            );

            #[derive(Reflect, FromReflect)]
            pub enum BloomCompositeModeInput {
                EnergyConserving,
                Additive,
            }
            from_to!(
                BloomCompositeMode,
                BloomCompositeModeInput,
                |value| match value {
                    Input::EnergyConserving => Self::EnergyConserving,
                    Input::Additive => Self::Additive,
                }
            );
        }

        impl_external_schematic! {
            #[schematic(from = Camera2dInput)]
            struct Camera2d {}
            // ---
            #[derive(Reflect, FromReflect)]
            #[reflect(Default)]
            pub struct Camera2dInput {
                pub clear_color: ClearColorConfigInput,
            }
            from_to_default!(
                Camera2d,
                Camera2dInput,
                |value: Input| Self {
                    clear_color: value.clear_color.into(),
                }
            );
        }

        impl_external_schematic! {
            #[schematic(from = Camera3dInput)]
            struct Camera3d {}
            // ---
            #[derive(Reflect, FromReflect)]
            #[reflect(Default)]
            pub struct Camera3dInput {
                pub clear_color: ClearColorConfigInput,
                pub depth_load_op: Camera3dDepthLoadOpInput,
            }
            from_to_default!(
                Camera3d,
                Camera3dInput,
                |value: Input| Self {
                    clear_color: value.clear_color.into(),
                    depth_load_op: value.depth_load_op.into()
                }
            );

            #[derive(Reflect, FromReflect)]
            #[reflect(Default)]
            pub enum Camera3dDepthLoadOpInput {
                Clear(f32),
                Load,
            }
            from_to_default!(
                Camera3dDepthLoadOp,
                Camera3dDepthLoadOpInput,
                |value: Input| match value {
                    Input::Clear(value) => Self::Clear(value),
                    Input::Load => Self::Load,
                }
            );
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
        #[reflect(Default)]
        pub enum ClearColorConfigInput {
            Default,
            Custom(Color),
            None,
        }
        from_to_default!(
            ClearColorConfig,
            ClearColorConfigInput,
            |value| match value {
                Input::Default => Self::Default,
                Input::Custom(color) => Self::Custom(color),
                Input::None => Self::None,
            }
        );
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
            from_to_default!(
                GltfExtras,
                GltfExtrasInput,
                |value: Input| Self {
                    value: value.value,
                }
            );
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
        use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};

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
            #[reflect(Default)]
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
            impl Default for CascadeShadowConfigInput {
                fn default() -> Self {
                    let base = CascadeShadowConfigBuilder::default();
                    Self {
                        num_cascades: base.num_cascades,
                        minimum_distance: base.minimum_distance,
                        maximum_distance: base.maximum_distance,
                        first_cascade_far_bound: base.first_cascade_far_bound,
                        overlap_proportion: base.overlap_proportion,
                    }
                }
            }
        }

        impl_external_schematic! {
            #[schematic(from = ClusterConfigInput)]
            enum ClusterConfig {}
            // ---
            #[derive(Reflect, FromReflect)]
            #[reflect(Default)]
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
            from_to_default!(
                ClusterConfig,
                ClusterConfigInput,
                |value: Input| match value {
                    Input::None => Self::None,
                    Input::Single => Self::Single,
                    Input::XYZ {
                        dimensions,
                        z_config,
                        dynamic_resizing,
                    } => Self::XYZ {dimensions, z_config, dynamic_resizing},
                    Input::FixedZ {
                        total,
                        z_slices,
                        z_config,
                        dynamic_resizing,
                    } => Self::FixedZ {total, z_slices, z_config, dynamic_resizing},
                }
            );
        }

        impl_external_schematic! {
            #[schematic(from = DirectionalLightInput)]
            struct DirectionalLight {}
            // ---
            #[derive(Reflect, FromReflect)]
            #[reflect(Default)]
            pub struct DirectionalLightInput {
                pub color: Color,
                pub illuminance: f32,
                pub shadows_enabled: bool,
                pub shadow_depth_bias: f32,
                pub shadow_normal_bias: f32,
            }
            from_to_default! {
                DirectionalLight,
                DirectionalLightInput,
                |value: Input| Self {
                    color: value.color,
                    illuminance: value.illuminance,
                    shadows_enabled: value.shadows_enabled,
                    shadow_depth_bias: value.shadow_depth_bias,
                    shadow_normal_bias: value.shadow_normal_bias,
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
            #[reflect(Default)]
            pub struct FogSettingsInput {
                pub color: Color,
                pub directional_light_color: Color,
                pub directional_light_exponent: f32,
                pub falloff: FogFalloffInput,
            }
            from_to_default! {
                FogSettings,
                FogSettingsInput,
                |value: Input| Self {
                    color: value.color,
                    directional_light_color: value.directional_light_color,
                    directional_light_exponent: value.directional_light_exponent,
                    falloff: value.falloff.into(),
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
            from_to! {
                FogFalloff,
                FogFalloffInput,
                |value: Input| match value {
                    Input::Linear {start, end} => Self::Linear {start, end},
                    Input::Exponential {density} => Self::Exponential {density},
                    Input::ExponentialSquared {density} => Self::ExponentialSquared {density},
                    Input::Atmospheric {extinction, inscattering} => Self::Atmospheric {extinction, inscattering},
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
            #[reflect(Default)]
            pub struct PointLightInput {
                pub color: Color,
                pub intensity: f32,
                pub range: f32,
                pub radius: f32,
                pub shadows_enabled: bool,
                pub shadow_depth_bias: f32,
                pub shadow_normal_bias: f32,
            }
            from_to_default! {
                PointLight,
                PointLightInput,
                |value: Input| Self {
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

        impl_external_schematic! {
            #[schematic(from = SpotLightInput)]
            struct SpotLight {}
            // ---
            #[derive(Reflect, FromReflect)]
            #[reflect(Default)]
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
            from_to_default! {
                SpotLight,
                SpotLightInput,
                |value: Input| Self {
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
        use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};
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
            #[reflect(Default)]
            pub struct ColorGradingInput {
                pub exposure: f32,
                pub gamma: f32,
                pub pre_saturation: f32,
                pub post_saturation: f32,
            }
            from_to_default! {
                ColorGrading,
                ColorGradingInput,
                |value: Input| Self {
                    exposure: value.exposure,
                    gamma: value.gamma,
                    pre_saturation: value.pre_saturation,
                    post_saturation: value.post_saturation,
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
            #[reflect(Default)]
            pub enum ProjectionInput {
                Perspective(PerspectiveProjection),
                Orthographic(OrthographicProjection),
            }
            from_to_default! {
                Projection,
                ProjectionInput,
                |value: Input| match value {
                    Input::Perspective(projection) => Self::Perspective(projection),
                    Input::Orthographic(projection) => Self::Orthographic(projection),
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
        use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};
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
            #[reflect(Default)]
            pub struct TextureAtlasSpriteInput {
                pub color: Color,
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

    #[cfg(feature = "bevy_text")]
    pub mod text {
        use bevy::app::App;
        use bevy::math::Vec2;
        use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};
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
        use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};
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
            pub struct UiImage {
                #[schematic(asset)]
                pub texture: Handle<Image>,
                #[reflect(default)]
                pub flip_x: bool,
                #[reflect(default)]
                pub flip_y: bool,
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
