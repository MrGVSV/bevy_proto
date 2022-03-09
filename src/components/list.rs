use super::ProtoComponent;
use crate::components::ReflectProtoComponent;
use crate::errors::ProtoError;
use bevy::prelude::Reflect;
use bevy::reflect::TypeRegistryArc as TypeRegistry;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::slice::Iter;

/// A list of [`ProtoComponent`] objects that can be used to create
/// real components dynamically
#[derive(Default)]
pub struct ComponentList {
    pub(crate) items: Vec<Box<dyn ProtoComponent>>,
}

impl ComponentList {
    pub fn new(items: Vec<Box<dyn ProtoComponent>>) -> Self {
        Self { items }
    }

    pub fn from_reflected(
        list: &[Box<dyn Reflect>],
        registry: &TypeRegistry,
    ) -> anyhow::Result<Self> {
        let registry = registry.read();
        let mut items = Vec::with_capacity(list.len());
        for item in list {
            let name: Cow<str> = Cow::Owned(item.type_name().to_string());

            // --- Get Registration --- //
            let registration = registry.get_with_name(item.type_name()).ok_or_else(|| {
                ProtoError::NotRegistered {
                    name: name.to_string(),
                }
            })?;

            // --- Get ProtoComponent Data --- //
            let id = registration.type_id();
            let proto_reflect = registry
                .get_type_data::<ReflectProtoComponent>(id)
                .ok_or_else(|| ProtoError::MissingReflection {
                    name: name.to_string(),
                })?;

            // --- Add Component --- //
            let proto_comp = proto_reflect.get_component(item.as_ref()).ok_or_else(|| {
                ProtoError::BadReflection {
                    name: name.to_string(),
                }
            })?;
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

impl Debug for ComponentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.iter().map(|comp| comp.name()))
            .finish()
    }
}

impl PartialEq for ComponentList {
    fn eq(&self, other: &Self) -> bool {
        if self.items.len() != other.items.len() {
            return false;
        }

        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.name() == b.name())
    }
}

impl IntoIterator for ComponentList {
    type Item = Box<dyn ProtoComponent>;
    type IntoIter = std::vec::IntoIter<Box<dyn ProtoComponent>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
