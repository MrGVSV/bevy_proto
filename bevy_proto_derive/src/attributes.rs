use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Error, LitStr, Path, Result, Token};

use crate::constants::{INTO_IDENT, WITH_IDENT};

/// ProtoComponent attributes applied on structs
pub(crate) enum ProtoCompAttr {
	/// Captures the `#[proto_comp(into = "ActualComponent")]` attribute
	///
	/// This is used to specify a separate Component that this marked struct will be cloned into.
	///
	/// Generates the following code:
	/// ```rust
	/// let component: ActualComponent = self.clone().into();
	/// commands.insert(component);
	/// ```
	Into(Ident),
	/// Captures the `#[proto_comp(with = "my_function")]` attribute
	///
	/// This is used to specify a custom function with which custom Components will be creatde and/or inserted.
	/// This is essentially identical to just simply implementing `ProtoComponent` yourself.
	///
	/// Generates the following code:
	/// ```rust
	/// my_function(self, commands, asset_server);
	/// ```
	With(Ident),
}

impl Parse for ProtoCompAttr {
	fn parse(input: ParseStream) -> Result<Self> {
		let path: Path = input.parse()?;
		let _: Token![=] = input.parse()?;
		let item: LitStr = input.parse()?;
		let ident = format_ident!("{}", item.value());

		if path == WITH_IDENT {
			Ok(Self::With(ident))
		} else if path == INTO_IDENT {
			Ok(Self::Into(ident))
		} else {
			Err(Error::new(Span::call_site(), "Unexpected path"))
		}
	}
}

impl ToTokens for ProtoCompAttr {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Self::Into(ident) => {
				let into_ident = quote! {
					let cloned = self.clone();
					let component: #ident = cloned.into();
					commands.insert(component);
				};
				into_ident.to_tokens(tokens);
			}
			Self::With(ident) => {
				let with_ident = quote! {
					#ident(self, commands, asset_server);
				};
				with_ident.to_tokens(tokens);
			}
		}
	}
}
