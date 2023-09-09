use bevy::asset::HandleId;
use bevy::utils::{AHasher, FixedState};
use std::any::TypeId;
use std::hash::{BuildHasher, Hash, Hasher};

/// A unique identifier for a [`Schematic`].
///
/// This can be used to generate stable references for items in a `Schematic`,
/// including deeply nested items.
///
/// This can be achieved by ensuring the same value is passed to [`SchematicId::next`].
///
/// [`Schematic`]: crate::schematics::Schematic
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SchematicId(u64);

impl SchematicId {
    /// Creates a new [`SchematicId`] from the given [`HandleId`] of the prototype asset
    /// and the [`TypeId`] of the schematic itself.
    pub(crate) fn new(proto: HandleId, schematic: TypeId) -> Self {
        Self(Self::generate(|hasher| {
            Hash::hash(&proto, hasher);
            Hash::hash(&schematic, hasher);
        }))
    }

    /// The hash value of this [`SchematicId`].
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Generate a new [`SchematicId`] from this one and the given seed.
    pub fn next(&self, seed: impl Hash) -> Self {
        Self(Self::generate(|hasher| {
            Hash::hash(self, hasher);
            Hash::hash(&seed, hasher);
        }))
    }

    /// Allows cloning this [`SchematicId`].
    ///
    /// This is used instead of the [`Clone`] trait to prevent consumers of this crate
    /// from accidentally cloning this ID and subsequently breaking stability guarantees.
    pub(crate) fn clone(&self) -> Self {
        Self(self.0)
    }

    fn generate(f: impl FnOnce(&mut AHasher)) -> u64 {
        let mut hasher = FixedState.build_hasher();
        f(&mut hasher);
        hasher.finish()
    }
}
