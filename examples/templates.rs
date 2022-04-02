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
use bevy::reflect::FromReflect;
use serde::Deserialize;

use bevy_proto::prelude::*;

#[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
struct NPC;

#[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
struct Occupation(OccupationType);

#[derive(Reflect, FromReflect, Component, ProtoComponent, Deserialize, Clone, Debug)]
#[reflect_value(ProtoComponent, Deserialize)]
enum OccupationType {
    Unemployed,
    Miner,
    Shopkeeper,
}

#[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
struct Health {
    max: u16,
}

#[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
struct Named(String);

/// Load the prototypes
fn load_prototypes(asset_server: Res<AssetServer>, mut manager: ProtoManager<Prototype>) {
    let handles = asset_server.load_folder("prototypes/templates").unwrap();
    manager.add_multiple_untyped(handles);
}

/// Spawn in the NPC
fn spawn_npc(mut manager: ProtoManager<Prototype>, mut has_ran: Local<bool>) {
    if *has_ran {
        return;
    }

    if !manager.all_loaded() {
        return;
    }

    manager.spawn("Alice");
    manager.spawn("Bob");
    manager.spawn("Urist");
    manager.spawn("Mystery");

    *has_ran = true;
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
        .add_plugin(ProtoPlugin::<Prototype>::default())
        // !!! Make sure you register your types !!! //
        .register_type::<NPC>()
        .register_type::<Occupation>()
        .register_type::<OccupationType>()
        .register_type::<Health>()
        .register_type::<Named>()
        .add_startup_system(load_prototypes)
        .add_system(spawn_npc)
        .add_system(on_spawn)
        .run();
}
