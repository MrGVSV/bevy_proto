use std::fmt::Formatter;

use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::Deserializer;

use crate::load::ProtoLoadContext;
use crate::path::{ProtoPath, ProtoPathDeserializer};
use crate::proto::Prototypical;

/// Deserializer for a sequence of [`ProtoPath`]s.
pub struct ProtoPathListDeserializer<'a, 'ctx, 'load_ctx, T: Prototypical> {
    context: &'a ProtoLoadContext<'ctx, 'load_ctx, T>,
}

impl<'a, 'ctx, 'load_ctx, T: Prototypical> ProtoPathListDeserializer<'a, 'ctx, 'load_ctx, T> {
    pub fn new(context: &'a ProtoLoadContext<'ctx, 'load_ctx, T>) -> Self {
        Self { context }
    }
}

impl<'a, 'ctx, 'load_ctx, 'de, T: Prototypical> DeserializeSeed<'de>
    for ProtoPathListDeserializer<'a, 'ctx, 'load_ctx, T>
{
    type Value = Vec<ProtoPath>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PathListVisitor<'a, 'ctx, 'load_ctx, T: Prototypical> {
            context: &'a ProtoLoadContext<'ctx, 'load_ctx, T>,
        }

        impl<'a, 'ctx, 'load_ctx, 'de, T: Prototypical> Visitor<'de>
            for PathListVisitor<'a, 'ctx, 'load_ctx, T>
        {
            type Value = Vec<ProtoPath>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "list of valid prototype paths")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let size_hint = seq.size_hint().unwrap_or_default();
                let mut paths = Vec::with_capacity(size_hint);

                while let Some(path) =
                    seq.next_element_seed(ProtoPathDeserializer::new(self.context))?
                {
                    paths.push(path);
                }

                Ok(paths)
            }
        }

        deserializer.deserialize_seq(PathListVisitor {
            context: self.context,
        })
    }
}
