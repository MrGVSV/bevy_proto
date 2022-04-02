use crate::errors::ProtoSpawnError;
use crate::manager::{NameToHandle, ProtoId};
use crate::prelude::Prototypical;
use crate::templates::apply_templates;
use bevy::asset::{Asset, Assets};
use bevy::ecs::system::Command;
use bevy::ecs::world::EntityMut;
use bevy::prelude::{Entity, Mut, World};
use std::marker::PhantomData;

pub(crate) struct ProtoCommand<T: Prototypical + Asset> {
    entity: Entity,
    id: ProtoId,
    phantom: PhantomData<T>,
}

impl<T: Prototypical + Asset> ProtoCommand<T> {
    pub fn new<I: Into<ProtoId>>(entity: Entity, id: I) -> Self {
        Self {
            entity,
            id: id.into(),
            phantom: PhantomData::default(),
        }
    }
}

impl<T: Prototypical + Asset> Command for ProtoCommand<T> {
    fn write(self, world: &mut World) {
        apply::<T>(self.id, self.entity, world);
    }
}

fn apply<T: Prototypical + Asset>(id: ProtoId, entity: Entity, world: &mut World) {
    let handle = match id {
        ProtoId::Handle(handle) => handle,
        ProtoId::Name(ref name) => *world
            .get_resource::<NameToHandle>()
            .unwrap()
            .read()
            .get(name)
            .ok_or_else(|| ProtoSpawnError::NotLoaded { id: id.clone() })
            .unwrap(),
    };
    world.resource_scope(|world, assets: Mut<Assets<T>>| {
        let proto = assets
            .get(handle)
            .ok_or_else(|| ProtoSpawnError::NotLoaded { id: id.clone() })
            .unwrap();
        let entity = &mut world
            .get_entity_mut(entity)
            .ok_or_else(|| ProtoSpawnError::InvalidEntity { entity })
            .unwrap();
        apply_proto(proto, &assets, entity);
    });
}

/// Applies components in a bottom-up fashion: from deepest template to the prototype itself
fn apply_proto<T: Prototypical + Asset>(proto: &T, assets: &Assets<T>, entity: &mut EntityMut) {
    apply_templates(proto, assets, &mut |template| {
        for component in template.components() {
            component.apply(entity);
        }
    })
    .ok();
}
