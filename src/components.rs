use anyhow::{anyhow, Error};
use bevy::ecs::world::EntityMut;
use bevy::log::error;
use bevy::prelude::{reflect_trait, AssetServer, Res, World};
use bevy::reflect::serde::ReflectDeserializer;
use bevy::reflect::{
    DynamicList, FromReflect, FromType, List, Reflect, TypeRegistration, TypeRegistry,
};
use std::any::{Any, TypeId};
use std::slice::Iter;

use crate::data::{ProtoCommands, ProtoData};
use crate::prototype::Prototypical;

/// A trait that allows components to be used within [`Prototypical`] structs
#[reflect_trait]
pub trait ProtoComponent: Reflect + Send + Sync + 'static {
    // fn attach(&self, entity: EntityMut);
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>);
    #[allow(unused_variables)]
    fn prepare(&self, world: &mut World, prototype: &dyn Prototypical, data: &mut ProtoData) {}
    fn as_reflect(&self) -> &dyn Reflect;
}

struct ComponentConstructor {
    // TODO: Create the map to allow types to be constructed from
    // OR create custom ReflectProtoComponent that casts Box<dyn Reflect> to T using from_reflect,
    // then casts the type to Box<dyn Reflect>
    // this is better because we can get it directly from the registration
    constructor: fn(Box<dyn Reflect>) -> Option<Box<dyn Reflect>>,
}

impl ComponentConstructor {
    pub fn new<T: FromReflect>() -> Self {
        Self {
            constructor: |reflect_value| {
                T::from_reflect(reflect_value.as_ref())
                    .map(|value| Box::new(value) as Box<dyn Reflect>)
            },
        }
    }

    pub fn construct(&self, value: Box<dyn Reflect>) -> Option<Box<dyn ProtoComponent>> {
        (self.constructor)(value)
    }
}

#[derive(Default)]
pub struct ComponentList {
    pub(crate) items: Vec<Box<dyn ProtoComponent>>,
}

impl ComponentList {
    pub fn new(items: Vec<Box<dyn ProtoComponent>>) -> Self {
        Self { items }
    }

    pub fn from_dynamic(list: DynamicList, registry: &TypeRegistry) -> anyhow::Result<Self> {
        let registry = registry.read();
        let mut items = Vec::with_capacity(list.len());
        for item in list.into_iter() {
            let name = item.type_name().to_string();
            let registration = registry
                .get_with_name(item.type_name())
                .ok_or_else(|| anyhow!("Could not find a type registration for {}", name))?;
            let id = registration.type_id();
            println!("Got: {:?}", id);
            {
                let x = registry.get_type_data::<ReflectProtoComponent>(id);
                dbg!(x.is_some());
            }
            let proto_comp = registry
				.get_type_data::<ReflectProtoComponent>(id)
				.ok_or_else(|| {
					anyhow!(
					"Could not reflect `ProtoComponent` for {:?}. Did you add a `#[reflect(ProtoComponent)]` to your component?",
					name
				)
				})?
				.get_boxed(item).expect("foo");
            // .map_err(|_| {
            // 	anyhow!(
            // 	"Could not get `Box<dyn ProtoComponent>` for {:?}. ",
            // 	name
            // )
            // })?;
            items.push(proto_comp);
        }

        Ok(Self { items })
    }

    pub fn iter(&self) -> Iter<'_, Box<dyn ProtoComponent>> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
