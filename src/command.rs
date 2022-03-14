use crate::manager::ProtoManager;
use crate::prelude::Prototypical;
use crate::utils::for_each_template;
use bevy::asset::{Asset, Assets, HandleId};
use bevy::ecs::system::Command;
use bevy::ecs::world::EntityMut;
use bevy::prelude::{Entity, Mut, World};
use std::marker::PhantomData;

enum ProtoId {
    Handle(HandleId),
    Name(String),
}

pub(crate) struct ProtoCommand<T: Prototypical + Asset> {
    entity: Entity,
    id: ProtoId,
    phantom: PhantomData<T>,
}

impl<T: Prototypical + Asset> ProtoCommand<T> {
    pub fn from_handle<H: Into<HandleId>>(entity: Entity, handle: H) -> Self {
        Self {
            entity,
            id: ProtoId::Handle(handle.into()),
            phantom: PhantomData::default(),
        }
    }

    pub fn from_name<S: Into<String>>(entity: Entity, name: S) -> Self {
        Self {
            entity,
            id: ProtoId::Name(name.into()),
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
            let manager = world.resource::<ProtoManager<T>>();
            manager
                .get_handle(name)
                .ok_or_else(|| format!("Could not find handle for prototype named `{}`.\n\
                This typically occurs when trying to load a prototype before the corresponding ProtoManager has had a chance to track it.\n\
                Try waiting a frame or two before applying the prototype, or use its handle directly.", name)).unwrap().id
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
                "Prototype with handle `{:?}` has not been loaded",
                handle
            );
        }
    });
}

/// Applies components in a bottom-up fashion: from deepest template to the prototype itself
fn apply_proto<T: Prototypical + Asset>(proto: &T, assets: &Assets<T>, entity: &mut EntityMut) {
    for_each_template(proto, assets, &mut |template| {
        apply_proto(template, assets, entity);
    });

    for component in proto.components() {
        component.apply(entity);
    }
}
