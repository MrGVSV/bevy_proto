//! This example demonstrates the hot-reloading capabilities of prototypes.
//!
//! The implementation details of this example aren't important
//! (except for enabling [`AssetPlugin::watch_for_changes`]).
//! Just run the example and try it out for yourself!

use bevy::prelude::*;

use bevy_proto::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Enable hot-reloading of assets:
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(ProtoPlugin::default())
        .add_startup_systems((setup, load))
        .add_systems((
            spawn.run_if(
                prototype_ready("ReloadableSprite").and_then(resource_changed::<Input<KeyCode>>()),
            ),
            inspect,
        ))
        .run();
}

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/hot_reload/ReloadableSprite.prototype.ron");
}

fn spawn(
    mut commands: ProtoCommands,
    keyboard_input: Res<Input<KeyCode>>,
    mut previous: Local<Option<Entity>>,
) {
    if previous.is_none() || keyboard_input.just_pressed(KeyCode::Space) {
        *previous = Some(commands.spawn("ReloadableSprite").id());
    }

    if keyboard_input.just_pressed(KeyCode::Z) {
        commands
            .entity(previous.unwrap())
            .insert("ReloadableSprite");
    }
}

// This relies on the `auto_name` feature to be useful
fn inspect(query: Query<DebugName, Added<Name>>) {
    for name in &query {
        info!("Spawned: {:?}", name);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_grow: 1.0,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_sections([
                    TextSection::new(
                        "Press <Space> to spawn a sprite prototype\n",
                        TextStyle {
                            font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "Or press <Z> to reload the previous one\n",
                        TextStyle {
                            font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                            font_size: 28.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "(try changing the prototype file between presses)",
                        TextStyle {
                            font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    ),
                ])
                .with_text_alignment(TextAlignment::Center),
            );
        });
}
