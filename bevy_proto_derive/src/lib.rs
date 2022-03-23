use proc_macro::TokenStream;

use crate::attributes::{parse_attrs, parse_fields, ProtoCompAttr};
use proc_macro2::Span;
use quote::quote;
use syn::*;

mod attributes;

/// Automatically implements [`ProtoComponent`] for the given
/// struct or enum. This works on all structs and enums, including tuple and unit structs.
///
/// **NOTE: `Clone`, `serde::Serialize`, and `serde::Deserialize` must also be implemented/derived**
///
/// # Examples
///
/// ```
/// # use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Serialize, Deserialize, ProtoComponent)]
/// struct SomeComponent {
///     some_string: String,
///     some_int: i32,
/// }
///
/// // Which generates:
/// //
/// // #[typetag::serde]
/// // impl bevy_proto::ProtoComponent for #ident { ///
/// // 	fn insert_self(
/// //    &self,
/// //    commands: &mut bevy_proto::ProtoCommands,
/// // 	  asset_server: &bevy::prelude::Res<bevy::prelude::AssetServer>,
/// //  ) {
/// //      let component = self.clone();
/// //      commands.insert(component);
/// //    }
/// //  }
/// ```
#[proc_macro_derive(ProtoComponent, attributes(proto_comp))]
pub fn proto_comp_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        attrs,
        generics,
        data,
        ..
    } = parse_macro_input!(input);

    let bevy_proto = get_crate();

    let attrs = match parse_attrs(&attrs) {
        Ok(attrs) => attrs,
        Err(err) => return err.into_compile_error().into(),
    };

    let entity_ident = Ident::new("entity", Span::call_site());
    let preloader_ident = Ident::new("preloader", Span::call_site());

    // The stream used to insert this ProtoComponent
    let inserter = attrs
        .iter()
        .find_map(|attr| match attr {
            ProtoCompAttr::Into(ty) => Some(quote! {
                // Convert then insert
                let component: #ty = self.clone().into();
                #entity_ident.insert(component);
            }),
            ProtoCompAttr::With(path) => Some(quote! {
                // Call function to handle insertion
                #path(self, #entity_ident);
            }),
        })
        .unwrap_or_else(|| {
            quote! {
                // Default to inserting the ProtoComponent itself
                #entity_ident.insert(self.clone());
            }
        });

    let fields = match parse_fields(&data) {
        Ok(fields) => fields,
        Err(err) => return err.into_compile_error().into(),
    };

    let preloaders = fields
        .iter()
        .flat_map(|field| field.get_preloaders(&preloader_ident))
        .collect::<Vec<_>>();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let output = quote! {
        impl #impl_generics #bevy_proto::prelude::ProtoComponent for #ident #ty_generics #where_clause {
            fn apply(&self, #entity_ident: &mut bevy::ecs::world::EntityMut) {
                #inserter
            }

            fn as_reflect(&self) -> &dyn bevy::prelude::Reflect {
                self
            }

            fn preload_assets(&mut self, #preloader_ident: &mut #bevy_proto::prelude::AssetPreloader) {
                use #bevy_proto::prelude::StoreHandle;
                #(#preloaders)*
            }
        }
    };

    output.into()
}

fn get_crate() -> proc_macro2::TokenStream {
    use proc_macro_crate::{crate_name, FoundCrate};

    let found_crate = crate_name("bevy_proto").expect("bevy_proto is present in `Cargo.toml`");
    match found_crate {
        FoundCrate::Itself => quote!(::bevy_proto),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( ::#ident )
        }
    }
}
