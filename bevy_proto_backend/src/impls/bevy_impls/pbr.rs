use bevy::app::App;
use bevy::math::{UVec3, Vec3};
use bevy::pbr::wireframe::Wireframe;
use bevy::pbr::{
    AlphaMode, CascadeShadowConfig, CascadeShadowConfigBuilder, ClusterConfig, ClusterZConfig,
    DirectionalLight, EnvironmentMapLight, FogFalloff, FogSettings, NotShadowCaster,
    NotShadowReceiver, PointLight, SpotLight,
};
use bevy::prelude::Color;
use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};

use crate::impls::macros::{from_to, from_to_default, register_schematic};
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
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
