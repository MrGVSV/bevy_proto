//! This example is a copy of Bevy's [`ui`] example, but powered by the `bevy_proto` plugin.
//!
//! The most notable change is the use of hot-reloading.
//! This allows us to modify the UI and see the changes in real-time!
//!
//! Prototypes aren't a perfect replacement for Rust code—
//! especially if there's more advanced logic involved—
//! but they can be a great tool for rapid prototyping.
//!
//! [`ui`]: https://github.com/bevyengine/bevy/blob/v0.10.1/examples/ui/ui.rs

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_proto::prelude::*;

const ROOT: &str = "Root";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(ProtoPlugin::default())
        // The original example sets the update mode to `desktop_app`,
        // but this doesn't play nice with hot-reloading.
        // .insert_resource(bevy::winit::WinitSettings::desktop_app())
        .register_type::<ScrollingList>()
        .register_type::<ScrollText>()
        .add_startup_systems((setup, load))
        .add_systems((spawn.run_if(prototype_ready(ROOT)), inspect, mouse_scroll))
        .run();
}

#[derive(Component, Default, Reflect, FromReflect)]
#[reflect(Schematic)]
struct ScrollText(String);

#[derive(Component, Reflect, FromReflect, Schematic)]
#[reflect(Schematic)]
struct ScrollingList {
    #[reflect(default)]
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}

impl Schematic for ScrollText {
    type Input = ScrollText;

    fn apply(input: &Self::Input, context: &mut SchematicContext) {
        let font = context
            .world()
            .resource::<AssetServer>()
            .load("fonts/JetBrainsMono-Regular.ttf");
        context
            .entity_mut()
            .unwrap()
            .insert(TextBundle::from_section(
                input.0.clone(),
                TextStyle {
                    font,
                    font_size: 20.,
                    color: Color::WHITE,
                },
            ));
    }

    fn remove(_input: &Self::Input, context: &mut SchematicContext) {
        context.entity_mut().unwrap().remove::<TextBundle>();
    }
}

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/bevy/ui/Root.prototype.ron");
}

fn spawn(
    mut commands: ProtoCommands,
    mut root: Local<Option<Entity>>,
    mut proto_asset_events: EventReader<ProtoAssetEvent>,
) {
    let root = *root.get_or_insert_with(|| commands.spawn(ROOT).id());

    for proto_asset_event in proto_asset_events.iter() {
        if proto_asset_event.is_modified(ROOT) {
            commands.entity(root).insert(ROOT);
        }
    }
}

// This relies on the `auto_name` feature to be useful
fn inspect(query: Query<DebugName, Added<Name>>) {
    for name in &query {
        println!("Spawned {:?}", name);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
