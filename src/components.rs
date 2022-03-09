//! Contains the [`ProtoComponent`] trait.
use bevy::prelude::{AssetServer, Res, World};

use crate::data::{ProtoCommands, ProtoData};
use crate::prototype::Prototypical;

/// Specifies how a type inserts components into an entity.
///
/// Types implementing [`ProtoComponent`] describe how to insert any number of components or bundles when spawning a prototype.
/// Any type which is `Send + Sync + 'static` can implement [`ProtoComponent`].
///
/// Notably, this means a [`ProtoComponent`] might not be a [`Component`][bevy::prelude::Component] itself.
/// The [`ProtoComponent`] can be a kind of [data transfer object](https://en.wikipedia.org/wiki/Data_transfer_object) which
/// describes inserting any arbitrary set of components or bundles.
///
/// # Examples
///
/// To just insert a type which is a [`Component`][bevy::prelude::Component], [`ProtoComponent`] can be derived:
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use bevy::prelude::*;
/// use bevy_proto::prelude::*;
///
/// #[derive(Clone, Serialize, Deserialize, Component, ProtoComponent)]
/// pub struct Movement {
///     speed: u16,
/// }
///
/// // Also works on tuple structs:
/// #[derive(Clone, Serialize, Deserialize, Component, ProtoComponent)]
/// struct Inventory(Option<Vec<String>>);
/// ```
///
/// The derived [`ProtoComponent`] implementation clones `Self` and inserts the cloned value into the entity.
/// A deriving type must also be [`Clone`], [`serde::Deserialize`], [`serde::Serialize`], and [`Component`][bevy::ecs::component::Component].
///
/// For other cases, [`ProtoComponent`] can be implemented manually:
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use bevy::prelude::*;
/// use bevy::ecs::system::EntityCommands;
/// use bevy_proto::prelude::*;
///
/// // We'll implement `ProtoComponent` on this `Inventory` struct.
/// // Our implementation will insert multiple different components.
/// #[derive(Serialize, Deserialize)] // Required
/// struct Inventory {
///     items: Items,
///     quest_items: QuestItems,
/// }
///
/// // This `Items` struct will be one of the component types we insert.
/// #[derive(Clone, Serialize, Deserialize, Component)]
/// struct Items(Vec<String>);
///
/// // We will also insert this separate `QuestItems` struct.
/// #[derive(Clone, Serialize, Deserialize, Component)]
/// struct QuestItems(Vec<String>);
///
/// #[typetag::serde] // Required
/// impl ProtoComponent for Inventory {
///     // The `Inventory` implementation of `insert_self` inserts two components:
///     // one for `Items`, and one for `QuestItems`.
///     fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
///         commands.insert(self.items.clone());
///         commands.insert(self.quest_items.clone());
///     }
/// }
/// ```
///
/// Implementations of insert_self can arbitrarily insert zero, one, or many components or bundles.
///
///  This trait allows components to be used within [`Prototypical`](crate::prototype::Prototypical) structs.
#[typetag::serde(tag = "type", content = "value")]
pub trait ProtoComponent: Send + Sync + 'static {
    /// Defines how this struct inserts components and/or bundles into an entity.
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>);
    /// Defines how this struct creates and inserts asset handles for later use.
    #[allow(unused_variables)]
    fn prepare(&self, world: &mut World, prototype: &dyn Prototypical, data: &mut ProtoData) {}
}
