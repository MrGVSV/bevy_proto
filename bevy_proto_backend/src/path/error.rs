use std::path::PathBuf;

use thiserror::Error;

/// Error enum used when creating a [`ProtoPath`].
///
/// [`ProtoPath`]: crate::path::ProtoPath
#[derive(Debug, Error)]
pub enum PathError {
    /// The path is malformed.
    ///
    /// This can happen when paths begin with invalid prefixes,
    /// such as Windows path prefixes (e.g. `C:`).
    #[error("received malformed path {0:?}")]
    MalformedPath(PathBuf),
    /// The base path is invalid.
    ///
    /// This can happen if a base path is missing a parent component.
    #[error("invalid base path {0:?}")]
    InvalidBase(PathBuf),
    /// The file with the given path does not exist.
    #[error("file does not exist {0:?}")]
    DoesNotExist(PathBuf),
    /// The base path is using an extension not found in the allowed
    /// extensions set from the respective [`Config`].
    ///
    /// [`Config`]: crate::proto::Config
    #[error("invalid extension {0:?}")]
    InvalidExtension(PathBuf),
    /// The path cannot be converted to a string.
    #[error("cannot convert path to a string")]
    ConversionError
}
