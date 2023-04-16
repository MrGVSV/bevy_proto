use std::error::Error;
use std::fmt::Debug;
use std::hash::Hash;

use bevy::asset::Asset;

use crate::children::{Children, PrototypicalChild};
use crate::deps::Dependencies;
use crate::load::ProtoLoadContext;
use crate::path::ProtoPath;
use crate::proto::config::Config;
use crate::schematics::{SchematicError, Schematics};
use crate::templates::Templates;

/// The trait used to define a prototype.
///
/// Prototypes are containers for [`Schematics`] that can be spawned in at runtime.
/// They may also inherit from other prototypes (where the inherited prototypes are known
/// as [`Templates`]) and contain a hierarchy of [`Children`].
pub trait Prototypical: Asset + Sized {
    /// The type used to identify this prototype.
    ///
    /// This can be used to control how a prototype is accessed.
    /// For example, if we wanted to split prototypes into "packages",
    /// then we could require a package identifier along with the
    /// prototype name in order to access the prototype:
    ///
    /// ```
    /// # use std::fmt::{Display, Formatter};
    /// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    /// struct MyProtoId {
    ///   package: String,
    ///   name: String,
    /// }
    ///
    /// impl Display for MyProtoId {
    ///   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    ///     write!(f, "{}::{}", self.package, self.name)
    ///   }
    /// }
    /// ```
    type Id: Clone + Debug + Eq + Hash + Send + Sync + ToString;
    /// The type of the child this prototype uses.
    ///
    /// This is used to configure how a child is processed.
    type Child: PrototypicalChild<Self>;
    /// The configuration type used for managing this prototype.
    type Config: Config<Self>;
    /// The error type used for deserializing.
    type Error: Error + From<SchematicError> + Send + Sync;

    /// The ID of this prototype.
    fn id(&self) -> &Self::Id;
    /// The path to this prototype's asset file.
    fn path(&self) -> &ProtoPath;
    /// An immutable reference to the collection of [`Schematics`] contained in this prototype.
    fn schematics(&self) -> &Schematics;
    /// A mutable reference to the collection of [`Schematics`] contained in this prototype.
    fn schematics_mut(&mut self) -> &mut Schematics;
    /// An immutable reference to the collection of [`Templates`] inherited by this prototype, if any.
    fn templates(&self) -> Option<&Templates>;
    /// A mutable reference to the collection of [`Templates`] inherited by this prototype, if any.
    fn templates_mut(&mut self) -> Option<&mut Templates>;
    /// An immutable reference to the collection of [`Dependencies`] used by this prototype.
    fn dependencies(&self) -> &Dependencies;
    /// A mutable reference to the collection of [`Dependencies`] used by this prototype.
    fn dependencies_mut(&mut self) -> &mut Dependencies;
    /// An immutable reference to the collection of [`Children`] contained in this prototype, if any.
    fn children(&self) -> Option<&Children<Self>>;
    /// A mutable reference to the collection of [`Children`] contained in this prototype, if any.
    fn children_mut(&mut self) -> Option<&mut Children<Self>>;
    /// Deserialize the given slice of bytes into an instance of [`Self`].
    fn deserialize(bytes: &[u8], ctx: &mut ProtoLoadContext<Self>) -> Result<Self, Self::Error>;
}
