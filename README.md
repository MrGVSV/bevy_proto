# bevy_proto

[![Crates.io](https://img.shields.io/crates/v/bevy_proto)](https://crates.io/crates/bevy_proto)
[![Docs](https://img.shields.io/docsrs/bevy_proto)](https://docs.rs/bevy_proto/latest/bevy_proto/)
[![License](https://img.shields.io/crates/l/bevy_proto)](./License.md)



Create entities in the Bevy game engine with a simple config file.

```yaml
name: "Simple Enemy"
template: Creature
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

## 📋 Features

* **Define** entities easily with config files:

  > ```yaml
  > name: Player
  > components:
  >   - type: Controllable
  >   - type: Health
  >     value:
  >       max: 20
  > ```

* **Inherit** functionality from other prototypes:

  >  ```yaml
  >  name: Skeleton
  >  templates: Enemy, Creature
  >  components:
  >    # ...
  >  ```

* **Include** assets to be loaded:

  > ```yaml
  > name: Bullet
  > components:
  >   - type: CustomSprite
  >     value:
  >       texture: "path/to/texture.png"
  > ```

## 📲 Installation

```toml
[dependencies]
bevy_proto = "0.3"
```

Then add it to your app like so:

```rust
use bevy::prelude::*;
use bevy_proto::plugin::ProtoPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // add dependent plugins, resources, etc. here
        // ...
        // Add this plugin after any other plugins/resources needed for prototype preparation
        .add_plugin(ProtoPlugin::default());
}
```

## 📓 Examples

Check out these examples for more details as to how to use this crate:

* [attributes](https://github.com/MrGVSV/bevy_proto/blob/main/examples/attributes.rs) - A showcase of the available derive helper attributes
* [basic](https://github.com/MrGVSV/bevy_proto/blob/main/examples/basic.rs) - The most basic way to add prototypes
* [bundles](https://github.com/MrGVSV/bevy_proto/blob/main/examples/bundles.rs) - A demonstration of a more complex prototype that includes assets
* [templates](https://github.com/MrGVSV/bevy_proto/blob/main/examples/templates.rs) - An example of how templates affect your prototypes

## 🕊 Bevy Compatibility

| bevy | bevy_proto |
| ---- | ---------- |
| 0.6  | 0.3.0      |
| 0.5  | 0.2.1      |

## ✨ Usage

### Creating Components

First, create a struct that implements `ProtoComponent`. This can be done one of two ways:

For simple components, `ProtoComponent` may be derived:

```rust
use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use bevy_proto::prelude::*;

#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
struct Movement {
    speed: u16
}

// Also works on tuple structs:
#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
struct Inventory (
    Option<Vec<String>>
);
```

> By default, the `ProtoComponent` is cloned into spawned entities.

Otherwise, you can define them manually (the two attributes are required with this method):

```rust
use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy_proto::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Component)] // Required
struct Inventory(Option<Vec<String>>);

#[typetag::serde] // Required
impl ProtoComponent for Inventory {
    // Required
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
        commands.insert(
            Self (self.0.clone())
        );
    }
}
```

> A `ProtoComponent` does *not* need to be a component itself. It can be used purely as a [DTO](https://en.wikipedia.org/wiki/Data_transfer_object) to generate other components or bundles. You have full access to the `EntityCommands` so you can insert bundles or even multiple components at once.
>
> Other ways of generating components from non-component `ProtoComponent` structs can be found in the [attributes](https://github.com/MrGVSV/bevy_proto/blob/main/examples/attributes.rs) example.

### Defining the Prototype

Define the Prototype in a config file. By default YAML (and by extension, valid JSON) files are supported:

```yaml
# assets/prototypes/adventurer.yaml
---
name: "Adventurer"
components:
  - type: Inventory
    value: ["sword"]
  - type: Movement
    value:
      speed: 10
```

> By default, all `.yaml` and `.json` files in the `assets/prototypes/` directory are processed as Prototypes.

#### Using Templates

A Prototype can also include a *template*. A template can be any Prototype and is used to define common components that should be inserted into its inheritors. This is helpful for reducing duplicate markup and quickly refactoring collections of Prototypes.

First, define the template:

```yaml
# assets/prototypes/npc.yaml
---
name: "NPC"
components:
  - type: Inventory
    value: ~
  - type: Movement
    value:
      speed: 10
```

Then inherit the template in another Prototype:

```yaml
# assets/prototypes/peasant.yaml
---
name: "Peasant"
template: NPC
```

You can also override template components:

```yaml
# assets/prototypes/adventurer.yaml
---
name: "Adventurer"
template: NPC
components:
  - type: Inventory
    value: ["sword"]
```

Multiple templates can be specified as well. However, conflicting components will be overridden in reverse order (templates listed first can override templates listed last):

```yaml
# assets/prototypes/fast_adventurer.yaml
---
name: "Fast Adventurer"
templates: Speedy, NPC # "Speedy" may override "NPC"
components:
  - type: Inventory
    value: ["sword"]
```

> Templates can be specified as a standard YAML list or as a comma-separated string (like in the example above). Additionally,  `templates` is an alias for `template`, so either one may be used.

### Spawning the Prototype

To spawn a prototype, add a system that has access to:

* `mut Commands`

* `Res<ProtoData>`
* `Res<AssetServer>`

Then write something like the following:

```rust
use bevy::prelude::*;
use bevy_proto::prelude::*;

fn spawn_adventurer(mut commands: Commands, data: Res<ProtoData>, asset_server: Res<AssetServer>) {
    let proto = data.get_prototype("Adventurer").expect("Prototype doesn't exist!");

    // Spawns in our "Adventurer" Prototype
    proto.spawn(&mut commands, &data, &asset_server);
}
```

The `spawn(...)` method returns the `EntityCommands` used to create the entity. This allows you to add additional
components, bundles, etc.:

```rust
use bevy::prelude::*;
use bevy_proto::prelude::*;

#[derive(Component)]
struct Friendly;

#[derive(Component)]
struct Named(String);

fn spawn_adventurer(mut commands: Commands, data: Res<ProtoData>, asset_server: Res<AssetServer>) {
  let proto = data.get_prototype("Adventurer").expect("Prototype doesn't exist!");

  let adventurer: Entity = proto
      .spawn(&mut commands, &data, &asset_server)
      .insert(Friendly)
      .insert(Named("Bob".to_string()))
      .id();
}
```

### Using Assets

For Prototypes that need access to assets, you can get access one of two ways:

The first is by loading the asset when being spawned in. This is preferred because it means the asset can be unloaded
when no longer needed.

```rust
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use bevy_proto::prelude::*;

#[derive(Component)]
struct Renderable {
    pub texture_path: Handle<Image>
}

#[derive(Serialize, Deserialize)]
struct Creature {
    pub texture_path: HandlePath
}

#[typetag::serde]
impl ProtoComponent for Creature {
    // Required
    fn insert_self(&self, proto_commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
        let handle: Handle<Image> = asset_server.load(self.texture_path.as_str());
        let entity_commands = proto_commands.raw_commands();

        entity_commands.insert(Renderable {
            texture_path: handle
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
use bevy_proto::prelude::*;

#[derive(Serialize, Deserialize)]
struct Renderable {
    pub texture_path: HandlePath
}
#[derive(Serialize, Deserialize)]
struct Creature {
    pub texture_path: HandlePath
}

#[typetag::serde]
impl ProtoComponent for Creature {
    // Required
    fn insert_self(&self, proto_commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
        let texture: Handle<Image> = proto_commands
            .get_handle(self, &self.texture_path)
            .expect("Expected Image handle to have been created");
        let entity_commands = proto_commands.raw_commands();

        entity_commands.insert_bundle(SpriteBundle {
            texture,
            ..Default::default()
        });
    }

    fn prepare(
        &self,
        world: &mut World,
        prototype: &dyn Prototypical,
        data: &mut ProtoData
    ) {
        // === Load Handles === //
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let texture: Handle<Image> = asset_server.load(self.texture_path.as_str());

        // === Save Handles === //
        data.insert_handle(prototype, self, &self.texture_path, texture);
    }
}
```

### Custom Prototypes

The default Prototype object looks like this:

```rust
use bevy_proto::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Prototype {
    /// The name of this prototype
	  pub name: String,
	  /// The names of this prototype's templates (if any)
	  pub templates: Vec<String>,
	  /// The components belonging to this prototype
	  pub components: Vec<Box<dyn ProtoComponent>>,
}
```

However, you can use your own Prototype object. Any struct that implements `Prototypical` can be used in place of the default Prototype. Then you just have to supply your own deserializer to the `ProtoPlugin` object.

```rust
use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy_proto::prelude::*;

#[derive(Serialize, Deserialize)]
struct CustomPrototype;
impl Prototypical for CustomPrototype {
  fn name(&self) -> &str {
    "CustomPrototype"
  }
  fn iter_components(&self) -> std::slice::Iter<'_, std::boxed::Box<(dyn ProtoComponent + 'static)>> { 
    todo!() 
  }
  fn create_commands<'w, 's, 'a, 'p>(
    &'p self, 
    entity_commands: EntityCommands<'w, 's, 'a>, 
    proto_data: &'p Res<'_, ProtoData>
  ) -> ProtoCommands<'w, 's, 'a, 'p> { 
    todo!() 
  }
}

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
    App::new()
        .add_plugins(DefaultPlugins)
        // ...
        .add_plugin(ProtoPlugin {
            options: Some(ProtoDataOptions {
                // Specify your custom deserializer
                deserializer: Box::new(CustomProtoDeserializer),

                // You can also change the prototype directories here
                directories: vec![String::from("assets/prototypes")],
                // And specify whether you want the prototype files to be recursively loaded
                recursive_loading: false,
                // You can also update the allowed extensions within those directories
                extensions: Some(vec!["yaml", "json"]),
            })
        });
}
```

> All fields in `ProtoDataOptions` must be specified if you wish to use a custom deserializer. Even if you want to continue using the defaults, you must still specify them. The additional fields shown above are also the defaults if you wish to copy them.

## ⚠️ Disclaimer

Before you install it into your project, please understand the limitations of this crate. While it makes working with
some entities easier, it may come at a bit of a performance cost depending on your project.

According to the [`bench`](/examples/bench.rs) example, spawning a Prototype can be about 1.8x slower than defining the
entity in the system manually (this may vary depending on the data being loaded). This difference becomes much smaller for release builds, but is still a tad slower. For some projects,— except maybe for those that are really intensive or
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
  holds onto the handle. This can be prevented by hosting the asset on a separate component and manually creating the handles when spawning that Prototype:

  ```rust
  use bevy::prelude::*;
  use bevy_proto::prelude::*;

  #[derive(Component)]
  struct OtherComponent(Handle<Image>);

  fn attach<T: Prototypical>(
    prototype: T, 
    my_asset: Handle<Image>,
    commands: &mut Commands,
    data: &Res<ProtoData>,
    asset_server: &Res<AssetServer>,
  ) {
    // Attach fictional OtherComponent with asset "my_asset" which should unload when despawned
    prototype.spawn(commands, data, asset_server).insert(OtherComponent(my_asset));
  }

  ```

With all of that said, this package is meant to speed up development and make changes to entity archetypes easier for
humans (especially non-programmers) to interact with. If the performance hit is too much for your project, you are
better off sticking with the standard methods of defining entities.
