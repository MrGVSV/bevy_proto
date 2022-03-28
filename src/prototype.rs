//! Provides the core abstractions [`Prototypical`] and [`Prototype`] for implementing prototypical structs.
use bevy::reflect::TypeUuid;
use std::fmt::Debug;
use std::slice::{Iter, IterMut};

use crate::components::{ComponentList, ProtoComponent};
use crate::deps::DependencyMap;
use crate::prelude::{Prototypical, TemplateList};

/// The default prototype object, providing the basics for the prototype system.
#[derive(Debug, PartialEq, TypeUuid)]
#[uuid = "64d16515-97e9-4762-a275-a80e1b6b5c27"]
pub struct Prototype {
    /// The name of this prototype.
    pub(crate) name: String,
    /// The list of template prototypes to inherit from, if any.
    ///
    /// These are listed as relative paths to this prototype's file. See [`TemplateList`]
    /// for more details.
    ///
    /// Additionally, to find out how these names are deserialized, check out
    /// [`TemplateListDeserializer`](crate::serde::TemplateListDeserializer).
    pub(crate) templates: TemplateList,
    /// The map of assets this prototype depends on.
    pub(crate) dependencies: DependencyMap,
    /// The components belonging to this prototype.
    pub(crate) components: ComponentList,
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

    fn dependencies(&self) -> &DependencyMap {
        &self.dependencies
    }

    fn dependencies_mut(&mut self) -> &mut DependencyMap {
        &mut self.dependencies
    }

    fn components(&self) -> Iter<'_, Box<dyn ProtoComponent>> {
        self.components.iter()
    }

    fn components_mut(&mut self) -> IterMut<'_, Box<dyn ProtoComponent>> {
        self.components.iter_mut()
    }
}
