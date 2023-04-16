//! This example demonstrates the basics of prototype hierarchies.
//!
//! Running the example, we can see blue arrows pointing from parent to children.
//! This relationship between [`Parent`] and [`Children`] is automatically created
//! when spawning a prototype containing children.
//!
//! Additionally, we can see a yellow arrow pointing from the key to the lock.
//! This relationship is defined via a custom [`Schematic`] and uses
//! [`ProtoEntity::EntityPath`] to configure the path to the entity within the
//! prototype file itself.
//!
//! Lastly, both the root parent and the middle child are wearing hats.
//! This is because the parent entity inherits the `HatSalesman` prototype.
//! This prototype includes a child entity that actually renders the hat.
//! But what about the middle child? How does it get its hat?
//! Well, the `HatSalesman` has another child with a matching `merge_key` as
//! the middle child.
//! This causes the `HatSalesman`'s child to merge into the parent's middle child.
//!
//! Overall, we're easily able to define very simple hierarchies and relationships
//! between prototypes using the tools provided in `bevy_proto`.
//!
//! [`ProtoEntity::EntityPath`]: bevy_proto_backend::tree::ProtoEntity::EntityPath

use bevy::prelude::*;
use bevy_proto::config::ProtoConfig;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

use bevy_proto::prelude::*;
use bevy_proto_backend::tree::EntityAccess;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ProtoPlugin::new(
            ProtoConfig::default().on_before_apply_prototype(Box::new(|prototype, _, tree| {
                // To see the end result of our hierarchy, let's inspect the generated
                // `EntityTree` for the `Parent` prototype.
                if prototype.id() == "Parent" {
                    println!("{:#?}", tree);

                    // We can also directly access the generated entities
                    // (check out the docs for `EntityAccess` for details):
                    let key_entity = tree.find_entity(&EntityAccess::from("/OtherChild/@0"));
                    println!("🔑 Key Entity: {:?}", key_entity.unwrap());
                }
            })),
        ))
        .add_plugin(ShapePlugin)
        .register_type::<Opens>()
        .register_type::<DrawRelations>()
        .register_type::<HasHat>()
        .add_startup_systems((setup, load))
        .add_systems((
            spawn.run_if(prototype_ready("Parent").and_then(run_once())),
            inspect,
            draw_relations,
        ))
        .run();
}

/// Component for keys, pointing to the entity they open.
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
struct Opens(
    // We can mark an `Entity` or `Option<Entity>` with `#[schematic(entity)]`
    // in order to let the derive macro know that this will reference an entity
    // within the prototype hierarchy.
    //
    // Under the hood, this generates a schematic input where this field is
    // replaced with a `ProtoEntity`, which can be used to access the `EntityTree`.
    //
    // To reiterate, this basic relationship is only definable amongst prototypes
    // within the same hierarchy— not just any entity in the world.
    #[schematic(entity)] Entity,
);

/// Marker component opting into the drawing of relationships.
#[derive(Component, Schematic, Reflect, FromReflect)]
#[reflect(Schematic)]
struct DrawRelations;

/// Marker component indicating a hat is present.
#[derive(Component, Schematic, Reflect, FromReflect)]
#[reflect(Schematic)]
struct HasHat;

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("examples/hierarchy/Parent.prototype.ron");
}

fn spawn(mut commands: ProtoCommands) {
    commands.spawn("Parent");
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

fn draw_relations(
    mut commands: Commands,
    keys: Query<(&Opens, &GlobalTransform), Added<DrawRelations>>,
    children: Query<(&Parent, &GlobalTransform, Option<&HasHat>), Added<DrawRelations>>,
    parents: Query<&GlobalTransform>,
) {
    for (parent, transform, has_hat) in &children {
        let mut start = parents.get(parent.get()).unwrap().translation().truncate();
        let mut end = transform.translation().truncate();

        // This is just to account for arrows directly above hats
        let is_directly_above = (end.x - start.x).abs() <= LINE_OFFSET_THRESHOLD;
        let end_multiplier = if has_hat.is_some() && is_directly_above {
            2.0
        } else {
            1.0
        };

        start += get_offset(start, end);
        end += get_offset(end, start) * end_multiplier;

        commands.spawn((
            ShapeBundle {
                path: get_arrow_path(start, end),
                transform: Transform::from_xyz(0.0, 0.0, -0.01),
                ..default()
            },
            Stroke::new(Color::BLUE, 5.0),
        ));
    }

    for (opens, transform) in &keys {
        let mut start = transform.translation().truncate();
        let mut end = parents.get(opens.0).unwrap().translation().truncate();

        start += get_offset(start, end);
        end += get_offset(end, start);

        commands.spawn((
            ShapeBundle {
                path: get_arrow_path(start, end),
                transform: Transform::from_xyz(0.0, 0.0, -0.02),
                ..default()
            },
            Stroke::new(Color::YELLOW, 3.0),
            Fill::color(Color::YELLOW),
        ));
    }
}

const LINE_OFFSET: f32 = 45.0;
const LINE_OFFSET_THRESHOLD: f32 = 10.0;

fn get_offset(pos: Vec2, other: Vec2) -> Vec2 {
    let dx = other.x - pos.x;
    let dy = other.y - pos.y;

    Vec2::new(
        if dx.abs() <= LINE_OFFSET_THRESHOLD {
            0.0
        } else {
            dx.signum() * LINE_OFFSET
        },
        if dy.abs() <= LINE_OFFSET_THRESHOLD {
            0.0
        } else {
            dy.signum() * LINE_OFFSET
        },
    )
}

fn get_arrow_path(start: Vec2, end: Vec2) -> Path {
    let line = shapes::Line(start, end);
    let dir = (end - start).normalize();
    let base = dir * 10.0;
    let points = vec![
        end,
        end - base.rotate(Vec2::from_angle(PI / 6.0)),
        end - base.rotate(Vec2::from_angle(-PI / 6.0)),
    ];

    let arrow_head = shapes::Polygon {
        closed: true,
        points,
    };

    GeometryBuilder::new().add(&line).add(&arrow_head).build()
}
