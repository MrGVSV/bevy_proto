//! This example is a copy of Bevy's [`asset_loading`] example, but powered by the `bevy_proto` plugin.
//!
//! The most notable change is the use of _inlined assets_.
//! Rather than having to define our simple cube and sphere shapes in separate asset files,
//! we can define them directly in the prototype.
//!
//! Prototypes aren't a perfect replacement for Rust code—
//! especially if there's more advanced logic involved—
//! but they can be a great tool for rapid prototyping.
//!
//! [`asset_loading`]: https://github.com/bevyengine/bevy/blob/v0.11.2/examples/asset/asset_loading.rs

use bevy::prelude::*;
use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ProtoPlugin::default()))
        .add_systems(Startup, load)
        .add_systems(
            Update,
            (
                spawn_camera.run_if(prototype_ready("Camera").and_then(run_once())),
                spawn_light.run_if(prototype_ready("Light").and_then(run_once())),
                spawn_models
                    .run_if(prototypes_ready(["Sphere", "Cube", "Monkey"]).and_then(run_once())),
                inspect,
            ),
        )
        .run();
}

fn load(mut prototypes: PrototypesMut) {
    prototypes
        .load_folder("examples/bevy/asset_loading")
        .unwrap();
}

fn spawn_camera(mut commands: ProtoCommands) {
    commands.spawn("Camera");
}

fn spawn_light(mut commands: ProtoCommands) {
    commands.spawn("Light");
}

fn spawn_models(mut commands: ProtoCommands) {
    commands.spawn("Sphere");
    commands.spawn("Cube");
    commands.spawn("Monkey");
}

// This relies on the `auto_name` feature to be useful
fn inspect(query: Query<DebugName, Added<Name>>) {
    for name in &query {
        println!("Spawned {:?}", name);
    }
}
