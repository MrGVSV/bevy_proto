use proc_macro::TokenStream;

use crate::attributes::{parse_attrs, parse_fields, ProtoCompAttr};
use proc_macro2::Span;
use quote::quote;
use syn::*;

mod attributes;

/// Automatically implements [`ProtoComponent`] for the given
/// struct or enum. This works on all structs and enums, including tuple and unit structs.
///
/// > NOTE: At minimum `Reflect` and `FromReflect` must also be derived/implemented. Depending
/// > on attribute usage, `Clone` and `Component` will also likely need to be derived/implemented.
///
/// # Container Attributes
///
/// These are attributes that can be applied to the container type.
///
/// * `#[proto_comp(into = "SomeComponent")]`
///    
///   This will convert this `ProtoComponent` into the given component using `.into()`
///   when inserting into the entity. The type must be in scope and must implement
///   `Component`. This requires that `Clone` also be derived/implemented.
///
/// * `#[proto_comp(into_bundle = "SomeBundle")]`
///
///   This will convert this `ProtoComponent` into the given bundle using `.into()`
///   when inserting into the entity. The type must be in scope and must implement
///   `Bundle`. This requires that `Clone` also be derived/implemented.
///
/// * `#[proto_comp(with = "some_func")]`
///
///   This will pass a reference to this `ProtoComponent` and a mutable reference
///   to the entity to the given function. The function must take the form:
///   `fn some_func(component: &MyComponent, entity: &mut EntityMut)` and it must
///   be in scope. This function will be called when inserting into the entity.
///
/// # Field Attributes
///
/// * `#[proto_comp(preload(type = "Image"))]`
///
///   This will mark this field to be used during the asset preloading phase. The
///   given `type` must correspond to a valid asset type and it must be in scope.
///
///   Additionally, you can specify a `dest` with the name (or index) of the field
///   to insert the strong handle into. If no `dest` is set, this asset will be
///   loaded as a dependency of the prototype. Some examples:
///
///   - `#[proto_comp(preload(type = "Image", dest = "my_handle"))]`
///   - `#[proto_comp(preload(type = "Mesh", dest = "1"))]`
///
/// # Examples
///
/// ```ignore
/// #[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
/// #[reflect(ProtoComponent)]
/// struct SomeComponent {
///     some_string: String,
///     some_int: i32,
/// }
///
/// // Which generates:
/// //
/// // impl ::bevy_proto::prelude::ProtoComponent for SomeComponent {
/// //   fn apply(&self, entity: &mut bevy::ecs::world::EntityMut) {
/// //     entity.insert(self.clone());
/// //   }
/// //   fn as_reflect(&self) -> &dyn bevy::prelude::Reflect {
/// //     self
/// //   }
/// //   fn preload_assets(&mut self, preloader: &mut ::bevy_proto::prelude::AssetPreloader) {
/// //     use ::bevy_proto::prelude::StoreHandle;
/// //   }
/// // }
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
            ProtoCompAttr::IntoBundle(ty) => Some(quote! {
                // Convert then insert
                let bundle: #ty = self.clone().into();
                #entity_ident.insert_bundle(bundle);
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
