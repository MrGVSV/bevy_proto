use bevy::app::App;
use bevy::reflect::Reflect;
use bevy::window::{PrimaryWindow, Window};

use crate::impls::macros::register_schematic;
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, Window, PrimaryWindow);
}

impl_external_schematic! {
    struct Window {}
}

impl_external_schematic! {
    #[schematic(from = PrimaryWindowInput)]
    struct PrimaryWindow;
    // ---
    #[derive(Reflect)]
    pub struct PrimaryWindowInput;
    impl From<PrimaryWindowInput> for PrimaryWindow {
        fn from(_: PrimaryWindowInput) -> Self {
            Self
        }
    }
}
