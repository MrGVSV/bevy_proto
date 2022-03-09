#![warn(missing_docs)]
//! Serializable entity configuration for the Bevy game engine.
//!
//! This crate provides several abstractions for specifying serializable entities and components:
//! - The [`ProtoComponent`](components::ProtoComponent) trait provides methods to load components from assets.
//! - The [`ProtoDeserializer`](data::ProtoDeserializer) trait describes component deserialization.
//! - [`ProtoPlugin`](plugin::ProtoPlugin) provides configuration for asset loading.
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
