//! This example demonstrates the hot-reloading capabilities of prototypes.
//!
//! It's pretty easy to enable hot-reloading in Bevy— just enable [`AssetPlugin::watch_for_changes`].
//! This allows changes to any prototype to be automatically picked up.
//!
//! This example also uses the [`ProtoAssetEvent`] event to reload an existing entity if
//! its prototype changes, which can allow for faster development.
//!
//! Please note that hot-reloading is far from perfect.
//! Changing the IDs of a prototype or its hierarchical structure may cause the
//! reload to fail or work unexpectedly upon prototype re-registration.
//! However, it can still be a great tool to use for fast prototyping.
//!
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
        .add_plugin(ProtoPlugin::new())
        .add_startup_systems((setup, load))
        .add_systems((spawn.run_if(prototype_ready("ReloadableSprite")), inspect))
        .run();
}

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/hot_reload/ReloadableSprite.prototype.ron");
}

fn spawn(
    mut commands: ProtoCommands,
    keyboard_input: Res<Input<KeyCode>>,
    mut previous: Local<Option<Entity>>,
    mut proto_asset_events: EventReader<ProtoAssetEvent>,
) {
    if previous.is_none() || keyboard_input.just_pressed(KeyCode::Space) {
        *previous = Some(commands.spawn("ReloadableSprite").id());
    }

    // Listen for changes:
    for proto_asset_event in proto_asset_events.iter() {
        match proto_asset_event {
            // Only trigger a re-insert of the prototype when modified and if IDs match
            ProtoAssetEvent::Modified { id, .. } if id == "ReloadableSprite" => {
                commands
                    .entity(previous.unwrap())
                    .insert("ReloadableSprite");
            }
            _ => {}
        }

        // Note: We could also have checked using the helper method:
        // if proto_asset_event.is_modified("ReloadableSprite") { ... }
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
                        "Modify the prototype file to see changes\n",
                        TextStyle {
                            font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "(press <Space> to spawn a new prototype)",
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
