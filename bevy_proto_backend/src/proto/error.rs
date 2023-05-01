use bevy::asset::{AssetPath, HandleUntyped};
use thiserror::Error;

/// The main error type for [prototype]-related operations.
///
/// [prototype]: crate::proto::Prototypical
#[derive(Debug, Error)]
pub enum ProtoError {
    /// Indicates that a [prototype] contains a [cycle].
    ///
    /// [prototype]: crate::proto::Prototypical
    /// [cycle]: crate::cycles
    #[error("found prototype cycle: `{cycle}`")]
    ContainsCycle {
        /// The identified cycle.
        cycle: String,
    },
    /// Indicates that a [prototype] with the given handle doesn't exist
    /// or isn't (fully) loaded.
    ///
    /// [prototype]: crate::proto::Prototypical
    #[error("the prototype with handle {0:?} either doesn't exist or isn't fully loaded")]
    DoesNotExist(HandleUntyped),
    /// Indicates that a prototype with the given handle is not registered.
    #[error("the prototype with handle {0:?} is not registered")]
    NotRegistered(HandleUntyped),
    /// Indicates that a prototype tried to be registered with an existing ID.
    #[error("attempted to register prototype with ID {id:?} (`{path:?}`), but one already exists with this ID (`{existing:?}`)")]
    AlreadyExists {
        id: String,
        path: Box<AssetPath<'static>>,
        existing: Box<AssetPath<'static>>,
    },
    /// Indicates that an operation that requires an entity was attempted on a prototype that doesn't require one.
    ///
    /// This includes attempting to register children on an entity-less prototype.
    #[error("expected prototype with ID {id:?} to require an entity")]
    RequiresEntity { id: String },
}
