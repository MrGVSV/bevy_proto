use crate::prelude::ProtoComponent;
use bevy::ecs::system::Command;
use bevy::prelude::{Entity, World};

pub struct ProtoCommand<T: ProtoComponent> {
    pub entity: Entity,
    pub component: T,
}

impl<T: ProtoComponent> Command for ProtoCommand<T> {
    fn write(self, world: &mut World) {
        // self.component.prepare()
        if let Some(mut _entity) = world.get_entity_mut(self.entity) {
        } else {
            panic!("Could not add a component (of type `{}`) to entity {:?} because it doesn't exist in this World.\n\
                    If this command was added to a newly spawned entity, ensure that you have not despawned that entity within the same stage.\n\
                    This may have occurred due to system order ambiguity, or if the spawning system has multiple command buffers", std::any::type_name::<T>(), self.entity);
        }
    }
}
