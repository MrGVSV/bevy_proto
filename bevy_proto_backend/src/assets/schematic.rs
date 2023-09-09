use crate::deps::DependenciesBuilder;
use crate::schematics::{SchematicContext, SchematicId};
use bevy::asset::Asset;
use bevy::reflect::{FromReflect, GetTypeRegistration};

/// Trait used to create a schematic for an [asset] which allows them to be
/// defined and loaded within a [schematic].
///
/// The implementor of this trait does not need to be an asset itself.
/// The actual asset type is defined by the associated [`Output`] type.
/// This allows assets to be defined with any number of schematics,
/// even for external assets.
///
/// This trait can either be manually implemented or [derived].
///
/// See the [module-level documentation] for details.
///
/// [asset]: Asset
/// [schematic]: crate::schematics::Schematic
/// [`Output`]: AssetSchematic::Output
/// [derived]: bevy_proto_derive::AssetSchematic
/// [module-level documentation]: crate::assets
pub trait AssetSchematic: 'static {
    /// The input type of the schematic.
    ///
    /// This is the type that is deserialized from a prototype file
    /// and used to store the asset's data.
    type Input: FromReflect + GetTypeRegistration;
    /// The output asset type.
    type Output: Asset;

    /// Loads the [output] asset from the given [input].
    ///
    /// [output]: Self::Output
    /// [input]: Self::Input
    fn load(input: &Self::Input, id: SchematicId, context: &mut SchematicContext) -> Self::Output;
}

/// Allows an [`AssetSchematic`] to preload its asset.
pub trait PreloadAssetSchematic: AssetSchematic {
    /// Preloads the [output] asset from the given [input].
    ///
    /// [output]: Self::Output
    /// [input]: Self::Input
    fn preload(
        input: Self::Input,
        id: SchematicId,
        context: &mut DependenciesBuilder,
    ) -> Self::Output;
}
