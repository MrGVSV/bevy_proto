use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::TypePath;

/// The output type of the `Schematic` or `AssetSchematic`.
#[derive(Default)]
pub(crate) enum OutputType {
    /// Outputs as `Self`.
    #[default]
    Reflexive,
    /// Outputs as the given type.
    Custom(TypePath),
}

impl ToTokens for OutputType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Reflexive => tokens.extend(quote!(Self)),
            Self::Custom(ty) => tokens.extend(quote!(#ty)),
        }
    }
}
