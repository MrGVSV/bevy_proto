use std::iter::Rev;
use std::path::PathBuf;
use std::slice::Iter;

/// A list of templates a [prototypical] struct will inherit from.
///
/// A _template_ is just the asset path of another prototype. This template will be applied
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
    asset_paths: Vec<PathBuf>,
}

impl TemplateList {
    pub fn new<P: Into<PathBuf>, I: IntoIterator<Item = P>>(paths: I) -> Self {
        Self {
            asset_paths: paths.into_iter().map(|p| p.into()).collect(),
        }
    }

    /// Gets an iterator over the templates in their defined order
    pub fn iter_defined_order(&self) -> Iter<'_, PathBuf> {
        self.asset_paths.iter()
    }

    /// Gets an iterator over the templates in order of inheritance
    pub fn iter_inheritance_order(&self) -> Rev<Iter<'_, PathBuf>> {
        self.asset_paths.iter().rev()
    }

    /// Returns true if this list is empty
    pub fn is_empty(&self) -> bool {
        self.asset_paths.is_empty()
    }

    /// Returns the length of the list
    pub fn len(&self) -> usize {
        self.asset_paths.len()
    }
}
