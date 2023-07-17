use proc_macro::TokenStream;

use quote::ToTokens;
use syn::parse_macro_input;

use crate::schematic::{DeriveSchematic, ExternalSchematic};

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
/// ##### `({lazy|preload})`
///
/// _Optional_
///
/// If `lazy`, then the asset won't be loaded until the schematic is actually used.
/// If `preload`, then the asset will be preloaded as a dependency of the schematic.
///
/// The `lazy` variant is not strictly needed, since it corresponds to the default behavior.
/// However, it exists for users who may want the distinction to be extra explicit.
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
