//! This example demonstrates how to load a prototype and use it.

use bevy::prelude::*;

use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // The `ProtoPlugin` adds all the necessary systems and resources
        // to load and spawn prototype assets.
        .add_plugin(ProtoPlugin::new())
        .add_startup_systems((setup, load))
        .add_systems((
            spawn.run_if(prototype_ready("Player").and_then(run_once())),
            inspect,
        ))
        .run();
}

/// A prototype is pretty much like any other asset and needs to be
/// loaded first in order to be used.
/// We can do this with the `AssetServer` like normal:
fn _load_with_server(asset_server: Res<AssetServer>, mut handle: Local<Handle<Prototype>>) {
    // Just like with any other asset, we need to store its strong handle
    // so that it remains loaded.
    // Normally, we'd use a resource for that, but here we'll just use a `Local<T>`
    *handle = asset_server.load("examples/loading/Player.prototype.ron");
}

/// This crate comes with a built-in way to store those handles automatically.
/// [`PrototypesMut`] provides a convenience method that will store the handles
/// until manually removed.
fn load(mut prototypes: PrototypesMut) {
    // This will load the asset and store the returned strong handle.
    // It also returns a cloned strong handle if needed.
    prototypes.load("examples/loading/Player.prototype.ron");
}

/// We're using the provided run-condition, [`prototype_ready`],
/// to check load status.
/// However, we can check load status manually using the [`Prototypes`]
/// system parameter.
///
/// It's very important that we don't rely on the `AssetServer` or
/// the `Assets<Prototype>` resource for this.
/// The reason is because prototypes need to go through a registration phase
/// once loaded.
/// This means they might be _loaded_ but not yet _yet_.
/// So prefer this method for checking the status before spawning in order
/// to avoid any runtime panics.
fn _is_ready(prototypes: Prototypes) -> bool {
    prototypes.is_ready("Player")

    // We could also check the prototype's load state here
    // (which accounts for the registration phase):
    // prototypes.get_load_state(&handle) == LoadState::Loaded
}

/// To spawn a prototype entity, we can use [`ProtoCommands`].
///
/// This wraps around a standard `Commands` but is built to properly handle
/// prototypes.
/// Once spawned, a [`ProtoEntityCommands`] is returned.
/// You can use this to further apply prototypes to the entity,
/// remove them, or get the entity's ID.
fn spawn(mut commands: ProtoCommands) {
    commands.spawn("Player");
}

/// With the `auto_name` feature enabled (which is enabled by default),
/// any prototype entity that doesn't already have a [`Name`] will
/// automatically be given the name of the prototype.
///
/// For example, spawning our `Player` prototype which has the name
/// "Player" will result in it being named "Player (Prototype)".
fn inspect(query: Query<DebugName, Added<Name>>) {
    for name in &query {
        println!("Spawned: {:?}", name);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
