use crate::assets::ProtoAssetEvent;
use crate::proto::{Config, ProtoError, Prototypical};
use bevy::asset::{Assets, Handle, HandleId};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{EventWriter, Res, ResMut};

#[derive(SystemParam)]
pub(super) struct RegistryParams<'w, T: Prototypical, C: Config<T>> {
    prototypes: Res<'w, Assets<T>>,
    config: ResMut<'w, C>,
    proto_events: EventWriter<'w, ProtoAssetEvent<T>>,
}

impl<'w, T: Prototypical, C: Config<T>> RegistryParams<'w, T, C> {
    pub fn prototypes(&self) -> &'w Assets<T> {
        Res::clone(&self.prototypes).into_inner()
    }

    pub fn config(&self) -> &C {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut C {
        &mut self.config
    }

    pub fn get_prototype(&self, handle: &Handle<T>) -> Result<&'w T, ProtoError> {
        self.prototypes()
            .get(handle)
            .ok_or_else(|| ProtoError::DoesNotExist(handle.clone_weak_untyped()))
    }

    pub fn get_strong_handle<H: Into<HandleId>>(&self, handle: H) -> Handle<T> {
        self.prototypes().get_handle(handle)
    }

    pub fn send_event(&mut self, event: ProtoAssetEvent<T>) {
        self.proto_events.send(event);
    }
}
