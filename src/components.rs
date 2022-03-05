use bevy::prelude::{AssetServer, Res, World};

use crate::data::{ProtoCommands, ProtoData};
use crate::prototype::Prototypical;

/// A trait that allows components to be used within [`Prototypical`] structs
#[typetag::serde(tag = "type", content = "value")]
pub trait ProtoComponent: Send + Sync + 'static {
	fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>);
	#[allow(unused_variables)]
	fn prepare(&self, world: &mut World, prototype: &dyn Prototypical, data: &mut ProtoData) {}
}
