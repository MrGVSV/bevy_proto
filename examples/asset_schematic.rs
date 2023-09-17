//! This example demonstrates how to use asset schematics.
//!
//! Asset schematics work similar to schematics,
//! but allow an asset to be defined directly in a prototype file.

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ProtoPlugin::new()))
        .register_type::<CurrentLevel>()
        // =============== //
        // Asset schematics are slightly more complex than standard schematics,
        // so we need to make sure we register them properly.
        // We could do this manually, but it's easier to use the extension trait
        // provided by this crate:
        .register_asset_schematic::<Level>()
        // =============== //
        .add_systems(Startup, (setup, load))
        .add_systems(
            Update,
            (
                spawn.run_if(prototype_ready("Level1").and_then(run_once())),
                inspect.before(on_level_load),
                on_level_load,
            ),
        )
        .run();
}

/// Normally assets are defined in their own dedicated files.
/// We can then reference these files by path within our prototype files.
///
/// Sometimes, however, it makes more sense to define an asset directly in
/// the prototype uses it.
/// This can be to avoid having to create a separate file for a single asset
/// or simply to facilitate faster prototyping.
///
/// To do this, we can derive `AssetSchematic` on the asset type.
#[derive(AssetSchematic, Reflect, TypeUuid)]
#[uuid = "c7f097c0-5ed7-4261-88b5-01f3045c031f"]
struct Level {
    name: String,
    /// An asset schematic can also reference other assets.
    #[asset_schematic(asset)]
    player: Handle<Image>,
}

/// We can then use these asset schematics in our standard schematics.
#[derive(Schematic, Component, Reflect)]
#[reflect(Schematic)]
struct CurrentLevel {
    /// By default, the `asset` attribute will convert the handle into a
    /// `ProtoAsset`, which only allows references by path.
    ///
    /// But we want to make use of our new asset schematic so we can define
    /// the asset inline!
    ///
    /// To do this, we can add the `inline` argument.
    /// This will convert the handle into a `InlinableProtoAsset`,
    /// which will allow us to decide between using an asset path or defining
    /// the asset inline.
    #[schematic(asset(inline))]
    level: Handle<Level>,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct LevelName;

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/asset_schematic/Level1.prototype.ron");
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        LevelName,
        Text2dBundle {
            transform: Transform::from_xyz(0.0, 200.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((Player, SpriteBundle::default()));
}

fn spawn(mut commands: ProtoCommands) {
    commands.spawn("Level1");
}

fn on_level_load(
    mut player_query: Query<&mut Handle<Image>, With<Player>>,
    mut level_name_query: Query<&mut Text, With<LevelName>>,
    level_query: Query<&CurrentLevel, Added<CurrentLevel>>,
    levels: Res<Assets<Level>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(current_level) = level_query.get_single() else {
        return;
    };

    let level = levels.get(&current_level.level).unwrap();
    println!("Loaded level: ==[{}]==", level.name);

    let mut level_name = level_name_query.single_mut();
    *level_name = Text::from_section(
        &level.name,
        TextStyle {
            font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
            font_size: 64.0,
            color: Color::WHITE,
        },
    );

    let mut player = player_query.single_mut();
    *player = level.player.clone();
}

// This relies on the `auto_name` feature to be useful
fn inspect(query: Query<DebugName, Added<Name>>) {
    for name in &query {
        println!("Spawned {:?}", name);
    }
}
