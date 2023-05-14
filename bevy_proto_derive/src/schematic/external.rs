use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::schematic::DeriveSchematic;

/// Definition struct for the [`impl_external_schematic`] macro.
///
/// # Example
///
/// ```ignore
/// impl_external_schematic!{
///   struct ExternalType {
///     #[schematic(asset)]
///     image: Handle<Image>
///   }
/// }
/// ```
///
/// [`impl_external_schematic`]: crate::impl_external_schematic!
pub(crate) struct ExternalSchematic {
    schematic_data: DeriveSchematic,
    other_content: TokenStream,
}

impl Parse for ExternalSchematic {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            schematic_data: input.parse::<DeriveSchematic>()?,
            other_content: input.parse::<TokenStream>()?,
        })
    }
}

impl ToTokens for ExternalSchematic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impls = &self.schematic_data;
        let other = &self.other_content;
        tokens.extend(quote! {
            #impls

            #other
        });
    }
}
