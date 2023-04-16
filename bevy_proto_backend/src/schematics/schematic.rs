use bevy::ecs::world::EntityMut;
use bevy::prelude::{FromReflect, Reflect};
use bevy::reflect::GetTypeRegistration;

use crate::deps::DependenciesBuilder;
use crate::tree::EntityTree;

/// Trait used to create a [prototype] schematic for modifying an [entity]
/// (or the [world] in general).
///
/// This trait can either be manually implemented or [derived].
///
/// See the [module-level documentation] for details.
///
/// # Example
///
/// ```
/// use bevy::ecs::world::EntityMut;
/// use bevy::prelude::{Component, FromReflect, Reflect};
/// use bevy_proto_backend::schematics::Schematic;
/// use bevy_proto_backend::tree::EntityTree;
/// #[derive(Component, Reflect, FromReflect)]
/// struct PlayerId(usize);
///
/// impl Schematic for PlayerId {
///   type Input = Self;
///
///   fn apply(input: &Self::Input, entity: &mut EntityMut, tree: &EntityTree) {
///     entity.insert(Self(input.0));
///   }
///
///   fn remove(input: &Self::Input, entity: &mut EntityMut, tree: &EntityTree) {
///     entity.remove::<Self>();
///   }
/// }
/// ```
///
/// [prototype]: crate::proto::Prototypical
/// [entity]: EntityMut
/// [world]: bevy::ecs::world::World
/// [derived]: bevy_proto_derive::Schematic
/// [module-level documentation]: crate::schematics
pub trait Schematic: Reflect {
    /// The input type to this schematic.
    ///
    /// This acts as an intermediary between serialized schematic information
    /// and the actual schematic instance.
    ///
    /// For types that don't need an intermediary type, this can just be
    /// set to `Self`.
    type Input: FromReflect + GetTypeRegistration;

    /// Controls how this schematic is applied to the given entity.
    fn apply(input: &Self::Input, entity: &mut EntityMut, tree: &EntityTree);
    /// Controls how this schematic is removed from the given entity.
    fn remove(input: &Self::Input, entity: &mut EntityMut, tree: &EntityTree);

    /// Allows dependency assets to be loaded when this schematic is loaded.
    #[allow(unused_variables)]
    fn preload_dependencies(input: &mut Self::Input, dependencies: &mut DependenciesBuilder) {}
}

/// A custom [`From`]-like trait used to convert the [input] of a [schematic]
/// to itself.
///
/// This is used by [derive macro] to automatically handle the conversion.
///
/// This trait is has a blanket implementation for any type where the input
/// type satisfies [`Into`] for the schematic type.
///
/// [input]: Schematic::Input
/// [schematic]: Schematic
/// [derive macro]: bevy_proto_derive::Schematic
pub trait FromSchematicInput<T> {
    fn from_input(input: T, entity: &mut EntityMut, tree: &EntityTree) -> Self;
}

impl<S, T: Into<S>> FromSchematicInput<T> for S {
    fn from_input(input: T, _entity: &mut EntityMut, _tree: &EntityTree) -> S {
        input.into()
    }
}
