pub mod bevy_impls;
mod macros;

pub(crate) fn register_impls(app: &mut bevy::app::App) {
    bevy_impls::register_impls(app);
}
