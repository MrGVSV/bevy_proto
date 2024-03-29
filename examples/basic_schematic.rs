//! This example demonstrates how to create basic schematics using
//! components/resources and the derive macro.

use bevy::prelude::*;

use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ProtoPlugin::new()))
        // =============== //
        // Make sure to register your types!
        .register_type::<Playable>()
        .register_type::<Alignment>()
        // If you didn't use `#[reflect(Schematic)]`,
        // you can still register it manually:
        .register_type::<MaxPlayers>()
        .register_type_data::<MaxPlayers, ReflectSchematic>()
        // =============== //
        .add_systems(Startup, (setup, load))
        .add_systems(
            Update,
            (
                spawn.run_if(
                    prototype_ready("Player")
                        .and_then(prototype_ready("PlayerConfig"))
                        .and_then(run_once()),
                ),
                inspect,
            ),
        )
        .run();
}

// A schematic can be pretty much anything that mutates the world.
// The simplest type of a schematic is just a regular Bevy component.
// For components, we can simply add the `Schematic` derive:
#[derive(Component, Schematic)]
// First thing's first, we need to derive `Reflect` so that we can register
// this type to the registry (speaking of, don't forget to do that!):
#[derive(Reflect)]
// Lastly, we need to register `ReflectSchematic`, which can do like this:
#[reflect(Schematic)]
struct Playable;

/// The derive also works for enums!
#[derive(Component, Schematic, Reflect, Debug)]
#[reflect(Schematic)]
enum Alignment {
    Good,
    Neutral,
    Evil,
}

/// The derive macro also has basic support for Bevy resources.
///
/// This can be done by specifying the "kind" as "resource".
/// It's also a good idea to set `entity: false` in the prototype file
/// so that the prototype doesn't spawn an entity.
///
/// Note that when a schematic is applied, it will replace the current instance
/// of the resource in the world.
#[derive(Resource, Schematic, Reflect)]
#[schematic(kind = "resource")]
struct MaxPlayers(u8);

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/basic_schematic/Player.prototype.ron");
    prototypes.load("examples/basic_schematic/PlayerConfig.prototype.ron");
}

fn spawn(mut commands: ProtoCommands) {
    commands.spawn("Player");
    // Since this schematic just defines a resource and doesn't need an entity,
    // we can use `apply` instead of `spawn`.
    commands.apply("PlayerConfig");
}

// This relies on the `auto_name` feature to be useful
fn inspect(
    query: Query<(DebugName, &Alignment), Added<Playable>>,
    max_players: Option<Res<MaxPlayers>>,
) {
    for (name, alignment) in &query {
        println!("===============");
        println!("Spawned Player:");
        println!("  ID: {name:?}");
        println!("  Alignment: {alignment:?}");
        println!("===============");
        match &max_players {
            Some(max_players) if max_players.is_added() => {
                println!("Max. Players: {}", max_players.0);
                println!("===============");
            }
            _ => {}
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
