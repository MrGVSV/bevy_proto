use syn::{Attribute, Error};

use crate::common::input::{parse_input_meta, InputType, OutputType, SchematicIo};
use crate::utils::constants::{ASSET_SCHEMATIC_ATTR, FROM_ATTR, INPUT_ATTR, INTO_ATTR};
use crate::utils::{
    define_attribute, parse_bool, parse_nested_meta, AttrArg, AttrArgValue, AttrTarget,
};

define_attribute!("no_preload" => NoPreloadArg(bool) for AttrTarget::Asset);

/// Attribute information on the container type.
#[derive(Default)]
pub(super) struct ContainerAttributes {
    no_preload: NoPreloadArg,
}

impl ContainerAttributes {
    pub fn new(attrs: &[Attribute], io: &mut SchematicIo) -> Result<Self, Error> {
        let mut this = Self::default();

        for attr in attrs {
            if !attr.path().is_ident(ASSET_SCHEMATIC_ATTR) {
                continue;
            }

            parse_nested_meta!(attr, |meta| {
                FROM_ATTR => io.try_set_input_ty(InputType::Existing(meta.value()?.parse()?), None),
                INTO_ATTR => io.try_set_output_ty(OutputType::Custom(meta.value()?.parse()?), None),
                INPUT_ATTR => parse_input_meta(meta, io),
                NoPreloadArg::NAME => this.no_preload.try_set(Some(parse_bool(&meta)?), meta.input.span()),
            })?;
        }

        Ok(this)
    }

    pub fn no_preload(&self) -> bool {
        self.no_preload.get().copied().unwrap_or_default()
    }
}
