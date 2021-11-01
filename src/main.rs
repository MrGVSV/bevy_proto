use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::utils::tracing::field::AsField;
use bevy::utils::tracing::Metadata;
use serde::{Deserialize, Serialize};
use std::any::Any;

use bevy_proto::{
	HandlePath, ProtoComponent, ProtoData, ProtoPlugin, Prototype, PrototypeDataContainer,
	Prototypical,
};

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_startup_system(hot_reload.system())
		// === Setup === //
		.add_plugin(ProtoPlugin::default())
		// /== Setup === //
		.add_startup_system(spawn.system())
		.run();
}

fn hot_reload(asset_server: Res<AssetServer>) {
	asset_server.watch_for_changes().unwrap();
}

#[derive(Serialize, Deserialize)]
struct SpriteBundleDef {
	pub texture_path: HandlePath,
	pub other_path: HandlePath,
}

#[typetag::serde]
impl ProtoComponent for SpriteBundleDef {
	fn insert_self(
		&self,
		entity: &mut EntityCommands,
		prototype: &dyn Prototypical,
		proto_res: &Res<ProtoData>,
	) {
		entity.insert_bundle(SpriteBundle {
			material: proto_res
				.get_handle::<ColorMaterial>(prototype, self, &self.texture_path)
				.unwrap(),
			..Default::default()
		});
	}

	fn prepare(
		&self,
		world: &mut World,
		prototype: &Box<dyn Prototypical>,
		proto_res: &mut ProtoData,
	) {
		// === Load Handles === //
		let asset_server = world.get_resource::<AssetServer>().unwrap();
		let texture: Handle<Texture> = asset_server.load(self.texture_path.as_str());
		let other_tex: Handle<Texture> = asset_server.load(self.other_path.as_str());

		// === Transform Handles === //
		let mut mat_res = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
		let mat = mat_res.add(texture.into());
		let other_mat = mat_res.add(other_tex.into());

		// === Save Handles === //
		proto_res.insert_handle(prototype, self, &self.texture_path, mat);
		proto_res.insert_handle(prototype, self, &self.other_path, other_mat);
	}
}

fn spawn(
	mut commands: Commands,
	proto_data: Res<ProtoData>,
	asset_server: Res<AssetServer>,
	mut mats: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	let proto = proto_data
		.get_prototype("Test Prototype")
		.expect("Should exist!");

	proto.spawn(&mut commands, &proto_data);
}
