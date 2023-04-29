use crate::proto::Prototypical;
use bevy::asset::Handle;

/// Asset lifecycle events for [prototype] assets.
///
/// This is analogous to [`AssetEvent`], but accounts for prototype
/// caching and registration.
/// This event should be preferred over using the `AssetEvent` directly.
///
/// [prototype]: Prototypical
/// [`AssetEvent`]: bevy::asset::AssetEvent
#[derive(Debug, PartialEq)]
pub enum ProtoAssetEvent<T: Prototypical> {
    /// This event is fired when a prototype has been successfully created,
    /// registered, and cached.
    Created {
        /// The ID of the created prototype.
        id: T::Id,
        /// A weak handle to the prototype asset.
        handle: Handle<T>,
    },
    /// This event is fired when a prototype has been modified.
    ///
    /// This includes when a prototype is directly modified or when one its
    /// dependencies is modified.
    Modified {
        /// The ID of the modified prototype.
        id: T::Id,
        /// A weak handle to the prototype asset.
        handle: Handle<T>,
    },
    /// This event is fired when a prototype has been fully unloaded.
    Removed {
        /// The ID of the created prototype.
        id: T::Id,
        /// A weak handle to the prototype asset.
        handle: Handle<T>,
    },
}

impl<T: Prototypical> ProtoAssetEvent<T> {
    /// Returns the ID of the prototype.
    pub fn id(&self) -> &T::Id {
        match self {
            ProtoAssetEvent::Created { id, .. } => id,
            ProtoAssetEvent::Modified { id, .. } => id,
            ProtoAssetEvent::Removed { id, .. } => id,
        }
    }

    /// Returns a weak handle to the prototype asset.
    pub fn handle(&self) -> &Handle<T> {
        match self {
            ProtoAssetEvent::Created { handle, .. } => handle,
            ProtoAssetEvent::Modified { handle, .. } => handle,
            ProtoAssetEvent::Removed { handle, .. } => handle,
        }
    }

    /// Returns true if the prototype with the given ID was created.
    pub fn is_created<I: for<'a> PartialEq<&'a T::Id>>(&self, id: I) -> bool {
        match self {
            ProtoAssetEvent::Created { id: created_id, .. } => id == created_id,
            _ => false,
        }
    }

    /// Returns true if the prototype with the given ID was modified.
    pub fn is_modified<I: for<'a> PartialEq<&'a T::Id>>(&self, id: I) -> bool {
        match self {
            ProtoAssetEvent::Modified {
                id: modified_id, ..
            } => id == modified_id,
            _ => false,
        }
    }

    /// Returns true if the prototype with the given ID was removed.
    pub fn is_removed<I: for<'a> PartialEq<&'a T::Id>>(&self, id: I) -> bool {
        match self {
            ProtoAssetEvent::Removed { id: removed_id, .. } => id == removed_id,
            _ => false,
        }
    }
}
