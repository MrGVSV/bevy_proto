use std::fmt::{Debug, Formatter};
use std::slice::Iter;

use crate::children::PrototypicalChild;
use crate::proto::Prototypical;

/// A collection of [children] for a [prototype].
///
/// # Order
///
/// Insertion order matters.
/// The order in which children are inserted is the order in which they
/// will be spawned and used.
/// This may be important for scenarios that may rely on the order of children
/// such as when using [`ProtoEntity`] to access a child entity.
///
/// [prototype]: Prototypical
/// [children]: PrototypicalChild
/// [`ProtoEntity`]: crate::tree::ProtoEntity
pub struct Children<T: Prototypical> {
    children: Vec<T::Child>,
}

impl<T: Prototypical> Children<T> {
    /// Insert a new child into the collection.
    ///
    /// # Panics
    ///
    /// Panics if the child's handle is weak.
    pub fn insert(&mut self, child: T::Child) {
        debug_assert!(
            child.handle().is_strong(),
            "child inserted with weak handle"
        );
        self.children.push(child)
    }

    /// Iterate over the children in the order they were inserted.
    pub fn iter(&self) -> Iter<'_, T::Child> {
        self.children.iter()
    }
}

impl<T: Prototypical> Default for Children<T> {
    fn default() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl<T: Prototypical> Debug for Children<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Children(")?;
        f.debug_list()
            .entries(self.iter().map(|child| child.handle()))
            .finish()?;
        write!(f, ")")
    }
}
