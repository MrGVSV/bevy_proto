//! Schematic types used to build [prototypes].
//!
//! A [`Schematic`] is the basic building-block for prototypes.
//! At its core, it represents some change in the world.
//!
//! Normally, schematics are used to modify a single entity,
//! often inserting a component or bundle.
//! However, they can do any number of operations.
//! They can modify existing components, insert resources,
//! load assets, and so on.
//!
//! # Deserialization
//!
//! Schematics are deserialized into a [`DynamicSchematic`] object.
//! This is used to store type-erased schematic data to be used generically.
//! In order for this to work correctly, it is important to remember to register
//! schematics to the type registry along with the [`ReflectSchematic`] type data.
//!
//! Registration must happen manually via [`App::register_type`] or with [`App::register_type_data`]
//! (which will register type data and the type if it hasn't been registered already).
//! The `ReflectSchematic` type data can be registered using `App::register_type_data`
//! or by including `#[reflect(Schematic)]` on the type definition.
//!
//! # Entity-ness
//!
//! Currently, schematics are [applied] and [removed] on a per-entity basis.
//! This means that even if a schematic doesn't require an entity,
//! it will still be given one on which to operate.
//!
//! In the future, this requirement will be lifted and schematics will be able
//! to request that an entity not be spawned when it's applied.
//!
//! [prototypes]: crate::proto::Prototypical
//! [`Schematic`]: schematic::Schematic
//! [`App::register_type`]: bevy::app::App::register_type
//! [`App::register_type_data`]: bevy::app::App::register_type_data
//! [applied]: Schematic::apply
//! [removed]: Schematic::remove

pub use bevy_proto_derive::Schematic;
pub use collection::*;
pub use context::*;
pub use dynamic::*;
pub use error::*;
pub use id::*;
pub use schematic::*;

mod collection;
mod context;
mod dynamic;
mod error;
mod id;
mod schematic;
