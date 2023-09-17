//! This example is a copy of Bevy's [`sprite_sheet`] example, but powered by the `bevy_proto` plugin.
//!
//! The most notable change is the use of _inlined assets_.
//! Rather than having to define our sprite sheet's [`TextureAtlas`] programmatically,
//! we can define it directly in the prototype.
//!
//! Prototypes aren't a perfect replacement for Rust code—
//! especially if there's more advanced logic involved—
//! but they can be a great tool for rapid prototyping.
//!
//! [`sprite_sheet`]: https://github.com/bevyengine/bevy/blob/v0.11.2/examples/2d/sprite_sheet.rs

use bevy::prelude::*;
use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(ProtoPlugin::new())
        .register_type::<AnimationIndices>()
        .register_type::<AnimationTimer>()
        .register_type::<TimerMode>() // Needs to be registered for `Timer` to be deserializable
        .add_systems(Startup, (setup, load))
        .add_systems(
            Update,
            (
                spawn.run_if(prototype_ready("Player").and_then(run_once())),
                animate_sprite,
                inspect,
            ),
        )
        .run();
}

#[derive(Component, Reflect, Schematic)]
#[reflect(Schematic)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut, Reflect, Schematic)]
#[reflect(Schematic)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/bevy/sprite_sheet/Player.prototype.ron");
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn(mut commands: ProtoCommands) {
    commands.spawn("Player");
}

// This relies on the `auto_name` feature to be useful
fn inspect(query: Query<DebugName, Added<Name>>) {
    for name in &query {
        println!("Spawned {:?}", name);
    }
}
