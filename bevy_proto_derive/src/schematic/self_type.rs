use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::TypePath;

/// The type of the `Schematic`.
#[derive(Default)]
pub(crate) enum SelfType {
    /// Corresponds to using `Self`.
    #[default]
    Reflexive,
    /// Specifies that the input type is `Self` but must be converted to the given type.
    Into(TypePath),
}

impl SelfType {
    /// Returns the "true" type of the schematic (i.e. `Self`).
    pub fn get_true_self(&self) -> TokenStream {
        quote!(Self)
    }

    /// Returns the "into" type of the schematic.
    ///
    /// This is the type actually being applied to the entity/world.
    pub fn get_into_self(&self) -> TokenStream {
        match self {
            Self::Reflexive => self.get_true_self(),
            Self::Into(ty) => ty.to_token_stream(),
        }
    }

    /// Compile-time assertions, if any.
    ///
    /// These are generated within an anonymous context and should either:
    /// 1. Enforce invariants at runtime
    /// 2. Provide clearer error outputs for users
    pub fn assertions(&self) -> Option<TokenStream> {
        None
    }
}

impl ToTokens for SelfType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.get_into_self());
    }
}
