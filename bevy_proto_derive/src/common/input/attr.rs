use crate::common::input::{InputType, SchematicIo};
use crate::utils::parse_nested_meta;
use crate::utils::{define_attribute, AttrArg, AttrTarget};
use proc_macro2::Ident;
use quote::ToTokens;
use std::fmt::{Debug, Formatter};
use syn::meta::ParseNestedMeta;
use syn::{Error, Visibility};

define_attribute!("vis" => InputVisArg(Visibility) for AttrTarget::InputVisibility, no_debug);
define_attribute!("name" => InputNameArg(Ident) for AttrTarget::Input);

impl Debug for InputVisArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => Ok(()),
            Some(Visibility::Inherited) => panic!("cannot explicitly set visibility to inherited"),
            Some(Visibility::Public(_)) => write!(f, "{} = pub", Self::NAME),
            Some(Visibility::Restricted(res)) => {
                write!(f, "{} = {}", Self::NAME, res.to_token_stream())
            }
        }
    }
}

/// Parses input attributes into the given [`SchematicIo`].
pub(crate) fn parse_input_meta(meta: ParseNestedMeta, io: &mut SchematicIo) -> Result<(), Error> {
    parse_nested_meta!(meta, |meta| {
        InputVisArg::NAME => io.try_set_input_vis(meta.value()?.parse()?, None),
        InputNameArg::NAME => io.try_set_input_ty(InputType::Generated(meta.value()?.parse()?), None),
    })
}
