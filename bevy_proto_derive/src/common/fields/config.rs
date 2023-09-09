use crate::common::data::DeriveType;
use crate::common::fields::{AssetConfig, EntityConfig};
use crate::utils::constants::{ASSET_ATTR, ENTITY_ATTR, FROM_ATTR};
use crate::utils::debug_attribute;
use crate::utils::{define_attribute, AttrArgValue, AttrTarget};
use proc_macro2::Span;
use quote::ToTokens;
use std::fmt::{Debug, Formatter};
use syn::{Error, Type};

define_attribute!("optional" => OptionalArg(bool) for AttrTarget::Field);

/// The base configuration for the field of a `Schematic` or `AssetSchematic`.
pub(crate) struct FieldConfig {
    /// The type of the derive.
    ///
    /// This isn't configured by the user, but is instead inferred from the derive type.
    ///
    /// The only reason this is stored is so that we can use it in error messages.
    derive_type: DeriveType,
    /// The kind of the field.
    ///
    /// This controls what the field does and all of its configuration.
    kind: Option<FieldKind>,
    /// Whether the field should be generated as wrapped in an `Option`.
    ///
    /// The [`ProtoFieldBuilder`] will automatically try to infer this from the field's type,
    /// but this attribute allows the user to configure it manually if needed.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[derive(Schematic)]
    /// struct Foo {
    ///   #[schematic(optional, entity)]
    ///   bar: Option<Entity>,
    /// }
    /// ```
    ///
    /// [`ProtoFieldBuilder`]: crate::common::fields::ProtoFieldBuilder
    optional: OptionalArg,
}

impl FieldConfig {
    pub fn try_init_from_kind(&mut self, ty: Type, span: Span) -> Result<(), Error> {
        match &self.kind {
            None => {
                self.kind = Some(FieldKind::From(ty));
            }
            Some(current) => {
                return Err(Error::new(
                    span,
                    format!("field already configured as `{:?}`", current),
                ));
            }
        }

        Ok(())
    }

    pub fn try_init_entity_kind(&mut self, span: Span) -> Result<&mut EntityConfig, Error> {
        match &self.kind {
            None => {
                self.kind = Some(FieldKind::Entity(EntityConfig::default()));
            }
            Some(FieldKind::Entity(_)) => {}
            Some(current) => {
                return Err(Error::new(
                    span,
                    format!("field already configured as `{:?}`", current),
                ));
            }
        }

        let FieldKind::Entity(config) = self.kind.as_mut().unwrap() else {
            unreachable!()
        };
        Ok(config)
    }

    pub fn try_init_asset_kind(&mut self, span: Span) -> Result<&mut AssetConfig, Error> {
        match &self.kind {
            None => {
                self.kind = Some(FieldKind::Asset(AssetConfig::default()));
            }
            Some(FieldKind::Asset(_)) => {}
            Some(current) => {
                return Err(Error::new(
                    span,
                    format!("field already configured as `{:?}`", current),
                ));
            }
        }

        let FieldKind::Asset(config) = self.kind.as_mut().unwrap() else {
            unreachable!()
        };
        Ok(config)
    }

    pub fn kind(&self) -> Option<&FieldKind> {
        self.kind.as_ref()
    }

    pub fn optional(&self) -> bool {
        self.optional.get().copied().unwrap_or_default()
    }

    pub fn try_set_optional(&mut self, value: bool, span: Span) -> Result<(), Error> {
        match self.kind() {
            None | Some(FieldKind::From(_)) => Err(Error::new(
                span,
                "cannot set `optional` on a field that is not marked as an `entity` or `asset`",
            )),
            _ => self.optional.try_set(Some(value), span),
        }
    }
}

impl Default for FieldConfig {
    fn default() -> Self {
        Self {
            derive_type: DeriveType::Schematic,
            kind: None,
            optional: OptionalArg::default(),
        }
    }
}

impl Debug for FieldConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let attr = self.derive_type.attr_name();
        write!(f, "#[{attr}(")?;

        debug_attribute(f, |write| {
            write(format_args!("{:?}", self.optional))?;
            write(format_args!("{:?}", self.kind))?;

            Ok(())
        })?;

        write!(f, ")]")
    }
}

pub(crate) enum FieldKind {
    From(Type),
    Entity(EntityConfig),
    Asset(AssetConfig),
}

impl Debug for FieldKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::From(ty) => write!(f, "{FROM_ATTR} = {}", ty.to_token_stream()),
            Self::Entity(config) => write!(f, "{ENTITY_ATTR}{:?}", config),
            Self::Asset(config) => write!(f, "{ASSET_ATTR}{:?}", config),
        }
    }
}
