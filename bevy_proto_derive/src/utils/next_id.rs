use crate::utils::constants::ID_IDENT;
use crate::utils::exports::Uuid as BevyUuid;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use uuid::Uuid;

/// Generate a compile-time-constant ID for use in `SchematicId::next` calls.
pub(crate) struct NextId;

impl ToTokens for NextId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let uuid = Uuid::new_v4().as_u128();
        tokens.extend(quote!(#ID_IDENT.next(#uuid)));
    }
}

/// Generate a runtime-random ID for use in `SchematicId::next` calls.
pub(crate) struct RandomId;

impl ToTokens for RandomId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote!(#ID_IDENT.next(#BevyUuid::new_v4())));
    }
}
