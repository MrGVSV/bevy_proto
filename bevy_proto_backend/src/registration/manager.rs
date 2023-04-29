use bevy::asset::Handle;
use bevy::ecs::system::SystemParam;
use bevy::prelude::ResMut;

use crate::proto::{ProtoError, Prototypical};
use crate::registration::params::RegistryParams;
use crate::registration::ProtoRegistry;

/// Manager [`SystemParam`] for [prototypes].
///
/// This is used to easily manager the [`ProtoRegistry`].
///
/// [prototypes]: Prototypical
#[derive(SystemParam)]
pub(crate) struct ProtoManager<'w, T: Prototypical> {
    registry: ResMut<'w, ProtoRegistry<T>>,
    registry_params: RegistryParams<'w, T>,
}

impl<'w, T: Prototypical> ProtoManager<'w, T> {
    pub fn register(&mut self, handle: &Handle<T>) -> Result<&'w T, ProtoError> {
        self.registry.register(handle, &mut self.registry_params)
    }

    pub fn reload(&mut self, handle: &Handle<T>) -> Result<&'w T, ProtoError> {
        self.registry.reload(handle, &mut self.registry_params)
    }

    pub fn unregister(&mut self, handle: &Handle<T>) -> Option<T::Id> {
        self.registry.unregister(handle, &mut self.registry_params)
    }
}
