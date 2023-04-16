use std::marker::PhantomData;

use bevy::asset::Assets;
use bevy::ecs::system::{Command, EntityCommands, SystemParam};
use bevy::ecs::world::EntityMut;
use bevy::prelude::{Commands, Entity, Mut, World};

use crate::proto::{Config, Prototypical};
use crate::registration::ProtoRegistry;
use crate::schematics::DynamicSchematic;
use crate::tree::{EntityTree, EntityTreeNode};

/// A system parameter similar to [`Commands`], but catered towards [prototypes].
///
/// [prototypes]: Prototypical
#[derive(SystemParam)]
pub struct ProtoCommands<'w, 's, T: Prototypical> {
    commands: Commands<'w, 's>,
    #[system_param(ignore)]
    _phantom: PhantomData<T>,
}

impl<'w, 's, T: Prototypical> ProtoCommands<'w, 's, T> {
    /// Spawn the prototype with the given [ID].
    ///
    /// This internally calls [`Commands::spawn`].
    ///
    /// [ID]: Prototypical::id
    pub fn spawn<I: Into<T::Id>>(&mut self, id: I) -> ProtoEntityCommands<'w, 's, '_, T> {
        let mut entity = ProtoEntityCommands::new(self.commands.spawn_empty().id(), self);
        entity.insert(id);
        entity
    }

    /// Spawn an empty entity.
    ///
    /// This internally calls [`Commands::spawn_empty`].
    pub fn spawn_empty(&mut self) -> ProtoEntityCommands<'w, 's, '_, T> {
        ProtoEntityCommands::new(self.commands.spawn_empty().id(), self)
    }

    /// Get the [`ProtoEntityCommands`] for the given entity.
    ///
    /// This internally calls [`Commands::entity`].
    ///
    /// # Panics
    ///
    /// This method panics if the requested entity does not exist.
    ///
    /// # See
    ///
    /// [`get_entity`] for the non-panicking version.
    ///
    /// [`get_entity`]: Self::get_entity
    pub fn entity(&mut self, entity: Entity) -> ProtoEntityCommands<'w, 's, '_, T> {
        ProtoEntityCommands::new(self.commands.entity(entity).id(), self)
    }

    /// Get the [`ProtoEntityCommands`] for the given entity, if it exists.
    ///
    /// This internally calls [`Commands::get_entity`].
    ///
    /// # See
    ///
    /// [`entity`] for the panicking version.
    ///
    /// [`entity`]: Self::entity
    pub fn get_entity(&mut self, entity: Entity) -> Option<ProtoEntityCommands<'w, 's, '_, T>> {
        self.commands
            .get_entity(entity)
            .as_ref()
            .map(EntityCommands::id)
            .map(|entity| ProtoEntityCommands::new(entity, self))
    }

    /// Get the [`ProtoEntityCommands`] for the given entity,
    /// spawning a new one if it doesn't exist.
    ///
    /// This internally calls [`Commands::get_or_spawn`].
    pub fn get_or_spawn(&mut self, entity: Entity) -> ProtoEntityCommands<'w, 's, '_, T> {
        ProtoEntityCommands::new(self.commands.get_or_spawn(entity).id(), self)
    }

    /// Returns the underlying [`Commands`].
    pub fn commands(&mut self) -> &mut Commands<'w, 's> {
        &mut self.commands
    }

    fn add<C: Command>(&mut self, command: C) {
        self.commands.add(command);
    }
}

/// A struct similar to [`EntityCommands`], but catered towards [prototypes].
///
/// [prototypes]: Prototypical
pub struct ProtoEntityCommands<'w, 's, 'a, T: Prototypical> {
    entity: Entity,
    proto_commands: &'a mut ProtoCommands<'w, 's, T>,
}

impl<'w, 's, 'a, T: Prototypical> ProtoEntityCommands<'w, 's, 'a, T> {
    fn new(entity: Entity, proto_commands: &'a mut ProtoCommands<'w, 's, T>) -> Self {
        Self {
            entity,
            proto_commands,
        }
    }

    /// Get the [`Entity`].
    pub fn id(&self) -> Entity {
        self.entity
    }

    /// Inserts the prototype with the given [ID] onto the entity.
    ///
    /// [ID]: Prototypical::id
    pub fn insert<I: Into<T::Id>>(&mut self, id: I) -> &mut Self {
        let id = id.into();
        self.proto_commands
            .add(ProtoInsertCommand::<T>::new(id, self.entity));
        self
    }

    /// Removes the prototype with the given [ID] from the entity.
    ///
    /// [ID]: Prototypical::id
    pub fn remove<I: Into<T::Id>>(&mut self, id: I) -> &mut Self {
        let id = id.into();
        self.proto_commands
            .add(ProtoRemoveCommand::<T>::new(id, self.entity));
        self
    }

    /// Returns the underlying [`ProtoCommands`].
    pub fn commands(&mut self) -> &mut ProtoCommands<'w, 's, T> {
        self.proto_commands
    }

    /// Returns the corresponding [`EntityCommands`].
    pub fn entity_commands(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        self.proto_commands.commands.entity(self.entity)
    }
}

/// A [command] to insert a [prototype] on an entity.
///
/// [command]: Command
/// [prototype]: Prototypical
pub struct ProtoInsertCommand<T: Prototypical> {
    data: ProtoCommandData<T>,
}

impl<T: Prototypical> ProtoInsertCommand<T> {
    pub fn new(id: T::Id, entity: Entity) -> Self {
        Self {
            data: ProtoCommandData { id, entity },
        }
    }
}

impl<T: Prototypical> Command for ProtoInsertCommand<T> {
    fn write(self, world: &mut World) {
        self.data.assert_is_registered(world);

        self.data
            .for_each_schematic(world, true, |schematic, entity, tree| {
                schematic.apply(entity, tree).unwrap();
            });
    }
}

/// A [command] to remove a [prototype] from an entity.
///
/// [command]: Command
/// [prototype]: Prototypical
pub struct ProtoRemoveCommand<T: Prototypical> {
    data: ProtoCommandData<T>,
}

impl<T: Prototypical> ProtoRemoveCommand<T> {
    pub fn new(id: T::Id, entity: Entity) -> Self {
        Self {
            data: ProtoCommandData { id, entity },
        }
    }
}

impl<T: Prototypical> Command for ProtoRemoveCommand<T> {
    fn write(self, world: &mut World) {
        self.data.assert_is_registered(world);

        self.data
            .for_each_schematic(world, false, |schematic, entity, tree| {
                schematic.remove(entity, tree).unwrap();
            });
    }
}

struct ProtoCommandData<T: Prototypical> {
    id: T::Id,
    entity: Entity,
}

impl<T: Prototypical> ProtoCommandData<T> {
    /// Asserts that the given prototype is registered, panicking if it isn't.
    fn assert_is_registered(&self, world: &World) {
        let registry = world.resource::<ProtoRegistry<T>>();
        let is_registered = registry.contains(&self.id);

        if !is_registered {
            if registry.load_queue().read().is_queued(&self.id) {
                panic!(
                    "could not apply command for prototype {:?}: is still loading (use `Prototypes::is_ready` to check load status)",
                    &self.id
                );
            } else {
                panic!(
                    "could not apply command for prototype {:?}: is not loaded",
                    &self.id
                );
            }
        }
    }

    fn for_each_entity<F>(&self, world: &mut World, is_apply: bool, callback: F)
    where
        F: Fn(&EntityTreeNode, &mut EntityMut, &EntityTree, &Assets<T>, &mut T::Config),
    {
        world.resource_scope(|world: &mut World, registry: Mut<ProtoRegistry<T>>| {
            world.resource_scope(|world: &mut World, mut config: Mut<T::Config>| {
                world.resource_scope(|world, prototypes: Mut<Assets<T>>| {
                    let entity_tree = registry
                        .get_tree_by_id(&self.id)
                        .unwrap()
                        .to_entity_tree(self.entity, world);

                    for node in entity_tree.iter() {
                        entity_tree.set_current(node);

                        let mut entity = world.entity_mut(node.entity());

                        #[cfg(feature = "auto_name")]
                        if is_apply && !entity.contains::<bevy::core::Name>() {
                            entity.insert(bevy::core::Name::new(format!(
                                "{} (Prototype)",
                                node.id()
                            )));
                        }

                        callback(node, &mut entity, &entity_tree, &prototypes, &mut config);
                    }
                })
            })
        });
    }

    /// Helper function to loop over the [schematics] for the given [prototype] and entity.
    ///
    /// [schematics]: DynamicSchematic
    /// [prototype]: Prototypical
    fn for_each_schematic<F>(&self, world: &mut World, is_apply: bool, callback: F)
    where
        F: Fn(&DynamicSchematic, &mut EntityMut, &EntityTree),
    {
        self.for_each_entity(
            world,
            is_apply,
            |node, entity, entity_tree, prototypes, config| {
                let on_before_prototype = if is_apply {
                    Config::<T>::on_before_apply_prototype
                } else {
                    Config::<T>::on_before_remove_prototype
                };
                let on_after_prototype = if is_apply {
                    Config::<T>::on_after_apply_prototype
                } else {
                    Config::<T>::on_after_remove_prototype
                };
                let on_before_schematic = if is_apply {
                    Config::<T>::on_before_apply_schematic
                } else {
                    Config::<T>::on_before_remove_schematic
                };
                let on_after_schematic = if is_apply {
                    Config::<T>::on_after_apply_schematic
                } else {
                    Config::<T>::on_after_remove_schematic
                };

                for handle_id in node.prototypes() {
                    let handle = prototypes.get_handle(*handle_id);
                    let proto = prototypes.get(&handle).unwrap();

                    on_before_prototype(config, proto, entity, entity_tree);

                    for (_, schematic) in proto.schematics().iter() {
                        on_before_schematic(config, schematic, entity, entity_tree);
                        callback(schematic, entity, entity_tree);
                        on_after_schematic(config, schematic, entity, entity_tree);
                    }

                    on_after_prototype(config, proto, entity, entity_tree);
                }
            },
        );
    }
}
