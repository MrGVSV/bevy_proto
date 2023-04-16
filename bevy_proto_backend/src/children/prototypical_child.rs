use std::fmt::Debug;
use std::hash::Hash;

use bevy::prelude::Handle;

use crate::proto::Prototypical;

/// The child type for a [prototype].
///
/// [prototype]: Prototypical
pub trait PrototypicalChild<T: Prototypical> {
    /// The key used to identify this child as unique.
    ///
    /// This allows children with the same key to be merged
    /// (assuming they can be).
    type Key: Clone + Debug + Eq + Hash + Send + Sync;

    /// Get this child's handle.
    ///
    /// Note that a strong handle should _always_ be stored.
    /// Otherwise, the child may be accidentally unloaded.
    fn handle(&self) -> &Handle<T>;

    /// The optional merge key for this child.
    ///
    /// If defined, compatible children with the same key will be merged
    /// into a single entity.
    fn merge_key(&self) -> Option<&Self::Key> {
        None
    }
}

/// Type alias for [`PrototypicalChild::Key`].
pub(crate) type MergeKey<T> = <<T as Prototypical>::Child as PrototypicalChild<T>>::Key;
