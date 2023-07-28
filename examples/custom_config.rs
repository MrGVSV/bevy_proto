//! This example demonstrates how to implement a custom config.
//!
//! Configs are used to define certain aspects of prototype registration
//! as well as handle various callbacks during the life cycle of a prototype.
//!
//! Note that you don't need to create a custom config to do most things.
//! Instead, you can just define your logic on the default [`ProtoConfig`].
//!
//! Custom configs are mainly useful if you want to store or access data
//! during one of the callbacks.

use bevy::prelude::*;

use bevy_proto::backend::proto::Config;
use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // =============== //
        // Be sure to add `ProtoPlugin` with the custom config:
        .add_plugins(ProtoPlugin::new_with_config(MyConfig::default()))
        // Note: If you don't need to define your own type/resource,
        // you can also just add some callbacks to the default `ProtoConfig`:
        //
        // .add_plugins(
        //     ProtoPlugin::new().with_config(ProtoConfig::default().on_register_prototype(Box::new(
        //         |prototype, _handle| {
        //             println!("Registered: {:?}", prototype.id());
        //         },
        //     ))),
        // )
        //
        // =============== //
        .add_systems(Startup, load)
        .add_systems(
            Update,
            (
                // =============== //
                // For custom configs we also need to use `ProtoCondition::<MyConfig>::prototype_ready`
                // instead of the default `prototype_ready` for our run condition:
                spawn.run_if(
                    ProtoCondition::<MyConfig>::prototype_ready("Player").and_then(run_once()),
                ),
                // =============== //
                on_config_change,
                inspect,
            ),
        )
        .run();
}

/// Our own custom config.
#[derive(Resource, Default)]
struct MyConfig {
    is_changed: bool,
    last_registered: Option<Handle<Prototype>>,
}

impl Config<Prototype> for MyConfig {
    // ============================= //
    // Define custom callbacks here.
    // ============================= //

    fn on_register_prototype(&mut self, prototype: &Prototype, handle: Handle<Prototype>) {
        // For example, we can print the ID of every prototype that's registered:
        println!("Registered: {:?}", prototype.id());

        // And since this is a resource, we can store whatever we want on it:
        self.last_registered = Some(handle.clone_weak());
        self.is_changed = true;
    }
}

// By default all bevy_proto system parameters use the standard `ProtoConfig`.
// However, if we want to use our custom config, we need to pass it as a type parameter.
// In other words, we need to use `PrototypesMut<MyConfig>` instead of `PrototypesMut`.
fn load(mut prototypes: PrototypesMut<MyConfig>) {
    prototypes.load("examples/custom_config/Player.prototype.ron");
}

// The same applies here: we need to use `ProtoCommands<MyConfig>` instead of `ProtoCommands`.
fn spawn(mut commands: ProtoCommands<MyConfig>) {
    commands.spawn("Player");
}

// Our custom config is like any resource and is inserted into the world
// when we add the `ProtoPlugin`.
fn on_config_change(mut config: ResMut<MyConfig>) {
    if !config.is_changed {
        return;
    }

    println!("Last Registered: {:?}", config.last_registered);

    config.is_changed = false;
}

// This relies on the `auto_name` feature to be useful
fn inspect(query: Query<DebugName, Added<Name>>) {
    for name in &query {
        println!("Spawned: {:?}", name);
    }
}
