//! Helper constants used in generated macro output.
//!
//! The purpose of these constants is so that we can easily make use
//! of them throughout modules without having to pass them around as
//! references (since these should never change).

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

pub(crate) const SCHEMATIC_ATTR: &str = "schematic";
pub(crate) const ASSET_SCHEMATIC_ATTR: &str = "asset_schematic";

pub(crate) const ASSET_ATTR: &str = "asset";
pub(crate) const ENTITY_ATTR: &str = "entity";
pub(crate) const INPUT_ATTR: &str = "input";
pub(crate) const FROM_ATTR: &str = "from";
pub(crate) const INTO_ATTR: &str = "into";

/// Ident for the `Schematic::Input` argument.
pub(crate) const INPUT_IDENT: ConstIdent = ConstIdent("__input__");
/// Ident for the `SchematicId` argument.
pub(crate) const ID_IDENT: ConstIdent = ConstIdent("__id__");
/// Ident for the `SchematicContext` and `AssetSchematicContext` argument.
pub(crate) const CONTEXT_IDENT: ConstIdent = ConstIdent("__context__");
/// Ident for the `DependenciesBuilder` argument.
pub(crate) const DEPENDENCIES_IDENT: ConstIdent = ConstIdent("__context__");
/// Ident used for temporary variables (to prevent accidental collisions).
pub(crate) const TEMP_IDENT: ConstIdent = ConstIdent("__temp__");

/// Helper struct used to generate a const-like [`Ident`] with the given name.
#[derive(Copy, Clone, PartialEq)]
pub(crate) struct ConstIdent(pub &'static str);

impl ToTokens for ConstIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Ident::new(self.0, Span::call_site()).to_tokens(tokens)
    }
}
