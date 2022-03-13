//! Provides the core abstractions [`Prototypical`] and [`Prototype`] for implementing prototypical structs.
use bevy::reflect::TypeUuid;
use std::fmt::Debug;
use std::slice::Iter;

use crate::components::{ComponentList, ProtoComponent};
use crate::prelude::{Prototypical, TemplateList};

/// The default prototype object, providing the basics for the prototype system.
#[derive(Debug, PartialEq, TypeUuid)]
#[uuid = "64d16515-97e9-4762-a275-a80e1b6b5c27"]
pub struct Prototype {
    /// The name of this prototype.
    pub name: String,
    /// The names of this prototype's templates (if any).
    ///
    /// See [`TemplateListDeserializer`](crate::serde::TemplateListDeserializer) to
    /// find out how these names are deserialized.
    pub templates: TemplateList,
    /// The components belonging to this prototype.
    pub components: ComponentList,
}

impl Prototypical for Prototype {
    fn name(&self) -> &str {
        &self.name
    }

    fn templates(&self) -> Option<&TemplateList> {
        Some(&self.templates)
    }

    fn templates_mut(&mut self) -> Option<&mut TemplateList> {
        Some(&mut self.templates)
    }

    fn iter_components(&self) -> Iter<'_, Box<dyn ProtoComponent>> {
        self.components.iter()
    }
}
