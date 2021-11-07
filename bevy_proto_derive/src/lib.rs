use proc_macro::{TokenStream, TokenTree};

use fields::ProtoCompDupeAttr;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::*;

mod constants;
mod fields;

/// Automatically implements [`ProtoComponent`](`bevy_proto::ProtoComponent`) for the given
/// struct. This works on all structs, including tuple and unit structs. Enums are not
/// currently supported.
///
/// **NOTE: [`serde::Serialize`] and [`serde::Deserialize`] must also be implemented/derived**
///
/// # Examples
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, ProtoComponent)]
/// struct SomeComponent {
/// 	// Optional: #[proto_comp(Clone)]
/// 	some_string: String,
///
/// 	#[proto_comp(Copy)]
/// 	some_int: i32,
/// }
/// ```
#[proc_macro_derive(ProtoComponent, attributes(proto_comp))]
pub fn proto_comp_derive(input: TokenStream) -> TokenStream {
	let item: Item = parse(input.clone()).expect("Could not parse input stream");
	let DeriveInput {
		ident, data, attrs, ..
	} = parse_macro_input!(input);

	let mut generator = match data {
		Data::Struct(data_struct) => proc_fields(data_struct.fields),
		_ => syn::Error::new(item.span(), "ProtoComp can only be applied on struct types")
			.to_compile_error(),
	};

	let output = quote! {
		#[typetag::serde]
		impl bevy_proto::ProtoComponent for #ident {
			fn insert_self(
				&self,
				commands: &mut bevy_proto::ProtoCommands,
				asset_server: &bevy::prelude::Res<bevy::prelude::AssetServer>,
			) {
				let component = #generator;
				commands.insert(component);
			}
		}
	};

	output.into()
}

/// Process all fields
///
/// # Arguments
///
/// * `fields`: The fields to process
///
/// returns: TokenStream
fn proc_fields(fields: Fields) -> TokenStream2 {
	match fields {
		Fields::Named(named) => {
			let inner = proc_named_fields(named);
			quote! {
				Self {
					#inner
				}
			}
		}
		Fields::Unnamed(unnamed) => {
			let inner = proc_unnnamed_fields(unnamed);
			quote! {
				Self (#inner);
			}
		}
		Fields::Unit => quote! {Self{}},
	}
}

/// Process all named fields
///
/// # Arguments
///
/// * `fields`: The fields to process
///
/// returns: TokenStream
fn proc_named_fields(fields: FieldsNamed) -> TokenStream2 {
	let field_stream = fields.named.iter().map(|field| {
		let dupe_type = fields::get_dupe_attr(&field);
		let Field { ident, .. } = field.clone();

		match dupe_type {
			Some(ProtoCompDupeAttr::AttrCopy) => quote! {
				#ident: self.#ident
			},
			Some(ProtoCompDupeAttr::AttrClone) | None => quote! {
				#ident: ::std::clone::Clone::clone(&self.#ident)
			},
		}
	});

	quote! {
		#(#field_stream),*
	}
}

/// Process all unnamed fields
///
/// # Arguments
///
/// * `fields`: The fields to process
///
/// returns: TokenStream
fn proc_unnnamed_fields(fields: FieldsUnnamed) -> TokenStream2 {
	let field_stream = fields.unnamed.iter().enumerate().map(|(index, field)| {
		let idx = Index::from(index);
		let dupe_type = fields::get_dupe_attr(&field);

		match dupe_type {
			Some(ProtoCompDupeAttr::AttrCopy) => quote! {
				self.#idx
			},
			_ => quote! {
				::std::clone::Clone::clone(&self.#idx)
			},
		}
	});

	quote! {
		#(#field_stream),*
	}
}
