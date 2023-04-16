# bevy_proto

[![Crates.io](https://img.shields.io/crates/v/bevy_proto)](https://crates.io/crates/bevy_proto)
[![Docs](https://img.shields.io/docsrs/bevy_proto)](https://docs.rs/bevy_proto/latest/bevy_proto/)
[![License](https://img.shields.io/crates/l/bevy_proto)](https://github.com/MrGVSV/bevy_proto/blob/main/License.md)

Spawn entities in Bevy with a simple configuration file, similar to Unity's prefabs.

This crate can be used for:

* Quick prototyping
* Modding support
* Data-defined behaviors

## 📋 Features

- **Define** entities easily with config files:

  > ```rust
  > (
  >   name: "player",
  >    schematics: {
  >     "game::creature::Health": (
  >       value: 100,
  >     ),
  >   },
  > )
  > ```

- **Inherit** functionality from other prototypes:

  > ```rust
  > (
  >   name: "Skeleton",
  >   templates: ["Enemy", "Creature"],
  >   // ...
  > )
  > ```

- **Setup** entire entity hierarchies:

  > ```rust
  > (
  >   name: "Blaster",
  >   schematics: {
  >     "game::weapon::AmmoEntity": (
  >       // Even reference other entities in the tree:
  >       entity: EntityPath("./@1"),
  >     ),
  >   },
  >   children: ["Scope", "AmmoPack"]
  > )
  > ```

- **Load** assets automatically:

  > ```rust
  > (
  >   name: "Puppy",
  >   schematics: {
  >     "game::image::GameImage": (
  >       handle: AssetPath("textures/puppy.png"),
  >     ),
  >   },
  > )
  > ```

- **Spawn!**

  > ```rust
  > fn spawn_player(mut commands: ProtoCommands) {
  >   commands.spawn("player");
  > }
  > ```

## 📲 Installation

Add the following to your `[dependencies]` section in `Cargo.toml`:

```rust
bevy_proto = "0.8"
```

Or use `cargo add`:

```
cargo add bevy_proto
```

## 📓 Examples

Check out the [examples](https://github.com/MrGVSV/bevy_proto/tree/main/examples) folder for examples and tutorials for
using the crate.

## 🕊 Bevy Compatibility

| bevy   | bevy_proto |
|--------|------------|
| 0.10.1 | 0.8.0      |

For previous versions of this crate, see the [Previous Versions](#-previous-versions) section below.

## 🥅 Goals

This crate is mostly feature-complete.

There are a few more things I think would be really nice to add. Below is a list of my current goals and what's been
accomplished thus far:

| Goal                                             | Status             |
|--------------------------------------------------|--------------------|
| Reflection support                               | :white_check_mark: |
| Nested prototypes                                | :white_check_mark: |
| Package-specifier                                | :construction:     |
| Configurable schematics filtering and processing | :construction:     |
| Prototype arguments                              | :construction:     |
| Entity-less prototypes                           | :construction:     |
| Value access                                     | :construction:     |
| Custom file format support                       | :construction:     |
| Improved documentation                           | :construction:     |
| Benchmarks                                       | :construction:     |

## 🕰 Previous Versions

Before version 0.8 of `bevy_proto`, this crate relied on the [`typetag`](https://github.com/dtolnay/typetag) crate. This
allowed us to accomplish similar goals by using `serde`'
s  [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) instead of `bevy`'s `FromReflect`. This
was nice in that it meant we could avoid having to register every type, but it had a few drawbacks.

For one, it had issues working on WASM, which made its usage platform-specific. Additionally, its life-before-main
behavior is not guaranteed to exist or work as intended by the Rust compiler, meaning it could suddenly stop working at
some future point in time.

Lastly, and most importantly, is that it's not the direction Bevy itself is currently taking. Bevy is building upon its
reflection library, and much of the community is following the same. As this becomes more and more standard (especially
with an editor), it will just be better to lean into reflection.

However, in the interest of allowing users to decide if they also want to make the switch, I have preserved the
original `typetag`-based code under a new crate:

[**`bevy_proto_typetag`**](https://github.com/MrGVSV/bevy_proto_typetag)

I imagine the crate won't see many updates outside those to bump the Bevy version. I may come back to it to create a
similar flow as this crate, but I'm not going to make any promises on that as I want this crate to be the main focus.

