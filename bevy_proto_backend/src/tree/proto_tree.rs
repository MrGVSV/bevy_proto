use std::fmt::{Debug, Formatter};

use bevy::asset::{Handle, HandleId};
use bevy::prelude::{Entity, World};
use bevy::utils::HashMap;
use indexmap::IndexSet;

use crate::children::MergeKey;
use crate::proto::Prototypical;
use crate::tree::EntityTree;

/// A cached tree structure that represents a single [prototype].
///
/// This is used to generate a corresponding [`EntityTree`] for processing.
///
/// [prototype]: Prototypical
pub(crate) struct ProtoTree<T: Prototypical> {
    /// The [ID] of this prototype.
    ///
    /// [ID]: Prototypical::Id
    id: T::Id,
    /// The stringified [ID] of this prototype.
    ///
    /// [ID]: Prototypical::Id
    id_str: String,
    /// The asset handle ID of this prototype.
    handle: HandleId,
    /// The set of template prototypes, in their reverse-application order.
    ///
    /// The first entry in the set should be this prototype itself.
    prototypes: IndexSet<HandleId>,
    /// The merge key of this prototype (if any).
    ///
    /// This is only applicable to child prototypes that define a [merge key].
    ///
    /// [merge key]: MergeKey
    merge_key: Option<MergeKey<T>>,
    /// The immediate children of this tree.
    children: Vec<ProtoTree<T>>,
    /// A mapping of [merge keys] to children.
    ///
    /// [merge keys]: MergeKey
    merge_keys: HashMap<MergeKey<T>, usize>,
}

impl<T: Prototypical> ProtoTree<T> {
    pub fn new(handle: Handle<T>, merge_key: Option<MergeKey<T>>, id: &T::Id) -> Self {
        Self {
            id: id.clone(),
            id_str: id.to_string(),
            handle: handle.id(),
            prototypes: IndexSet::from([handle.id()]),
            merge_key,
            children: Vec::new(),
            merge_keys: HashMap::new(),
        }
    }

    pub fn id_str(&self) -> &str {
        &self.id_str
    }

    pub fn handle(&self) -> HandleId {
        self.handle
    }

    /// Append the given tree as a new child of this one.
    pub fn append_child(&mut self, tree: Self) {
        if let Some(merge_key) = tree.merge_key.as_ref() {
            if let Some(index) = self.merge_keys.get(merge_key) {
                self.children[*index].inherit(tree);
            } else {
                self.merge_keys
                    .insert(merge_key.clone(), self.children.len());
                self.children.push(tree);
            }
        } else {
            self.children.push(tree);
        }
    }

    /// Merge the given tree into this one by inheriting it.
    pub fn inherit(&mut self, tree: Self) {
        // 1. Inherit all prototypes
        for prototype in tree.prototypes {
            self.prototypes.insert(prototype);
        }

        // 2. Merge children
        for child in tree.children {
            self.append_child(child);
        }
    }

    /// The set of prototypes for this tree (in reverse-application order).
    pub fn prototypes(&self) -> &IndexSet<HandleId> {
        &self.prototypes
    }

    /// The immediate children of this tree.
    pub fn children(&self) -> &[ProtoTree<T>] {
        &self.children
    }

    /// Converts this tree to a corresponding [`EntityTree`], using the given root [`Entity`].
    pub fn to_entity_tree(&self, root: Entity, world: &mut World) -> EntityTree<'_> {
        EntityTree::new(self, root, world)
    }
}

impl<T: Prototypical> Clone for ProtoTree<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            id_str: self.id_str.clone(),
            handle: self.handle,
            prototypes: self.prototypes.clone(),
            merge_key: self.merge_key.clone(),
            children: self.children.clone(),
            merge_keys: self.merge_keys.clone(),
        }
    }
}

impl<T: Prototypical> Debug for ProtoTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("ProtoTree<{}>", std::any::type_name::<T>()))
            .field("id", &self.id)
            .field("handle", &self.handle)
            .field("prototypes", &self.prototypes)
            .field("merge_key", &self.merge_key)
            .field("children", &self.children)
            .field("merge_keys", &self.merge_keys)
            .finish()
    }
}
