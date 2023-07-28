use std::marker::PhantomData;

use bevy::asset::Assets;
use bevy::ecs::system::{Command, EntityCommands, SystemParam};
use bevy::prelude::{Commands, Entity, Mut, World};

use crate::proto::{Config, Prototypical};
use crate::registration::ProtoRegistry;
use crate::schematics::{DynamicSchematic, SchematicContext};
use crate::tree::EntityTreeNode;

/// A system parameter similar to [`Commands`], but catered towards [prototypes].
///
/// [prototypes]: Prototypical
#[derive(SystemParam)]
pub struct ProtoCommands<'w, 's, T: Prototypical, C: Config<T>> {
    commands: Commands<'w, 's>,
    #[system_param(ignore)]
    _phantom: PhantomData<(T, C)>,
}

impl<'w, 's, T: Prototypical, C: Config<T>> ProtoCommands<'w, 's, T, C> {
    /// Spawn the prototype with the given [ID].
    ///
    /// This internally calls [`Commands::spawn`].
    ///
    /// [ID]: Prototypical::id
    pub fn spawn<I: Into<T::Id>>(&mut self, id: I) -> ProtoEntityCommands<'w, 's, '_, T, C> {
        let mut entity = ProtoEntityCommands::new(self.commands.spawn_empty().id(), self);
        entity.insert(id);
        entity
    }

    /// Spawn an empty entity.
    ///
    /// This internally calls [`Commands::spawn_empty`].
    pub fn spawn_empty(&mut self) -> ProtoEntityCommands<'w, 's, '_, T, C> {
        ProtoEntityCommands::new(self.commands.spawn_empty().id(), self)
    }

    /// Apply the prototype with the given [ID] to the world.
    ///
    /// This should only be called on prototypes that do not [require an entity].
    /// To spawn this prototype as a new entity, use [`spawn`] instead.
    ///
    /// [ID]: Prototypical::id
    /// [require an entity]: Prototypical::requires_entity
    /// [`spawn`]: Self::spawn
    pub fn apply<I: Into<T::Id>>(&mut self, id: I) {
        self.add(ProtoInsertCommand::<T, C>::new(id.into(), None));
    }

    /// Remove the prototype with the given [ID] from the world.
    ///
    /// This should only be called on prototypes that do not [require an entity].
    /// To remove this prototype from an entity, use [`ProtoEntityCommands::remove`] instead.
    ///
    /// [ID]: Prototypical::id
    /// [require an entity]: Prototypical::requires_entity
    pub fn remove<I: Into<T::Id>>(&mut self, id: I) {
        self.add(ProtoInsertCommand::<T, C>::new(id.into(), None));
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
    pub fn entity(&mut self, entity: Entity) -> ProtoEntityCommands<'w, 's, '_, T, C> {
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
    pub fn get_entity(&mut self, entity: Entity) -> Option<ProtoEntityCommands<'w, 's, '_, T, C>> {
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
    pub fn get_or_spawn(&mut self, entity: Entity) -> ProtoEntityCommands<'w, 's, '_, T, C> {
        ProtoEntityCommands::new(self.commands.get_or_spawn(entity).id(), self)
    }

    /// Returns the underlying [`Commands`].
    pub fn commands(&mut self) -> &mut Commands<'w, 's> {
        &mut self.commands
    }

    fn add<Cmd: Command>(&mut self, command: Cmd) {
        self.commands.add(command);
    }
}

/// A struct similar to [`EntityCommands`], but catered towards [prototypes].
///
/// [prototypes]: Prototypical
pub struct ProtoEntityCommands<'w, 's, 'a, T: Prototypical, C: Config<T>> {
    entity: Entity,
    proto_commands: &'a mut ProtoCommands<'w, 's, T, C>,
}

impl<'w, 's, 'a, T: Prototypical, C: Config<T>> ProtoEntityCommands<'w, 's, 'a, T, C> {
    fn new(entity: Entity, proto_commands: &'a mut ProtoCommands<'w, 's, T, C>) -> Self {
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
            .add(ProtoInsertCommand::<T, C>::new(id, Some(self.entity)));
        self
    }

    /// Removes the prototype with the given [ID] from the entity.
    ///
    /// [ID]: Prototypical::id
    pub fn remove<I: Into<T::Id>>(&mut self, id: I) -> &mut Self {
        let id = id.into();
        self.proto_commands
            .add(ProtoRemoveCommand::<T, C>::new(id, Some(self.entity)));
        self
    }

    /// Returns the underlying [`ProtoCommands`].
    pub fn commands(&mut self) -> &mut ProtoCommands<'w, 's, T, C> {
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
pub struct ProtoInsertCommand<T: Prototypical, C: Config<T>> {
    data: ProtoCommandData<T, C>,
}

impl<T: Prototypical, C: Config<T>> ProtoInsertCommand<T, C> {
    pub fn new(id: T::Id, entity: Option<Entity>) -> Self {
        Self {
            data: ProtoCommandData {
                id,
                entity,
                _phantom: PhantomData,
            },
        }
    }
}

impl<T: Prototypical, C: Config<T>> Command for ProtoInsertCommand<T, C> {
    fn apply(self, world: &mut World) {
        self.data.assert_is_registered(world);

        self.data
            .for_each_schematic(world, true, |schematic, context| {
                schematic.apply(context).unwrap();
            });
    }
}

/// A [command] to remove a [prototype] from an entity.
///
/// [command]: Command
/// [prototype]: Prototypical
pub struct ProtoRemoveCommand<T: Prototypical, C: Config<T>> {
    data: ProtoCommandData<T, C>,
}

impl<T: Prototypical, C: Config<T>> ProtoRemoveCommand<T, C> {
    pub fn new(id: T::Id, entity: Option<Entity>) -> Self {
        Self {
            data: ProtoCommandData {
                id,
                entity,
                _phantom: PhantomData,
            },
        }
    }
}

impl<T: Prototypical, C: Config<T>> Command for ProtoRemoveCommand<T, C> {
    fn apply(self, world: &mut World) {
        self.data.assert_is_registered(world);

        self.data
            .for_each_schematic(world, false, |schematic, context| {
                schematic.remove(context).unwrap();
            });
    }
}

struct ProtoCommandData<T: Prototypical, C: Config<T>> {
    id: T::Id,
    entity: Option<Entity>,
    _phantom: PhantomData<C>,
}

impl<T: Prototypical, C: Config<T>> ProtoCommandData<T, C> {
    /// Asserts that the given prototype is registered, panicking if it isn't.
    fn assert_is_registered(&self, world: &World) {
        let registry = world.resource::<ProtoRegistry<T, C>>();
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
        F: Fn(&EntityTreeNode, &mut SchematicContext, &Assets<T>, &mut C),
    {
        world.resource_scope(|world: &mut World, registry: Mut<ProtoRegistry<T, C>>| {
            world.resource_scope(|world: &mut World, mut config: Mut<C>| {
                world.resource_scope(|world, prototypes: Mut<Assets<T>>| {
                    let entity_tree = registry
                        .get_tree_by_id(&self.id)
                        .unwrap()
                        .to_entity_tree(self.entity, world);

                    for node in entity_tree.iter() {
                        entity_tree.set_current(node);

                        let mut context = SchematicContext::new(world, &entity_tree);

                        #[cfg(feature = "auto_name")]
                        if let Some(mut entity) = context.entity_mut() {
                            if is_apply && !entity.contains::<bevy::core::Name>() {
                                entity.insert(bevy::core::Name::new(format!(
                                    "{} (Prototype)",
                                    node.id()
                                )));
                            }
                        }

                        callback(node, &mut context, &prototypes, &mut config);
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
        F: Fn(&DynamicSchematic, &mut SchematicContext),
    {
        self.for_each_entity(world, is_apply, |node, context, prototypes, config| {
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

                if proto.requires_entity() && context.entity().is_none() {
                    panic!(
                        "could not apply command for prototype {:?}: requires entity",
                        proto.id()
                    );
                }

                on_before_prototype(config, proto, context);

                for (_, schematic) in proto.schematics().iter() {
                    on_before_schematic(config, schematic, context);
                    callback(schematic, context);
                    on_after_schematic(config, schematic, context);
                }

                on_after_prototype(config, proto, context);
            }
        });
    }
}
