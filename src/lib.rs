#![warn(missing_docs)]
//! Serializable entity configuration for the Bevy game engine.
//!
//! This crate provides several abstractions for specifying serializable entities and components:
//! * A [`ProtoComponent`] trait that defines how to apply zero or more components/bundles to an entity
//! * A [`Prototypical`] trait that defines how to store and apply these [`ProtoComponent`] objects
//!   - Includes a general-purpose [`Prototype`] struct that implements [`Prototypical`] by default
//! * [`ProtoPlugin`] for quickly inserting the functionality into Bevy apps
//!
//! # Examples
//!
//! Define a serialized prototype in Bevy Reflect format:
//!
//! ```yaml
//! # assets/prototypes/simple-enemy.prototype.yaml
//! name: "Simple Enemy"
//! components:
//!   - type: my_crate::Enemy
//!     struct: {}
//!   - type: my_crate::Attack
//!     struct:
//!       damage:
//!         type: i32
//!         value: 10
//! ```
//!
//! Implement [`ProtoComponent`] for the component types:
//!
//! ```
//! use bevy::prelude::*;
//! use bevy::reflect::FromReflect;
//! use bevy_proto::prelude::*;
//!
//! #[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
//! #[reflect(ProtoComponent)]
//! struct Enemy;
//!
//! #[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
//! #[reflect(ProtoComponent)]
//! struct Attack {
//!   damage: u16
//! }
//! ```
//!
//! Add the plugin and register your types:
//!
//! ```
//! use bevy::prelude::*;
//! use bevy::reflect::FromReflect;
//! use bevy_proto::prelude::*;
//!
//! # #[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
//! # #[reflect(ProtoComponent)]
//! # struct Enemy;
//! #
//! # #[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
//! # #[reflect(ProtoComponent)]
//! # struct Attack {
//! #   damage: u16
//! # }
//! #
//! fn main() {
//!   let app = App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(ProtoPlugin::<Prototype>::default())
//!     .register_type::<Enemy>()
//!     .register_type::<Attack>();
//!   // ...
//! }
//! ```
//!
//! Load your prototype:
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_proto::prelude::*;
//!
//! fn load_prototype(asset_server: Res<AssetServer>, mut manager: ProtoManager<Prototype>) {
//!   let handle: Handle<Prototype> = asset_server.load("prototypes/simple-enemy.prototype.yaml");
//!   manager.add(handle);
//! }
//! ```
//!
//! Finally, spawn the prototype:
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_proto::prelude::*;
//!
//! fn spawn_enemy(mut commands: Commands, manager: ProtoManager<Prototype>) {
//!   let proto = manager.get("Simple Enemy").expect("Prototype should exist!");
//!
//!   // Spawns in our "Simple Enemy" Prototype
//!   proto.spawn(&mut commands);
//! }
//!
//! ```
//!
//! [`ProtoComponent`]: components::ProtoComponent
//! [`Prototypical`]: prototypical::Prototypical
//! [`Prototype`]: prototype::Prototype
//! [`ProtoPlugin`]: plugin::ProtoPlugin
extern crate bevy_proto_derive;
extern crate self as bevy_proto;

pub use bevy_proto_derive::ProtoComponent;
pub use components::{ProtoComponent, ReflectProtoComponent};
pub use plugin::ProtoPlugin;
pub use prototype::Prototype;
pub use prototypical::Prototypical;

#[macro_use]
mod utils;
mod command;
mod components;
mod config;
mod deps;
mod errors;
mod handle;
mod loader;
mod manager;
mod plugin;
mod prototype;
mod prototypical;
pub mod serde;
mod templates;

pub mod prelude {
    //! Includes all public types and the macro to derive [`ProtoComponent`](super::components::ProtoComponent).

    pub use super::components::{ComponentList, ProtoComponent, ReflectProtoComponent};
    pub use super::config::{ProtoConfig, ProtoFilter};
    pub use super::deps::DependencyMap;
    pub use super::errors::ProtoLoadError;
    pub use super::handle::{HandlePath, StoreHandle};
    pub use super::loader::AssetPreloader;
    pub use super::manager::{ProtoId, ProtoIdRef, ProtoManager};
    pub use super::plugin::ProtoPlugin;
    pub use super::prototype::Prototype;
    pub use super::prototypical::Prototypical;
    pub use super::templates::TemplateList;
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
