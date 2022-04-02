# bevy_proto

[![Crates.io](https://img.shields.io/crates/v/bevy_proto)](https://crates.io/crates/bevy_proto)
[![Docs](https://img.shields.io/docsrs/bevy_proto)](https://docs.rs/bevy_proto/latest/bevy_proto/)
[![License](https://img.shields.io/crates/l/bevy_proto)](./License.md)



Create entities in the Bevy game engine with a simple config file. 

```yaml
name: "Simple Enemy"
template: Creature
components:
  - type: my_crate::Enemy
    struct: {}
  - type: my_crate::Attack
    struct:
      damage: 
        type: i32
        value: 10
  - type: my_crate::Armed
    struct:
      weapons:
        type: vec::Vec<alloc::string::String>
        value: [ "laser sword", "ray-blaster" ]
      primary: 
        type: alloc::string::String
        value: "laser sword"
```

## 📋 Features

* **Define** entities easily with config files:

  > ```yaml
  > name: Player
  > components:
  >   - type: my_crate::Controllable
  >     struct: {}
  >   - type: my_crate::Health
  >     struct:
  >       max:
  >         type: u8
  >         value: 20
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
  >   - type: my_crate::CustomSprite
  >     struct:
  >       texture:
  >         type: alloc::string::String
  >         value: "path/to/texture.png"
  > ```

## 📲 Installation

```toml
[dependencies]
bevy_proto = "0.4"
```

Then add it to your app like so:

```rust
use bevy::prelude::*;
use bevy_proto::prelude::*;

fn main() {
  let app = App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(ProtoPlugin::<Prototype>::default());
  // ...
}
```

## 📓 Examples

Check out these examples for more details as to how to use this crate:

* [attributes](https://github.com/MrGVSV/bevy_proto/blob/main/examples/attributes.rs) - A showcase of the available derive helper attributes
* [basic](https://github.com/MrGVSV/bevy_proto/blob/main/examples/basic.rs) - The most basic example of how this crate works
* [bundles](https://github.com/MrGVSV/bevy_proto/blob/main/examples/bundles.rs) - A demonstration of a more complex prototype that includes assets
* [config](https://github.com/MrGVSV/bevy_proto/blob/main/examples/config.rs) - Shows how to add extra configuration to the plugin
* [handles](https://github.com/MrGVSV/bevy_proto/blob/main/examples/handles.rs) - An automated version of the [bundles](https://github.com/MrGVSV/bevy_proto/blob/main/examples/bundles.rs) example
* [templates](https://github.com/MrGVSV/bevy_proto/blob/main/examples/templates.rs) - An example of how templates affect your prototypes

## 🕊 Bevy Compatibility

| bevy | bevy_proto |
| ---- | ---------- |
| 0.7  | 0.4        |
| 0.6  | 0.3.0      |
| 0.5  | 0.2.1      |

## ✨ Usage

### Creating Components

First, create a struct that implements `ProtoComponent`. This can be done one of two ways:

For simple components, `ProtoComponent` may be derived:

```rust
use bevy::prelude::*;
use bevy::reflect::FromReflect;
use bevy_proto::prelude::*;

#[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
struct Movement {
  speed: u16
}
```

> By default, the `ProtoComponent` is cloned into spawned entities.

Otherwise, you can define them manually:

```rust
use bevy::prelude::*;
use bevy::ecs::world::EntityMut;
use bevy::reflect::FromReflect;
use bevy_proto::prelude::*;

#[derive(Reflect, FromReflect, Component, Clone)]
#[reflect(ProtoComponent)]
struct Inventory(Vec<String>);

impl ProtoComponent for Inventory {
  
  fn apply(&self, entity: &mut EntityMut) {
    entity.insert( self.clone() );
  }
  
  fn as_reflect(&self) -> &dyn Reflect {
      self
  }
  
}
```

> A `ProtoComponent` does *not* need to be a component itself. It can be used purely as a [DTO](https://en.wikipedia.org/wiki/Data_transfer_object) to generate other components or bundles. You have full access to the `EntityMut` so you can insert bundles or even multiple components at once.
>
> Other ways of generating components from non-component `ProtoComponent` structs can be found in the [attributes](https://github.com/MrGVSV/bevy_proto/blob/main/examples/attributes.rs) example.

#### Registering Components

This crate uses Bevy's reflection system to deserialize your `ProtoComponent` objects. So make sure you register your types!

```rust
use bevy::reflect::FromReflect;
use bevy::prelude::*;
use bevy_proto::*;

#[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
struct MyComponent;

fn main() {
  let app = App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(ProtoPlugin::<Prototype>::default())
    .register_type::<MyComponent>();
  // ...
}
```

### Defining the Prototype

Define the Prototype in a config file. By default YAML (and by extension, valid JSON) files are supported. The file must conform to the Bevy Reflect format (which is currently a bit verbose but should become much simpler relatively soon).

```yaml
# assets/prototypes/adventurer.prototype.yaml
---
name: "Adventurer"
components:
  - type: my_crate::Inventory
    struct:
      type: vec::Vec<alloc::string::String>
      value: ["sword"]
  - type: my_crate::Movement
    struct:
      speed:
        type: i32
        value: 10
```

> By default, all `.prototype.yaml`  files can be processed. To allow other file types to be read, you can enable its respective feature (currently:  `json` and `ron`).

#### Using Templates

A Prototype can also include a *template*. A template is any other Prototype and is used to define common components that should be inserted into its inheritors. This is helpful for reducing duplicate markup and quickly refactoring collections of Prototypes.

First, define the template as any other prototype:

```yaml
# assets/prototypes/base_npc.prototype.yaml
---
name: "NPC"
components:
  - type: my_crate::Inventory
    struct:
      type: vec::Vec<alloc::string::String>
      value: []
  - type: my_crate::Movement
    struct:
      speed:
        type: i32
        value: 10
```

Then inherit the template in another Prototype by adding the relative path to its file. If no extension is added to the path, it will be expanded to include the extension of the current file. 

```yaml
# assets/prototypes/peasant.prototype.yaml
---
name: "Peasant"
template: base_npc # Same as "./base_npc.prototype.yaml"
```

You can also override template components:

```yaml
# assets/prototypes/adventurer.prototype.yaml
---
name: "Adventurer"
template: base_npc
components:
  - type: my_crate::Inventory
    struct:
      type: vec::Vec<alloc::string::String>
      value: ["sword"]
```

Multiple templates can be specified as well. However, conflicting components will be overridden in reverse order (templates listed first can override templates listed last):

```yaml
# assets/prototypes/fast_adventurer.prototype.yaml
---
name: "Fast Adventurer"
templates: speedy, base_npc # "speedy" may override "base_npc"
components:
  - type: my_crate::Inventory
    struct:
      type: vec::Vec<alloc::string::String>
      value: ["sword"]
```

> Templates can be specified as a standard list or as a comma-separated string (like in the example above). Additionally,  `templates` is an alias for `template`, so either one may be used.

### Loading the Prototype

Prototypes must be loaded into your app before they can be used. This can be done by simply using the `AssetServer`:

```rust
use bevy::prelude::*;
use bevy_proto::prelude::*;

fn load_prototype(asset_server: Res<AssetServer>, mut manager: ProtoManager) {
  let handle: Handle<Prototype> = asset_server.load("prototypes/adventurer.prototype.yaml");
  manager.add(handle);
}
```

> Passing the handle to `ProtoManager::add` stores it so that it will stay loaded (Bevy assets remain loaded as long as a _strong_ handle for it exists). You could have just as easily used your own resource to store this handle.

### Spawning the Prototype

Once your prototype is loaded, spawning it is easy!

```rust
use bevy::prelude::*;
use bevy_proto::prelude::*;

fn spawn_adventurer(mut commands: Commands, manager: ProtoManager) {
  if let Some(proto) = manager.get("Adventurer") {
    // Spawns in our "Adventurer" Prototype
    proto.spawn(&mut commands);
  }
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

fn spawn_adventurer(mut commands: Commands, manager: ProtoManager) {
  if let Some(proto) = manager.get("Adventurer") {
    let adventurer: Entity = proto
      .spawn(&mut commands)
      .insert(Friendly)
      .insert(Named("Bob".to_string()))
      .id();
    
    // ...
  }
}
```

### Using Assets

For Prototypes that need access to assets, you can get access one of two ways:

1. Using the `AssetServer` to load the asset directly
2. Preloading the asset as a dependency of the prototype

#### The `AssetServer` method

This method is generally preferred if you only want the asset to stay loaded as long as the entity. You can access the `AssetServer` using the `EntityMut` passed to your `PRotoComponent`.

```rust
use bevy::ecs::world::EntityMut;
use bevy::reflect::FromReflect;
use bevy::prelude::*;
use bevy_proto::prelude::*;

#[derive(Component)]
struct Renderable {
  texture: Handle<Image>
}

#[derive(Reflect, FromReflect, Component)]
#[reflect(ProtoComponent)]
struct Creature {
  texture_path: String
}

impl ProtoComponent for Creature {
  fn apply(&self, entity: &mut EntityMut) {
    let asset_server = entity.world().resource::<AssetServer>();
    let handle: Handle<Image> = asset_server.load(&self.texture_path);
    
    entity.insert(Renderable {
      texture: handle
    });
  }
  
  fn as_reflect(&self) -> &dyn Reflect {
      self
  }
  
}
```

#### The Preload Method

The other method is to preload the assets. This can be done manually with the `preload_assets` method or automatically with the `preload` attribute.

##### Manually

```rust
use bevy::ecs::world::EntityMut;
use bevy::reflect::FromReflect;
use bevy::prelude::*;
use bevy_proto::prelude::*;

#[derive(Component)]
struct Renderable {
  texture: Handle<Image>
}

#[derive(Reflect, FromReflect, Component)]
#[reflect(ProtoComponent)]
struct Creature {
  texture_path: HandlePath<Image>
}

impl ProtoComponent for Creature {
  fn apply(&self, entity: &mut EntityMut) {
    let asset_server = entity.world().resource::<AssetServer>();
    let handle: Handle<Image> = asset_server.load(&self.texture_path);
    
    entity.insert(Renderable {
      texture: handle
    });
  }
  
  // Here we can preload any assets when the prototype is first loaded
  fn preload_assets(&mut self, preloader: &mut AssetPreloader) {
    // Load the asset and return the handle
    let handle: Handle<Image> = preloader.preload(self.texture_path.path());
    self.texture_path.store_handle(handle);
    
    // OR load the asset as a dependency
    // (automatically stays loaded at least as long as the prototype)
    let _: Handle<Image> = preloader.preload_dependency(self.texture_path.path());
  }
  
  fn as_reflect(&self) -> &dyn Reflect {
      self
  }
}
```

##### Automatically

```rust
use bevy::ecs::world::EntityMut;
use bevy::reflect::FromReflect;
use bevy::prelude::*;
use bevy_proto::prelude::*;

#[derive(Component)]
struct Renderable {
  texture: Handle<Image>
}

#[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
#[proto_comp(into = "Renderable")]
struct Creature {
  #[proto_comp(preload(type = "Image", dest = "texture_path"))]
  texture_path: HandlePath<Image>
}

impl From<Creature> for Renderable {
  fn from(creature: Creature) -> Self {
    Self {
      texture: creature.texture_path.handle().unwrap().clone()
    }
  }
}
```

### Custom Prototypes

The default Prototype object looks like this:

```rust
use bevy::reflect::TypeUuid;
use bevy_proto::prelude::*;

#[derive(Debug, PartialEq, TypeUuid)]
#[uuid = "64d16515-97e9-4762-a275-a80e1b6b5c27"]
pub struct Prototype {
    /// The name of this prototype.
    pub(crate) name: String,
    /// The list of template prototypes to inherit from, if any.
    ///
    /// These are listed as relative paths to this prototype's file. See [`TemplateList`]
    /// for more details.
    ///
    /// Additionally, to find out how these names are deserialized, check out
    /// [`TemplateListDeserializer`](crate::serde::TemplateListDeserializer).
    pub(crate) templates: TemplateList,
    /// The map of assets this prototype depends on.
    pub(crate) dependencies: DependencyMap,
    /// The components belonging to this prototype.
    pub(crate) components: ComponentList,
}
```

However, you can use your own Prototype object. Any struct that implements `Prototypical` and `ProtoDeserializable` can be used in place of the default Prototype.

## ⚠️ Disclaimer

Before you install it into your project, please understand the limitations of this crate. While it makes working with some entities easier, it may come at a bit of a performance cost depending on your project.

According to the [`bench`](/examples/bench.rs) example, spawning a Prototype can be about 4–6x slower than defining the entity in the system manually (this may vary depending on the data being loaded). This difference becomes much smaller for release builds, but is still a tad slower. For many projects,— except maybe for those that are really intensive or spawn lots of entities very frequently,— this may not be a problem.

Still, it's good to consider the drawbacks of using this system and see if it's right for your own project. Here's a breakdown of the top current/potential issues:

* *Dynamic Dispatch* - This crate uses dynamic trait objects to add or remove any component on a Prototype. However, this comes at a cost since the compiler can no longer know the types in advance, preventing things like static dispatch, monomorphization, etc.
* *Template Recursion* - Prototypes with templates apply each template recursively. This can be slow for large template trees with lots of templates.

With all of that said, this package is meant to speed up development and make changes to entity archetypes easier for humans (especially non-programmers) to interact with. If the performance hit is too much for your project, you are better off sticking with the standard methods of defining entities.
