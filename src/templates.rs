use std::iter::Rev;
use std::slice::Iter;

/// A list of templates a [prototypical] struct will inherit from.
///
/// A _template_ is just the name of another prototype. This template will be applied
/// along with the actual prototype. In this way, prototypes can "inherit" from
/// other prototypes, reducing duplication and improving configuration.
///
/// Templates are listed in reverse order of inheritance, where templates that are
/// listed last may be overridden by templates listed first. For example, a list
/// of templates like `["IFoo", "IBar", "IBaz"]` means that [prototypical] information
/// generated under `"IBaz"` may be overridden by `"IBar"` or `"IFoo"`.
///
/// The reason for this is to position more specific or relevant templates first
/// so that it's more visible at a glance.
///
/// [prototypical]: crate::prelude::Prototypical
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TemplateList {
    items: Vec<String>,
}

impl TemplateList {
    pub fn new(items: Vec<String>) -> Self {
        TemplateList { items }
    }

    /// Gets an iterator over the templates in their defined order
    pub fn iter_defined_order(&self) -> Iter<'_, String> {
        self.items.iter()
    }

    /// Gets an iterator over the templates in order of inheritance
    pub fn iter_inheritance_order(&self) -> Rev<Iter<'_, String>> {
        self.items.iter().rev()
    }

    /// Returns true if this list is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the length of the list
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
