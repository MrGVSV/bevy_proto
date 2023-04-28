#[cfg(feature = "bevy_animation")]
pub mod animation;
pub mod asset;
pub mod core;
#[cfg(feature = "bevy_core_pipeline")]
pub mod core_pipeline;
#[cfg(feature = "bevy_gltf")]
pub mod gltf;
#[cfg(feature = "bevy_pbr")]
pub mod pbr;
#[cfg(feature = "bevy_render")]
pub mod render;
#[cfg(feature = "bevy_sprite")]
pub mod sprite;
#[cfg(feature = "bevy_text")]
pub mod text;
pub mod transform;
#[cfg(feature = "bevy_ui")]
pub mod ui;
pub mod window;

pub(super) fn register_impls(app: &mut bevy::app::App) {
    asset::register(app);
    core::register(app);
    transform::register(app);
    window::register(app);

    #[cfg(feature = "bevy_animation")]
    animation::register(app);
    #[cfg(feature = "bevy_core_pipeline")]
    core_pipeline::register(app);
    #[cfg(feature = "bevy_gltf")]
    gltf::register(app);
    #[cfg(feature = "bevy_pbr")]
    pbr::register(app);
    #[cfg(feature = "bevy_render")]
    render::register(app);
    #[cfg(feature = "bevy_sprite")]
    sprite::register(app);
    #[cfg(feature = "bevy_text")]
    text::register(app);
    #[cfg(feature = "bevy_ui")]
    ui::register(app);
}
