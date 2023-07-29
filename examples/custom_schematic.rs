//! This example demonstrates how to manually implement `Schematic`
//! for custom behavior.

use bevy::prelude::*;

use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ProtoPlugin::new()))
        // =============== //
        // Make sure to register your types!
        .register_type::<Foo>()
        // ================ //
        .add_systems(Startup, load)
        .add_systems(
            Update,
            (
                spawn.run_if(prototype_ready("CustomSchematic").and_then(run_once())),
                inspect,
            ),
        )
        .run();
}

// Here, we'll define a schematic that does a couple things to our entity:
// 1. Create a camera if one doesn't exist in the world
// 2. Add a `SpriteBundle` to the entity with a user-defined image
// 3. Print a message to the console
//
// First thing's first, we need to derive `Reflect` so that we can register
// this type to the registry (speaking of, don't forget to do that!):
#[derive(Reflect)]
// We also need to register `ReflectSchematic`, which can do like this:
#[reflect(Schematic)]
// Notice that our struct contains no fields. How do we configure this?
// Well, we could add fields, but for the sake of demonstration we'll
// leave them out and use a custom `Input` type instead.
// We'll call it `FooInput`.
struct Foo;

// This struct provides configuration for `Foo` in the prototype file.
// Input types must implement `FromReflect` (so don't opt out of that!).
#[derive(Reflect)]
struct FooInput {
    /// The asset path of the image to load.
    image: String,
    /// The text to output.
    message: Option<String>,
}

// This is where we actually define the logic for `Foo`.
impl Schematic for Foo {
    // Generally, this is `Self`.
    // However, we decided to use `FooInput`, so that can go here.
    type Input = FooInput;

    fn apply(input: &Self::Input, context: &mut SchematicContext) {
        // 1. Create a camera if one doesn't exist in the world
        let world = context.world_mut();
        let change_tick = world.change_tick();
        let last_change_tick = world.last_change_tick();
        let needs_camera =
            world
                .query::<&Camera2d>()
                .is_empty(world, last_change_tick, change_tick);
        if needs_camera {
            world.spawn((Camera2dBundle::default(), FooCamera));
        }

        // 2. Add a `SpriteBundle` to the entity with a user-defined image
        let texture = world.resource::<AssetServer>().load(&input.image);
        context.entity_mut().unwrap().insert(SpriteBundle {
            texture,
            ..default()
        });

        // 3. Print a message to the console
        if let Some(msg) = &input.message {
            println!("Message: {}", msg);
        }
    }

    fn remove(_input: &Self::Input, context: &mut SchematicContext) {
        // It's important we handle any necessary cleanup when removing a schematic.
        let world = context.world_mut();
        let mut camera_query = world.query_filtered::<Entity, With<FooCamera>>();
        let cameras = camera_query.iter(world).collect::<Vec<Entity>>();
        for entity in cameras {
            world.despawn(entity);
        }

        context.entity_mut().unwrap().remove::<SpriteBundle>();
    }

    fn preload_dependencies(input: &mut Self::Input, dependencies: &mut DependenciesBuilder) {
        // This method is optional, but it allows us to preload our assets.
        let _: Handle<Image> = dependencies.add_dependency(input.image.clone());
    }
}

/// Simple marker component for a camera spawned in by `Foo`.
#[derive(Component)]
struct FooCamera;

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/custom_schematic/CustomSchematic.prototype.ron");
}

fn spawn(mut commands: ProtoCommands) {
    commands.spawn("CustomSchematic");
}

// This relies on the `auto_name` feature to be useful
fn inspect(query: Query<DebugName, Added<Name>>) {
    for name in &query {
        println!("Spawned: {:?}", name);
    }
}
