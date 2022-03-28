use crate::command::ProtoCommand;
use crate::manager::{HandleToName, NameToHandle, ProtoIdRef};
use crate::{Prototype, Prototypical};
use bevy::asset::{Asset, Assets, Handle, HandleId};
use bevy::ecs::system::{EntityCommands, SystemParam};
use bevy::prelude::{Commands, Res};

/// A system param that provides easy usage of loaded [prototypical] assets.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use bevy_proto::prelude::*;
///
/// fn my_system(manager: ProtoManager, mut commands: Commands) {
///   if let Some(proto) = manager.get("My Prototype") {
///     proto.spawn(&mut commands);
///   }
/// }
/// ```
///
/// [prototypical]: crate::Prototypical
#[derive(SystemParam)]
pub struct ProtoManager<'w, 's, T: Prototypical + Asset = Prototype> {
    commands: Commands<'w, 's>,
    handle_to_name: Res<'w, HandleToName>,
    name_to_handle: Res<'w, NameToHandle>,
    prototypes: Res<'w, Assets<T>>,
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

    /// Spawns the [prototypical] asset with the given name or handle.
    ///
    /// [prototypical]: crate::Prototypical
    pub fn spawn<'a, I: Into<ProtoIdRef<'a>>>(&'a mut self, id: I) -> EntityCommands<'w, 's, 'a> {
        let entity = self.commands.spawn().id();
        self.commands.add(ProtoCommand::<T>::new(entity, id.into()));
        self.commands.entity(entity)
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
}
