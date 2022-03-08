use crate::components::ComponentList;
use crate::prelude::ReflectProtoComponent;
use bevy::prelude::Reflect;
use bevy::reflect::serde::ReflectSerializer;
use bevy::reflect::TypeRegistry;
use serde::ser::SerializeSeq;
use serde::Serialize;
use std::any::Any;

pub struct ComponentListSerializer<'a> {
	pub list: &'a ComponentList,
	pub registry: &'a TypeRegistry,
}

impl<'a> ComponentListSerializer<'a> {
	pub fn new(list: &'a ComponentList, registry: &'a TypeRegistry) -> Self {
		Self { list, registry }
	}
}

impl<'a> Serialize for ComponentListSerializer<'a> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		ComponentListValueSerializer {
			list: self.list,
			registry: self.registry,
		}
		.serialize(serializer)
	}
}

pub struct ComponentListValueSerializer<'a> {
	pub list: &'a ComponentList,
	pub registry: &'a TypeRegistry,
}

impl<'a> Serialize for ComponentListValueSerializer<'a> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_seq(Some(self.list.len()))?;
		let registry = self.registry.read();
		for value in self.list.iter() {
			let reflected_value = value.as_reflect();
			state.serialize_element(&ReflectSerializer::new(reflected_value, &registry))?;
		}
		state.end()
	}
}
