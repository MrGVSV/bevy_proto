use crate::load::ProtoLoadContext;
use crate::proto::Prototypical;
use crate::schematics::SchematicError;
use bevy::prelude::FromWorld;
use std::error::Error;

/// Configures how a [prototype] should be loaded.
///
/// [prototype]: Prototypical
pub trait Loader<T: Prototypical>: FromWorld + Clone + Send + Sync + 'static {
    /// Error type returned by this loader during deserialization and loading.
    type Error: Error + From<SchematicError> + Send + Sync;

    /// Deserialize the given slice of bytes into an instance of `T`.
    fn deserialize(bytes: &[u8], ctx: &mut ProtoLoadContext<T, Self>) -> Result<T, Self::Error>;

    /// A list of supported extensions.
    ///
    /// Extensions should be in order of most-specific to least specific,
    /// and should not be prepended by a dot (`.`).
    /// Generally, this means it should be in order of longest to shortest.
    ///
    /// For example, this could return:
    ///
    /// ```
    /// # use bevy::prelude::Resource;
    /// # use bevy_proto_backend::proto::{Config, Prototypical};
    /// # use bevy_proto_backend::load::{Loader, ProtoLoadContext};
    /// use bevy_proto_backend::schematics::SchematicError;
    /// # #[derive(Default, Clone)]
    /// struct MyPrototypeLoader;
    /// impl<T: Prototypical> Loader<T> for MyPrototypeLoader {
    /// #  type Error = SchematicError;
    ///   fn extensions(&self) -> &[&'static str] {
    ///     &[
    ///       // Most Specific (Longest) //
    ///       "prototype.yaml",
    ///       "prototype.ron",
    ///       "yaml",
    ///       "ron",
    ///       // Least Specific (Shortest) //
    ///     ]
    ///   }
    /// #  fn deserialize(bytes: &[u8], ctx: &mut ProtoLoadContext<T, Self>) -> Result<T, Self::Error> {
    /// #    todo!()
    /// #  }
    /// }
    /// ```
    fn extensions(&self) -> &[&'static str];
}
