use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use crate::proto::Prototypical;

/// The node type used in a [`CycleChecker`].
///
/// Note that the variants do not matter for the [`PartialEq`] or [`Hash`]
/// implementations for this enum— only the contained IDs matter.
///
/// [`CycleChecker`]: crate::cycles::CycleChecker
pub(crate) enum CycleNode<'a, T: Prototypical> {
    /// This node represents a prototype's template.
    Template { id: Cow<'a, T::Id> },
    /// This node represents a prototype's child.
    Child { id: Cow<'a, T::Id> },
}

impl<'a, T: Prototypical> CycleNode<'a, T> {
    pub fn id(&self) -> &T::Id {
        match self {
            Self::Template { id, .. } => id.as_ref(),
            Self::Child { id, .. } => id.as_ref(),
        }
    }
}

impl<'a, T: Prototypical> Clone for CycleNode<'a, T> {
    fn clone(&self) -> Self {
        match self {
            CycleNode::Template { id } => Self::Template { id: id.clone() },
            CycleNode::Child { id } => Self::Child { id: id.clone() },
        }
    }
}

impl<'a, T: Prototypical> Debug for CycleNode<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CycleNode::Template { id } => f.debug_struct("Template").field("id", id).finish(),
            CycleNode::Child { id } => f.debug_struct("Child").field("id", id).finish(),
        }
    }
}

impl<'a, T: Prototypical> Eq for CycleNode<'a, T> {}

impl<'a, T: Prototypical> PartialEq<Self> for CycleNode<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<'a, T: Prototypical> Hash for CycleNode<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
