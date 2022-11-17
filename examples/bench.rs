use std::time::Instant;

use bevy::prelude::*;
use bevy_proto::prelude::*;
use serde::{Deserialize, Serialize};

const ENTITY_COUNT: u128 = 200_000;
const BATCH_SIZE: u128 = 5_000;
const BATCH_COUNT: u128 = ENTITY_COUNT / BATCH_SIZE;

fn spawn_sprites_proto(
    mut commands: Commands,
    data: Res<ProtoData>,
    asset_server: Res<AssetServer>,
) {
    println!("Spawning via Prototype:");
    let mut total: u128 = 0;
    let mut before = Instant::now();
    let proto = data.get_prototype("Sprite Test").expect("Should exist!");

    for _ in 0..BATCH_COUNT {
        for _ in 0..BATCH_SIZE {
            proto.spawn(&mut commands, &data, &asset_server);
        }
        println!("Prototype Batch: {:.2?}", before.elapsed());
        total += before.elapsed().as_millis();
        before = Instant::now();
    }

    println!(
        "Prototypes: {}ms for {} (avg. batch {}ms)",
        total,
        ENTITY_COUNT,
        total / BATCH_COUNT
    );
}

fn spawn_sprites_programmatic(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning Programmatically:");
    let mut total: u128 = 0;
    let mut before = Instant::now();

    for _ in 0..BATCH_COUNT {
        for _ in 0..BATCH_SIZE {
            commands.spawn(SpriteBundle {
                texture: asset_server.load("textures/sprite.png"),
                ..Default::default()
            });
        }
        println!("Programmatic Batch: {:.2?}", before.elapsed());
        total += before.elapsed().as_millis();
        before = Instant::now();
    }

    println!(
        "Programmatic: {}ms for {} (avg. batch {}ms)",
        total,
        ENTITY_COUNT,
        total / BATCH_COUNT
    );
}

fn main() {
    println!(
        "Entity Count: {} | Batch Size: {}",
        ENTITY_COUNT, BATCH_SIZE
    );
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ProtoPlugin::default())
        .add_startup_system(spawn_sprites_proto.label("prototype"))
        .add_startup_system(spawn_sprites_programmatic.after("prototype"))
        .run();
}

/// The code below is covered in the `bundles` example. It's an implementation
/// detail we don't need to focus on for this particular example

#[derive(Serialize, Deserialize, Component)]
struct SpriteBundleDef {
    pub texture_path: HandlePath,
}

#[typetag::serde]
impl ProtoComponent for SpriteBundleDef {
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &AssetServer) {
        // === Get Prepared Assets === //
        let texture: Handle<Image> = asset_server.get_handle(&self.texture_path);

        // === Generate Bundle === //
        let my_bundle = SpriteBundle {
            texture,
            ..Default::default()
        };

        // === Insert Generated Bundle === //
        commands.insert(my_bundle);
    }

    fn prepare(&self, world: &mut World, prototype: &dyn Prototypical, data: &mut ProtoData) {
        // === Load Handles === //
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let handle: Handle<Image> = asset_server.load(self.texture_path.as_str());

        // === Save Handles === //
        data.insert_handle(prototype, self, handle);
    }
}
