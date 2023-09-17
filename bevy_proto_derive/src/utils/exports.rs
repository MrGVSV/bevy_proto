//! Common type paths used in generated macro output.

use crate::utils::{get_bevy_crate, get_proto_crate};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// Defines a path to a type for the given crate.
macro_rules! create_export {
    (bevy :: $($segment: ident ::)* [$name: ident]) => {
        create_export!(@export $($segment ::)* [$name] with get_bevy_crate);
    };
    (bevy_proto :: $($segment: ident ::)* [$name: ident]) => {
        create_export!(@export $($segment ::)* [$name] with get_proto_crate);
    };
    (@export $($segment: ident ::)* [$name: ident] with $get_crate: ident) => {
        pub(crate) struct $name;
        impl ToTokens for $name {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let root = $get_crate();
                tokens.extend(quote!(#root :: $($segment ::)* $name));
            }
        }
    };
}

create_export!(bevy_proto::assets::[ProtoAsset]);
create_export!(bevy_proto::assets::[AssetSchematic]);
create_export!(bevy_proto::assets::[PreloadAssetSchematic]);
create_export!(bevy_proto::assets::[InlinableProtoAsset]);
create_export!(bevy_proto::assets::__private::[PreloadProtoAssetInput]);
create_export!(bevy_proto::deps::[DependenciesBuilder]);
create_export!(bevy_proto::schematics::[Schematic]);
create_export!(bevy_proto::schematics::[FromSchematicInput]);
create_export!(bevy_proto::schematics::[FromSchematicPreloadInput]);
create_export!(bevy_proto::schematics::[SchematicId]);
create_export!(bevy_proto::schematics::[SchematicContext]);
create_export!(bevy_proto::tree::[EntityAccess]);

create_export!(bevy::reflect::[Reflect]);
create_export!(bevy::reflect::[FromReflect]);
create_export!(bevy::assets::[AssetServer]);
create_export!(bevy::utils::[Uuid]);
