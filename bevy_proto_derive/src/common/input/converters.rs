use crate::common::input::{InputType, OutputType, SchematicIo};
use crate::utils::constants::{CONTEXT_IDENT, DEPENDENCIES_IDENT, INPUT_IDENT};
use crate::utils::exports::{FromReflect, FromSchematicInput, FromSchematicPreloadInput, Reflect};
use crate::utils::NextId;
use proc_macro2::TokenStream;
use quote::quote;

/// Generates a statement that sets [`INPUT_IDENT`] to the output type using `FromSchematicInput`.
///
/// Returns `None` if no conversion is necessary.
pub(crate) fn generate_input_conversion(io: &SchematicIo) -> Option<TokenStream> {
    let input_ty = io.input_ty();
    let output_ty = io.output_ty();

    match (input_ty, output_ty) {
        (InputType::Reflexive, OutputType::Reflexive) => {
            // === No Conversion Necessary === //
            None
        }
        (_, OutputType::Custom(output_ty)) => {
            // === Input -> Self -> Output === //
            Some(quote! {
                let #INPUT_IDENT = <Self as #FromSchematicInput<Self::Input>>::from_input(
                    #INPUT_IDENT, #NextId, #CONTEXT_IDENT
                );
                let #INPUT_IDENT = <#output_ty as #FromSchematicInput<Self>>::from_input(
                    #INPUT_IDENT, #NextId, #CONTEXT_IDENT
                );
            })
        }
        _ => {
            // === Input -> Self === //
            Some(quote! {
                let #INPUT_IDENT = <Self as #FromSchematicInput<Self::Input>>::from_input(
                    #INPUT_IDENT, #NextId, #CONTEXT_IDENT
                );
            })
        }
    }
}

/// Generates a statement that sets [`INPUT_IDENT`] to the output type using `FromSchematicPreloadInput`.
///
/// Returns `None` if no conversion is necessary.
pub(crate) fn generate_preload_input_conversion(io: &SchematicIo) -> Option<TokenStream> {
    let input_ty = io.input_ty();
    let output_ty = io.output_ty();

    match (input_ty, output_ty) {
        (InputType::Reflexive, OutputType::Reflexive) => {
            // === No Conversion Necessary === //
            None
        }
        (_, OutputType::Custom(output_ty)) => {
            // === Input -> Self -> Output === //
            Some(quote! {
                let #INPUT_IDENT = <Self as #FromSchematicPreloadInput<Self::Input>>::from_preload_input(
                    #INPUT_IDENT, #NextId, #DEPENDENCIES_IDENT
                );
                let #INPUT_IDENT = <#output_ty as #FromSchematicPreloadInput<Self>>::from_preload_input(
                    #INPUT_IDENT, #NextId, #DEPENDENCIES_IDENT
                );
            })
        }
        _ => {
            // === Input -> Self === //
            Some(quote! {
                let #INPUT_IDENT = <Self as #FromSchematicPreloadInput<Self::Input>>::from_preload_input(
                    #INPUT_IDENT, #NextId, #DEPENDENCIES_IDENT
                );
            })
        }
    }
}

/// Generates a statement that sets [`INPUT_IDENT`] to a cloned instance of its
/// concrete type using `FromReflect`.
pub(crate) fn generate_from_reflect_conversion() -> TokenStream {
    quote! {
        let #INPUT_IDENT = <Self::Input as #FromReflect>::from_reflect(
            &*#Reflect::clone_value(#INPUT_IDENT)
        ).unwrap_or_else(|| {
            panic!(
                "{} should have a functioning `FromReflect` impl",
                std::any::type_name::<Self::Input>()
            )
        });
    }
}
