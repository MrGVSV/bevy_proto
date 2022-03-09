#![warn(missing_docs)]
//! Serializable entity configuration for the Bevy game engine.
//!
//! This crate provides several abstractions for specifying serializable entities and components:
//! - The [`ProtoComponent`](components::ProtoComponent) trait provides methods to load components from assets.
//! - The [`ProtoDeserializer`](data::ProtoDeserializer) trait describes component deserialization.
//! - [`ProtoPlugin`](plugin::ProtoPlugin) provides configuration for asset loading.
//!
//! # Examples
//! Define a serialized prototype:
//! ```
//! use bevy::prelude::*;
//! use bevy_proto::prelude::*;
//! use serde::{Deserialize, Serialize};
//!
//! // Define a serialized prototype.
//! // In this example we would load this from a .yaml file in "assets/prototypes".
//!
//! // name: "Simple Enemy"
//! // components:
//! //     - type: Enemy
//! //     - type: Attack
//! //       value:
//! //         damage: 10
//!
//! // Implement `ProtoComponent` for the component types:
//!
//! #[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
//! struct Enemy;
//!
//! #[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
//! struct Attack {
//!     damage: u16
//! }
//!
//! // Add the plugin:
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
//!
//! // Finally, spawn a prototype with a system:
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

pub mod components;
pub mod data;
pub mod plugin;
pub mod prototype;
#[macro_use]
mod utils;

pub mod prelude {
    //! Includes all public types and the macro to derive [`ProtoComponent`](super::components::ProtoComponent).

    pub use super::components::*;
    pub use super::data::*;
    pub use super::plugin::*;
    pub use super::prototype::{Prototype, Prototypical};
    pub use bevy_proto_derive::*;
}
