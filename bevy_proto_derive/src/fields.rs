//! This module provides some helper functions for processing field data,
//! namely their attributes

use proc_macro::{TokenStream, TokenTree};
use std::iter::Peekable;
use std::slice::Iter;

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::*;

use crate::constants::{ATTR_IDENT, CLONE_IDENT, COPY_IDENT};

/// Enum used to specify how an item's data should be duplicated
#[derive(Debug)]
pub(crate) enum ProtoCompDupeAttr {
	/// Specifies the tagged item should be duplicated via Copy
	AttrCopy,
	/// Specifies the tagged item should be duplicated via Clone
	AttrClone,
	// TODO: Allow custom duplication method:
	//   AttrCustom(Duplicator),
}

impl ProtoCompDupeAttr {
	/// Attempts to parse some metadata into a [`ProtoCompDupeAttr`]
	///
	/// # Arguments
	///
	/// * `meta`: The meta to parse
	///
	/// returns: Option<ProtoCompDupeAttr>
	fn from_meta(meta: &Meta) -> Option<Self> {
		let path = meta.path();
		if path == COPY_IDENT {
			Some(ProtoCompDupeAttr::AttrCopy)
		} else if path == CLONE_IDENT {
			Some(ProtoCompDupeAttr::AttrClone)
		} else {
			None
		}
	}
}

impl Default for ProtoCompDupeAttr {
	fn default() -> Self {
		Self::AttrClone
	}
}

/// For the given field, attempt to find an attribute determining how its
/// data should be duplicated (e.g. Copy, Clone, etc.).
///
/// Only the first valid attribute will be used
///
/// # Arguments
///
/// * `field`: The field to inspect
///
/// returns: Option<ProtoCompDupeAttr>
pub(crate) fn get_dupe_attr(field: &Field) -> Option<ProtoCompDupeAttr> {
	let mut dupe: Option<ProtoCompDupeAttr> = None;
	let mut iter: Peekable<Iter<Attribute>> = field.attrs.iter().peekable();

	/// Try to find a valid attribute specifying the duplication method
	/// This loop checks each attribute and finds the first one to meet
	/// that conditiion.
	while dupe.is_none() && iter.peek().is_some() {
		let meta = find_attr_meta(&mut iter)?;

		dupe = match meta {
			Meta::List(ref list) => {
				let nested = list.nested.first().unwrap();

				match nested {
					NestedMeta::Meta(meta) => ProtoCompDupeAttr::from_meta(meta),
					_ => None,
				}
			}
			_ => None,
		};
	}

	dupe
}

/// Tries to find an attribute with path: [`ATTR_NAME`]
///
/// # Arguments
///
/// * `attrs`: The list of attributes for this item
///
/// returns: Option<&Attribute>
pub(crate) fn find_attr<'a>(attrs: &'a mut Peekable<Iter<Attribute>>) -> Option<&'a Attribute> {
	attrs.find(|attr| attr.path == ATTR_IDENT)
}

/// Tries to find an attribute with path: [`ATTR_NAME`], and returns its [`Meta`] if found
///
/// # Arguments
///
/// * `attrs`: The list of attributes for this item
///
/// returns: Option<Meta>
fn find_attr_meta(attrs: &mut Peekable<Iter<Attribute>>) -> Option<Meta> {
	find_attr(attrs).and_then(|attr| attr.parse_meta().ok())
}
