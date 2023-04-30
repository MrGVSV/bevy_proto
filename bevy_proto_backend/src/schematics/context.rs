use crate::tree::{EntityAccess, EntityTree};
use bevy::ecs::world::{EntityMut, EntityRef};
use bevy::prelude::{Entity, World};

/// The context in which a schematic instance exists.
pub struct SchematicContext<'a, 'b> {
    world: &'a mut World,
    tree: &'a EntityTree<'b>,
}

impl<'a, 'b> SchematicContext<'a, 'b> {
    pub(crate) fn new(world: &'a mut World, tree: &'a EntityTree<'b>) -> Self {
        Self { world, tree }
    }

    /// Returns a reference to the world.
    pub fn world(&self) -> &World {
        self.world
    }

    /// Returns a mutable reference to the world.
    pub fn world_mut(&mut self) -> &mut World {
        self.world
    }

    /// Returns a reference to the entity this schematic is being applied to, if any.
    pub fn entity(&self) -> Option<EntityRef> {
        Some(self.world.entity(self.tree.entity()))
    }

    /// Returns a mutable reference to the entity this schematic is being applied to, if any.
    pub fn entity_mut(&mut self) -> Option<EntityMut> {
        Some(self.world.entity_mut(self.tree.entity()))
    }

    /// Find an entity in the tree using the given [`EntityAccess`].
    pub fn find_entity(&self, access: &EntityAccess) -> Option<Entity> {
        self.tree.find_entity(access)
    }

    /// Returns a reference to the entity tree.
    pub fn tree(&self) -> &EntityTree {
        self.tree
    }
}
