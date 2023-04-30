# Examples

## Recommended Order

While the examples can each be examined on their own, there is a general ordering to them for those trying to learn the
basics of this crate.

The recommended order is:

1. [loading.rs](./loading.rs) - Loading and spawning prototypes
2. [hot_reload.rs](./hot_reload.rs)- How to enable and use hot reloading with prototypes
3. [basic_schematic.rs](./basic_schematic.rs) - The basics of using schematics
4. [templates.rs](./templates.rs) - How to inherit functionality from other prototypes
5. [derive_schematic.rs](./derive_schematic.rs) - A deeper look at the derive macro
6. [custom_schematic.rs](./custom_schematic.rs) - Creating a custom `Schematic`implementation
7. [hierarchy.rs](./hierarchy.rs) - Defining complex prototype hierarchies with children
8. [cycles.rs](./cycles.rs) - How prototype cycles are handled

### Bevy Examples

The [`bevy`](./bevy) folder contains examples based on
the [official Bevy examples](https://github.com/bevyengine/bevy/tree/latest/examples).
These examples are meant to show how we can modify the Bevy examples to make use of prototypes.

## Example Prototypes

Most examples use their own set of defined prototypes. These all exist within
the [assets/examples](https://github.com/MrGVSV/bevy_proto/tree/main/assets/examples) directory.

## Running the Examples

```
cargo run --package bevy_proto --example <EXAMPLE NAME>
```

Most examples should run using the default features.
However, some may require a non-default feature be enabled.

You can add `--features <FEATURE NAME>` to enable them.
Or run with `--all-features` to enable all features.