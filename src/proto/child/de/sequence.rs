use std::fmt::Formatter;

use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::Deserializer;

use bevy_proto_backend::children::ProtoChildBuilder;

use crate::proto::child::de::child::ProtoChildDeserializer;
use crate::proto::child::de::PROTO_CHILD;
use crate::proto::{ProtoChild, Prototype};

pub(crate) struct ProtoChildrenDeserializer<'a, 'ctx, 'load_ctx> {
    builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype>,
}

impl<'a, 'ctx, 'load_ctx> ProtoChildrenDeserializer<'a, 'ctx, 'load_ctx> {
    pub fn new(builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype>) -> Self {
        Self { builder }
    }
}

impl<'a, 'ctx, 'load_ctx, 'de> DeserializeSeed<'de>
    for ProtoChildrenDeserializer<'a, 'ctx, 'load_ctx>
{
    type Value = Vec<ProtoChild>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProtoChildrenVisitor<'a, 'ctx, 'load_ctx> {
            pub(crate) builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype>,
        }
        impl<'a, 'ctx, 'load_ctx, 'de> Visitor<'de> for ProtoChildrenVisitor<'a, 'ctx, 'load_ctx> {
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
