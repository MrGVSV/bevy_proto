use std::fmt::{Debug, Formatter};

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::meta::ParseNestedMeta;
use syn::spanned::Spanned;
use syn::{parse_quote, Attribute, Error, Field, LitStr, Type};

use crate::schematic::input::InputType;
use crate::schematic::ATTRIBUTE;

const ASSET_ATTR: &str = "asset";
const ASSET_LAZY: &str = "lazy";
const ASSET_PRELOAD: &str = "preload";
const ASSET_PATH: &str = "path";

const ENTITY_ATTR: &str = "entity";
const ENTITY_PATH: &str = "path";

const FROM_ATTR: &str = "from";

/// Specifies how to replace a field in a generated `Schematic::Input` type.
#[derive(Default)]
pub(crate) enum ReplacementType {
    /// No replacement needed.
    #[default]
    None,
    /// This field contains a handle to an asset.
    Asset(AssetConfig),
    /// This field contains a reference to another entity.
    Entity(EntityConfig),
    From(Type),
}

impl ReplacementType {
    /// Attempts this [`ReplacementType`].
    ///
    /// Returns `Ok(())` if the change is valid.
    /// Returns `Err(error)` if the change is not valid, such as when trying
    /// to convert to a new `ReplacementType` when already designated as a different one.
    pub fn try_set(&mut self, value: Self, span: Span) -> Result<(), Error> {
        match (&self, value) {
            (Self::None, value) => {
                *self = value;
                Ok(())
            }
            (Self::Entity(EntityConfig::Undefined), value @ Self::Entity(_)) => {
                *self = value;
                Ok(())
            }
            (current, _) => Err(Error::new(
                span,
                format_args!("field already configured as {:?}", current),
            )),
        }
    }

    /// Returns a generated [`Type`] to be used as a field's replacement
    /// type for a generated `Schematic::Input`.
    ///
    /// Returns `None` for [`ReplacementType::None`], denoting that the field does not require
    /// a replacement type.
    pub fn generate_type(&self, proto_crate: &TokenStream) -> Option<Type> {
        match self {
            ReplacementType::None => None,
            ReplacementType::Asset(_) => Some(parse_quote!(#proto_crate::proto::ProtoAsset)),
            ReplacementType::Entity(_) => Some(parse_quote!(#proto_crate::tree::EntityAccess)),
            ReplacementType::From(ty) => Some(ty.clone()),
        }
    }
}

impl Debug for ReplacementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplacementType::None => Ok(()),
            ReplacementType::Asset(config) => write!(f, "#[schematic(asset{:?})]", config),
            ReplacementType::Entity(config) => write!(f, "#[schematic(entity{:?})]", config),
            ReplacementType::From(ty) => write!(f, "#[schematic(from = {})]", ty.to_token_stream()),
        }
    }
}

/// Specifies the configuration for asset-containing fields.
#[derive(Default)]
pub(crate) struct AssetConfig {
    path: Option<LitStr>,
    load_type: AssetLoadType,
}

impl AssetConfig {
    pub fn path(&self) -> Option<&LitStr> {
        self.path.as_ref()
    }

    /// Returns true if the asset should be preloaded.
    pub fn is_preload(&self) -> bool {
        matches!(self.load_type, AssetLoadType::Preload)
    }

    pub fn try_set_path(&mut self, path: LitStr, span: Span) -> Result<(), Error> {
        if self.path.is_some() {
            return Err(self.get_error(span));
        }

        self.path = Some(path);

        Ok(())
    }

    pub fn try_set_load_type(&mut self, load_type: AssetLoadType, span: Span) -> Result<(), Error> {
        if !matches!(self.load_type, AssetLoadType::Undefined) {
            return Err(self.get_error(span));
        }

        self.load_type = load_type;

        Ok(())
    }

    fn get_error(&self, span: Span) -> Error {
        Error::new(
            span,
            format_args!("field already configured as #[schematic(asset{:?})]", self),
        )
    }
}

impl Debug for AssetConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.load_type, &self.path) {
            (AssetLoadType::Undefined, None) => Ok(()),
            (load_type, None) => write!(f, "({:?})", load_type),
            (AssetLoadType::Undefined, Some(path)) => {
                write!(f, "(path = {:?})", path.value())
            }
            (load_type, Some(path)) => {
                write!(f, "({:?}, path = {:?})", load_type, path.value())
            }
        }
    }
}

#[derive(Default)]
pub(crate) enum AssetLoadType {
    #[default]
    Undefined,
    Preload,
    Lazy,
}

impl Debug for AssetLoadType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined => Ok(()),
            Self::Preload => write!(f, "preload"),
            Self::Lazy => write!(f, "lazy"),
        }
    }
}

#[derive(Default)]
pub(crate) enum EntityConfig {
    #[default]
    Undefined,
    Path(LitStr),
}

impl Debug for EntityConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityConfig::Undefined => Ok(()),
            EntityConfig::Path(path) => write!(f, "(path = {:?})", path.value()),
        }
    }
}

#[derive(Default)]
pub(crate) struct FieldAttributes {
    reflect_attrs: Vec<Attribute>,
    replacement_ty: ReplacementType,
}

impl FieldAttributes {
    /// Create a [`FieldAttributes`] for the given field.
    ///
    /// # Arguments
    ///
    /// * `field`: The field to process
    /// * `ident`: The [`Ident`] of the `Schematic`
    /// * `input_ty`: A mutable reference to the [`InputType`] of the `Schematic`
    ///
    pub fn new(field: &Field, ident: &Ident, input_ty: &mut InputType) -> Result<Self, Error> {
        let mut data = FieldAttributesBuilder {
            attrs: Self::default(),
            ident,
            input_ty,
        };

        for attr in &field.attrs {
            if attr.path().is_ident("reflect") {
                data.attrs.reflect_attrs.push(attr.clone());
            }

            if !attr.path().is_ident(ATTRIBUTE) {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                let ident = meta
                    .path
                    .get_ident()
                    .ok_or_else(|| Error::new(meta.path.span(), "unsupported argument"))?
                    .to_string();

                match (ident.as_str(), &data.input_ty) {
                    (attr @ (ASSET_ATTR | ENTITY_ATTR | FROM_ATTR), InputType::Existing(ty)) => {
                        Err(meta.error(format_args!(
                            "cannot use #[schematic({})] when input type is already defined as {}",
                            attr,
                            ty.to_token_stream()
                        )))
                    }
                    (ASSET_ATTR, _) => data.parse_asset_meta(meta),
                    (ENTITY_ATTR, _) => data.parse_entity_meta(meta),
                    (FROM_ATTR, _) => data.parse_from_meta(meta),
                    (_, _) => Err(meta.error(format_args!(
                        "unsupported argument, expected one of: {:?}",
                        [ASSET_ATTR, ENTITY_ATTR, FROM_ATTR]
                    ))),
                }
            })?;
        }

        Ok(data.attrs)
    }

    /// The field's [`ReplacementType`].
    pub fn replacement_ty(&self) -> &ReplacementType {
        &self.replacement_ty
    }

    /// Reflection attributes to pass to the generated input.
    pub fn reflect_attrs(&self) -> &[Attribute] {
        &self.reflect_attrs
    }

    /// Compile-time assertions, if any.
    ///
    /// These are generated within an anonymous context and should either:
    /// 1. Enforce invariants at runtime
    /// 2. Provide clearer error outputs for users
    pub fn assertions(&self) -> Option<TokenStream> {
        None
    }
}

struct FieldAttributesBuilder<'a> {
    attrs: FieldAttributes,
    ident: &'a Ident,
    input_ty: &'a mut InputType,
}

impl<'a> FieldAttributesBuilder<'a> {
    /// Parse a `#[schematic(asset)]` attribute.
    ///
    /// This takes in the meta starting at `asset`.
    fn parse_asset_meta(&mut self, meta: ParseNestedMeta) -> Result<(), Error> {
        self.require_input(meta.path.span())?;

        let config = if let ReplacementType::Asset(config) = &mut self.attrs.replacement_ty {
            config
        } else {
            self.attrs.replacement_ty.try_set(
                ReplacementType::Asset(AssetConfig::default()),
                meta.path.span(),
            )?;
            let ReplacementType::Asset(config) = &mut self.attrs.replacement_ty else {unreachable!()};
            config
        };

        if meta.input.is_empty() {
            return Ok(());
        }

        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident(ASSET_LAZY) {
                config.try_set_load_type(AssetLoadType::Lazy, meta.input.span())
            } else if meta.path.is_ident(ASSET_PRELOAD) {
                config.try_set_load_type(AssetLoadType::Preload, meta.input.span())
            } else if meta.path.is_ident(ASSET_PATH) {
                config.try_set_path(meta.value()?.parse()?, meta.input.span())
            } else {
                Err(meta.error(format_args!(
                    "unsupported argument, expected one of: {:?}",
                    [ASSET_LAZY, ASSET_PRELOAD, ASSET_PATH]
                )))
            }
        })
    }

    /// Parse a `#[schematic(entity)]` attribute.
    ///
    /// This takes in the meta starting at `entity`.
    fn parse_entity_meta(&mut self, meta: ParseNestedMeta) -> Result<(), Error> {
        self.attrs.replacement_ty.try_set(
            ReplacementType::Entity(EntityConfig::Undefined),
            meta.path.span(),
        )?;
        self.require_input(meta.path.span())?;

        if meta.input.is_empty() {
            return Ok(());
        }

        meta.parse_nested_meta(|meta| {
            let config = if meta.path.is_ident(ENTITY_PATH) {
                EntityConfig::Path(meta.value()?.parse()?)
            } else {
                return Err(meta.error(format_args!(
                    "unsupported argument, expected one of: {:?}",
                    [ENTITY_PATH]
                )));
            };

            self.attrs
                .replacement_ty
                .try_set(ReplacementType::Entity(config), meta.input.span())
        })
    }

    /// Parse a `#[schematic(from)]` attribute.
    ///
    /// This takes in the meta starting at `from`.
    fn parse_from_meta(&mut self, meta: ParseNestedMeta) -> Result<(), Error> {
        self.require_input(meta.path.span())?;

        self.attrs.replacement_ty.try_set(
            ReplacementType::From(meta.value()?.parse()?),
            meta.input.span(),
        )
    }

    /// Method used to require that a `Schematic::Input` type be generated for the attribute
    /// (identified by the given [`Span`]).
    fn require_input(&mut self, span: Span) -> Result<(), Error> {
        match &self.input_ty {
            InputType::Reflexive => {
                *self.input_ty = InputType::new_generated(self.ident);
                Ok(())
            }
            InputType::Existing(ty) => Err(Error::new(span, format_args!("attribute requires input type generation, but the input type is already specified as {}", ty.to_token_stream()))),
            InputType::Generated(_) => {
                Ok(())
            }
        }
    }
}
