use std::fmt::Formatter;

use crate::loader::ProtoLoader;
use bevy_proto_backend::children::ProtoChildBuilder;
use bevy_proto_backend::load::Loader;
use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::Deserializer;

use crate::de::child::PROTO_CHILD;
use crate::de::ProtoChildDeserializer;
use crate::proto::{ProtoChild, Prototype};

pub struct ProtoChildrenDeserializer<'a, 'ctx, 'load_ctx, L: Loader<Prototype> = ProtoLoader> {
    builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype, L>,
}

impl<'a, 'ctx, 'load_ctx, L: Loader<Prototype>> ProtoChildrenDeserializer<'a, 'ctx, 'load_ctx, L> {
    pub fn new(builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype, L>) -> Self {
        Self { builder }
    }
}

impl<'a, 'ctx, 'load_ctx, 'de, L: Loader<Prototype>> DeserializeSeed<'de>
    for ProtoChildrenDeserializer<'a, 'ctx, 'load_ctx, L>
{
    type Value = Vec<ProtoChild>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProtoChildrenVisitor<'a, 'ctx, 'load_ctx, L: Loader<Prototype>> {
            pub(crate) builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype, L>,
        }
        impl<'a, 'ctx, 'load_ctx, 'de, L: Loader<Prototype>> Visitor<'de>
            for ProtoChildrenVisitor<'a, 'ctx, 'load_ctx, L>
        {
            type Value = Vec<ProtoChild>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "a `{}` list", PROTO_CHILD)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut children = Vec::with_capacity(seq.size_hint().unwrap_or_default());

                while let Some(child) =
                    seq.next_element_seed(ProtoChildDeserializer::new(self.builder))?
                {
                    children.push(child);
                }

                Ok(children)
            }
        }

        deserializer.deserialize_seq(ProtoChildrenVisitor {
            builder: self.builder,
        })
    }
}
