use bevy::app::App;
use bevy::core::Name;
use bevy::reflect::Reflect;

use crate::impls::macros::register_schematic;
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(app, Name);
}

impl_external_schematic! {
    #[schematic(from = NameInput)]
    struct Name {}
    // ---
    #[derive(Reflect)]
    pub struct NameInput(String);
    impl From<NameInput> for Name {
        fn from(input: NameInput) -> Self {
            Name::new(input.0)
        }
    }
}
