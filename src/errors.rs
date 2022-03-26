use bevy::asset::HandleId;
use std::ffi::OsString;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtoError {
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
    #[error("Prototype is not currently loaded: {handle:?}")]
    NotLoaded { handle: HandleId },
}
