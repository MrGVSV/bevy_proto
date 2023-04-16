use std::fmt::{Debug, Formatter};

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::meta::ParseNestedMeta;
use syn::spanned::Spanned;
use syn::{Attribute, Error, LitStr, Visibility};

use crate::schematic::input::InputType;
use crate::schematic::self_type::SelfType;
use crate::schematic::utils::filter_attributes;

const INPUT_ATTR: &str = "input";
const INPUT_VIS: &str = "vis";
const INPUT_NAME: &str = "name";

const FROM_ATTR: &str = "from";

const INTO_ATTR: &str = "into";

const KIND_ATTR: &str = "kind";
const KIND_BUNDLE: &str = "bundle";
const KIND_RESOURCE: &str = "resource";

/// Attribute information on the container type.
#[derive(Default)]
pub(crate) struct ContainerAttributes {
    input_vis: Option<Visibility>,
    kind: SchematicKind,
}

impl ContainerAttributes {
    pub fn new(
        attrs: &[Attribute],
        self_ty: &mut SelfType,
        input_ty: &mut InputType,
    ) -> Result<Self, Error> {
        let mut data = ContainerAttributesBuilder {
            attrs: Self::default(),
            self_ty,
            input_ty,
        };

        for attr in filter_attributes(attrs) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(FROM_ATTR) {
                    data.try_set_input_ty(InputType::Existing(meta.value()?.parse()?))
                } else if meta.path.is_ident(INTO_ATTR) {
                    data.try_set_self_ty(SelfType::Into(meta.value()?.parse()?))
                } else if meta.path.is_ident(INPUT_ATTR) {
                    data.parse_input_meta(meta)
                } else if meta.path.is_ident(KIND_ATTR) {
                    data.parse_kind_meta(meta)
                } else {
                    Err(meta.error(format_args!(
                        "unsupported argument, expected one of: {:?}",
                        [INPUT_ATTR, FROM_ATTR, INTO_ATTR, KIND_ATTR]
                    )))
                }
            })?;
        }

        Ok(data.attrs)
    }

    /// Compile-time assertions, if any.
    ///
    /// These are generated within an anonymous context and should either:
    /// 1. Enforce invariants at runtime
    /// 2. Provide clearer error outputs for users
    pub fn assertions(&self) -> Option<TokenStream> {
        None
    }

    pub fn input_vis(&self) -> Option<&Visibility> {
        self.input_vis.as_ref()
    }

    pub fn kind(&self) -> &SchematicKind {
        &self.kind
    }
}

pub(crate) struct ContainerAttributesBuilder<'a> {
    attrs: ContainerAttributes,
    self_ty: &'a mut SelfType,
    input_ty: &'a mut InputType,
}

impl<'a> ContainerAttributesBuilder<'a> {
    fn parse_input_meta(&mut self, meta: ParseNestedMeta) -> Result<(), Error> {
        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident(INPUT_VIS) {
                match &self.attrs.input_vis {
                    Some(vis) => Err(meta.error(format_args!(
                        "visibility already set to {}",
                        vis.to_token_stream()
                    ))),
                    None => {
                        self.attrs.input_vis = Some(meta.value()?.parse()?);
                        Ok(())
                    }
                }
            } else if meta.path.is_ident(INPUT_NAME) {
                self.try_set_input_ty(InputType::Generated(meta.value()?.parse()?))
            } else {
                Err(meta.error(format_args!(
                    "unsupported argument, expected one of: {:?}",
                    [INPUT_VIS, INPUT_NAME]
                )))
            }
        })
    }

    fn parse_kind_meta(&mut self, meta: ParseNestedMeta) -> Result<(), Error> {
        if !matches!(self.attrs.kind, SchematicKind::Undefined) {
            return Err(meta.error(format_args!(
                "schematic kind already configured as #[schematic{:?}]",
                &self.attrs.kind
            )));
        }

        let kind: LitStr = meta.value()?.parse()?;
        let kind_str = kind.value();

        match kind_str.as_str() {
            KIND_BUNDLE => {
                self.attrs.kind = SchematicKind::Bundle;
                Ok(())
            }
            KIND_RESOURCE => {
                self.attrs.kind = SchematicKind::Resource;
                Ok(())
            }
            _ => Err(Error::new(
                kind.span(),
                format_args!(
                    "unsupported argument, expected one of: {:?}",
                    [KIND_BUNDLE, KIND_RESOURCE]
                ),
            )),
        }
    }

    fn try_set_input_ty(&mut self, value: InputType) -> Result<(), Error> {
        match self.self_ty {
            SelfType::Into(ty) => Err(Error::new(
                value.span(),
                format_args!(
                    "cannot specify input type when schematic type already set to {}",
                    ty.to_token_stream()
                ),
            )),
            _ => match &self.input_ty {
                InputType::Reflexive => {
                    *self.input_ty = value;
                    Ok(())
                }
                InputType::Existing(ty) => Err(Error::new(
                    value.span(),
                    format_args!("input type already specified as {}", ty.to_token_stream()),
                )),
                InputType::Generated(ident) => Err(Error::new(
                    value.span(),
                    format_args!("input type already specified as {}", ident),
                )),
            },
        }
    }

    fn try_set_self_ty(&mut self, value: SelfType) -> Result<(), Error> {
        match self.input_ty {
            InputType::Existing(ty) => Err(Error::new(
                value.span(),
                format_args!(
                    "cannot specify schematic type when input type already set to {}",
                    ty.to_token_stream()
                ),
            )),
            _ => match &self.self_ty {
                SelfType::Reflexive => {
                    *self.self_ty = value;
                    Ok(())
                }
                SelfType::Into(ty) => Err(Error::new(
                    value.span(),
                    format_args!(
                        "schematic type already specified as {}",
                        ty.to_token_stream()
                    ),
                )),
            },
        }
    }
}

#[derive(Default)]
pub(crate) enum SchematicKind {
    #[default]
    Undefined,
    Bundle,
    Resource,
}

impl Debug for SchematicKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SchematicKind::Undefined => Ok(()),
            SchematicKind::Bundle => write!(f, "(kind = {KIND_BUNDLE:?})"),
            SchematicKind::Resource => write!(f, "(kind = {KIND_RESOURCE:?})"),
        }
    }
}
