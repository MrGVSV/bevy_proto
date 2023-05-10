use crate::load::ProtoLoadContext;
use crate::proto::Prototypical;
use crate::schematics::SchematicError;
use bevy::asset::{AssetPath, Handle};
use bevy::prelude::FromWorld;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

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

    /// Callback for when a [prototype] is loaded.
    ///
    /// This is called right after deserialization, but before any preprocessing.
    ///
    /// This can be used to modify the prototype before it is processed,
    /// handle side effects, or even reject the prototype with an error.
    ///
    /// By default, this will do nothing and return the prototype as-is.
    ///
    /// Note: Currently, this logic can also be implemented in [`deserialize`], however,
    /// this may change in the future so it's best to put this kind of logic here.
    ///
    /// [prototype]: Prototypical
    /// [`deserialize`]: Loader::deserialize
    fn on_load_prototype(&self, prototype: T, meta: &ProtoLoadMeta<T>) -> Result<T, Self::Error> {
        let _ = meta;
        Ok(prototype)
    }
}

/// Metadata about a [prototype] that is being loaded.
///
/// [prototype]: Prototypical
pub struct ProtoLoadMeta<T: Prototypical> {
    /// The path to the prototype.
    pub path: AssetPath<'static>,
    /// A strong handle to the prototype.
    pub handle: Handle<T>,
    /// The depth of this prototype in the raw prototype tree.
    ///
    /// Children loaded by path are not included in this depth
    /// since they are loaded separately.
    pub depth: usize,
}

impl<T: Prototypical> Debug for ProtoLoadMeta<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProtoLoadMeta")
            .field("path", &self.path)
            .field("depth", &self.depth)
            .field("handle", &self.handle)
            .finish()
    }
}

impl<T: Prototypical> Clone for ProtoLoadMeta<T> {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            handle: self.handle.clone(),
            depth: self.depth,
        }
    }
}

impl<T: Prototypical> Eq for ProtoLoadMeta<T> {}

impl<T: Prototypical> PartialEq for ProtoLoadMeta<T> {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth && self.handle == other.handle && self.path == other.path
    }
}

impl<T: Prototypical> Hash for ProtoLoadMeta<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.handle.hash(state);
        self.depth.hash(state);
    }
}
