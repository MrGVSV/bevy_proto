use bevy::app::AppExit;
use std::time::Instant;

use bevy::prelude::*;
use bevy::reflect::FromReflect;
use bevy_proto::prelude::*;

const ENTITY_COUNT: u128 = 10_000;
const BATCH_SIZE: u128 = 5_000;
const BATCH_COUNT: u128 = ENTITY_COUNT / BATCH_SIZE;

// DEBUG

// For 200,000 entities:
// Prototype: 7092ms | Normal: 796ms | Prototype (Complex): 9571ms | Normal (Complex): 2138ms

// For 100,000 entities:
// Prototype: 2203ms | Normal: 193ms | Prototype (Complex): 2469ms | Normal (Complex): 579ms

// For 50,000 entities:
// Prototype: 940ms | Normal: 344ms | Prototype (Complex): 1783ms | Normal (Complex): 354ms

// For 10,000 entities:
// Prototype: 396ms | Normal: 63ms | Prototype (Complex): 638ms | Normal (Complex): 146ms

// RELEASE

// For 200,000 entities:
// Prototype: 193ms | Normal: 33ms | Prototype (Complex): 317ms | Normal (Complex): 111ms

// For 100,000 entities:
// Prototype: 143ms | Normal: 22ms | Prototype (Complex): 261ms | Normal (Complex): 79ms

// For 50,000 entities:
// Prototype: 63ms | Normal: 9ms | Prototype (Complex): 102ms | Normal (Complex): 35ms

// For 10,000 entities:
// Prototype: 17ms | Normal: 3ms | Prototype (Complex): 17ms | Normal (Complex): 5ms

fn spawn_protos(
    mut commands: Commands,
    manager: ProtoManager,
    mut stopwatches: ResMut<Stopwatches>,
) {
    let proto = manager.get("Simple").unwrap();

    println!("Spawning via Prototype:");
    let mut total: u128 = 0;
    let mut now = Instant::now();
    for _ in 0..BATCH_COUNT {
        for _ in 0..BATCH_SIZE {
            proto.spawn(&mut commands);
        }
        println!("Prototype Batch: {:.2?}", now.elapsed());
        total += now.elapsed().as_millis();
        now = Instant::now();
    }

    println!(
        "Prototypes: {}ms for {} (avg. batch {}ms)",
        total,
        ENTITY_COUNT,
        total / BATCH_COUNT
    );

    stopwatches.proto = Instant::now();
}

fn watch_protos(
    mut commands: Commands,
    query: Query<(Entity, &Foo)>,
    stopwatches: Res<Stopwatches>,
    mut state: ResMut<State<BenchState>>,
    mut totals: ResMut<Totals>,
    mut completed: Local<bool>,
) {
    if query.is_empty() {
        if *completed {
            state.set(BenchState::BenchNormal).unwrap();
        }
        return;
    }

    let elapsed = stopwatches.proto.elapsed();
    totals.proto = elapsed.as_millis();
    println!(
        "Entities Spawned in {}ms (total count: {})",
        totals.proto,
        query.iter().len()
    );

    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }

    *completed = true;
}

fn spawn_normal(mut commands: Commands, mut stopwatches: ResMut<Stopwatches>) {
    println!("Spawning Normally:");
    let mut total: u128 = 0;
    let mut now = Instant::now();
    for _ in 0..BATCH_COUNT {
        for _ in 0..BATCH_SIZE {
            commands.spawn().insert(Foo {
                name: String::from("Hello World!"),
                index: 123,
            });
        }
        println!("Normal Batch: {:.2?}", now.elapsed());
        total += now.elapsed().as_millis();
        now = Instant::now();
    }

    println!(
        "Normal: {}ms for {} (avg. batch {}ms)",
        total,
        ENTITY_COUNT,
        total / BATCH_COUNT
    );

    stopwatches.normal = Instant::now();
}

fn watch_normal(
    mut commands: Commands,
    query: Query<(Entity, &Foo)>,
    stopwatches: Res<Stopwatches>,
    mut state: ResMut<State<BenchState>>,
    mut totals: ResMut<Totals>,
    mut completed: Local<bool>,
) {
    if query.is_empty() {
        if *completed {
            state.set(BenchState::BenchProtoComplex).unwrap();
        }
        return;
    }

    let elapsed = stopwatches.normal.elapsed();
    totals.normal = elapsed.as_millis();
    println!(
        "Entities Spawned in {}ms (total count: {})",
        totals.normal,
        query.iter().len()
    );

    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }

    *completed = true;
}

fn spawn_protos_complex(
    mut commands: Commands,
    manager: ProtoManager,
    mut stopwatches: ResMut<Stopwatches>,
) {
    let proto = manager.get("Complex").unwrap();

    println!("Spawning via Prototype (Complex):");
    let mut total: u128 = 0;
    let mut now = Instant::now();
    for _ in 0..BATCH_COUNT {
        for _ in 0..BATCH_SIZE {
            proto.spawn(&mut commands);
        }
        println!("Prototype (Complex) Batch: {:.2?}", now.elapsed());
        total += now.elapsed().as_millis();
        now = Instant::now();
    }

    println!(
        "Prototypes (Complex): {}ms for {} (avg. batch {}ms)",
        total,
        ENTITY_COUNT,
        total / BATCH_COUNT
    );

    stopwatches.proto_complex = Instant::now();
}

fn watch_protos_complex(
    mut commands: Commands,
    query: Query<(Entity, &Foo)>,
    stopwatches: Res<Stopwatches>,
    mut state: ResMut<State<BenchState>>,
    mut totals: ResMut<Totals>,
    mut completed: Local<bool>,
) {
    if query.is_empty() {
        if *completed {
            state.set(BenchState::BenchNormalComplex).unwrap();
        }
        return;
    }

    let elapsed = stopwatches.proto_complex.elapsed();
    totals.proto_complex = elapsed.as_millis();
    println!(
        "Entities Spawned in {}ms (total count: {})",
        totals.proto_complex,
        query.iter().len()
    );

    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }

    *completed = true;
}

fn spawn_normal_complex(mut commands: Commands, mut stopwatches: ResMut<Stopwatches>) {
    println!("Spawning Normally (Complex):");
    let mut total: u128 = 0;
    let mut now = Instant::now();
    for _ in 0..BATCH_COUNT {
        for _ in 0..BATCH_SIZE {
            commands
                .spawn()
                .insert(Foo {
                    name: String::from("Hello World!"),
                    index: 123,
                })
                .insert(Bar { x: 1.23 })
                .insert(Foo {
                    name: String::from("Goodbye!"),
                    index: 123,
                });
        }
        println!("Normal (Complex) Batch: {:.2?}", now.elapsed());
        total += now.elapsed().as_millis();
        now = Instant::now();
    }

    println!(
        "Normal (Complex): {}ms for {} (avg. batch {}ms)",
        total,
        ENTITY_COUNT,
        total / BATCH_COUNT
    );

    stopwatches.normal_complex = Instant::now();
}

fn watch_normal_complex(
    mut commands: Commands,
    query: Query<(Entity, &Foo)>,
    stopwatches: Res<Stopwatches>,
    mut state: ResMut<State<BenchState>>,
    mut totals: ResMut<Totals>,
    mut completed: Local<bool>,
) {
    if query.is_empty() {
        if *completed {
            state.set(BenchState::Done).unwrap();
        }
        return;
    }

    let elapsed = stopwatches.normal_complex.elapsed();
    totals.normal_complex = elapsed.as_millis();
    println!(
        "Entities Spawned in {}ms (total count: {})",
        totals.normal_complex,
        query.iter().len()
    );

    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }

    *completed = true;
}

fn print_totals(totals: Res<Totals>, mut writer: EventWriter<AppExit>) {
    println!("---\nFor {} entities:", ENTITY_COUNT.to_string());
    println!(
        "Prototype: {}ms | Normal: {}ms | Prototype (Complex): {}ms | Normal (Complex): {}ms",
        totals.proto, totals.normal, totals.proto_complex, totals.normal_complex,
    );
    writer.send_default();
}

struct Stopwatches {
    proto: Instant,
    normal: Instant,
    proto_complex: Instant,
    normal_complex: Instant,
}

impl Default for Stopwatches {
    fn default() -> Self {
        Self {
            proto: Instant::now(),
            normal: Instant::now(),
            proto_complex: Instant::now(),
            normal_complex: Instant::now(),
        }
    }
}

#[derive(Default)]
struct Totals {
    proto: u128,
    normal: u128,
    proto_complex: u128,
    normal_complex: u128,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum BenchState {
    Load,
    BenchProto,
    BenchNormal,
    BenchProtoComplex,
    BenchNormalComplex,
    Done,
}

#[derive(Reflect, FromReflect, ProtoComponent, Component, Clone)]
#[reflect(ProtoComponent)]
struct Foo {
    index: usize,
    name: String,
}

#[derive(Reflect, FromReflect, ProtoComponent, Component, Clone)]
#[reflect(ProtoComponent)]
struct Bar {
    x: f32,
}

fn load_prototype(asset_server: Res<AssetServer>, mut manager: ProtoManager) {
    let handles = asset_server.load_folder("prototypes/bench").unwrap();
    manager.add_multiple_untyped(handles);
}

fn check_loaded(manager: ProtoManager, mut state: ResMut<State<BenchState>>) {
    if manager.all_loaded() {
        state.set(BenchState::BenchProto).unwrap();
    }
}

fn main() {
    println!(
        "Entity Count: {} | Batch Size: {}",
        ENTITY_COUNT, BATCH_SIZE
    );
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ProtoPlugin::<Prototype>::default())
        .register_type::<Foo>()
        .register_type::<Bar>()
        .add_state(BenchState::Load)
        .init_resource::<Stopwatches>()
        .init_resource::<Totals>()
        .add_system_set(SystemSet::on_enter(BenchState::Load).with_system(load_prototype))
        .add_system_set(SystemSet::on_update(BenchState::Load).with_system(check_loaded))
        .add_system_set(SystemSet::on_enter(BenchState::BenchProto).with_system(spawn_protos))
        .add_system_set(SystemSet::on_update(BenchState::BenchProto).with_system(watch_protos))
        .add_system_set(SystemSet::on_enter(BenchState::BenchNormal).with_system(spawn_normal))
        .add_system_set(SystemSet::on_update(BenchState::BenchNormal).with_system(watch_normal))
        .add_system_set(
            SystemSet::on_enter(BenchState::BenchProtoComplex).with_system(spawn_protos_complex),
        )
        .add_system_set(
            SystemSet::on_update(BenchState::BenchProtoComplex).with_system(watch_protos_complex),
        )
        .add_system_set(
            SystemSet::on_enter(BenchState::BenchNormalComplex).with_system(spawn_normal_complex),
        )
        .add_system_set(
            SystemSet::on_update(BenchState::BenchNormalComplex).with_system(watch_normal_complex),
        )
        .add_system_set(SystemSet::on_enter(BenchState::Done).with_system(print_totals))
        .run();
}
