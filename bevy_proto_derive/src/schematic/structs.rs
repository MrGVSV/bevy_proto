use proc_macro2::{Ident, TokenStream};
use syn::{Error, Fields};

use crate::schematic::field_attributes::FieldAttributes;
use crate::schematic::fields::SchematicField;
use crate::schematic::input::InputType;

pub(crate) enum SchematicStruct {
    Unit,
    Unnamed(Vec<SchematicField>),
    Named(Vec<SchematicField>),
}

impl SchematicStruct {
    /// Compile-time assertions, if any.
    ///
    /// These are generated within an anonymous context and should either:
    /// 1. Enforce invariants at runtime
    /// 2. Provide clearer error outputs for users
    pub fn assertions(&self) -> Option<TokenStream> {
        match self {
            SchematicStruct::Unit => None,
            SchematicStruct::Named(fields) | SchematicStruct::Unnamed(fields) => {
                fields.iter().map(|field| field.assertions()).collect()
            }
        }
    }
}

pub(crate) struct SchematicStructBuilder<'a> {
    pub ident: &'a Ident,
    pub input_ty: &'a mut InputType,
    pub proto_crate: &'a TokenStream,
    pub bevy_crate: &'a TokenStream,
}

impl<'a> SchematicStructBuilder<'a> {
    pub fn build(&mut self, fields: Fields) -> Result<SchematicStruct, Error> {
        Ok(match fields {
            Fields::Named(fields) => SchematicStruct::Named(
                fields
                    .named
                    .into_iter()
                    .map(|field| {
                        Ok(SchematicField::new(
                            FieldAttributes::new(&field, self.ident, self.input_ty)?,
                            field.ident.unwrap(),
                            field.ty,
                            self.proto_crate,
                            self.bevy_crate,
                        ))
                    })
                    .collect::<Result<_, Error>>()?,
            ),
            Fields::Unnamed(fields) => SchematicStruct::Unnamed(
                fields
                    .unnamed
                    .into_iter()
                    .enumerate()
                    .map(|(index, field)| {
                        Ok(SchematicField::new(
                            FieldAttributes::new(&field, self.ident, self.input_ty)?,
                            index,
                            field.ty,
                            self.proto_crate,
                            self.bevy_crate,
                        ))
                    })
                    .collect::<Result<_, Error>>()?,
            ),
            Fields::Unit => SchematicStruct::Unit,
        })
    }
}
