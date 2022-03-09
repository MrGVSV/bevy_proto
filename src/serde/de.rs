use crate::components::ComponentList;
use crate::prelude::Prototype;
use bevy::reflect::{serde::ReflectDeserializer, TypeRegistry};
use serde::de::{DeserializeSeed, Error, MapAccess, SeqAccess, Visitor};
use serde::Deserializer;
use std::fmt::Formatter;

/// A deserializer for [`Prototype`] data
pub struct PrototypeDeserializer<'a> {
    registry: &'a TypeRegistry,
}

impl<'a> PrototypeDeserializer<'a> {
    pub fn new(registry: &'a TypeRegistry) -> Self {
        Self { registry }
    }
}

impl<'a, 'de> DeserializeSeed<'de> for PrototypeDeserializer<'a> {
    type Value = Prototype;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ProtoVisitor {
            registry: self.registry,
        })
    }
}

struct ProtoVisitor<'a> {
    registry: &'a TypeRegistry,
}

impl<'a, 'de> Visitor<'de> for ProtoVisitor<'a> {
    type Value = Prototype;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a Prototype definition")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut name = None;
        let mut templates = None;
        let mut components = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => name = Some(map.next_value()?),
                "template" | "templates" => templates = Some(map.next_value()?),
                "components" => {
                    components =
                        Some(map.next_value_seed(ComponentListDeserializer::new(self.registry))?)
                }
                invalid => return Err(A::Error::custom(format!("Invalid entry: {}", invalid))),
            }
        }

        Ok(Prototype {
            name: name.ok_or_else(|| A::Error::custom("Missing `name` property"))?,
            templates: templates.unwrap_or_default(),
            components: components.unwrap_or_default(),
        })
    }
}

/// A custom deserializer for [`ComponentList`] data
///
/// This can be used in your own custom [`Prototypical`](crate::Prototypical) struct to
/// easily deserialize a list of components.
pub struct ComponentListDeserializer<'a> {
    registry: &'a TypeRegistry,
}

impl<'a> ComponentListDeserializer<'a> {
    pub fn new(registry: &'a TypeRegistry) -> Self {
        Self { registry }
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
        let mut list = Vec::default();
        let registry = self.registry.read();
        while let Some(value) = seq.next_element_seed(ReflectDeserializer::new(&registry))? {
            list.push(value);
        }
        ComponentList::from_reflected(&list, &self.registry).map_err(V::Error::custom)
    }
}
