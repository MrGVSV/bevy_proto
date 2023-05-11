//! This example demonstrates how to implement a custom loader.
//!
//! Loaders are what actually controls how an asset is loaded and what
//! formats are supported.
//!
//! The example below shows how to implement a custom loader for
//! files ending with the `.custom.yaml` extension.
//! It also shows how we can modify a prototype before it's finalized.
//!
//! It should be noted that bevy_proto already supports YAML files,
//! which can be enabled using the `yaml` feature.

use bevy::prelude::*;
use serde::de::DeserializeSeed;

use bevy_proto::backend::load::{Loader, ProtoLoadContext, ProtoLoadMeta};
use bevy_proto::de::PrototypeDeserializer;
use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // =============== //
        // Be sure to add `ProtoPlugin` with the custom loader:
        .add_plugin(ProtoPlugin::new_with_loader(MyLoader::default()))
        // =============== //
        .register_type::<Player>()
        .register_type::<Health>()
        .register_type::<Mana>()
        .add_startup_system(load)
        .add_systems((
            spawn.run_if(prototype_ready("Player").and_then(run_once())),
            inspect,
        ))
        .run();
}

/// A custom loader for `.custom.yaml` files.
#[derive(Clone, Default)]
struct MyLoader;

impl Loader<Prototype> for MyLoader {
    /// For simplicity, we'll just reuse bevy_proto's error type.
    type Error = PrototypeError;

    fn deserialize(
        bytes: &[u8],
        ctx: &mut ProtoLoadContext<Prototype, Self>,
    ) -> Result<Prototype, Self::Error> {
        // To deserialize the default `Prototype`, we can use the `PrototypeDeserializer`:
        let deserializer = PrototypeDeserializer::new(ctx);

        // And for our format, we'll use YAML:
        deserializer
            .deserialize(serde_yaml::Deserializer::from_slice(bytes))
            .map_err(|err| PrototypeError::custom(format_args!("YAML error: {}", err)))
    }

    fn extensions(&self) -> &[&'static str] {
        // It's important we arrange these from most specific to least specific
        // (from shortest to longest), and that they do not start with a dot (`.`).
        &["custom.yaml", "yaml"]
    }

    fn on_load_prototype(
        &self,
        mut prototype: Prototype,
        meta: &ProtoLoadMeta<Prototype>,
    ) -> Result<Prototype, Self::Error> {
        // This is where we can process a prototype before it is finalized by the asset loader.

        // For example, we can log the prototype's name and path:
        println!("Loaded prototype: {:?} ({:?})", prototype.id(), meta.path);

        // We can modify it in some way:
        if prototype.schematics().contains::<Player>() {
            if let Some(mana) = prototype.schematics_mut().get_mut::<Mana>() {
                mana.input_mut().downcast_mut::<Mana>().unwrap().0 *= 2;
            } else {
                prototype.schematics_mut().insert::<Mana>(Mana(50));
            }
        }

        // Or we can reject it if it doesn't meet our requirements:
        if prototype.schematics().contains::<Player>()
            && !prototype.schematics().contains::<Health>()
        {
            return Err(PrototypeError::custom(format_args!(
                "Prototype with ID {:?} contains a `Player` component but not a `Health` component!",
                prototype.id()
            )));
        }

        // Finally, we return the prototype.
        Ok(prototype)
    }
}

#[derive(Component, Schematic, Reflect, FromReflect)]
#[reflect(Schematic)]
struct Player;

#[derive(Component, Schematic, Reflect, FromReflect)]
#[reflect(Schematic)]
struct Health(i32);

#[derive(Component, Schematic, Reflect, FromReflect)]
#[reflect(Schematic)]
struct Mana(i32);

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/custom_loader/Player.custom.yaml");
}

fn spawn(mut commands: ProtoCommands) {
    commands.spawn("Player");
}

fn inspect(query: Query<(&Health, &Mana), Added<Player>>) {
    for (health, mana) in &query {
        println!("Spawned player with {:?} HP and {:?} MP", health.0, mana.0);
    }
}
