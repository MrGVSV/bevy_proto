use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, Error, FoundCrate};
use quote::quote;

/// Returns a path to either the `bevy_proto_backend` crate or `bevy_proto::backend` module.
pub(crate) fn get_proto_crate() -> TokenStream {
    match crate_name("bevy_proto_backend") {
        Ok(found_crate) => get_crate_path(found_crate),
        Err(Error::CrateNotFound { .. }) => {
            let path = get_crate_path(
                crate_name("bevy_proto")
                    .expect("bevy_proto or bevy_proto_backend should be present in `Cargo.toml`"),
            );
            quote!(#path::backend)
        }
        Err(error) => panic!("{}", error),
    }
}

/// Returns a path to the `bevy` crate.
pub(crate) fn get_bevy_crate() -> TokenStream {
    get_crate_path(crate_name("bevy").expect("bevy is present in `Cargo.toml`"))
}

fn get_crate_path(found_crate: FoundCrate) -> TokenStream {
    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( ::#ident )
        }
    }
}
