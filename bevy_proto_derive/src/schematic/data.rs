use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, Error, Fields};

use crate::schematic::input::InputType;
use crate::schematic::structs::{SchematicStruct, SchematicStructBuilder};
use crate::schematic::variants::SchematicVariant;

/// The data of the item deriving `Schematic`.
pub(crate) enum SchematicData {
    Struct(SchematicStruct),
    Enum(Vec<SchematicVariant>),
}

impl SchematicData {
    pub fn from_data(
        data: Data,
        ident: &Ident,
        input_ty: &mut InputType,
        proto_crate: &TokenStream,
        bevy_crate: &TokenStream,
    ) -> Result<Self, Error> {
        let mut builder = SchematicStructBuilder {
            ident,
            input_ty,
            proto_crate,
            bevy_crate,
        };

        let data = match data {
            Data::Struct(data) => SchematicData::Struct(builder.build(data.fields)?),
            Data::Enum(data) => SchematicData::Enum(
                data.variants
                    .into_iter()
                    .map(|variant| {
                        Ok(match &variant.fields {
                            Fields::Named(_) => SchematicVariant {
                                ident: variant.ident,
                                data: builder.build(variant.fields)?,
                            },
                            Fields::Unnamed(_) => SchematicVariant {
                                ident: variant.ident,
                                data: builder.build(variant.fields)?,
                            },
                            Fields::Unit => SchematicVariant {
                                ident: variant.ident,
                                data: SchematicStruct::Unit,
                            },
                        })
                    })
                    .collect::<Result<_, Error>>()?,
            ),
            Data::Union(data) => {
                return Err(Error::new(data.union_token.span, "unions not supported"))
            }
        };

        Ok(data)
    }

    /// Compile-time assertions, if any.
    ///
    /// These are generated within an anonymous context and should either:
    /// 1. Enforce invariants at runtime
    /// 2. Provide clearer error outputs for users
    pub fn assertions(&self) -> Option<TokenStream> {
        let assertions = match self {
            SchematicData::Struct(data) => data.assertions(),
            SchematicData::Enum(data) => data.iter().map(|variant| variant.assertions()).collect(),
        }?;

        Some(quote! {
            mod DataAssertions {
                #assertions
            }
        })
    }
}
