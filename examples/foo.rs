#![allow(unused_doc_comments)]
//! TODO: Remove file

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::reflect::FromReflect;
use serde::{Deserialize, Serialize};

use bevy_proto::prelude::*;

#[derive(Reflect, FromReflect, ProtoComponent, Component, Clone, Deserialize, Serialize)]
#[reflect(ProtoComponent, Deserialize, Serialize)]
struct Person {
    pub name: String,
}

#[derive(Reflect, FromReflect, ProtoComponent, Component, Copy, Clone, Deserialize, Serialize)]
#[reflect(ProtoComponent, Deserialize, Serialize)]
struct Ordered {
    pub order: i32,
}

struct MyProto {
    handle: Handle<Prototype>,
}

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut manager: ResMut<ProtoManager<Prototype>>,
) {
    let handle: Handle<Prototype> = asset_server.load("prototypes/test-2.prototype.yaml");
    manager.spawn(handle.clone());
    commands.insert_resource(MyProto { handle })
}

fn test(
    mut commands: Commands,
    assets: Res<Assets<Prototype>>,
    res: Option<Res<MyProto>>,
    mut flag: Local<bool>,
    mut flag1: Local<bool>,
    asset_server: Res<AssetServer>,
) {
    if let Some(res) = &res {
        if asset_server.get_load_state(res.handle.id) == LoadState::Loaded {
            println!("Loaded!, {}", assets.get(res.handle.id).is_some());
        }
    }

    if *flag1 {
        return;
    }
    if let Some(res) = res {
        if let Some(proto) = assets.get(res.handle.id) {
            if !*flag {
                *flag = true;
                return;
            }
            proto.spawn(&mut commands);
            *flag1 = true;
        }
    }
}

fn print_objects(q: Query<(Entity, &Person, &Ordered)>) {
    for (e, p, o) in q.iter() {
        println!("{}: {} ({:?})", o.order, p.name, e);
    }
}

fn list_protos(assets: Res<Assets<Prototype>>) {
    for (p, o) in assets.iter() {
        println!("{:?}: {}", p, o.name());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_type::<Person>()
        .register_type::<Ordered>()
        // This plugin should come AFTER any others that it might rely on
        // In this case, we need access to what's added by [`DefaultPlugins`]
        // so we place this line after that one
        .add_plugin(ProtoPlugin::<Prototype>::default())
        // Add our spawner system (this one only runs once at startup)
        // .add_startup_system(spawn_person)
        .add_startup_system(load)
        // .add_system(introduce)
        .add_system(test)
        .add_system(print_objects)
        // .add_system(list_protos)
        .run();
}
