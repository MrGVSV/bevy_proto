use crate::loader::AssetPreloader;
use bevy::ecs::world::EntityMut;
use bevy::prelude::Reflect;
use bevy::reflect::{FromReflect, FromType};

/// Specifies how a type inserts components into an entity.
///
/// Types implementing [`ProtoComponent`] describe how to insert any number of components or
/// bundles when spawning a prototype. Any type which is `Send + Sync + 'static` and which also
/// implements Bevy's [`Reflect`] trait can implement [`ProtoComponent`].
///
/// Notably, this means a [`ProtoComponent`] might not be a [`Component`] itself. The [`ProtoComponent`]
/// can be a kind of [data transfer object] which _describes_ inserting any arbitrary set
/// of components or bundles.
///
/// Implementations of [`apply`] can arbitrarily insert zero, one, or many components/bundles.
///
/// This trait allows components to be used within [`Prototypical`](crate::prototype::Prototypical) structs.
///
/// # Examples
///
/// To just insert a type which is a [`Component`], [`ProtoComponent`] can be derived:
///
/// ```
/// use bevy::prelude::*;
/// use bevy::reflect::FromReflect;
/// use serde::Deserialize;
/// use bevy_proto::prelude::*;
///
/// #[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
/// #[reflect(ProtoComponent)]
/// pub struct Movement {
///     speed: u16,
/// }
///
/// // Also works on tuple structs:
/// #[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
/// #[reflect(ProtoComponent)]
/// struct Inventory(Option<Vec<String>>);
///
/// // And enums:
/// #[derive(Reflect, FromReflect, Component, ProtoComponent, Clone, Deserialize)]
/// #[reflect_value(ProtoComponent, Deserialize)]
/// enum Action {
///   Run,
///   Walk,
///   Jump,
/// }
/// ```
///
/// The derived [`ProtoComponent`] implementation clones `Self` and inserts the cloned value
/// into the entity. A deriving type must also be [`Clone`], [`Reflect`], [`FromReflect`],
/// and [`Component`].
///
/// For other cases, [`ProtoComponent`] can be implemented manually:
///
/// ```
/// use bevy::ecs::world::EntityMut;
/// use bevy::prelude::*;
/// use bevy::reflect::FromReflect;
/// use bevy_proto::prelude::*;
///
/// // We'll implement `ProtoComponent` on this `Inventory` struct.
/// // Our implementation will insert multiple different components.
/// #[derive(Reflect, FromReflect)] // Required
/// #[reflect(ProtoComponent)] // Required
/// struct Inventory {
///     items: Items,
///     quest_items: QuestItems,
/// }
///
/// // This `Items` struct will be one of the component types we insert.
/// #[derive(Reflect, FromReflect, Component, Clone)]
/// struct Items(Vec<String>);
///
/// // We will also insert this separate `QuestItems` struct.
/// #[derive(Reflect, FromReflect, Component, Clone)]
/// struct QuestItems(Vec<String>);
///
/// impl ProtoComponent for Inventory {
///     // The `Inventory` implementation of `apply` inserts two components:
///     // one for `Items`, and one for `QuestItems`.
///     fn apply(&self, entity: &mut EntityMut) {
///         entity.insert(self.items.clone());
///         entity.insert(self.quest_items.clone());
///     }
///
///     // This method is required for internal usage. Thankfully, we can just
///     // return `self` in most cases.
///     fn as_reflect(&self) -> &dyn Reflect {
///         self
///     }
/// }
/// ```
///
/// [`Reflect`]: bevy::reflect::Reflect
/// [`Component`]: bevy::prelude::Component
/// [data transfer object]: https://en.wikipedia.org/wiki/Data_transfer_object
/// [`FromReflect`]: bevy::reflect::FromReflect
/// [`apply`]: ProtoComponent::apply
pub trait ProtoComponent: Reflect + Send + Sync + 'static {
    /// Applies this component to the given entity.
    ///
    /// This includes inserting or removing components and/or bundles.
    fn apply(&self, entity: &mut EntityMut);
    /// Returns the reflected trait object for this component.
    fn as_reflect(&self) -> &dyn Reflect;
    /// The [type name] of this component.
    ///
    /// [type name]: std::any::type_name
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    /// Set assets for preloading when this component is first loaded.
    ///
    /// This is automatically called once per [prototypical] object during its asset loading phase.
    ///
    /// [prototypical]: crate::prelude::Prototypical
    #[allow(unused_variables)]
    fn preload_assets(&mut self, preloader: &mut AssetPreloader) {}
}

/// A component used to convert a reflected object into a valid [`ProtoComponent`] trait object.
///
/// This is specifically meant to convert dynamic objects like `DynamicStruct`, which can't
/// be downcast normally.
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
