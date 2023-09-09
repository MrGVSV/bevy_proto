use std::fmt::{Debug, Formatter};

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Error, GenericArgument, LitStr, PathArguments, Type};

use crate::utils::constants::ASSET_ATTR;
use crate::utils::{debug_attribute, NextId, RandomId};
use crate::utils::{define_attribute, AttrArg, AttrArgValue, AttrTarget};

define_attribute!("preload" => AssetPreloadArg(bool) for AttrTarget::Asset);
define_attribute!("inline" => AssetInlineArg(bool) for AttrTarget::Asset);
define_attribute!("untyped" => AssetUntypedArg(bool) for AttrTarget::Asset);
define_attribute!("unique" => AssetUniqueArg(bool) for AttrTarget::Asset);
define_attribute!("type" => AssetTypeArg(Type) for AttrTarget::Asset);
define_attribute!("path" => AssetPathArg(LitStr) for AttrTarget::Asset);

/// Specifies the configuration for asset-containing fields.
#[derive(Default)]
pub(crate) struct AssetConfig {
    /// Used to specify whether the asset should be preloaded.
    ///
    /// Form: `#[asset_schematic(asset(preload))]`.
    preload: AssetPreloadArg,
    /// Used to specify whether the asset should result in a new asset handle being created.
    ///
    /// Form: `#[asset_schematic(asset(unique))]`.
    unique: AssetUniqueArg,
    /// Used to specify whether the asset should be allowed to be defined inlined.
    ///
    /// Form: `#[asset_schematic(asset(inline))]`.
    inline: AssetInlineArg,
    /// Used to specify a static path to an asset.
    ///
    /// Form: `#[asset_schematic(asset(path = "path/to/asset.png"))]`.
    path: AssetPathArg,
    /// Used to set the type of the asset.
    ///
    /// Form: `#[asset_schematic(asset(type = path::to::Foo))]`.
    custom_type: AssetTypeArg,
    /// Used to specify whether the asset should be defined as a `HandleUntyped`.
    ///
    /// Form: `#[asset_schematic(asset(untyped))]`.
    untyped: AssetUntypedArg,
}

impl AssetConfig {
    pub fn preload(&self) -> bool {
        self.preload.get().copied().unwrap_or_default()
    }

    pub fn try_set_preload(&mut self, value: bool, span: Span) -> Result<(), Error> {
        if self.unique() {
            return Err(self.preload.invariant_error::<AssetUniqueArg>(span));
        }

        self.preload.try_set(Some(value), span)
    }

    pub fn unique(&self) -> bool {
        self.unique.get().copied().unwrap_or_default()
    }

    pub fn try_set_unique(&mut self, value: bool, span: Span) -> Result<(), Error> {
        if self.preload() {
            return Err(self.unique.invariant_error::<AssetPreloadArg>(span));
        }

        self.unique.try_set(Some(value), span)
    }

    pub fn inline(&self) -> bool {
        self.inline.get().copied().unwrap_or_default()
    }

    pub fn try_set_inline(&mut self, value: bool, span: Span) -> Result<(), Error> {
        if self.path().is_some() {
            return Err(self.inline.invariant_error::<AssetPathArg>(span));
        }

        self.inline.try_set(Some(value), span)
    }

    pub fn path(&self) -> Option<&LitStr> {
        self.path.get()
    }

    pub fn try_set_path(&mut self, value: LitStr, span: Span) -> Result<(), Error> {
        if self.inline() {
            return Err(self.inline.invariant_error::<AssetInlineArg>(span));
        }

        self.path.try_set(Some(value), span)
    }

    pub fn custom_type(&self) -> Option<&Type> {
        self.custom_type.get()
    }

    pub fn try_set_custom_type(&mut self, value: Type, span: Span) -> Result<(), Error> {
        if self.untyped() {
            return Err(self.custom_type.invariant_error::<AssetUntypedArg>(span));
        }

        self.custom_type.try_set(Some(value), span)
    }

    pub fn untyped(&self) -> bool {
        self.untyped.get().copied().unwrap_or_default()
    }

    // TODO: Handle untyped assets
    // pub fn try_set_untyped(&mut self, value: bool, span: Span) -> Result<(), Error> {
    //     if self.custom_type().is_some() {
    //         return Err(self.untyped.invariant_error::<AssetTypeArg>(span));
    //     }
    //
    //     self.untyped.try_set(Some(value), span)
    // }

    /// Create a token generator that creates schematic IDs.
    ///
    /// This will choose between a random or stable ID based on the asset's `unique` attribute.
    pub fn asset_id(&self) -> AssetIdGenerator {
        AssetIdGenerator {
            random: self.unique(),
        }
    }

    pub fn try_extract_asset_type<'a>(&'a self, defined_ty: &'a Type) -> Result<&'a Type, Error> {
        if let Some(ty) = self.custom_type() {
            Ok(ty)
        } else {
            AssetConfig::extract_asset_type(defined_ty)
        }
    }

    /// Attempts to extract the asset type from either a `Handle` or an `Option<Handle>`.
    fn extract_asset_type(ty: &Type) -> Result<&Type, Error> {
        let create_error = || {
            Error::new(ty.span(), format_args!(
                "could not automatically extract asset type: please specify it manually using `{}({} = path::to::YourAssetType)`",
                ASSET_ATTR,
                AssetTypeArg::NAME
            ))
        };

        match ty {
            Type::Path(type_path) if type_path.qself.is_none() => {
                if let Some(segment) = type_path.path.segments.last() {
                    let PathArguments::AngleBracketed(args) = &segment.arguments else {
                        return Err(create_error());
                    };
                    let Some(GenericArgument::Type(ty)) = args.args.first() else {
                        return Err(create_error());
                    };

                    if segment.ident == "Handle" {
                        return Ok(ty);
                    } else if segment.ident == "Option" {
                        return Self::extract_asset_type(ty);
                    }
                }
            }
            _ => {}
        }

        Err(create_error())
    }
}

impl Debug for AssetConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        debug_attribute(f, |write| {
            write(format_args!("{:?}", self.preload))?;
            write(format_args!("{:?}", self.inline))?;
            write(format_args!("{:?}", self.untyped))?;
            write(format_args!("{:?}", self.custom_type))?;
            write(format_args!("{:?}", self.path))?;

            Ok(())
        })?;

        write!(f, ")")
    }
}

pub(crate) struct AssetIdGenerator {
    random: bool,
}

impl ToTokens for AssetIdGenerator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.random {
            RandomId.to_tokens(tokens);
        } else {
            NextId.to_tokens(tokens);
        }
    }
}
