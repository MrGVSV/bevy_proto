use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, ToTokens};
use syn::{Token, TypePath};

/// The type of the `Schematic::Input`.
#[derive(Default)]
pub(crate) enum InputType {
    /// Corresponds to using `Self`.
    #[default]
    Reflexive,
    /// Specifies an existing type.
    ///
    /// Most often this will be a user-generated type made to be used in attribute arguments
    /// like `from = path::to::Type`.
    Existing(TypePath),
    /// Indicates that a new input type will need to be generated with the given identifier.
    Generated(Ident),
}

impl InputType {
    /// Creates a new [`InputType::Generated`] based on the given identifier.
    ///
    /// The new identifier will be suffixed with `Input`
    /// (e.g. `Foo` becomes `FooInput`).
    pub fn new_generated(ident: &Ident) -> Self {
        Self::Generated(format_ident!("{}Input", ident))
    }
}

impl ToTokens for InputType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            InputType::Reflexive => Token![Self](Span::call_site()).to_tokens(tokens),
            InputType::Existing(path) => path.to_tokens(tokens),
            InputType::Generated(ident) => ident.to_tokens(tokens),
        }
    }
}
