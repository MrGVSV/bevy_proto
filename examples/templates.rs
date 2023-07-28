//! This example demonstrates how to use prototypes as templates.
//!
//! Templates allow prototypes to inherit functionality from other
//! prototypes.
//! In this example, our `Player` inherits from the following prototypes:
//!
//! 1. `Small`
//! 2. `Big`
//! 3. `Green`
//! 4. `Red`
//!
//! The order of templates matters.
//! Templates closer to the start overwrite duplicate schematics
//! in templates closer to the end.
//! So in this case, `Player` overwrites `Small` which overwrites `Big`
//! which overwrites `Green` which overwrites `Red`.
//!
//! > Again, these overwrites only occur on schematics of the same type.
//!
//! In the end, since `Small` and `Big` share the same schematics, and
//! `Green` and `Red` also share schematics (that differ from the other two),
//! we should expect to see the schematics from `Small` and `Green` applied.
//! And indeed we see our `Player` as both small and green.

use bevy::prelude::*;

use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ProtoPlugin::new()))
        .register_type::<Colored>()
        .register_type::<Scaled>()
        .add_systems(Startup, (load, setup))
        .add_systems(
            Update,
            (
                spawn.run_if(prototype_ready("Player").and_then(run_once())),
                on_spawn,
                inspect,
            ),
        )
        .run();
}

/// A component used to tint a sprite.
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
struct Colored(Color);

/// A component used to scale a sprite.
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
struct Scaled(f32);

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/templates/Player.prototype.ron");
}

fn spawn(mut commands: ProtoCommands) {
    commands.spawn("Player");
}

fn on_spawn(
    mut colored_query: Query<(&Colored, &mut Sprite), Added<Colored>>,
    mut scaled_query: Query<(&Scaled, &mut Transform), Added<Scaled>>,
) {
    for (colored, mut sprite) in &mut colored_query {
        sprite.color = colored.0;
    }

    for (scaled, mut transform) in &mut scaled_query {
        transform.scale *= scaled.0;
    }
}

// This relies on the `auto_name` feature to be useful
fn inspect(query: Query<DebugName, Added<Name>>) {
    for name in &query {
        println!("Spawned: {:?}", name);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
