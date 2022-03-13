#![warn(missing_docs)]
//! Serializable entity configuration for the Bevy game engine.
//!
//! This crate provides several abstractions for specifying serializable entities and components:
//! - The [`ProtoComponent`](components::ProtoComponent) trait provides methods to load components from assets.
//! - The [`ProtoDeserializer`](data::ProtoDeserializer) trait describes component deserialization.
//! - [`ProtoPlugin`](plugin::ProtoPlugin) provides configuration for asset loading.
//!
//! # Examples
//!
//! Define a serialized prototype:
//! ```yaml
//! # assets/prototypes/simple-enemy.yaml
//! name: "Simple Enemy"
//! components:
//!     - type: Enemy
//!     - type: Attack
//!       value:
//!         damage: 10
//! ```
//!
//! Implement `ProtoComponent` for the component types:
//! ```
//! use bevy::prelude::*;
//! use bevy_proto::prelude::*;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
//! struct Enemy;
//!
//! #[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
//! struct Attack {
//!     damage: u16
//! }
//! ```
//!
//! Add the plugin:
//! ```
//! use bevy::prelude::*;
//! use bevy_proto::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!   
//!         .add_plugin(ProtoPlugin {
//!             options: Some(ProtoDataOptions {
//!                // You can also change the prototype directories here
//!                directories: vec![String::from("assets/prototypes")],
//!                // And specify whether you want the prototype files to be recursively loaded
//!                recursive_loading: false,
//!                // You can also update the allowed extensions within those directories
//!                extensions: Some(vec!["yaml", "json"]),
//!                ..ProtoDataOptions::default()
//!            })
//!         });
//! }
//! ```
//!
//! Finally, spawn a prototype with a system:
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_proto::prelude::*;
//!
//! fn spawn_enemy(mut commands: Commands, data: Res<ProtoData>, asset_server: Res<AssetServer>) {
//!     let proto = data.get_prototype("Simple Enemy").expect("Prototype doesn't exist!");
//!
//!     // Spawns in our "Simple Enemy" Prototype
//!     proto.spawn(&mut commands, &data, &asset_server);
//! }
//!
//! ```
//!
extern crate bevy_proto_derive;

mod components;
pub use bevy_proto_derive::ProtoComponent;
pub use components::ProtoComponent;
mod plugin;
pub use plugin::ProtoPlugin;
mod prototype;
pub use prototype::{deserialize_templates_list, Prototype, Prototypical};

pub mod data;
#[macro_use]
mod utils;

pub mod prelude {
    //! Includes all public types and the macro to derive [`ProtoComponent`](super::components::ProtoComponent).

    pub use super::components::ProtoComponent;
    pub use super::data::*;
    pub use super::plugin::ProtoPlugin;
    pub use super::prototype::{Prototype, Prototypical};
    pub use bevy_proto_derive::*;
}

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
