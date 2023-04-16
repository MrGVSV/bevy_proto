use std::borrow::{Borrow, Cow};
use std::fmt::{Debug, Display, Formatter};

use indexmap::IndexSet;

use crate::cycles::CycleNode;
use crate::proto::Prototypical;

/// A helper struct used to check for prototype cycles.
///
/// This essentially works like a stack.
/// When a node is [pushed] onto the stack when it already exists within it,
/// a [`Cycle`] is created.
///
/// [pushed]: CycleChecker::try_push
#[derive(Debug)]
pub(crate) struct CycleChecker<'a, T: Prototypical> {
    root: Cow<'a, T::Id>,
    ancestry: IndexSet<CycleNode<'a, T>>,
}

impl<'a, T: Prototypical> CycleChecker<'a, T> {
    pub fn new<I: Into<Cow<'a, T::Id>>>(root: I) -> Self {
        Self {
            root: root.into(),
            ancestry: IndexSet::new(),
        }
    }

    // Attempts to push a new node onto the stack, returning a [Cycle] if one is generated.
    pub fn try_push(&mut self, node: CycleNode<'a, T>) -> Result<(), Cycle<T>> {
        if self.root.as_ref() == node.id() {
            return Err(Cycle {
                curr: node,
                start_index: None,
                root: &self.root,
                ancestry: &self.ancestry,
            });
        }

        if let Some(start_index) = self.ancestry.get_index_of(&node) {
            return Err(Cycle {
                curr: node,
                start_index: Some(start_index),
                root: &self.root,
                ancestry: &self.ancestry,
            });
        }
        self.ancestry.insert(node);
        Ok(())
    }

    /// Pops the last node from the stack.
    pub fn pop(&mut self) {
        self.ancestry.pop();
    }
}

/// Contains information about a detected prototype cycle.
pub struct Cycle<'a, 'b, T: Prototypical> {
    curr: CycleNode<'a, T>,
    start_index: Option<usize>,
    root: &'b T::Id,
    ancestry: &'b IndexSet<CycleNode<'a, T>>,
}

impl<'a, 'b, T: Prototypical> Cycle<'a, 'b, T> {
    /// The id of the prototype that started the cycle.
    ///
    /// Note that the "start" of a cycle is not well-defined.
    /// Any ID within the cycle could be considered the start.
    /// The ID returned by this method just happens to be the
    /// what caused the cycle to be detected.
    pub fn id(&self) -> &T::Id {
        self.curr.id()
    }

    /// Returns true if the given ID is contained within the cycle.
    pub fn cycle_contains<I: Borrow<T::Id>>(&self, id: I) -> bool {
        let id = id.borrow();
        // Note: The node type doesn't matter here
        let node = CycleNode::Template {
            id: Cow::Borrowed(id),
        };
        self.curr == node
            || self.root == id
            || self
                .ancestry
                .get_index_of(&node)
                .map(|index| index >= self.start_index.unwrap_or_default())
                .unwrap_or_default()
    }

    /// Returns true if the given ID is contained within the ancestry tree.
    pub fn contains<I: Borrow<T::Id>>(&self, id: I) -> bool {
        let id = id.borrow();
        // Note: The node type doesn't matter here
        let node = CycleNode::Template {
            id: Cow::Borrowed(id),
        };
        self.curr == node || self.root == id || self.ancestry.contains(&node)
    }

    /// Returns an iterator over the cycle.
    ///
    /// To iterate over the full ancestry that led to the cycle,
    /// try using [`iter_full`](Self::iter_full).
    pub fn iter_cycle(&self) -> impl Iterator<Item = &T::Id> {
        self.ancestry
            .iter()
            .skip(self.start_index.unwrap_or_default())
            .map(|node| node.id())
            .chain(std::iter::once(self.curr.id()))
    }

    /// Returns an iterator over the full ancestry that led to the cycle.
    ///
    /// To iterate over just the cycle, try using [`iter_cycle`](Self::iter_cycle).
    pub fn iter_full(&self) -> impl Iterator<Item = &T::Id> {
        std::iter::once(self.root).chain(
            self.ancestry
                .iter()
                .map(|node| node.id())
                .chain(std::iter::once(self.curr.id())),
        )
    }
}

impl<'a, 'b, T: Prototypical> Debug for Cycle<'a, 'b, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        list.entries(self.ancestry.iter());
        list.entry(&self.curr);
        list.finish()
    }
}

impl<'a, 'b, T: Prototypical> Display for Cycle<'a, 'b, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let nodes = self
            .ancestry
            .iter()
            .skip(self.start_index.unwrap_or_default())
            .chain(std::iter::once(&self.curr))
            .enumerate();

        write!(f, "{:?}", self.root.to_string())?;

        for (index, node) in nodes {
            match node {
                CycleNode::Template { id } if index == 0 => {
                    write!(f, " inherits {:?}", id.to_string())?;
                }
                CycleNode::Child { id } if index == 0 => {
                    write!(f, " contains {:?}", id.to_string())?;
                }
                CycleNode::Template { id } => {
                    write!(f, " which inherits {:?}", id.to_string())?;
                }
                CycleNode::Child { id } => {
                    write!(f, " which contains {:?}", id.to_string())?;
                }
            }
        }
        Ok(())
    }
}
