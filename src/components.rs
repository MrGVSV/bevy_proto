use bevy::ecs::component::Component;
use bevy::prelude::{AssetServer, Res, World};

use crate::{ProtoCommands, ProtoData, Prototypical};

pub trait NoStorageComponent: Send + Sync + 'static {}

/// A trait that allows components to be used within [`Prototypical`] structs
#[typetag::serde(tag = "type", content = "value")]
pub trait ProtoComponent: Send + Sync + 'static {
	fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>);
	#[allow(unused_variables)]
	fn prepare(&self, world: &mut World, prototype: &Box<dyn Prototypical>, data: &mut ProtoData) {}
}
