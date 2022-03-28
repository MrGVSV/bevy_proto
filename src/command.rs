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
        apply::<T>(&self.id, self.entity, world);
    }
}

fn apply<T: Prototypical + Asset>(id: &ProtoId, entity: Entity, world: &mut World) {
    let handle = match id {
        ProtoId::Handle(handle) => *handle,
        ProtoId::Name(name) => {
            let name_to_handle = world.resource::<NameToHandle>();
            *name_to_handle
                .read()
                .get(name)
                .ok_or_else(|| format!("Could not find handle for prototype named `{}`.\n\
                This typically occurs when trying to load a prototype before the corresponding ProtoManager has had a chance to track it.\n\
                Try waiting a frame or two before applying the prototype, or use its handle directly.", name)).unwrap()
        }
    };
    world.resource_scope(|world, assets: Mut<Assets<T>>| {
        if let Some(proto) = assets.get(handle) {
            if let Some(ref mut entity) = world.get_entity_mut(entity) {
                apply_proto(proto, &assets, entity);
            } else {
                panic!("Could not get access to entity {:?} because it doesn't exist in this World.\n\
                        If this command was added to a newly spawned entity, ensure that you have not despawned that entity within the same stage.\n\
                        This may have occurred due to system order ambiguity, or if the spawning system has multiple command buffers", entity);
            }
        } else {
            panic!(
                "{}",
                ProtoSpawnError::NotLoaded{ id: id.clone() }
            );
        }
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
