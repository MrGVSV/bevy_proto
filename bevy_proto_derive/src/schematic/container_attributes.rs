use std::fmt::{Debug, Formatter};

use crate::common::input::{parse_input_meta, InputType, OutputType, SchematicIo};
use crate::utils::constants::{FROM_ATTR, INPUT_ATTR, INTO_ATTR, SCHEMATIC_ATTR};
use syn::meta::ParseNestedMeta;
use syn::{Attribute, Error, LitStr};

use crate::utils::{parse_nested_meta, unsupported_arg};

const KIND_ATTR: &str = "kind";
const KIND_BUNDLE: &str = "bundle";
const KIND_RESOURCE: &str = "resource";

/// Attribute information on the container type.
#[derive(Default)]
pub(super) struct ContainerAttributes {
    kind: SchematicKind,
}

impl ContainerAttributes {
    pub fn new(attrs: &[Attribute], io: &mut SchematicIo) -> Result<Self, Error> {
        let mut this = Self::default();

        for attr in attrs {
            if !attr.path().is_ident(SCHEMATIC_ATTR) {
                continue;
            }

            parse_nested_meta!(attr, |meta| {
                FROM_ATTR => io.try_set_input_ty(InputType::Existing(meta.value()?.parse()?), None),
                INTO_ATTR => io.try_set_output_ty(OutputType::Custom(meta.value()?.parse()?), None),
                INPUT_ATTR => parse_input_meta(meta, io),
                KIND_ATTR => this.parse_kind_meta(meta),
            })?;
        }

        Ok(this)
    }

    pub fn kind(&self) -> &SchematicKind {
        &self.kind
    }

    fn parse_kind_meta(&mut self, meta: ParseNestedMeta) -> Result<(), Error> {
        if !matches!(self.kind, SchematicKind::Undefined) {
            return Err(meta.error(format_args!(
                "schematic kind already configured as #[schematic{:?}]",
                &self.kind
            )));
        }

        let kind: LitStr = meta.value()?.parse()?;
        let kind_str = kind.value();

        match &kind_str {
            _ if kind_str == KIND_BUNDLE => {
                self.kind = SchematicKind::Bundle;
                Ok(())
            }
            _ if kind_str == KIND_RESOURCE => {
                self.kind = SchematicKind::Resource;
                Ok(())
            }
            _ => Err(unsupported_arg(&meta, Some(&[KIND_BUNDLE, KIND_RESOURCE]))),
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
