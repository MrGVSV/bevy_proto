use bevy::app::App;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::core_3d::{Camera3dDepthLoadOp, Camera3dDepthTextureUsage};
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::prepass::{DepthPrepass, NormalPrepass};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::{Camera2d, Camera3d};
use bevy::reflect::{std_traits::ReflectDefault, Reflect};

use crate::impls::macros::{from_to_default, register_schematic};
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
}

impl_external_schematic! {
    struct BloomSettings {}
}

impl_external_schematic! {
    #[schematic(from = Camera2dInput)]
    struct Camera2d {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct Camera2dInput {
        pub clear_color: ClearColorConfig,
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
        pub clear_color: ClearColorConfig,
        pub depth_load_op: Camera3dDepthLoadOp,
        pub depth_texture_usages: Camera3dDepthTextureUsage,
    }
    from_to_default!(
        Camera3d,
        Camera3dInput,
        |value: Input| Self {
            clear_color: value.clear_color,
            depth_load_op: value.depth_load_op,
            depth_texture_usages: value.depth_texture_usages,
        }
    );
}

impl_external_schematic! {
    enum DebandDither {}
}

impl_external_schematic! {
    struct DepthPrepass {}
}

impl_external_schematic! {
    struct Fxaa {}
}

impl_external_schematic! {
    struct NormalPrepass {}
}

impl_external_schematic! {
    enum Tonemapping {}
}
