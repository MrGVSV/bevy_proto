use std::fmt::Formatter;

use serde::de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::loader::ProtoLoader;
use bevy_proto_backend::children::ProtoChildBuilder;
use bevy_proto_backend::load::Loader;
use bevy_proto_backend::path::ProtoPathDeserializer;

use crate::prelude::Prototype;
use crate::proto::{ProtoChildValue, PrototypeDeserializer};

const PROTO_CHILD_VALUE: &str = "ProtoChildValue";
const PROTO_CHILD_VALUE_PATH: &str = "Path";
const PROTO_CHILD_VALUE_INLINE: &str = "Inline";

#[derive(Deserialize, Debug)]
#[serde(variant_identifier)]
enum ProtoChildValueVariant {
    Path,
    Inline,
}

pub(crate) struct ProtoChildValueDeserializer<
    'a,
    'ctx,
    'load_ctx,
    L: Loader<Prototype> = ProtoLoader,
> {
    builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype, L>,
}

impl<'a, 'ctx, 'load_ctx, L: Loader<Prototype>>
    ProtoChildValueDeserializer<'a, 'ctx, 'load_ctx, L>
{
    pub fn new(builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype, L>) -> Self {
        Self { builder }
    }
}

impl<'a, 'ctx, 'load_ctx, 'de, L: Loader<Prototype>> DeserializeSeed<'de>
    for ProtoChildValueDeserializer<'a, 'ctx, 'load_ctx, L>
{
    type Value = ProtoChildValue;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProtoChildValueVisitor<'a, 'ctx, 'load_ctx, L: Loader<Prototype>> {
            builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype, L>,
        }
        impl<'a, 'ctx, 'load_ctx, 'de, L: Loader<Prototype>> Visitor<'de>
            for ProtoChildValueVisitor<'a, 'ctx, 'load_ctx, L>
        {
            type Value = ProtoChildValue;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "{PROTO_CHILD_VALUE} variant")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (variant, value) = data.variant::<ProtoChildValueVariant>()?;

                match variant {
                    ProtoChildValueVariant::Path => {
                        let path = value.newtype_variant_seed(ProtoPathDeserializer::new(
                            self.builder.context(),
                        ))?;
                        Ok(ProtoChildValue::Path(path))
                    }
                    ProtoChildValueVariant::Inline => {
                        let prototype = value.newtype_variant_seed(PrototypeDeserializer::new(
                            self.builder.context_mut(),
                        ))?;
                        Ok(ProtoChildValue::Inline(prototype))
                    }
                }
            }
        }

        deserializer.deserialize_enum(
            PROTO_CHILD_VALUE,
            &[PROTO_CHILD_VALUE_PATH, PROTO_CHILD_VALUE_INLINE],
            ProtoChildValueVisitor {
                builder: self.builder,
            },
        )
    }
}
