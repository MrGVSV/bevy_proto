use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

use bevy::utils::hashbrown::hash_map::{IntoIter, Iter, IterMut};
use bevy::utils::HashMap;

use crate::schematics::{DynamicSchematic, Schematic};

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

    /// Returns true if the given schematic is contained.
    pub fn contains<T: Schematic>(&self) -> bool {
        self.0.contains_key(std::any::type_name::<T>())
    }

    /// Returns true if the given [type name] of a schematic is contained.
    ///
    /// [type name]: std::any::type_name
    pub fn contains_by_name(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Get a reference to the given schematic.
    pub fn get<T: Schematic>(&self) -> Option<&DynamicSchematic> {
        self.0.get(std::any::type_name::<T>())
    }

    /// Get a reference to the schematic with the given [type name].
    ///
    /// [type name]: std::any::type_name
    pub fn get_by_name(&self, key: &str) -> Option<&DynamicSchematic> {
        self.0.get(key)
    }

    /// Get a mutable reference to the given schematic.
    pub fn get_mut<T: Schematic>(&mut self) -> Option<&mut DynamicSchematic> {
        self.0.get_mut(std::any::type_name::<T>())
    }

    /// Get a mutable reference to the schematic with the given [type name].
    ///
    /// [type name]: std::any::type_name
    pub fn get_mut_by_name(&mut self, key: &str) -> Option<&mut DynamicSchematic> {
        self.0.get_mut(key)
    }

    /// Insert a new schematic.
    pub fn insert<T: Schematic>(&mut self, input: T::Input) -> Option<DynamicSchematic> {
        let schematic = DynamicSchematic::new::<T>(input);
        let key = Cow::Borrowed(std::any::type_name::<T>());
        self.0.insert(key, schematic)
    }

    /// Insert a new schematic dynamically.
    pub fn insert_dynamic(&mut self, schematic: DynamicSchematic) -> Option<DynamicSchematic> {
        let key = Cow::Borrowed(schematic.type_info().type_name());
        self.0.insert(key, schematic)
    }

    /// Remove the given schematic.
    pub fn remove<T: Schematic>(&mut self) -> Option<DynamicSchematic> {
        self.0.remove(std::any::type_name::<T>())
    }

    /// Remove the schematic with the given [type name].
    ///
    /// [type name]: std::any::type_name
    pub fn remove_by_name(&mut self, key: &str) -> Option<DynamicSchematic> {
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

impl FromIterator<(Cow<'static, str>, DynamicSchematic)> for Schematics {
    fn from_iter<T: IntoIterator<Item = (Cow<'static, str>, DynamicSchematic)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl IntoIterator for Schematics {
    type Item = (Cow<'static, str>, DynamicSchematic);
    type IntoIter = IntoIter<Cow<'static, str>, DynamicSchematic>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
