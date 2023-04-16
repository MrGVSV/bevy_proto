use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

use bevy::utils::hashbrown::hash_map::{Iter, IterMut};
use bevy::utils::HashMap;

use crate::schematics::DynamicSchematic;

/// A collection of [schematics] for a [prototype].
///
/// Internally, this stores a mapping of the schematic's type name
/// to its corresponding [`DynamicSchematic`].
///
/// # Order
///
/// Insertion order does _not_ matter.
/// Schematics are treated as a map and as such,
/// their application order is not guaranteed.
///
/// This also means that there cannot be duplicate schematics.
/// Inserting a schematic will overwrite existing instances.
///
/// [schematics]: crate::schematics::Schematic
/// [prototype]: crate::proto::Prototypical
#[derive(Default)]
pub struct Schematics(HashMap<Cow<'static, str>, DynamicSchematic>);

impl Schematics {
    /// Create an empty [`Schematics`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Returns true if the given [type name] of a schematic is contained.
    ///
    /// [type name]: std::any::type_name
    pub fn contains(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Get a reference to the schematic with the given [type name].
    ///
    /// [type name]: std::any::type_name
    pub fn get(&self, key: &str) -> Option<&DynamicSchematic> {
        self.0.get(key)
    }

    /// Get a mutable reference to the schematic with the given [type name].
    ///
    /// [type name]: std::any::type_name
    pub fn get_mut(&mut self, key: &str) -> Option<&mut DynamicSchematic> {
        self.0.get_mut(key)
    }

    /// Insert a new schematic.
    pub fn insert(&mut self, schematic: DynamicSchematic) -> Option<DynamicSchematic> {
        let key = Cow::Borrowed(schematic.type_info().type_name());
        self.0.insert(key, schematic)
    }

    /// Remove the schematic with the given [type name].
    ///
    /// [type name]: std::any::type_name
    pub fn remove(&mut self, key: &str) -> Option<DynamicSchematic> {
        self.0.remove(key)
    }

    /// Returns an iterator over all the schematics.
    pub fn iter(&self) -> Iter<'_, Cow<'static, str>, DynamicSchematic> {
        self.0.iter()
    }

    /// Returns a mutable iterator over all the schematics.
    pub fn iter_mut(&mut self) -> IterMut<'_, Cow<'static, str>, DynamicSchematic> {
        self.0.iter_mut()
    }

    /// The number of contained schematics.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if there are no stored schematics.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Debug for Schematics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Schematics(")?;
        f.debug_list().entries(self.0.values()).finish()?;
        write!(f, ")")
    }
}
