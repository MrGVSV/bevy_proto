#![allow(unused_doc_comments)]

use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy::reflect::FromReflect;

use bevy_proto::prelude::*;

/// This is the component we will use with our prototype
/// It must impl/derive Reflect and FromReflect from Bevy in order to compile.
/// You must also add `#[reflect(ProtoComponent)]` so Bevy's type registry
/// knows this type implements `ProtoComponent`.
#[derive(Reflect, FromReflect, Component, Clone)]
#[reflect(ProtoComponent)]
struct Person {
    pub name: String,
}

/// This is where we manually implement the [`ProtoComponent`] trait.
impl ProtoComponent for Person {
    fn apply(&self, entity: &mut EntityMut) {
        // Here, we're given mutable access to an entity.
        // We can do anything we want with it really, but for simplicity we'll
        // just insert the `Person` component.
        entity.insert(self.clone());
    }

    fn as_reflect(&self) -> &dyn Reflect {
        // This method allows the internal loader to read this component as a reflected value
        self
    }
}

/// For simple types, deriving [`ProtoComponent`] can be used to automatically
/// generate the required `impl` block.
///
/// The [`Person`] component defined above could have simply been written as:
/// ```
/// #[derive(Reflect, FromReflect, ProtoComponent, Component, Clone)]
/// #[reflect(ProtoComponent)]
/// struct Person {
///     pub name: String,
/// }
/// ```
///
/// Note: If you're deriving `ProtoComponent`, you must also derive/impl `Clone`
/// since it's used internally to insert the component.
#[derive(Copy, Clone, Reflect, FromReflect, ProtoComponent, Component)]
#[reflect(ProtoComponent)]
struct Ordered {
    pub order: i32,
}

/// Load the prototypes.
fn load_prototypes(asset_server: Res<AssetServer>, mut manager: ProtoManager) {
    // We can load a prototype by simply using the `AssetServer`
    let handle: Handle<Prototype> = asset_server.load("prototypes/basic/Person_01.prototype.yaml");

    // Since assets require at least one strong handle to be stored in order to stay loaded,
    // we'll need to hold on to the returned handle. Normally we do this by maintaining a
    // resource to keep track of them. However, `ProtoManager` comes with a convenient
    // method to do just that:
    manager.add(handle);

    // Alternatively, we could just load the entire folder
    let handles = asset_server.load_folder("prototypes/basic").unwrap();
    manager.add_multiple_untyped(handles);
}

/// Spawn in the person.
///
/// The `has_ran` parameter just ensures that we only spawn these prototypes once.
fn spawn_person(mut commands: Commands, manager: ProtoManager, mut has_ran: Local<bool>) {
    // Check we haven't already spawned our prototypes
    if *has_ran {
        return;
    }

    // Check that all of our stored handles are fully loaded
    if !manager.all_loaded() {
        return;
    }

    /// Here, we attempt to get our prototype by name.
    /// We'll raise an exception if it's not found, just so we can fail fast.
    /// In reality, you'll likely want to handle this prototype not existing.
    if let Some(proto) = manager.get("Person Test 1") {
        // Spawn in the prototype!
        proto.spawn(&mut commands);

        // Spawn it again!
        proto.spawn(&mut commands);

        // Insert on an existing entity!
        let entity = commands.spawn().id();
        proto.insert(entity, &mut commands);
    }

    // Spawn in others!
    for i in 2..=3 {
        if let Some(proto) = manager.get(format!("Person Test {}", i).as_str()) {
            proto.spawn(&mut commands);
        }
    }

    // Prevent future spawning
    *has_ran = true;
}

/// A system to test our spawner. This makes each entity introduce itself when first spawned in
fn introduce(query: Query<(&Person, &Ordered), Added<Person>>) {
    for (person, ordered) in query.iter() {
        println!("{}. Hello! My name is {}", ordered.order, person.name);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // A `ProtoPlugin<T>` can be added for any custom prototypical type.
        // This crate defines `Prototype` by default so we'll just use that.
        .add_plugin(ProtoPlugin::<Prototype>::default())
        // !!! Make sure you register your types !!! //
        .register_type::<Person>()
        .register_type::<Ordered>()
        // Add our systems
        .add_startup_system(load_prototypes)
        .add_system(spawn_person)
        .add_system(introduce)
        .run();
}
