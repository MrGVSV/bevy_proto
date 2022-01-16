use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::*;

use crate::attributes::ProtoCompAttr;

mod attributes;
mod constants;

/// Automatically implements [`ProtoComponent`] for the given
/// struct or enum. This works on all structs and enums, including tuple and unit structs.
///
/// **NOTE: `Clone`, `serde::Serialize`, and `serde::Deserialize` must also be implemented/derived**
///
/// # Examples
///
/// ```
/// # use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Serialize, Deserialize, ProtoComponent)]
/// struct SomeComponent {
/// 	some_string: String,
/// 	some_int: i32,
/// }
///
/// // Which generates:
/// //
/// // #[typetag::serde]
/// // impl bevy_proto::ProtoComponent for #ident { ///
/// // 	fn insert_self(
/// //    &self,
/// //    commands: &mut bevy_proto::ProtoCommands,
/// // 	  asset_server: &bevy::prelude::Res<bevy::prelude::AssetServer>,
/// //  ) {
/// //      let component = self.clone();
/// //      commands.insert(component);
/// //    }
/// //  }
/// ```
#[proc_macro_derive(ProtoComponent, attributes(proto_comp))]
pub fn proto_comp_derive(input: TokenStream) -> TokenStream {
	let DeriveInput {
		ident, data, attrs, ..
	} = parse_macro_input!(input);

	let mut generator = None;
	for attr in attrs {
		let struct_attr: Result<ProtoCompAttr> = attr.parse_args();
		if let Ok(struct_attr) = struct_attr {
			generator = Some(quote! { #struct_attr });
			break;
		}
	}

	let generator = if let Some(generator) = generator {
		generator
	} else {
		match data {
			Data::Struct(..) | Data::Enum(..) => {
				quote! {
					let component = self.clone();
					commands.insert(component);
				}
			}
			_ => syn::Error::new(
				Span::call_site(),
				"ProtoComponent can only be applied on struct types",
			)
			.to_compile_error(),
		}
	};

	let output = quote! {
		#[typetag::serde]
		impl bevy_proto::prelude::ProtoComponent for #ident {
			fn insert_self(
				&self,
				commands: &mut bevy_proto::prelude::ProtoCommands,
				asset_server: &bevy::prelude::Res<bevy::prelude::AssetServer>,
			) {
				#generator;
			}
		}
	};

	output.into()
}
