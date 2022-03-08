use crate::components::{ComponentList, ReflectProtoComponent};
use crate::prelude::ProtoComponent;
use anyhow::anyhow;
use bevy::prelude::Struct;
use bevy::reflect::serde::ReflectDeserializer;
use bevy::reflect::{DynamicList, DynamicStruct, List, TypeRegistry};
use serde::de::{DeserializeSeed, Error, SeqAccess, Visitor};
use std::any::{Any, TypeId};
use std::borrow::Borrow;

pub struct ComponentListDeserializer<'a> {
	registry: &'a TypeRegistry,
}

impl<'a> ComponentListDeserializer<'a> {
	pub fn new(registry: &'a TypeRegistry) -> Self {
		ComponentListDeserializer { registry }
	}
}

impl<'a, 'de> DeserializeSeed<'de> for ComponentListDeserializer<'a> {
	type Value = ComponentList;

	fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_seq(ComponentListVisitor {
			registry: self.registry,
		})
	}
}

struct ComponentListVisitor<'a> {
	registry: &'a TypeRegistry,
}

impl<'a, 'de> Visitor<'de> for ComponentListVisitor<'a> {
	type Value = ComponentList;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("component list")
	}

	fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
	where
		V: SeqAccess<'de>,
	{
		let mut list = DynamicList::default();
		let registry = self.registry.read();
		while let Some(value) = seq.next_element_seed(ReflectDeserializer::new(&registry))? {
			list.push_box(value);
		}
		ComponentList::from_dynamic(list, &self.registry).map_err(V::Error::custom)
	}
}
