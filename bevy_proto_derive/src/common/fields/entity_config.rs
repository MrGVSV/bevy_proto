use crate::utils::debug_attribute;
use crate::utils::{define_attribute, AttrArgValue, AttrTarget};
use proc_macro2::Span;
use std::fmt::{Debug, Formatter};
use syn::{Error, LitStr};

define_attribute!("path" => EntityPathArg(LitStr) for AttrTarget::Field);

#[derive(Default)]
pub(crate) struct EntityConfig {
    /// Represents a static path to an entity in the prototype tree.
    path: EntityPathArg,
}

impl EntityConfig {
    pub fn path(&self) -> Option<&LitStr> {
        self.path.get()
    }

    pub fn try_set_path(&mut self, value: LitStr, span: Span) -> Result<(), Error> {
        self.path.try_set(Some(value), span)
    }
}

impl Debug for EntityConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        debug_attribute(f, |write| {
            write(format_args!("{:?}", self.path))?;

            Ok(())
        })?;

        write!(f, ")")
    }
}
