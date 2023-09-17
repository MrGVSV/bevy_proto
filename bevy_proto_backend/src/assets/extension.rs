use crate::assets::{AssetSchematic, InlinableProtoAsset, ProtoAsset};
use bevy::app::App;
use bevy::asset::{AddAsset, Handle};
use bevy::reflect::TypePath;

/// [`App`] extension trait for working with [`AssetSchematics`].
///
/// [`AssetSchematics`]: AssetSchematic
pub trait AssetSchematicAppExt {
    /// Registers an [`AssetSchematic`].
    ///
    /// This is a convenience method for the following registrations:
    /// - `Handle<T::Output>`
    /// - `ProtoAsset<T::Output>`
    /// - `InlinableProtoAsset<T>`
    /// - `T::Input`
    fn register_asset_schematic<T: AssetSchematic + TypePath>(&mut self) -> &mut Self;
}

impl AssetSchematicAppExt for App {
    fn register_asset_schematic<T: AssetSchematic + TypePath>(&mut self) -> &mut Self {
        self.add_asset::<T::Output>()
            .register_type::<Handle<T::Output>>()
            .register_type::<Option<Handle<T::Output>>>()
            .register_type::<ProtoAsset<T::Output>>()
            .register_type::<Option<ProtoAsset<T::Output>>>()
            .register_type::<InlinableProtoAsset<T>>()
            .register_type::<Option<InlinableProtoAsset<T>>>()
            .register_type::<T::Input>()
    }
}
