use crate::common::data::{DeriveType, SchematicVariant};
use crate::common::fields::SchematicFields;
use crate::common::input::SchematicIo;
use syn::{Data, Error};

/// The shape and data of a schematic.
pub(crate) enum SchematicData {
    Struct(SchematicFields),
    Enum(Vec<SchematicVariant>),
}

impl SchematicData {
    pub fn new(data: Data, io: &mut SchematicIo, derive_type: DeriveType) -> Result<Self, Error> {
        match data {
            Data::Struct(data) => Ok(Self::Struct(SchematicFields::new(
                &data.fields,
                io,
                derive_type,
            )?)),
            Data::Enum(data) => Ok(Self::Enum(
                data.variants
                    .into_iter()
                    .map(|variant| {
                        let fields = SchematicFields::new(&variant.fields, io, derive_type)?;
                        Ok(SchematicVariant {
                            ident: variant.ident,
                            fields,
                        })
                    })
                    .collect::<Result<_, Error>>()?,
            )),
            Data::Union(data) => Err(Error::new(data.union_token.span, "unions not supported")),
        }
    }
}
