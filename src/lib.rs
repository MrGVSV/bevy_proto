extern crate bevy_proto_derive;

pub mod components;
pub mod data;
pub mod plugin;
pub mod prototype;
#[macro_use]
mod utils;

pub mod prelude {
	pub use bevy_proto_derive::*;

	pub use super::components::*;
	pub use super::data::*;
	pub use super::plugin::*;
	pub use super::prototype::{Prototype, Prototypical};
}
