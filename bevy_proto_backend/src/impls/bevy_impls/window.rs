use bevy::app::App;
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
    struct PrimaryWindow;
}
