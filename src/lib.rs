//! Define and spawn entities with simple configuration files.
//!
//! # Capabilities
//!
//! * Define entities in configuration files called _[prototypes]_
//! * Inherit from other prototype files
//! * Establish entity hierarchies
//! * Load or preload assets
//!
//! This is all built on a backend crate called [`bevy_proto_backend`].
//! If you want to define your own prototype schema,
//! feel free to use that crate directly!
//!
//! # Example
//!
//! Prototypes can be defined using a RON file ending in `.prototype.ron`:
//!
//! ```text
//! // Player.prototype.ron
//! (
//!   name: "Player",
//!   schematics: {
//!     "bevy_proto::custom::SpriteBundle": (
//!       texture: AssetPath("textures/player.png"),
//!     ),
//!   }
//! )
//! ```
//!
//! Then they can be loaded and spawned from any system:
//!
//! ```
//! use bevy_proto::prelude::*;
//!
//! fn load_player(mut prototypes: PrototypesMut) {
//!   prototypes.load("prototypes/Player.prototype.ron");
//! }
//!
//! fn spawn_player(mut commands: ProtoCommands) {
//!   commands.spawn("Player");
//! }
//! ```
//!
//! # Cargo Features
//!
//! | Feature           | Default | Description                                                    |
//! | ----------------- | ------- | -------------------------------------------------------------- |
//! | auto_name         | ✅      | Automatically insert [`Name`] components on spawned prototypes |
//! | custom_schematics | ✅      | Enables some [custom schematics] defined by this crate         |
//! | ron               | ✅      | Enables RON deserialization                                    |
//! | yaml              | ❌      | Enables YAML deserialization                                   |
//! | bevy_animation    | ✅      | Registers types under Bevy's `bevy_animation` feature          |
//! | bevy_audio        | ✅      | Registers types under Bevy's `bevy_audio` feature              |
//! | bevy_gltf         | ✅      | Registers types under Bevy's `bevy_gltf` feature               |
//! | bevy_pbr          | ✅      | Registers types under Bevy's `bevy_pbr` feature                |
//! | bevy_render       | ✅      | Registers types under Bevy's `bevy_render` feature             |
//! | bevy_scene        | ✅      | Registers types under Bevy's `bevy_scene` feature              |
//! | bevy_sprite       | ✅      | Registers types under Bevy's `bevy_sprite` feature             |
//! | bevy_text         | ✅      | Registers types under Bevy's `bevy_text` feature               |
//!
//! [prototypes]: proto::Prototype
//! [`Name`]: bevy::core::Name
//! [custom schematics]: custom

mod conditions;
pub mod config;
#[cfg(feature = "custom_schematics")]
pub mod custom;
pub mod hooks;
pub mod loader;
mod plugin;
pub mod proto;
mod schematics;

/// Provides the basics needed to use this crate.
///
/// This is meant to be used like:
/// ```
/// use bevy_proto::prelude::*;
/// ```
pub mod prelude {
    use crate::config::ProtoConfig;
    pub use bevy_proto_backend::deps::DependenciesBuilder;
    pub use bevy_proto_backend::proto::Prototypical;
    pub use bevy_proto_backend::schematics::{ReflectSchematic, Schematic, SchematicContext};

    pub use super::conditions::*;
    pub use super::plugin::ProtoPlugin;
    pub use super::proto::Prototype;

    /// A helper SystemParam for managing [prototypes].
    ///
    /// For the mutable version, see [`PrototypesMut`].
    ///
    /// [prototypes]: Prototype
    pub type Prototypes<'w, C = ProtoConfig> =
        bevy_proto_backend::proto::Prototypes<'w, Prototype, C>;

    /// A helper SystemParam for managing [prototypes].
    ///
    /// For the immutable version, see [`Prototypes`].
    ///
    /// [prototypes]: Prototype
    pub type PrototypesMut<'w, C = ProtoConfig> =
        bevy_proto_backend::proto::PrototypesMut<'w, Prototype, C>;

    /// A system parameter similar to [`Commands`], but catered towards [prototypes].
    ///
    /// [`Commands`]: bevy::prelude::Commands
    /// [prototypes]: Prototype
    pub type ProtoCommands<'w, 's, C = ProtoConfig> =
        bevy_proto_backend::proto::ProtoCommands<'w, 's, Prototype, C>;

    /// A struct similar to [`EntityCommands`], but catered towards [prototypes].
    ///
    /// [`EntityCommands`]: bevy::ecs::system::EntityCommands
    /// [prototypes]: Prototype
    pub type ProtoEntityCommands<'w, 's, 'a, C = ProtoConfig> =
        bevy_proto_backend::proto::ProtoEntityCommands<'w, 's, 'a, Prototype, C>;

    /// Asset lifecycle events for [prototype] assets.
    ///
    /// This is analogous to [`AssetEvent`], but accounts for prototype
    /// caching and registration.
    /// This event should be preferred over using the `AssetEvent` directly.
    ///
    /// [prototype]: Prototype
    /// [`AssetEvent`]: bevy::asset::AssetEvent
    pub type ProtoAssetEvent = bevy_proto_backend::proto::ProtoAssetEvent<Prototype>;
}

/// Provides access to the [backend crate] that `bevy_proto` is built on.
///
/// [backend crate]: bevy_proto_backend
pub mod backend {
    pub use bevy_proto_backend::*;
}
