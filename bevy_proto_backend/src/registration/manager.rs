use bevy::asset::Handle;
use bevy::ecs::system::SystemParam;
use bevy::prelude::ResMut;

use crate::proto::{Config, ProtoError, Prototypical};
use crate::registration::params::RegistryParams;
use crate::registration::ProtoRegistry;

/// Manager [`SystemParam`] for [prototypes].
///
/// This is used to easily manager the [`ProtoRegistry`].
///
/// [prototypes]: Prototypical
#[derive(SystemParam)]
pub(crate) struct ProtoManager<'w, T: Prototypical, C: Config<T>> {
    registry: ResMut<'w, ProtoRegistry<T, C>>,
    registry_params: RegistryParams<'w, T, C>,
}

impl<'w, T: Prototypical, C: Config<T>> ProtoManager<'w, T, C> {
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
