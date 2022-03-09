//! Contains the [`ProtoComponent`] trait.
use bevy::prelude::{AssetServer, Res, World};

use crate::data::{ProtoCommands, ProtoData};
use crate::prototype::Prototypical;

/// Specifies how a struct inserts components into an entity.
///
/// Any struct which is `Send + Sync + 'static` can implement [`ProtoComponent`].
/// The implementing struct may or may not be a component itself.
/// Commonly, [data transfer objects](https://en.wikipedia.org/wiki/Data_transfer_object)
/// can implement [`ProtoComponent`] to generate components or bundles.
///
/// The [`insert_self`][`ProtoComponent::insert_self`] method provides full mutable access to [`EntityCommands`][bevy::ecs::system::EntityCommands].
/// Implementations can arbitrarily insert zero, one, or many components or bundles at once into an entity.
///
/// This trait allows components to be used within [`Prototypical`](crate::prototype::Prototypical) structs.
///
/// # Examples
///
/// For simple components, [`ProtoComponent`] can be derived:
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use bevy::prelude::*;
/// use bevy_proto::prelude::*;
///
/// #[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
/// pub struct Movement {
///     speed: u16,
/// }
///
/// // Also works on tuple structs:
/// #[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
/// struct Inventory (Option<Vec<String>>);
/// ```
///
/// The derived implementation clones `Self` and inserts the cloned value into the entity.
/// To derive [`ProtoComponent`], a struct must also be [`Clone`], [`serde::Deserialize`], [`serde::Serialize`], and [`Component`][bevy::ecs::component::Component].
///
/// [`ProtoComponent`] can also be implemented manually:
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use bevy::prelude::*;
/// use bevy::ecs::system::EntityCommands;
/// use bevy_proto::prelude::*;
///
/// #[derive(Serialize, Deserialize, Component)] // Required
/// struct Inventory(Option<Vec<String>>);
///
/// #[typetag::serde] // Required
/// impl ProtoComponent for Inventory {
///     // Required
///     fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
///         commands.insert(
///             Self (self.0.clone())
///         );
///     }
/// }
/// ```
#[typetag::serde(tag = "type", content = "value")]
pub trait ProtoComponent: Send + Sync + 'static {
    /// Defines how this struct inserts components and/or bundles into an entity.
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>);
    /// Defines how this struct creates and inserts asset handles for later use.
    #[allow(unused_variables)]
    fn prepare(&self, world: &mut World, prototype: &dyn Prototypical, data: &mut ProtoData) {}
}
