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
pub enum ProtoAssetEvent<T: Prototypical> {
    /// This event is fired when a prototype has been successfully created,
    /// registered, and cached.
    Created {
        /// The ID of the created prototype.
        id: T::Id,
        /// A weak handle to the prototype asset.
        handle: Handle<T>,
    },
    /// This event is fired when a prototype has been modified, such as when
    /// a change is made to the file while hot-reloading is enabled.
    Modified {
        /// The ID of the modified prototype.
        id: T::Id,
        /// A weak handle to the prototype asset.
        handle: Handle<T>,
    },
    /// This event is fired when a prototype has been fully unloaded.
    Removed {
        /// The ID of the removed prototype.
        ///
        /// This will almost always be `Some` unless a duplicate removal happens
        /// to occur at the same time, in which case it may be `None`.
        id: Option<T::Id>,
        /// A weak handle to the prototype asset.
        handle: Handle<T>,
    },
}
