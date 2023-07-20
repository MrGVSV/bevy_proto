use bevy::app::App;
use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::core_3d::{Camera3dDepthLoadOp, Camera3dDepthTextureUsage};
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::prepass::{DepthPrepass, NormalPrepass};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::{Camera2d, Camera3d, Color};
use bevy::reflect::{std_traits::ReflectDefault, Reflect};

use crate::impls::macros::{from_to, from_to_default, register_schematic};
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
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

    // Can be removed if https://github.com/bevyengine/bevy/pull/5781 is ever merged
    app.register_type::<BloomPrefilterSettingsInput>()
        .register_type::<BloomCompositeModeInput>()
        .register_type::<ClearColorConfigInput>()
        .register_type::<Camera3dDepthLoadOpInput>();
}

impl_external_schematic! {
    #[schematic(from = BloomSettingsInput)]
    struct BloomSettings {}
    // ---
    #[derive(Reflect)]
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

    #[derive(Reflect)]
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

    #[derive(Reflect)]
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
    #[derive(Reflect)]
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
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct Camera3dInput {
        pub clear_color: ClearColorConfigInput,
        pub depth_load_op: Camera3dDepthLoadOpInput,
        pub depth_texture_usages: Camera3dDepthTextureUsage,
    }
    from_to_default!(
        Camera3d,
        Camera3dInput,
        |value: Input| Self {
            depth_texture_usages: value.depth_texture_usages.into(),
            clear_color: value.clear_color.into(),
            depth_load_op: value.depth_load_op.into()
        }
    );

    #[derive(Reflect)]
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
    #[derive(Reflect)]
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
    #[derive(Reflect)]
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

#[derive(Reflect)]
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
