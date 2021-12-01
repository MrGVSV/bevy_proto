# bevy_proto

[![Crates.io](https://img.shields.io/crates/v/bevy_proto)](https://crates.io/crates/bevy_proto)

[![Docs](https://img.shields.io/docsrs/bevy_proto)](https://docs.rs/bevy_proto/latest/bevy_proto/)

[![License](https://img.shields.io/crates/l/bevy_proto)](./License.md)



Create entities in the Bevy game engine with a simple config file.

```yaml
---
name: "Simple Enemy"
components:
  - type: Enemy
  - type: Attack
    value:
      damage: 10
  - type: Armed
    value:
      weapons: [ "laser sword", "ray-blaster" ]
      primary: "laser sword"
```

## Installation

```toml
[dependencies]
bevy_proto = "0.1.0"
```

Then add it to your app like so:

```rust
use bevy_proto::ProtoPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // add dependent plugins, resources, etc. here
        // ...
        // Add this plugin after any other plugins/resources needed for prototype preparation
        .add_plugin(ProtoPlugin::default())
        // other plugins, resources, not needed by ProtoPlugin
        // ...
        .run();
}
```

### Disclaimer

Before you install it into your project, please understand the limitations of this crate. While it makes working with
some entities easier, it may come at a bit of a performance cost depending on your project.

According to the [`bench`](/examples/bench.rs) example, spawning a Prototype can be about 1.8x slower than defining the
entity in the system manually (this may vary depending on the data being loaded). This difference becomes much smaller
for release builds, but is still a tad slower. For some projects,— except maybe for those that are really intensive or
spawn lots of entities very frequently,— this may not be a problem.

Still, it's good to consider the drawbacks of using this system and see if it's right for your own project. Here's a
breakdown of the top current/potential issues:

* *Dynamic Dispatch* - This crate uses dynamic trait objects to add or remove any component on a Prototype. However,
  this comes at a cost since the compiler can no longer know the types in advance, preventing things like static
  dispatch, monomorphization, etc.

* *Load Time* - This is likely not noticeable for smaller projects (although I haven't tested with hundreds of
  Prototypes myself), but the crate currently does need to load all Prototypes at startup. This is so it can prepare any
  other needed resources and assets in order to spawn the Prototype.

* *Assets* - This crate also (currently) stores all required assets in its own resource `ProtoData`. This means that
  resources that may only be needed once will be kept loaded during the entire lifetime of the application, since it
  holds onto the handle. This can be prevented by hosting the asset on a separate component and manually creating the
  handles when spawning that Prototype:

  ```rust
  // Attach fictional OtherComponent with asset "my_asset" which should unload when despawned
  prototype.spawn(...).insert(OtherComponent(my_asset));
  ```

With all of that said, this package is meant to speed up development and make changes to entity archetypes easier for
humans (especially non-programmers) to interact with. If the performance hit is too much for your project, you are
better off sticking with the standard methods of defining entities.

## Usage

### Creating Components

First, create a struct that implements `ProtoComponent`. This can be done one of two ways:

For simple components, `ProtoComponent` may be derived:

```rust
use bevy_proto::proto_comp;

#[derive(Serialize, Deserialize, ProtoComponent)]
struct Creature {
    // Optional: #[proto_comp(Clone)]
    pub species: String,
    #[proto_comp(Copy)]
    pub legs: i32
}
```

> By default, the fields of a `ProtoComponent` are cloned into spawned entities. This can be somewhat controlled via the `proto_comp` attribute, which can tell the compiler to use the `Copy` trait instead.

Otherwise, you can define them manually (the two attributes are required with this method):

```rust
use bevy_proto::{ProtoComponent, ProtoCommands};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] // Required
struct Creature {
    pub species: String,
    pub legs: i32
}

#[typetag::serde] // Required
impl ProtoComponent for Creature {
    // Required
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
        commands.insert(Self {
            name: self.species.clone(),
            legs: self.legs
        });
    }
}
```

> A `ProtoComponent` does *not* need to be a component itself. It can be used purely as a [DTO](https://en.wikipedia.org/wiki/Data_transfer_object) to generate other components or bundles. You have full access to the `EntityCommands` so you can insert bundles or even multiple components at once.

### Defining the Prototype

Define the Prototype in a config file. By default YAML (and by extension, valid JSON) files are supported:

```yaml
# assets/prototypes/my_prototype.yaml
---
name: "Dog"
components:
  - type: Creature
    value:
      species: "Canis lupus familiaris"
      legs: 4
```

> By default, all `.yaml` and `.json` files in the `assets/prototypes/` directory are processed as Prototypes.

### Spawning the Prototype

To spawn a prototype, add a system that has access to:

* `mut Commands`

* `Res<ProtoData>`
* `Res<AssetServer>`

Then write something like the following:

```rust
fn spawn_dog(mut commands: Commands, data: Res<ProtoData>, asset_server: Res<AssetServer>) {
    let proto = data.get_prototype("Dog").expect("Tried to get a Prototype that doesn't exist!");

    // Spawns in our "Dog" Prototype
    proto.spawn(&mut commands, &data, &asset_server);
}
```

The `spawn(...)` method returns the `EntityCommands` used to create the entity. This allows you to add additional
components, bundles, etc.:

```rust
let dog: Entity = proto
.spawn( & mut commands, & data, & asset_server)
.insert(Domesticated{
name: "Spot",
owner: "Me"
})
.id();
```

### Using Assets

For Prototypes that need access to assets, you can get access one of two ways:

The first is by loading the asset when being spawned in. This is preferred because it means the asset can be unloaded
when no longer needed.

```rust
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use bevy_proto::{HandlePath, ProtoComponent, ProtoCommands};

#[derive(Serialize, Deserialize)]
struct Renderable {
    pub texture_path: HandlePath
}

#[typetag::serde]
impl ProtoComponent for Creature {
    // Required
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
        let handle: Handle<Texture> = asset_server.load(self.texture_path.as_str());

        entity.insert(SomeTexture {
            texture_handle: handle
        });
    }
}
```

The second, is by preparing the asset for later use. This retains the asset in the `ProtoData` resource, which then must
be disposed of manually when no longer needed. Setting up an asset is done via the `prepare` method:

```rust
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use bevy_proto::{HandlePath, ProtoComponent, ProtoCommands, Prototypical};

#[derive(Serialize, Deserialize)]
struct Renderable {
    pub texture_path: HandlePath
}

#[typetag::serde]
impl ProtoComponent for Creature {
    // Required
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
        let material: Handle<ColorMaterial> = slice
            .get_handle(self, &self.texture_path)
            .expect("Expected ColorMaterial handle to have been created");

        entity.insert_bundle(SpriteBundle {
            material,
            ..Default::default()
        });
    }

    fn prepare(
        &self,
        world: &mut World,
        prototype: &Box<dyn Prototypical>,
        data: &mut ProtoData
    ) {
        // === Load Handles === //
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let texture: Handle<Texture> = asset_server.load(self.texture_path.as_str());

        // === Transform Handles === //
        let mut mat_res = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let mat = mat_res.add(texture.into());

        // === Save Handles === //
        data.insert_handle(prototype, self, &self.texture_path, mat);
    }
}
```

### Custom Prototypes

The default Prototype object looks like this:

```rust
pub struct Prototype {
    /// The name of this prototype
    pub name: String,
    /// The components belonging to this prototype
    pub components: Vec<Box<dyn ProtoComponent>>,
}
```

However, you can use your own Prototype object. Any struct that implements `Prototypical` can be used in place of the
default Prototype. Then you just have to supply your own deserializer to the `ProtoPlugin` object.

```rust
use bevy_proto::{ProtoDataOptions, ProtoDeserializer, ProtoPlugin, Prototypical};

#[derive(Clone)]
struct CustomProtoDeserializer;

impl ProtoDeserializer for CustomProtoDeserializer {
    fn deserialize(&self, data: &str) -> Option<Box<dyn Prototypical>> {
        // Deserialize using your custom prototypical object
        if let Ok(value) = serde_yaml::from_str::<CustomPrototype>(data) {
            Some(Box::new(value))
        } else {
            None
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // ...
        .add_plugin(ProtoPlugin {
            options: ProtoDataOptions {
                // Specify your custom deserializer
                deserializer: Box::new(CustomProtoDeserializer),

                // You can also change the prototype directories here
                directories: vec![String::from("assets/prototypes")],
                // Or update the allowed extensions within those directories
                extensions: Some(vec!["yaml", "json"]),
            }
        })
        // other plugins, resources, not needed by ProtoPlugin
        // ...
        .run();
}
```

> All fields in `ProtoDataOptions` must be specified if you wish to use a custom deserializer. Even if you want to continue using the defaults, you must still specify them. The additional fields shown above are also the defaults if you wish to copy them.
