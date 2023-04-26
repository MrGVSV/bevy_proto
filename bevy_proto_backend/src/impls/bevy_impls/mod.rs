#[cfg(feature = "bevy_animation")]
mod animation;
mod asset;
mod core;
#[cfg(feature = "bevy_core_pipeline")]
mod core_pipeline;
#[cfg(feature = "bevy_gltf")]
mod gltf;
#[cfg(feature = "bevy_pbr")]
mod pbr;
#[cfg(feature = "bevy_render")]
mod render;
#[cfg(feature = "bevy_sprite")]
mod sprite;
#[cfg(feature = "bevy_text")]
mod text;
mod transform;
#[cfg(feature = "bevy_ui")]
mod ui;
mod window;

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
