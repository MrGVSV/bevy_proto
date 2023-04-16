use thiserror::Error;

/// [`Schematic`]-related error.
///
/// [`Schematic`]: crate::schematics::Schematic
#[derive(Debug, Error)]
pub enum SchematicError {
    /// A call to a [`FromReflect`] method failed.
    ///
    /// This should normally never happen for types that derive `FromReflect`.
    /// Manual implementors should double-check their logic to ensure all edge-cases
    /// are accounted for.
    ///
    /// [`FromReflect`]: bevy::reflect::FromReflect
    #[error("a call to a `FromReflect` method failed")]
    FromReflectFail,
    /// An invalid type was passed.
    #[error("expected type `{expected}` but found `{found}`")]
    TypeMismatch {
        expected: &'static str,
        found: String,
    },
}
