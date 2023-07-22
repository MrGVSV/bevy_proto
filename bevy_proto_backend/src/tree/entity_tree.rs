use std::borrow::Cow;
use std::cell::Cell;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::iter::Rev;
use std::num::NonZeroUsize;

use bevy::asset::HandleId;
use bevy::ecs::system::Command;
use bevy::prelude::{AddChild, Entity, World};
use bevy::utils::HashMap;
use indexmap::set::Iter;
use indexmap::IndexSet;

use crate::proto::{ProtoInstance, Prototypical};
use crate::tree::{AccessOp, ChildAccess, EntityAccess, ProtoTree, SiblingAccess};

/// A tree structure containing all the entities to be mutated by a [prototype].
///
/// The tree is processed breadth-first by [`ProtoCommands`].
/// However, it is generated with all the necessary entities,
/// allowing those entities to be safely retrieved within a [`Schematic`].
///
/// [prototype]: Prototypical
/// [`ProtoCommands`]: crate::proto::ProtoCommands
/// [`Schematic`]: crate::schematics::Schematic
pub struct EntityTree<'a> {
    /// All the nodes in the tree.
    nodes: Box<[EntityTreeNode<'a>]>,
    /// A mapping of a node to its parent node (if it has one).
    parents: HashMap<usize, usize>,
    /// A mapping of a node to its collection of children.
    children: HashMap<usize, EntityChildren<'a>>,
    /// The current node index being processed.
    ///
    /// This must be manually updated during processing.
    current: Cell<usize>,
}

impl<'a> EntityTree<'a> {
    pub(crate) fn new<T: Prototypical>(
        tree: &'a ProtoTree<T>,
        root: Option<Entity>,
        world: &mut World,
    ) -> Self {
        let mut nodes = vec![EntityTreeNode {
            id: tree.id_str(),
            index: 0,
            entity: root,
            prototypes: tree.prototypes(),
        }];
        let mut queue = VecDeque::new();
        queue.push_back((0, root, tree));

        let mut parents = HashMap::<usize, usize>::new();
        let mut children = HashMap::<usize, EntityChildren>::new();

        while let Some((parent_index, parent_entity, tree)) = queue.pop_front() {
            let mut entity_children = EntityChildren::default();

            for child in tree.children() {
                let index = nodes.len();

                let child_index = entity_children.insert(index, child.id_str());
                parents.insert(index, parent_index);

                let entity = if child.requires_entity() {
                    Some(Self::init_entity(
                        ProtoInstance::new(child.handle(), child_index),
                        parent_entity,
                        world,
                    ))
                } else {
                    None
                };

                nodes.push(EntityTreeNode {
                    id: child.id_str(),
                    index,
                    entity,
                    prototypes: child.prototypes(),
                });

                queue.push_back((index, entity, child));
            }

            children.insert(parent_index, entity_children);
        }

        Self {
            nodes: nodes.into_boxed_slice(),
            parents,
            children,
            current: Cell::new(0),
        }
    }

    /// Get the current entity being processed, if any.
    pub fn entity(&self) -> Option<Entity> {
        self.current().entity
    }

    /// Find an entity in the tree using the given [`EntityAccess`].
    pub fn find_entity(&self, access: &EntityAccess) -> Option<Entity> {
        self.get(access).and_then(EntityTreeNode::entity)
    }

    pub(crate) fn get(&self, access: &EntityAccess) -> Option<&EntityTreeNode<'a>> {
        let mut current = self.current.get();
        for op in access.ops() {
            match op {
                AccessOp::Root => {
                    current = 0;
                }
                AccessOp::Parent => {
                    current = *self.parents.get(&current)?;
                }
                AccessOp::Child(ChildAccess::At(index)) => {
                    current = self.children.get(&current)?.get_at(*index)?;
                }
                AccessOp::Child(ChildAccess::Id(id, occurrence)) => {
                    let last_index = self
                        .children
                        .get(&current)?
                        .children
                        .len()
                        .saturating_sub(1);

                    let (start, end) = if occurrence.get().is_negative() {
                        (last_index, 0)
                    } else {
                        (0, last_index)
                    };

                    current = self.children.get(&current)?.get(
                        id,
                        start,
                        end,
                        occurrence.unsigned_abs(),
                    )?;
                }
                AccessOp::Sibling(SiblingAccess::At(offset)) => {
                    let parent = self.parents.get(&current)?;
                    let siblings = self.children.get(parent)?;
                    let child_index = siblings
                        .children
                        .iter()
                        .position(|child| *child == current)?;
                    let index = offset.get().saturating_add_unsigned(child_index);

                    current = siblings.get_at(index)?;
                }
                AccessOp::Sibling(SiblingAccess::Id(id, occurrence)) => {
                    let parent = self.parents.get(&current)?;
                    let siblings = self.children.get(parent)?;
                    let child_index = siblings
                        .children
                        .iter()
                        .position(|child| *child == current)?;

                    let (start, end) = if occurrence.get().is_negative() {
                        (child_index.saturating_sub(1), 0)
                    } else {
                        (
                            child_index.saturating_add(1),
                            siblings.len().saturating_sub(1),
                        )
                    };

                    current = siblings.get(id, start, end, occurrence.unsigned_abs())?;
                }
            }
        }

        self.nodes.get(current)
    }

    pub(crate) fn current(&self) -> &EntityTreeNode<'a> {
        &self.nodes[self.current.get()]
    }

    pub(crate) fn set_current(&self, node: &EntityTreeNode) {
        self.current.set(node.index);
    }

    pub(crate) fn iter(&self) -> EntityTreeIter<'_, 'a, '_> {
        EntityTreeIter::from_index(0, self)
    }

    /// Spawns an [`Entity`] with the proper parent-child relationship,
    /// along with any additional components.
    ///
    /// If a child entity with the given [`ProtoInstance`] already exists,
    /// it will be returned instead of spawning a new entity.
    fn init_entity(instance: ProtoInstance, parent: Option<Entity>, world: &mut World) -> Entity {
        if let Some(entity) = Self::find_existing_entity(&instance, parent, world) {
            return entity;
        }

        let entity = world.spawn(instance).id();
        if let Some(parent) = parent {
            Command::apply(
                AddChild {
                    parent,
                    child: entity,
                },
                world,
            );
        }
        entity
    }

    /// Attempts to find an existing child entity with the given [`ProtoInstance`].
    fn find_existing_entity(
        instance: &ProtoInstance,
        parent: Option<Entity>,
        world: &mut World,
    ) -> Option<Entity> {
        let parent = parent?;
        let children = world.get::<bevy::prelude::Children>(parent)?;
        for child in children {
            if let Some(child_instance) = world.get::<ProtoInstance>(*child) {
                if child_instance == instance {
                    return Some(*child);
                }
            }
        }

        None
    }
}

/// A wrapper around [`write!`] that respects pretty-formatting and indentation.
macro_rules! smart_write {
    ($dst:expr, $depth: expr, $($arg:tt)*) => {
        if $dst.alternate() {
            write!($dst, "{}", "\t".repeat($depth))?;
            writeln!($dst, $($arg)*)
        } else {
            write!($dst, $($arg)*)
        }
    };
}

impl<'a> Debug for EntityTree<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn recurse(
            depth: usize,
            is_last: bool,
            node: &EntityTreeNode,
            tree: &EntityTree,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            smart_write!(f, depth - 1, "EntityTree {{")?;
            smart_write!(f, depth, "id: {:?}, ", node.id)?;

            smart_write!(f, depth, "entity: {:?}, ", node.entity)?;

            match tree.children.get(&node.index) {
                None => {
                    smart_write!(f, depth, "children: []")?;
                }
                Some(children) if children.is_empty() => {
                    smart_write!(f, depth, "children: []")?;
                }
                Some(children) => {
                    smart_write!(f, depth, "children: [")?;

                    for (index, child) in children.iter().enumerate() {
                        let node = &tree.nodes[*child];
                        recurse(depth + 2, index + 1 >= children.len(), node, tree, f)?;
                    }

                    smart_write!(f, depth, "]")?;
                }
            }

            if !is_last {
                smart_write!(f, depth - 1, "}}, ")
            } else {
                smart_write!(f, depth - 1, "}}")
            }
        }

        recurse(1, true, &self.nodes[0], self, f)
    }
}

/// Node item stored in a [`EntityTree`].
///
/// This essentially represents a single entity
/// (or a single prototype with all template information flattened).
pub(crate) struct EntityTreeNode<'a> {
    id: &'a str,
    index: usize,
    entity: Option<Entity>,
    prototypes: &'a IndexSet<HandleId>,
}

impl<'a> EntityTreeNode<'a> {
    /// The stringified [`Prototypical::Id`] of the prototype.
    pub fn id(&self) -> &str {
        self.id
    }

    /// The corresponding [`Entity`], if any.
    pub fn entity(&self) -> Option<Entity> {
        self.entity
    }

    /// An iterator over this node's prototype and templates,
    /// in the order that they should be applied.
    pub fn prototypes(&self) -> Rev<Iter<'_, HandleId>> {
        self.prototypes.iter().rev()
    }
}

/// Metadata about a node's children.
#[derive(Default, Debug)]
struct EntityChildren<'a> {
    /// The node indexes of this node's children.
    children: Vec<usize>,
    /// The node indexes of the children for a given ID.
    id_to_child: HashMap<Cow<'a, str>, Vec<usize>>,
}

impl<'a> EntityChildren<'a> {
    /// Insert a new child.
    ///
    /// The given index should be the node index of the child in the entire tree,
    /// _not_ the index of the child among its siblings.
    fn insert<I: Into<Cow<'a, str>>>(&mut self, child: usize, id: I) -> usize {
        let index = self.children.len();
        self.children.push(child);
        self.id_to_child.entry(id.into()).or_default().push(index);
        index
    }

    /// Get the index of the child node with the given ID.
    ///
    /// The `start` and `end` determine both the search range and direction.
    /// The `occurrence` dictates the occurrence into that range to match on.
    pub fn get(
        &self,
        id: &str,
        start: usize,
        end: usize,
        occurrence: NonZeroUsize,
    ) -> Option<usize> {
        self.id_to_child.get(id).and_then(|occurrences| {
            let range = if start <= end {
                start..=end
            } else {
                end..=start
            };
            let mut occurrence = occurrence.get();

            // Determine the search direction of the occurrences
            let iter: Box<dyn Iterator<Item = &usize>> = if start <= end {
                Box::new(occurrences.iter())
            } else {
                Box::new(occurrences.iter().rev())
            };

            for child_index in iter {
                // If the child index is within the range, that counts as an occurrence
                if range.contains(child_index) {
                    if occurrence <= 1 {
                        // Target occurrence reached -> return tree index of child
                        return self.children.get(*child_index).copied();
                    } else {
                        // Move towards target occurrence
                        occurrence = occurrence.checked_sub(1)?;
                    }
                }
            }

            None
        })
    }

    /// Get the index of the child node at the given child index (aka its index among its siblings).
    pub fn get_at(&self, child_index: isize) -> Option<usize> {
        let index = if child_index.is_negative() {
            self.children.len().checked_add_signed(child_index)?
        } else {
            child_index.unsigned_abs()
        };
        self.children.get(index).copied()
    }

    /// Returns an iterator over the child node indexes.
    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.children.iter()
    }

    /// Returns the number of contained children.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Returns true if this contains no children.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

/// Iterator that iterates over an [`EntityTree`] in breadth-first order.
pub(crate) struct EntityTreeIter<'a: 'node, 'tree, 'node> {
    tree: &'a EntityTree<'tree>,
    queue: VecDeque<&'node EntityTreeNode<'tree>>,
}

impl<'a: 'node, 'tree, 'node> EntityTreeIter<'a, 'tree, 'node> {
    fn from_index(root: usize, tree: &'a EntityTree<'tree>) -> Self {
        let queue = tree
            .nodes
            .get(root)
            .map(|node| VecDeque::from([node]))
            .unwrap_or_default();
        Self { tree, queue }
    }
}

impl<'a: 'node, 'tree, 'node> Iterator for EntityTreeIter<'a, 'tree, 'node> {
    type Item = &'node EntityTreeNode<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop_front()?;

        if let Some(children) = self.tree.children.get(&node.index) {
            for child in children.iter() {
                let child_node = self.tree.nodes.get(*child).unwrap();
                self.queue.push_back(child_node);
            }
        }

        Some(node)
    }
}
