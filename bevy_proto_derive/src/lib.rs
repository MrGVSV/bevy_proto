use proc_macro::TokenStream;

use crate::asset_schematic::{DeriveAssetSchematic, ExternalAssetSchematic};
use quote::ToTokens;
use syn::parse_macro_input;

use crate::schematic::{DeriveSchematic, ExternalSchematic};

mod asset_schematic;
mod common;
mod schematic;
mod utils;

/// Derive the `Schematic` trait.
///
/// This macro will generate the impl for `Schematic` as well as a corresponding
/// `Schematic::Input` type if one is needed.
///
/// # Attributes
///
/// ## Reflection Attributes
///
/// Because this derive macro might generate a custom input type that also relies on reflection,
/// any reflection attributes on a _field_ are included on the generated field.
///
/// For example, adding `#[reflect(default)]` to a field will have it also have the generated
/// input's field be marked with `#[reflect(default)]`.
///
/// This, of course, only applies when an input actually needs to be generated.
///
/// ## Container Attributes
///
/// ### `#[schematic(kind = {"resource"|"bundle"})]`
///
/// Tells the derive macro what kind of schematic this is.
///
/// If `"resource"`, then the macro will insert the schematic type as a resource.
/// If `"bundle"`, then the macro will insert the schematic as a bundle/component.
///
/// The default behavior is `"bundle"`, however, it may be set to that
/// if wanting to be explicit about which behavior to use.
///
/// ### `#[schematic(input)]`
///
/// If the macro needs to generate an input type, this attribute gives a bit of control over it.
///
/// #### Arguments
///
/// ##### `(vis = VISIBILITY)`
///
/// _Optional_
///
/// Controls the visibility of the generated input type.
///
/// By default, the type is hidden behind an anonymous context.
/// Specifying a visibility hoists this out of that context and gives it that visibility.
///
/// ##### `(name = IDENT)`
///
/// _Optional_
///
/// Controls the name of the generated input type.
///
/// This is generally used in conjunction with the `vis` argument.
///
/// ### `#[schematic(from = path::to::Type)]`
///
/// Instead of generating an input type, this attribute has the derive macro use the given one.
///
/// For this to work, one of the following traits must be satisfied:
/// - `From<CustomInput> for MyType`
/// - `FromSchematicInput<CustomInput> for MyType`
///
/// This is useful for defining custom logic or controlling the serialized representation.
///
/// ### `#[schematic(into = path::to::Type)]`
///
/// This designates the type to be used in place of `Self`.
///
/// This can be used to create schematics for external types.
///
/// For this to work, one of the following traits must be satisfied:
/// - `From<CustomSchematic> for ExternalType`
/// - `FromSchematicInput<CustomSchematic> for ExternalType`
///
/// ## Field Attributes
///
/// ### `#[schematic(asset)]`
///
/// This attribute is used with a `Handle<T>` field to easily load assets.
///
/// Note that it must be used with a typed handle.
/// Untyped handles are currently not supported.
///
/// The generated field will be of type `ProtoAsset`.
///
/// By default, all assets are lazy-loaded— that is, they're only loaded once the schematic is used.
///
/// #### Arguments
///
/// ##### `(preload)`
///
/// _Optional_
///
/// If present, then the asset will be preloaded as a dependency of the schematic.
///
/// Cannot be used with the `unique` argument.
///
/// ##### `(unique)`
///
/// _Optional_
///
/// Note: This attribute does nothing without the `inline` argument.
///
/// By default, when an asset is loaded inline it will only create a single asset.
/// Additional loads will point to the same asset.
///
/// In some cases, it's desirable to have each load create a new asset.
/// This attribute allows a unique asset to be created upon each load.
///
/// Cannot be used with the `preload` argument.
///
/// ##### `(inline)`
///
/// _Optional_
///
/// Normally, assets are defined in their own unique file and referenced by path.
///
/// However, there are times when it would be better to define an asset inline with a schematic.
/// This attribute allows an asset to either be defined by path or inline.
///
/// The asset type within the `Handle` or the type specified by the `type` argument must implement `AssetSchematic`.
/// This will result in the field being generated as an `InlinableProtoAsset`.
///
/// Cannot be used with the `path` argument.
///
/// ##### `(path = "path/to/asset.png")`
///
/// _Optional_
///
/// This defines a single asset path to be used.
///
/// This can be used when a particular asset should always be used.
/// When this argument is found, the corresponding field will be removed from the generated input type.
///
/// Cannot be used with the `inline` argument.
///
/// ##### `(type = path::to::AssetType)`
///
/// _Optional_
///
/// When using a typed handle, this macro will attempt to infer the asset type from the handle's type.
/// However, if this fails or if defining an asset schematic type that is not the asset itself,
/// this attribute can be used with the desired type.
///
/// ### `#[schematic(entity)]`
///
/// This attribute is used with an `Entity` or `Option<Entity>` field to easily setup basic entity relations.
///
/// The generated field will be of type `EntityAccess`, allowing it to be deserialized from
/// a `ProtoEntity` enum.
///
/// #### Arguments
///
/// ##### `(path = "../path/to/entity/@2")`
///
/// _Optional_
///
/// This defines a single entity path to be used.
///
/// This can be used when a particular entity should always be used.
/// When this argument is found, the corresponding field will be removed from the generated input type.
///
/// ### `#[schematic(from = path::to::FieldType)]`
///
/// This controls what type is used for the field in a generated input type.
///
/// For this to work, one of the following traits must be satisfied:
/// - `From<CustomFieldInput> for MyFieldType`
/// - `FromSchematicInput<CustomFieldInput> for MyFieldType`
///
/// This is useful for defining custom logic or controlling the serialized representation.
///
/// ### `#[schematic(optional)]`
///
/// Entity and asset fields are able to be defined as optional.
/// Normally, this macro will determine whether this is the case by the type.
/// If this fails, this attribute can be used to force the field to be optional.
///
/// It can also be used to opt-out by specifying `#[schematic(optional = false)]`.
#[proc_macro_derive(Schematic, attributes(schematic))]
pub fn derive_schematic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveSchematic);
    input.into_token_stream().into()
}

/// Internal macro used to easily implement `Schematic` on external types.
///
/// Takes a type definition:
///
/// ```ignore
/// impl_external_schematic! {
///   struct SomeExternalType {
///     foo: usize
///   }
/// }
/// ```
///
/// The type definition itself is consumed and used to generate the
/// schematic impls and associated output.
///
/// All other tokens after the type definition are kept intact,
/// allowing for other types to be defined, such as a custom input.
///
/// ```ignore
/// impl_external_schematic! {
///   #[schematic(from = SomeExternalTypeInput)]
///   struct SomeExternalType {
///     // We can leave out the definition for `foo` here
///     // since we're using a custom input type
///   }
///
///   #[derive(Reflect)]
///   pub struct SomeExternalTypeInput(usize);
///   impl From<SomeExternalTypeInput> for SomeExternalType {
///     fn from(input: SomeExternalTypeInput) -> Self {
///       SomeExternalType {
///         foo: input.0
///       }
///     }
///   }
/// }
/// ```
#[doc(hidden)]
#[proc_macro]
pub fn impl_external_schematic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExternalSchematic);
    input.to_token_stream().into()
}

/// Derive the `AssetSchematic` trait.
///
/// This macro will generate the impl for `AssetSchematic` as well as a corresponding
/// `Schematic::Input` type if one is needed.
///
/// # Attributes
///
/// This macro supports most of the attributes that the [`Schematic` derive macro] supports,
/// but with the `asset_schematic` namespace rather than `schematic`.
///
/// ## Container Attributes
///
/// This macro shares many of the same container attributes as the `Schematic` derive macro including:
/// - `#[asset_schematic(input)]`
/// - `#[asset_schematic(from = path::to::Type)]`
/// - `#[asset_schematic(into = path::to::Type)]`
///
/// ### `#[asset_schematic(no_preload)]`
///
/// By default, this macro will also generate the impl for `PreloadAssetSchematic` to allow
/// the asset to be preloaded.
///
/// This is useful for assets that are used frequently and/or are small in size,
/// but sometimes this isn't always desirable or even possible.
///
/// This attribute disables the generation of the `PreloadAssetSchematic` impl.
///
/// ## Field Attributes
///
/// This macro shares many of the same field attributes as the `Schematic` derive macro including:
/// - `#[asset_schematic(asset)]`
/// - `#[asset_schematic(from = path::to::FieldType)]`
/// - `#[asset_schematic(optional)]`
///
/// For the `asset` attribute, the following arguments are supported:
/// - `(inline)`
/// - `(path = "path/to/asset.png")`
/// - `(type = path::to::AssetType)`
///
/// [`Schematic` derive macro]: derive_schematic
#[proc_macro_derive(AssetSchematic, attributes(asset_schematic))]
pub fn derive_asset_schematic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveAssetSchematic);
    input.into_token_stream().into()
}

/// Internal macro used to easily implement `AssetSchematic` on external types.
///
/// Takes a type definition:
///
/// ```ignore
/// impl_external_asset_schematic! {
///   struct SomeExternalAssetType {
///     foo: usize
///   }
/// }
/// ```
#[doc(hidden)]
#[proc_macro]
pub fn impl_external_asset_schematic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExternalAssetSchematic);
    input.to_token_stream().into()
}
