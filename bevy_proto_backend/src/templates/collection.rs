use std::fmt::{Debug, Formatter};

use bevy::asset::HandleUntyped;
use indexmap::IndexMap;

use crate::path::ProtoPath;

/// A collection of template [prototypes] for a given prototype.
///
/// Internally, this maps a [`ProtoPath`] to its asset handle.
///
/// # Order
///
/// Insertion order matters.
/// The order in which a template is inserted dictates where in the template
/// chain it lies.
/// Templates are applied in reverse order, so templates that were inserted
/// later may be overwritten by templates that were inserted earlier.
///
/// [prototypes]: crate::proto::Prototypical
#[derive(Default, Eq, PartialEq)]
pub struct Templates {
    templates: IndexMap<ProtoPath, HandleUntyped>,
}

impl Templates {
    /// Get the handle to the prototype at the given [path].
    ///
    /// The returned handle will be _strong_.
    ///
    /// [path]: ProtoPath
    pub fn get<K: AsRef<ProtoPath>>(&self, path: K) -> Option<&HandleUntyped> {
        self.templates.get(path.as_ref())
    }

    /// Insert a prototype's [path] and asset handle.
    ///
    /// If a prototype at the given path already existed,
    /// it will be replaced and its strong asset handle will be returned.
    ///
    /// # Panics
    ///
    /// Panics if the given handle is weak.
    ///
    /// [path]: ProtoPath
    pub fn insert<K: Into<ProtoPath>, V: Into<HandleUntyped>>(
        &mut self,
        path: K,
        handle: V,
    ) -> Option<HandleUntyped> {
        let handle = handle.into();
        debug_assert!(handle.is_strong(), "template inserted with weak handle");
        self.templates.insert(path.into(), handle)
    }

    /// Remove the handle to the prototype at the given [path].
    ///
    /// The returned handle will be _strong_.
    ///
    /// [path]: ProtoPath
    pub fn remove<K: AsRef<ProtoPath>>(&mut self, path: K) -> Option<HandleUntyped> {
        self.templates.remove(path.as_ref())
    }

    /// Iterate over the templates in the order they were inserted.
    pub fn iter(&self) -> indexmap::map::Iter<'_, ProtoPath, HandleUntyped> {
        self.templates.iter()
    }

    /// The number of templates.
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    /// Returns true if there are no templates.
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
}

impl Debug for Templates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Templates(")?;
        f.debug_map().entries(self.templates.iter()).finish()?;
        write!(f, ")")
    }
}
