use bevy::ecs::component::Component;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{AssetServer, Res, World};
use serde::{Deserialize, Serialize};

use crate::{ProtoData, PrototypeDataContainer, Prototypical};

/// A trait that allows components to be used within [`Prototypical`] structs
#[typetag::serde(tag = "type", content = "value")]
pub trait ProtoComponent: Component {
	fn insert_self(
		&self,
		entity: &mut EntityCommands,
		prototype: &dyn Prototypical,
		data: &Res<ProtoData>,
	);
	fn prepare(&self, world: &mut World, prototype: &Box<dyn Prototypical>, data: &mut ProtoData) {}
}
