use crate::manager::ProtoId;
use bevy::prelude::Entity;
use std::ffi::OsString;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtoLoadError {
    #[error("could not find a type registration for {name}")]
    NotRegistered { name: String },
    #[error("could not reflect `ProtoComponent` for {name}. Did you add a `#[reflect(ProtoComponent)]` to your type?")]
    MissingReflection { name: String },
    #[error("could not reflect {name}")]
    BadReflection { name: String },
    #[error("{name} is not whitelisted")]
    NotWhitelisted { name: String },
    #[error("{name} is blacklisted")]
    Blacklisted { name: String },
    #[error("missing deserializer")]
    MissingDeserializer,
    #[error("unknown extension: {ext:?}")]
    UnknownExtension { ext: OsString },
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ProtoSpawnError {
    #[error("Found a circular dependency in the following prototypes: {tree}. {msg}")]
    CircularDependency { tree: String, msg: String },
    #[error("Prototype is not currently loaded: {id:?}")]
    NotLoaded { id: ProtoId },
    #[error("Could not find a handle for prototype named: {name}")]
    MissingHandle { name: String },
    #[error("The given entity does not exist in the world: {entity:?}. Was it despawned?")]
    InvalidEntity { entity: Entity },
}
