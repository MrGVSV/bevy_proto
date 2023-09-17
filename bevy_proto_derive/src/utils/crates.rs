use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, Error, FoundCrate};
use std::sync::OnceLock;

static PROTO_CRATE: OnceLock<String> = OnceLock::new();
static BEVY_CRATE: OnceLock<String> = OnceLock::new();

/// Returns a path to either the `bevy_proto_backend` crate or `bevy_proto::backend` module.
pub(crate) fn get_proto_crate() -> TokenStream {
    PROTO_CRATE
        .get_or_init(|| match crate_name("bevy_proto_backend") {
            Ok(found_crate) => get_crate_path(found_crate),
            Err(Error::CrateNotFound { .. }) => {
                let path =
                    get_crate_path(crate_name("bevy_proto").expect(
                        "bevy_proto or bevy_proto_backend should be present in `Cargo.toml`",
                    ));
                format!("{}::backend", path)
            }
            Err(error) => panic!("{}", error),
        })
        .parse()
        .expect("`get_proto_crate` should always return a valid `TokenStream`")
}

/// Returns a path to the `bevy` crate.
pub(crate) fn get_bevy_crate() -> TokenStream {
    BEVY_CRATE
        .get_or_init(|| {
            get_crate_path(crate_name("bevy").expect("bevy is present in `Cargo.toml`"))
        })
        .parse()
        .expect("`get_bevy_crate` should always return a valid `TokenStream`")
}

fn get_crate_path(found_crate: FoundCrate) -> String {
    match found_crate {
        FoundCrate::Itself => String::from("crate"),
        FoundCrate::Name(name) => {
            format!("::{}", name)
        }
    }
}
