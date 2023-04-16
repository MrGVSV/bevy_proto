use bevy::asset::{Assets, HandleId};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Res, ResMut};

use crate::proto::{ProtoError, Prototypical};
use crate::registration::ProtoRegistry;

/// Manager [`SystemParam`] for [prototypes].
///
/// This is used to easily manager the [`ProtoRegistry`].
///
/// [prototypes]: Prototypical
#[derive(SystemParam)]
pub(crate) struct ProtoManager<'w, T: Prototypical> {
    registry: ResMut<'w, ProtoRegistry<T>>,
    prototypes: Res<'w, Assets<T>>,
    config: ResMut<'w, <T as Prototypical>::Config>,
}

impl<'w, T: Prototypical> ProtoManager<'w, T> {
    pub fn register<H: Into<HandleId>>(&mut self, handle: H) -> Result<&T, ProtoError> {
        self.registry
            .register(handle, &self.prototypes, &mut self.config)
    }

    pub fn unregister<H: Into<HandleId>>(&mut self, handle: H) -> bool {
        self.registry
            .unregister(handle, &self.prototypes, &mut self.config)
    }
}
