use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

/// Returns a path to the `bevy_proto_backend` crate.
pub(crate) fn get_proto_crate() -> TokenStream {
    get_crate_path(
        crate_name("bevy_proto_backend").expect("bevy_proto_backend is present in `Cargo.toml`"),
    )
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
            quote!( #ident )
        }
    }
}
