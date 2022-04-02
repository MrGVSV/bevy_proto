use crate::manager::ProtoId;
use bevy::prelude::Entity;
use std::ffi::OsString;
use thiserror::Error;

/// Errors related to loading a [prototypical] asset.
///
/// [prototypical]: crate::Prototypical
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ProtoLoadError {
    /// The given type is not registered in Bevy's type registry.
    #[error("could not find a type registration for {name}")]
    NotRegistered {
        /// The [name](std::any::type_name) of the type.
        name: String,
    },
    /// [`ReflectProtoComponent`] is not added to the reflected traits for the given type.
    ///
    /// This can be resolved by adding `#[reflect(ProtoComponent)]` to the type definition.
    ///
    /// [`ReflectProtoComponent`]: crate::components::ReflectProtoComponent
    #[error("could not reflect `ProtoComponent` for {name}. Did you add a `#[reflect(ProtoComponent)]` to your type?")]
    MissingReflection {
        /// The [name](std::any::type_name) of the type.
        name: String,
    },
    /// There was an issue reflecting this type.
    #[error("could not reflect {name}")]
    BadReflection {
        /// The [name](std::any::type_name) of the type.
        name: String,
    },
    /// The given [`ProtoComponent`] is not whitelisted.
    ///
    /// [`ProtoComponent`]: crate::ProtoComponent
    #[error("{name} is not whitelisted")]
    NotWhitelisted {
        /// The [name](std::any::type_name) of the type.
        name: String,
    },
    /// The given [`ProtoComponent`] is blacklisted.
    ///
    /// [`ProtoComponent`]: crate::ProtoComponent
    #[error("{name} is blacklisted")]
    Blacklisted {
        /// The [name](std::any::type_name) of the type.
        name: String,
    },
    /// Unknown extension found for a [prototypical] asset.
    ///
    /// [prototypical]: crate::Prototypical
    #[error("unknown extension: {ext:?}")]
    UnknownExtension {
        /// The extension.
        ext: OsString,
    },
}

/// Errors related to spawning a [prototypical] asset.
///
/// [prototypical]: crate::Prototypical
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ProtoSpawnError {
    /// A circular dependency was found.
    ///
    /// This typically happens when a template references a template that references itself.
    /// For example, the following will result in a circular dependency:
    ///
    /// ```text
    /// A: B, C
    /// B: A
    /// // Circular dependency:
    /// // A -> B -> A
    /// ```
    #[error("Found a circular dependency in the following prototypes: {tree}. {msg}")]
    CircularDependency {
        /// A visual representation of the circular dependency.
        tree: String,
        /// An optional message displayed at the end of the error to give more context.
        msg: String,
    },
    /// The given [prototypical] asset is not currently loaded.
    ///
    /// [prototypical]: crate::Prototypical
    #[error("Prototype is not currently loaded: {id:?}")]
    NotLoaded {
        /// The [ID](crate::prelude::ProtoId) of the prototype.
        id: ProtoId,
    },
    /// The entity to attach to does not exist or is invalid.
    #[error("The given entity does not exist in the world: {entity:?}. Was it despawned?")]
    InvalidEntity {
        /// The entity.
        entity: Entity,
    },
}
