use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use bevy_proto::{HandlePath, ProtoCommands, ProtoComponent, ProtoData, ProtoPlugin, Prototypical};

#[derive(Serialize, Deserialize)]
struct SpriteBundleDef {
	pub texture_path: HandlePath,
}

#[typetag::serde]
impl ProtoComponent for SpriteBundleDef {
	fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
		// === Get Prepared Assets === //
		let material: Handle<ColorMaterial> = commands
			.get_handle(self, &self.texture_path)
			.expect("Expected ColorMaterial handle to have been created");

		// === Generate Bundle === //
		let my_bundle = SpriteBundle {
			material,
			..Default::default()
		};

		// === Insert Generated Bundle === //
		commands.insert_bundle(my_bundle);
	}

	/// Here, we prepare any assets that this bundle/component might need that require additional setup.
	/// Since we want to load a texture AND add it to the ColorMaterial asset store, we need to
	/// do so in this prepare method.
	///
	/// Please keep in mind the ordering here. Rust's borrow checker still applies here: we can't have
	/// both a mutable and immutable access to world at the same time. Therefore, you will need to break
	/// your world access into chunks, getting whatever handles or data you need along the way
	fn prepare(&self, world: &mut World, prototype: &Box<dyn Prototypical>, data: &mut ProtoData) {
		// === Load Handles === //
		let asset_server = world.get_resource::<AssetServer>().unwrap();
		let texture: Handle<Texture> = asset_server.load(self.texture_path.as_str());

		// === Transform Handles === //
		let mut mat_res = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
		let mat = mat_res.add(texture.into());

		// === Save Handles === //
		data.insert_handle(prototype, self, &self.texture_path, mat);
	}
}

fn spawn_sprite(mut commands: Commands, data: Res<ProtoData>, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	/// Here, we attempt to get our prototype by name.
	/// We'll raise an exception if it's not found, just so we can fail fast.
	/// In reality, you'll likely want to handle this prototype not existing.
	let proto = data.get_prototype("Sprite Test").expect("Should exist!");

	// Spawn in the prototype!
	proto.spawn(&mut commands, &data, &asset_server);
}

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		// This plugin should come AFTER any others that it might rely on
		// In this case, we need access to what's added by [`DefaultPlugins`]
		// so we place this line after that one
		.add_plugin(ProtoPlugin::default())
		// Add our spawner system (this one only runs once at startup)
		.add_startup_system(spawn_sprite.system())
		.run();
}
