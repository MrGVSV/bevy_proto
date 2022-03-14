use crate::loader::AssetPreloader;
use bevy::ecs::world::EntityMut;
use bevy::prelude::Reflect;
use bevy::reflect::{FromReflect, FromType};

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
pub trait ProtoComponent: Reflect + Send + Sync + 'static {
    /// Applies this component to the given entity.
    ///
    /// This includes inserting or removing components and/or bundles.
    fn apply(&self, entity: &mut EntityMut);
    /// Returns the reflected trait object for this component.
    fn as_reflect(&self) -> &dyn Reflect;
    /// The [type name] of this component
    ///
    /// [type name]: std::any::type_name
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    #[allow(unused_variables)]
    fn preload_assets(&mut self, preloader: &mut AssetPreloader) {}
}

#[derive(Clone)]
pub struct ReflectProtoComponent {
    get_boxed: fn(&dyn Reflect) -> Option<Box<dyn ProtoComponent>>,
}

impl ReflectProtoComponent {
    /// Get the underlying [component](ProtoComponent) from the given reflected value
    pub fn get_component(&self, reflect_value: &dyn Reflect) -> Option<Box<dyn ProtoComponent>> {
        (self.get_boxed)(reflect_value)
    }
}

impl<T: ProtoComponent + FromReflect> FromType<T> for ReflectProtoComponent {
    fn from_type() -> Self {
        Self {
            get_boxed: |reflect_value| {
                T::from_reflect(reflect_value)
                    .map(|value| Box::new(value) as Box<dyn ProtoComponent>)
            },
        }
    }
}
