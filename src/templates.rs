use bevy::asset::HandleUntyped;
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
    paths: Vec<String>,
    handles: Vec<HandleUntyped>,
}

impl TemplateList {
    pub fn new(paths: Vec<String>, handles: Vec<HandleUntyped>) -> Self {
        Self { paths, handles }
    }

    pub fn with_paths(paths: Vec<String>) -> Self {
        Self {
            paths,
            handles: Vec::new(),
        }
    }

    pub fn set_paths(&mut self, paths: Vec<String>) {
        self.paths = paths;
    }

    pub fn set_handles(&mut self, handles: Vec<HandleUntyped>) {
        self.handles = handles;
    }

    pub fn push_handle<H: Into<HandleUntyped>>(&mut self, handle: H) {
        self.handles.push(handle.into());
    }

    pub fn iter_paths(&self) -> Iter<'_, String> {
        self.paths.iter()
    }

    /// Gets an iterator over the templates in their defined order
    pub fn iter_defined_order(&self) -> Iter<'_, HandleUntyped> {
        self.handles.iter()
    }

    /// Gets an iterator over the templates in order of inheritance
    pub fn iter_inheritance_order(&self) -> Rev<Iter<'_, HandleUntyped>> {
        self.handles.iter().rev()
    }

    /// Returns true if this list is empty
    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }

    /// Returns the length of the list
    pub fn len(&self) -> usize {
        self.handles.len()
    }
}
