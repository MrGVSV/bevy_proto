//! This example demonstrates how to create basic schematics using
//! components/resources and the derive macro.

use bevy::prelude::*;

use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // =============== //
        // Make sure to register your types!
        .register_type::<Playable>()
        .register_type::<Alignment>()
        // If you didn't use `#[reflect(Schematic)]`,
        // you can still register it manually:
        .register_type::<PlayerHealth>()
        .register_type_data::<PlayerHealth, ReflectSchematic>()
        // =============== //
        .add_plugin(ProtoPlugin::default())
        .add_startup_systems((setup, load))
        .add_systems((
            spawn.run_if(prototype_ready("Player").and_then(run_once())),
            inspect,
        ))
        .run();
}

// A schematic can be pretty much anything that mutates the world.
// The simplest type of a schematic is just a regular Bevy component.
// For components, we can simply add the `Schematic` derive:
#[derive(Component, Schematic)]
// First thing's first, we need to derive `Reflect` so that we can register
// this type to the registry (speaking of, don't forget to do that!):
#[derive(Reflect)]
// For the basic schematic types we also need to derive `FromReflect` so that
// we can convert the deserialized data into a real instance of our type:
#[derive(FromReflect)]
// Lastly, we need to register `ReflectSchematic`, which can do like this:
#[reflect(Schematic)]
struct Playable;

/// The derive also works for enums!
#[derive(Component, Schematic, Reflect, FromReflect, Debug)]
#[reflect(Schematic)]
enum Alignment {
    Good,
    Neutral,
    Evil,
}

/// The derive macro also has basic support for Bevy resources.
///
/// This can be done by specifying the "kind".
///
/// Note that when a schematic is applied, it will replace the current instance
/// of the resource in the world.
#[derive(Resource, Schematic, Reflect, FromReflect)]
#[schematic(kind = "resource")]
struct PlayerHealth(u16);

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/basic_schematic/Player.prototype.ron");
}

fn spawn(mut commands: ProtoCommands) {
    commands.spawn("Player");
}

// This relies on the `auto_name` feature to be useful
fn inspect(query: Query<(DebugName, &Alignment), Added<Playable>>) {
    for (name, alignment) in &query {
        println!("===============");
        println!("Spawned Player:");
        println!("  ID: {name:?}");
        println!("  Alignment: {alignment:?}");
        println!("===============");
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
