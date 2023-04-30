//! Helper [idents] used in generated macro output.
//!
//! The purpose of these idents is to use their const-likeness so that we
//! can easily make use of them throughout modules without having to pass
//! them around as references (since these should never change).
//!
//! [idents]: Ident

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

/// Ident for the `Schematic::Input` argument.
pub(crate) const INPUT_IDENT: ConstIdent = ConstIdent("__input__");
/// Ident for the `SchematicContext` argument.
pub(crate) const CONTEXT_IDENT: ConstIdent = ConstIdent("__context__");
/// Ident for the `DependenciesBuilder` argument.
pub(crate) const DEPENDENCIES_IDENT: ConstIdent = ConstIdent("__dependencies__");

/// Helper struct used to generate a const-like [`Ident`] with the given name.
#[derive(Copy, Clone, PartialEq)]
pub(crate) struct ConstIdent(&'static str);

impl ToTokens for ConstIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Ident::new(self.0, Span::call_site()).to_tokens(tokens)
    }
}
