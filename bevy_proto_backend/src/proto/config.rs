use bevy::asset::Handle;
use bevy::ecs::world::EntityMut;
use bevy::prelude::{FromWorld, Resource};

use crate::cycles::{Cycle, CycleResponse};
use crate::proto::Prototypical;
use crate::schematics::DynamicSchematic;
use crate::tree::EntityTree;

/// Configuration for a [prototype].
///
/// This is used to configure the [`ProtoBackendPlugin`]
/// and also provide callback methods to hook into certain operations.
///
/// [prototype]: Prototypical
/// [`ProtoBackendPlugin`]: crate::ProtoBackendPlugin
#[allow(unused_variables)]
pub trait Config<T: Prototypical>: Resource + FromWorld {
    /// A list of supported extensions.
    ///
    /// Extensions should be in order of most-specific to least specific,
    /// and should not be prepended by a dot (`.`).
    /// Generally, this means it should be in order of longest to shortest.
    ///
    /// For example, this could return:
    ///
    /// ```
    /// # use bevy::prelude::Resource;
    /// # use bevy_proto_backend::proto::{Config, Prototypical};
    /// # #[derive(Default)]
    /// struct Foo;
    /// # impl Resource for Foo {}
    /// impl<T: Prototypical> Config<T> for Foo {
    ///   fn extensions(&self) -> Box<[&'static str]> {
    ///     vec![
    ///       // Most Specific (Longest) //
    ///       "prototype.yaml",
    ///       "prototype.ron",
    ///       "yaml"
    ///       "ron",
    ///       // Least Specific (Shortest) //
    ///     ].into_boxed_slice()
    ///   }
    /// }
    /// ```
    fn extensions(&self) -> Box<[&'static str]>;

    /// Callback method that's triggered when a [prototype] is registered.
    ///
    /// Prototypes are registered when they are first loaded.
    ///
    /// This method will be given the registered prototype and its corresponding _strong_ [`Handle`].
    ///
    /// [prototype]: Prototypical
    fn on_register_prototype(&mut self, prototype: &T, handle: Handle<T>) {}

    /// Callback method that's triggered when a [prototype] is reloaded.
    ///
    /// Prototypes are reloaded whenever they or one of their dependencies are modified.
    ///
    /// This method will be given the reloaded prototype and its corresponding _strong_ [`Handle`].
    ///
    /// [prototype]: Prototypical
    fn on_reload_prototype(&mut self, prototype: &T, handle: Handle<T>) {}

    /// Callback method that's triggered when a [prototype] is unregistered.
    ///
    /// Prototypes are unregistered when they are unloaded.
    ///
    /// This method will be given the unregistered prototype's ID and its corresponding _strong_ [`Handle`].
    ///
    /// [prototype]: Prototypical
    fn on_unregister_prototype(&mut self, id: &T::Id, handle: Handle<T>) {}

    /// Callback method that's triggered _before_ a [prototype] is applied to an entity.
    ///
    /// This is only called when using [`ProtoCommands`].
    /// Applying a prototype manually won't automatically trigger this callback.
    ///
    /// [prototype]: Prototypical
    /// [`ProtoCommands`]: crate::proto::ProtoCommands
    fn on_before_apply_prototype(
        &mut self,
        prototype: &T,
        entity: &mut EntityMut,
        tree: &EntityTree,
    ) {
    }

    /// Callback method that's triggered _after_ a [prototype] is applied to an entity.
    ///
    /// This is only called when using [`ProtoCommands`].
    /// Applying a prototype manually won't automatically trigger this callback.
    ///
    /// [prototype]: Prototypical
    /// [`ProtoCommands`]: crate::proto::ProtoCommands
    fn on_after_apply_prototype(
        &mut self,
        prototype: &T,
        entity: &mut EntityMut,
        tree: &EntityTree,
    ) {
    }

    /// Callback method that's triggered _before_ a [prototype] is removed from an entity.
    ///
    /// This is only called when using [`ProtoCommands`].
    /// Removing a prototype manually won't automatically trigger this callback.
    ///
    /// [prototype]: Prototypical
    /// [`ProtoCommands`]: crate::proto::ProtoCommands
    fn on_before_remove_prototype(
        &mut self,
        prototype: &T,
        entity: &mut EntityMut,
        tree: &EntityTree,
    ) {
    }

    /// Callback method that's triggered _after_ a [prototype] is removed from an entity.
    ///
    /// This is only called when using [`ProtoCommands`].
    /// Removing a prototype manually won't automatically trigger this callback.
    ///
    /// [prototype]: Prototypical
    /// [`ProtoCommands`]: crate::proto::ProtoCommands
    fn on_after_remove_prototype(
        &mut self,
        prototype: &T,
        entity: &mut EntityMut,
        tree: &EntityTree,
    ) {
    }

    /// Callback method that's triggered _before_ a [schematic] is applied to an entity.
    ///
    /// This is only called when using [`ProtoCommands`].
    /// Applying a prototype manually won't automatically trigger this callback.
    ///
    /// [schematic]: crate::schematics::Schematic
    /// [`ProtoCommands`]: crate::proto::ProtoCommands
    fn on_before_apply_schematic(
        &mut self,
        schematic: &DynamicSchematic,
        entity: &mut EntityMut,
        tree: &EntityTree,
    ) {
    }

    /// Callback method that's triggered _after_ a [schematic] is applied to an entity.
    ///
    /// This is only called when using [`ProtoCommands`].
    /// Applying a prototype manually won't automatically trigger this callback.
    ///
    /// [schematic]: crate::schematics::Schematic
    /// [`ProtoCommands`]: crate::proto::ProtoCommands
    fn on_after_apply_schematic(
        &mut self,
        schematic: &DynamicSchematic,
        entity: &mut EntityMut,
        tree: &EntityTree,
    ) {
    }

    /// Callback method that's triggered _before_ a [schematic] is removed from an entity.
    ///
    /// This is only called when using [`ProtoCommands`].
    /// Removing a prototype manually won't automatically trigger this callback.
    ///
    /// [schematic]: crate::schematics::Schematic
    /// [`ProtoCommands`]: crate::proto::ProtoCommands
    fn on_before_remove_schematic(
        &mut self,
        schematic: &DynamicSchematic,
        entity: &mut EntityMut,
        tree: &EntityTree,
    ) {
    }

    /// Callback method that's triggered _after_ a [schematic] is removed from an entity.
    ///
    /// This is only called when using [`ProtoCommands`].
    /// Removing a prototype manually won't automatically trigger this callback.
    ///
    /// [schematic]: crate::schematics::Schematic
    /// [`ProtoCommands`]: crate::proto::ProtoCommands
    fn on_after_remove_schematic(
        &mut self,
        schematic: &DynamicSchematic,
        entity: &mut EntityMut,
        tree: &EntityTree,
    ) {
    }

    /// Controls how [cycles] should be handled.
    ///
    /// When `#[cfg(debug_assertions)]` is enabled, the default behavior will be to panic.
    /// Otherwise, the default behavior is to cancel the operation leading to the cycle.
    ///
    /// [cycles]: crate::cycles
    fn on_cycle(&self, cycle: &Cycle<T>) -> CycleResponse {
        if cfg!(debug_assertions) {
            CycleResponse::Panic
        } else {
            CycleResponse::Cancel
        }
    }
}
