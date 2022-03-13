use crate::components::ComponentList;
use crate::prelude::Prototype;
use bevy::log::warn;
use bevy::reflect::serde::ReflectSerializer;
use bevy::reflect::TypeRegistry;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};

/// A serializer for [`Prototype`] data
pub struct PrototypeSerializer<'a> {
    prototype: &'a Prototype,
    registry: &'a TypeRegistry,
}

impl<'a> PrototypeSerializer<'a> {
    pub fn new(prototype: &'a Prototype, registry: &'a TypeRegistry) -> Self {
        Self {
            prototype,
            registry,
        }
    }
}

impl<'a> Serialize for PrototypeSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(2))?;
        state.serialize_entry("name", &self.prototype.name)?;
        if !self.prototype.templates.is_empty() {
            warn!("Attempting to serialize a prototype with templates is not supported. Templates will be excluded.")
        }
        let comp_list = &self.prototype.components;
        let comp_serializer = ComponentListSerializer::new(&comp_list, self.registry);

        state.serialize_entry("components", &comp_serializer)?;
        state.end()
    }
}

/// A custom serializer for [`ComponentList`] data
///
/// This can be used in your own custom [`Prototypical`](crate::Prototypical) struct to
/// easily serialize a list of components.
pub struct ComponentListSerializer<'a> {
    list: &'a ComponentList,
    registry: &'a TypeRegistry,
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
        let mut state = serializer.serialize_seq(Some(self.list.len()))?;
        let registry = self.registry.read();
        for value in self.list.iter() {
            let reflected_value = value.as_reflect();
            state.serialize_element(&ReflectSerializer::new(reflected_value, &registry))?;
        }
        state.end()
    }
}
