use crate::manager::{HandleToName, NameToHandle, ProtoHandles, ProtoIdRef};
use crate::Prototypical;
use bevy::asset::{Asset, AssetServer, Assets, Handle, HandleId, LoadState};
use bevy::ecs::system::{EntityCommands, SystemParam};
use bevy::prelude::{Commands, HandleUntyped, Res};

/// A system param that provides easy usage of loaded [prototypical] assets.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use bevy_proto::prelude::*;
///
/// fn my_system(manager: ProtoManager<Prototype>, mut commands: Commands) {
///   if let Some(proto) = manager.get("My Prototype") {
///     proto.spawn(&mut commands);
///   }
/// }
/// ```
///
/// [prototypical]: crate::Prototypical
#[derive(SystemParam)]
pub struct ProtoManager<'w, 's, T: Prototypical + Asset> {
    commands: Commands<'w, 's>,
    handle_to_name: Res<'w, HandleToName>,
    name_to_handle: Res<'w, NameToHandle>,
    handles: Res<'w, ProtoHandles<T>>,
    prototypes: Res<'w, Assets<T>>,
    asset_server: Res<'w, AssetServer>,
}

impl<'w, 's, T: Prototypical + Asset> ProtoManager<'w, 's, T> {
    /// Checks if a [prototypical] asset is loaded.
    ///
    /// This is the preferred method for checking the load state of a prototype since
    /// prototypes also need to be registered before use (which happens automatically
    /// after successfully loading).
    ///
    /// [prototypical]: crate::Prototypical
    pub fn is_loaded<'a, I: Into<ProtoIdRef<'a>>>(&self, id: I) -> bool {
        let handle = match id.into() {
            ProtoIdRef::Name(name) => {
                if let Some(handle) = self.name_to_handle.read().get(name) {
                    *handle
                } else {
                    return false;
                }
            }
            ProtoIdRef::Handle(handle) => handle,
        };

        self.prototypes.contains(handle)
    }

    /// Returns true if _all_ stored [prototypical] asset handles are fully loaded.
    ///
    /// [prototypical]: crate::Prototypical
    pub fn all_loaded(&self) -> bool {
        let handles = self
            .handles
            .read()
            .iter()
            .map(|(id, _)| *id)
            .collect::<Vec<HandleId>>();
        self.asset_server.get_group_load_state(handles) == LoadState::Loaded
    }

    /// Spawns the [prototypical] asset with the given name or handle.
    ///
    /// [prototypical]: crate::Prototypical
    pub fn spawn<'a, I: Into<ProtoIdRef<'a>>>(
        &'a mut self,
        id: I,
    ) -> Option<EntityCommands<'w, 's, 'a>> {
        // The reason we go about this unsafety instead of just manually adding the
        // `ProtoCommand` is that the prototypical asset may have additional
        // spawning logic in its `spawn` method.

        // SAFE: Self exists and is properly aligned
        unsafe {
            let tmp = std::ptr::read(self);
            if let Some(proto) = tmp.get(id) {
                Some(proto.spawn(&mut self.commands))
            } else {
                None
            }
        }
    }

    /// Get a loaded [prototypical] asset.
    ///
    /// Returns `None` if the prototype is not currently loaded.
    ///
    /// [prototypical]: crate::Prototypical
    pub fn get<'a, I: Into<ProtoIdRef<'a>>>(&self, id: I) -> Option<&T> {
        let handle = match id.into() {
            ProtoIdRef::Name(name) => *self.name_to_handle.read().get(name)?,
            ProtoIdRef::Handle(handle) => handle,
        };

        self.prototypes.get(handle)
    }

    /// Get a _strong_ handle to the [prototypical] asset with the given name.
    ///
    /// [prototypical]: crate::Prototypical
    pub fn get_handle(&self, name: &str) -> Option<Handle<T>> {
        let handle = *self.name_to_handle.read().get(name)?;
        Some(self.prototypes.get_handle(handle))
    }

    /// Get the name of the [prototypical] asset with the given handle.
    ///
    /// [prototypical]: crate::Prototypical
    pub fn get_name<H: Into<HandleId>>(&self, handle: H) -> Option<String> {
        self.handle_to_name
            .read()
            .get(&handle.into())
            .map(|name| name.to_string())
    }

    /// Add a handle to a [prototypical] asset so that it can be kept loaded.
    ///
    /// If multiple handles need to be stored, try using the [`add_multiple`] method.
    ///
    /// Returns the previously stored strong handle (if any).
    ///
    /// # Panics
    ///
    /// Panics if the given handle is not _strong_.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_proto::prelude::*;
    ///
    /// fn load_proto_system(asset_server: Res<AssetServer>, mut manager: ProtoManager<Prototype>) {
    ///   let handle = asset_server.load("prototypes/my_proto.prototype.yaml");
    ///   manager.add(handle);
    /// }
    /// ```
    ///
    /// [prototypical]: crate::Prototypical
    /// [`add_multiple`]: Self::add_multiple
    pub fn add(&mut self, handle: Handle<T>) -> Option<Handle<T>> {
        assert!(handle.is_strong(), "The given handle must be strong");
        if !self.handles.read().contains_key(&handle.id) {
            self.handles.write().insert(handle.id, handle)
        } else {
            None
        }
    }

    /// Add multiple [prototypical] asset handles so that they can be kept loaded.
    ///
    /// This method is preferred over just using [`add`](Self::add) in a loop as it only
    /// needs to lock the handle store once.
    ///
    /// [prototypical]: crate::Prototypical
    pub fn add_multiple<I: IntoIterator<Item = Handle<T>>>(&mut self, handles: I) {
        let mut writer = self.handles.write();
        for handle in handles {
            assert!(handle.is_strong(), "The given handle must be strong");
            writer.insert(handle.id, handle);
        }
    }

    /// Add multiple untyped [prototypical] asset handles so that they can be kept loaded.
    ///
    /// This method is preferred over just using [`add`](Self::add) in a loop as it only
    /// needs to lock the handle store once.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_proto::prelude::ProtoManager;
    /// use bevy_proto::Prototype;
    ///
    /// fn load_proto_system(asset_server: Res<AssetServer>, mut manager: ProtoManager<Prototype>) {
    ///   let handles = asset_server.load_folder("prototypes").unwrap();
    ///   manager.add_multiple_untyped(handles);
    /// }
    /// ```
    ///
    /// [prototypical]: crate::Prototypical
    pub fn add_multiple_untyped<I: IntoIterator<Item = HandleUntyped>>(&mut self, handles: I) {
        let handles = handles.into_iter().map(|handle| handle.typed());
        self.add_multiple(handles);
    }

    /// Remove a stored handle so that its [prototypical] asset _may_ be unloaded.
    ///
    /// Returns the stored strong handle (if any).
    ///
    /// [prototypical]: crate::Prototypical
    pub fn remove<H: Into<HandleId>>(&mut self, handle: H) -> Option<Handle<T>> {
        let handle = handle.into();
        if self.handles.read().contains_key(&handle) {
            self.handles.write().remove(&handle)
        } else {
            None
        }
    }
}
