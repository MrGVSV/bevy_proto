use crate::command::ProtoCommand;
use crate::deps::DependencyMap;
use crate::prelude::{ProtoComponent, TemplateList};
use bevy::asset::Asset;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, Entity};
use std::slice::{Iter, IterMut};

/// Allows access to a prototype's name and components so that it can be spawned into the World.
pub trait Prototypical: 'static + Send + Sync {
    /// The name of the prototype.
    ///
    /// This should be unique amongst all prototypes in the world.
    fn name(&self) -> &str;

    /// Returns a reference to the [`TemplateList`] associated with this prototype, if any.
    fn templates(&self) -> Option<&TemplateList> {
        None
    }

    /// Returns a mutable reference to the [`TemplateList`] associated with this prototype, if any.
    fn templates_mut(&mut self) -> Option<&mut TemplateList> {
        None
    }

    /// Returns a reference to the [`DependencyMap`] associated with this prototype.
    fn dependencies(&self) -> &DependencyMap;

    /// Returns a mutable reference to the [`DependencyMap`] associated with this prototype.
    fn dependencies_mut(&mut self) -> &mut DependencyMap;

    /// Returns an iterator of [`ProtoComponent`] trait objects.
    fn components(&self) -> Iter<'_, Box<dyn ProtoComponent>>;

    /// Returns a mutable iterator of [`ProtoComponent`] trait objects.
    fn components_mut(&mut self) -> IterMut<'_, Box<dyn ProtoComponent>>;

    /// Spawns an entity with this prototype's component structure.
    ///
    /// # Arguments
    ///
    /// * `commands`: The world `Commands`
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_proto::prelude::{ProtoManager, Prototype, Prototypical};
    ///
    /// fn spawn_system(mut commands: Commands, manager: ProtoManager) {
    ///   if let Some(proto) = manager.get("My Prototype") {
    ///     let entity = proto.spawn(&mut commands).id();
    ///     // ...
    ///   }
    /// }
    /// ```
    fn spawn<'a, 'p, 'w, 's>(
        &'p self,
        commands: &'a mut Commands<'w, 's>,
    ) -> EntityCommands<'w, 's, 'a>
    where
        Self: Asset + Sized,
    {
        let entity = commands.spawn().id();
        self.insert(entity, commands)
    }

    /// Inserts this prototype's component structure to the given entity.
    ///
    /// __Note:__ This _will_ override existing components of the same type.
    ///
    /// # Arguments
    ///
    /// * `entity`: The entity to insert into
    /// * `commands`: The world `Commands`
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_proto::prelude::{ProtoManager, Prototype, Prototypical};
    ///
    /// #[derive(Component)]
    /// struct Player(pub Entity);
    ///
    /// fn update_system(mut commands: Commands, player: Query<&Player>, manager: ProtoManager) {
    ///   if let Some(proto) = manager.get("My Prototype") {
    ///     // Get the player entity
    ///     let entity = player.single().0;
    ///
    ///     // Insert the new components
    ///     let entity = proto.insert(entity, &mut commands).id();
    ///
    ///     // ...
    ///   }
    /// }
    /// ```
    fn insert<'a, 'p, 'w, 's>(
        &'p self,
        entity: Entity,
        commands: &'a mut Commands<'w, 's>,
    ) -> EntityCommands<'w, 's, 'a>
    where
        Self: Asset + Sized,
    {
        commands.add(ProtoCommand::<Self>::new(entity, self.name()));
        commands.entity(entity)
    }
}