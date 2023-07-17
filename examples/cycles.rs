//! This example demonstrates how cycles are handled.
//!
//! A cycle is where a prototype recursively references itself.
//! In other words, it is found in its own template chain or child hierarchy.
//!
//! In this example, `CycleA` is loaded.
//! However, it inherits from `CycleB`, which contains child `CycleC`,
//! which inherits `CycleA`— oh no, a cycle!
//!
//! By default, this will panic, resulting in the following message:
//!
//! ```
//! found prototype cycle: `"examples/cycles/CycleA.prototype.ron" inherits "examples/cycles/CycleB.prototype.ron" which contains "examples/cycles/CycleC.prototype.ron" which inherits "examples/cycles/CycleA.prototype.ron"`
//! ```
//!
//! However, we can also choose to simply ignore the cycle by configuring [`ProtoConfig`].
//! If ignored, a detected cycle will just skip the recursion and move on.
//! If we did that here, we'd spawn two entities: `CycleA (CycleB)` and `CycleC`
//! where `CycleC` does not inherit from `CycleA` (due to the skipped cycle).

use bevy::prelude::*;

use bevy_proto::config::ProtoConfig;
use bevy_proto::prelude::*;
use bevy_proto_backend::cycles::CycleResponse;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(
            ProtoPlugin::new().with_config(ProtoConfig::default().on_cycle(Box::new(|cycle| {
                println!("Handling cycle: {:?}", cycle);
                // Here we can configure what CycleResponse is returned.
                // For debug builds, the default behavior is to panic.
                CycleResponse::Panic
            }))),
        )
        .add_systems(Startup, load)
        .add_systems(Update, (
            spawn.run_if(prototype_ready("CycleA").and_then(run_once())),
            inspect,
        ))
        .run();
}

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/cycles/CycleA.prototype.ron");
}

fn spawn(mut commands: ProtoCommands) {
    commands.spawn("CycleA");
}

// This relies on the `auto_name` feature to be useful
fn inspect(query: Query<DebugName, Added<Name>>) {
    for name in &query {
        println!("Spawned: {:?}", name);
    }
}
