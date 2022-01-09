//! This example demonstrates the usage of templates.
//!
//! The best way to see how it works is to browse through the associated prototypes in
//! `assets/prototypes/templates/`. Then run this example and see how the template prototype
//! influences its inheritors.
//!
//! Essentially, all inheritors of a template take on that template's components. If they
//! define their own version of a component, then that version will supersede the template's.
//! And lastly, if the component defines their own components, these will be applied as normal.
//!
//! Use templates to reduce markup duplication and bundle common components.
//!

#![allow(unused_doc_comments)]

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use bevy_proto::{ProtoComponent, ProtoData, ProtoPlugin};

#[derive(Serialize, Deserialize, ProtoComponent)]
struct NPC;

#[derive(Serialize, Deserialize, ProtoComponent, Component)]
struct Occupation(OccupationType);

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
enum OccupationType {
	Unemployed,
	Miner,
	Shopkeeper,
}

#[derive(Serialize, Deserialize, ProtoComponent, Component)]
struct Health {
	max: u16,
}

#[derive(Serialize, Deserialize, ProtoComponent, Component)]
struct Named(String);

/// Spawn in the NPC
fn spawn_npc(mut commands: Commands, data: Res<ProtoData>, asset_server: Res<AssetServer>) {
	let proto = data.get_prototype("Alice").expect("Should exist!");
	proto.spawn(&mut commands, &data, &asset_server);
	let proto = data.get_prototype("Bob").expect("Should exist!");
	proto.spawn(&mut commands, &data, &asset_server);
	let proto = data.get_prototype("Urist").expect("Should exist!");
	proto.spawn(&mut commands, &data, &asset_server);
	let proto = data.get_prototype("Mystery").expect("Should exist!");
	proto.spawn(&mut commands, &data, &asset_server);
}

/// Handle the NPC spawning
fn on_spawn(query: Query<(&Health, &Occupation, Option<&Named>), Added<NPC>>) {
	for (health, occupation, name) in query.iter() {
		let name = if let Some(name) = name {
			format!("'{}'", name.0)
		} else {
			String::from("<UNKNOWN>")
		};
		println!(
			"NPC {} => MaxHP: {} | Occupation: {:?}",
			name, health.max, occupation.0
		);
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(ProtoPlugin::with_dir("assets/prototypes/templates"))
		.add_startup_system(spawn_npc.system())
		.add_system(on_spawn.system())
		.run();
}
