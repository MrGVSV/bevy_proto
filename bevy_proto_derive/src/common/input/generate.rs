use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Generics};
use to_phantom::ToPhantom;

use crate::common::data::{SchematicData, SchematicVariant};
use crate::common::fields::{SchematicField, SchematicFields};
use crate::common::input::{InputType, SchematicIo};
use crate::utils::constants::{CONTEXT_IDENT, DEPENDENCIES_IDENT, ID_IDENT, INPUT_IDENT};
use crate::utils::exports::{
    DependenciesBuilder, FromSchematicInput, FromSchematicPreloadInput, Reflect, SchematicContext,
    SchematicId,
};

/// Generates the input type for the schematic.
pub(crate) fn generate_input(
    io: &SchematicIo,
    data: &SchematicData,
    generics: &Generics,
    no_preload: bool,
) -> Result<Option<TokenStream>, Error> {
    let input_ident = match io.input_ty() {
        InputType::Generated(ident) => ident,
        _ => return Ok(None),
    };

    let base_ident = io.ident();
    // Defaults to the visibility of the schematic to avoid "private type in public interface" errors
    let vis = io.input_vis().unwrap_or(io.vis());

    let (impl_generics, impl_ty_generics, where_clause) = generics.split_for_impl();

    // Ex: FooInput<T>
    let input_ty = quote!(#input_ident #impl_ty_generics);
    // Ex: FooInput<T: Bar>
    let input_ty_def = quote!(#input_ident #impl_generics);
    // Used to allow generics to be defined on the input type
    let phantom_ty = if generics.params.is_empty() {
        None
    } else {
        Some(generics.to_phantom())
    };

    let make_from_impl = |body: TokenStream| {
        quote! {
            impl #impl_generics #FromSchematicInput<#input_ty> for #base_ident #impl_ty_generics #where_clause {
                fn from_input(#INPUT_IDENT: #input_ty, #ID_IDENT: #SchematicId, #CONTEXT_IDENT: &mut #SchematicContext) -> Self {
                    #body
                }
            }
        }
    };

    let make_from_preload_impl = |body: TokenStream| {
        if no_preload {
            None
        } else {
            Some(quote! {
                impl #impl_generics #FromSchematicPreloadInput<#input_ty> for #base_ident #impl_ty_generics #where_clause {
                    fn from_preload_input(#INPUT_IDENT: #input_ty, #ID_IDENT: #SchematicId, #DEPENDENCIES_IDENT: &mut #DependenciesBuilder) -> Self {
                        #body
                    }
                }
            })
        }
    };

    Ok(match data {
        SchematicData::Struct(SchematicFields::Unit) => {
            let from_impl = make_from_impl(quote!(Self));
            let from_preload_impl = make_from_preload_impl(quote!(Self));

            Some(quote! {
                #[derive(#Reflect)]
                #vis struct #input_ty_def #where_clause;

                #from_impl

                #from_preload_impl
            })
        }
        SchematicData::Struct(SchematicFields::Unnamed(fields)) => {
            let definitions = fields
                .iter()
                .filter(SchematicField::requires_input_field)
                .map(|field| field.generate_definition());

            let conversions = fields
                .iter()
                .map(|field| field.generate_conversion(None))
                .collect::<Result<Vec<_>, Error>>()?;

            let from_impl = make_from_impl(quote! {
                Self(
                    #(#conversions),*
                )
            });

            let preload_conversions = fields
                .iter()
                .map(|field| field.generate_preload_conversion(None))
                .collect::<Result<Vec<_>, Error>>()?;

            let from_preload_impl = make_from_preload_impl(quote! {
                Self(
                    #(#preload_conversions),*
                )
            });

            let phantom_ty = phantom_ty.map(|phantom_ty| {
                quote! {
                    #[reflect(ignore)]
                    #phantom_ty,
                }
            });

            Some(quote! {
                #[derive(#Reflect)]
                #vis struct #input_ty_def (
                    #(#definitions,)*
                    #phantom_ty
                ) #where_clause;

                #from_impl

                #from_preload_impl
            })
        }
        SchematicData::Struct(SchematicFields::Named(fields)) => {
            let definitions = fields
                .iter()
                .filter(SchematicField::requires_input_field)
                .map(|field| field.generate_definition());

            let members = fields
                .iter()
                .map(SchematicField::member)
                .collect::<Vec<_>>();

            let conversions = fields
                .iter()
                .map(|field| field.generate_conversion(None))
                .collect::<Result<Vec<_>, Error>>()?;

            let from_impl = make_from_impl(quote! {
                Self {
                    #(#members: #conversions),*
                }
            });

            let preload_conversions = fields
                .iter()
                .map(|field| field.generate_preload_conversion(None))
                .collect::<Result<Vec<_>, Error>>()?;

            let from_preload_impl = make_from_preload_impl(quote! {
                Self {
                    #(#members: #preload_conversions),*
                }
            });

            let phantom_ty = phantom_ty.map(|phantom_ty| {
                quote! {
                    #[reflect(ignore)]
                    __phantom_ty__: #phantom_ty,
                }
            });

            Some(quote! {
                #[derive(#Reflect)]
                #vis struct #input_ty_def #where_clause {
                    #(#definitions,)*
                    #phantom_ty
                }

                #from_impl

                #from_preload_impl
            })
        }
        SchematicData::Enum(variants) => {
            let definitions = variants.iter().map(SchematicVariant::generate_definition);

            let conversions = variants
                .iter()
                .map(|variant| variant.generate_conversion_arm(io))
                .collect::<Result<Vec<_>, Error>>()?;

            let from_impl = make_from_impl(quote! {
                match #INPUT_IDENT {
                    #(#conversions,)*
                    _ => unreachable!(),
                }
            });

            let preload_conversions = variants
                .iter()
                .map(|variant| variant.generate_preload_conversion_arm(io))
                .collect::<Result<Vec<_>, Error>>()?;

            let from_preload_impl = make_from_preload_impl(quote! {
                match #INPUT_IDENT {
                    #(#preload_conversions,)*
                    _ => unreachable!(),
                }
            });

            let phantom_ty = phantom_ty.map(|phantom_ty| {
                quote! {
                    _Phantom(#[reflect(ignore)] #phantom_ty),
                }
            });

            Some(quote! {
                #[derive(#Reflect)]
                #vis enum #input_ty_def #where_clause {
                    #(#definitions,)*
                    #phantom_ty
                }

                #from_impl

                #from_preload_impl
            })
        }
    })
}
