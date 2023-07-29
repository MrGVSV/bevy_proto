use bevy::app::App;
use bevy::pbr::wireframe::Wireframe;
use bevy::pbr::{
    CascadeShadowConfig, CascadeShadowConfigBuilder, ClusterConfig, DirectionalLight,
    EnvironmentMapLight, FogFalloff, FogSettings, NotShadowCaster, NotShadowReceiver, PointLight,
    SpotLight,
};
use bevy::reflect::{std_traits::ReflectDefault, Reflect};

use crate::impls::macros::{from_to_default, register_schematic};
use crate::proto::ProtoColor;
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(
        app,
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
    #[schematic(from = CascadeShadowConfigInput)]
    struct CascadeShadowConfig {}
    // ---
    #[derive(Reflect)]
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
    enum ClusterConfig {}
}

impl_external_schematic! {
    struct DirectionalLight {}
}

impl_external_schematic! {
    pub struct EnvironmentMapLight {
        #[schematic(asset(lazy))]
        pub diffuse_map: Handle<Image>,
        #[schematic(asset(lazy))]
        pub specular_map: Handle<Image>,
    }
}

impl_external_schematic! {
    #[schematic(from = FogSettingsInput)]
    struct FogSettings {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct FogSettingsInput {
        pub color: ProtoColor,
        pub directional_light_color: ProtoColor,
        pub directional_light_exponent: f32,
        pub falloff: FogFalloff,
    }
    from_to_default! {
        FogSettings,
        FogSettingsInput,
        |value: Input| Self {
            color: value.color.into(),
            directional_light_color: value.directional_light_color.into(),
            directional_light_exponent: value.directional_light_exponent,
            falloff: value.falloff,
        }
    }
}

impl_external_schematic! {
    struct NotShadowCaster;
}

impl_external_schematic! {
    struct NotShadowReceiver;
}

impl_external_schematic! {
    struct PointLight {}
}

impl_external_schematic! {
    struct SpotLight {}
}

impl_external_schematic! {
    struct Wireframe;
}
