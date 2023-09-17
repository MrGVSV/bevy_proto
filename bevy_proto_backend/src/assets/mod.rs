//! Schematic types used to build [assets].
//!
//! # Handling Assets
//!
//! Bevy uses a [`Handle`] type to reference assets.
//! Unfortunately, this type isn't very useful serialized as it doesn't directly
//! track the asset's path.
//!
//! In order to reference assets in a serialized format,
//! this crate defines a special [`ProtoAsset`] type which does keep track of the path.
//! This can then be used in the input type for a [`Schematic`] to load assets.
//! And, in fact, when deriving `Schematic`, a `Handle`-containing field
//! can be marked with the `#[schematic(asset)]` attribute to automatically handle this.
//!
//! ```
//! # use bevy::prelude::*;
//! # use bevy_proto_derive::Schematic;
//! # use bevy_proto_backend::schematics::ReflectSchematic;
//! #[derive(Component, Reflect, Schematic)]
//! #[reflect(Schematic)]
//! struct Player {
//!   #[schematic(asset)]
//!   sprite: Handle<Image>
//! }
//! ```
//!
//! This will generate a schematic input type with the appropriate `ProtoAsset` field:
//!
//! ```
//! # use bevy::prelude::*;
//! # use bevy_proto_backend::assets::ProtoAsset;
//! #[derive(Reflect)]
//! struct PlayerInput {
//!   sprite: ProtoAsset<Image>
//! }
//! ```
//!
//! See the [derive macro documentation](bevy_proto_derive::Schematic) for more details
//! on this attribute and its various arguments.
//!
//! # Asset Schematics
//!
//! While referencing an asset by path is good enough for most cases,
//! sometimes it would be better to just define an asset as part of a prototype,
//! without needing to create a completely separate asset file.
//!
//! This is where [`AssetSchematic`] comes in.
//! This trait defines an [`Input`] data type and an [`Output`] asset type,
//! which can be used to create assets inline with a given schematic.
//! Because the implementor does not need to be the asset type itself,
//! any number of schematics can be defined for a single asset type.
//!
//! An asset schematic is used much like normal assets,
//! except, instead of using `ProtoAsset`, [`InlinableProtoAsset`] is used.
//!
//! The `AssetSchematic` trait can be derived and can even reference other assets.
//! It can then be referenced in a schematic using the `#[schematic(asset)]` attribute
//! with the `inline` argument.
//!
//! ```
//! # use bevy::prelude::*;
//! # use bevy::reflect::{TypePath, TypeUuid};
//! # use bevy_proto_derive::{AssetSchematic, Schematic};
//! # use bevy_proto_backend::schematics::ReflectSchematic;
//! #[derive(AssetSchematic, TypeUuid, TypePath)]
//! #[uuid = "a3de79e1-0364-4fdf-9a82-9fae288e0aed"]
//! struct Level {
//!   name: String,
//!   #[asset_schematic(asset)]
//!   background: Handle<Image>
//! }
//!
//! #[derive(Component, Reflect, Schematic)]
//! #[reflect(Schematic)]
//! struct CurrentLevel {
//!   #[schematic(asset(inline))]
//!   sprite: Handle<Level>
//! }
//! ```
//!
//! Asset schematics have a lot of moving parts that need to be registered in the app.
//! To make things easier, this crate comes with an [extension trait] which can be used
//! to automatically register all of the necessary types.
//!
//! ```ignore
//! app.register_asset_schematic::<MyAssetSchematic>();
//! ```
//!
//! See the [derive macro documentation](bevy_proto_derive::AssetSchematic) for more details.
//!
//! [assets]: bevy::asset::Asset
//! [`Handle`]: bevy::asset::Handle
//! [`ProtoAsset`]: ProtoAsset
//! [`Schematic`]: crate::schematics::Schematic
//! [`AssetSchematic`]: AssetSchematic
//! [`Input`]: AssetSchematic::Input
//! [`Output`]: AssetSchematic::Output
//! [`InlinableProtoAsset`]: InlinableProtoAsset
//! [extension trait]: AssetSchematicAppExt

pub use bevy_proto_derive::AssetSchematic;
pub use event::*;
pub use extension::*;
pub use proto::*;
pub use schematic::*;

mod event;
mod extension;
mod proto;
mod schematic;
