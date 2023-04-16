use std::path::PathBuf;

use thiserror::Error;

use bevy_proto_backend::schematics::SchematicError;

/// Error type for a [`Prototype`].
///
/// [`Prototype`]: crate::prelude::Prototype
#[derive(Debug, Error)]
pub enum PrototypeError {
    /// The path of the prototype being loaded is missing an extension.
    #[error("expected extension")]
    MissingExtension(PathBuf),
    /// The path of the prototype being loaded has an unsupported extension.
    #[error("extension {0:?} is not supported")]
    UnsupportedExtension(String),
    /// Error loading RON file.
    #[cfg(feature = "ron")]
    #[error("RON error in {0:?}: {1}")]
    SpannedRonError(PathBuf, ron::de::SpannedError),
    /// Error loading YAML file.
    #[cfg(feature = "yaml")]
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    SchematicError(#[from] SchematicError),
}
