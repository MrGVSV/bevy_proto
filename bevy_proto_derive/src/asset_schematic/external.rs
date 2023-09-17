use crate::asset_schematic::DeriveAssetSchematic;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

/// Definition struct for the [`impl_external_asset_schematic`] macro.
///
/// # Example
///
/// ```ignore
/// impl_external_asset_schematic!{
///   struct ExternalType {
///     #[schematic(asset)]
///     image: Handle<Image>
///   }
/// }
/// ```
///
/// [`impl_external_asset_schematic`]: crate::impl_external_asset_schematic
pub(crate) struct ExternalAssetSchematic {
    schematic_data: DeriveAssetSchematic,
    other_content: TokenStream,
}

impl Parse for ExternalAssetSchematic {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            schematic_data: input.parse::<DeriveAssetSchematic>()?,
            other_content: input.parse::<TokenStream>()?,
        })
    }
}

impl ToTokens for ExternalAssetSchematic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impls = &self.schematic_data;
        let other = &self.other_content;
        tokens.extend(quote! {
            #impls

            #other
        });
    }
}
