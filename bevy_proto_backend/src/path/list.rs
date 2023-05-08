use std::fmt::Formatter;

use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::Deserializer;

use crate::load::{Loader, ProtoLoadContext};
use crate::path::{ProtoPath, ProtoPathDeserializer};
use crate::proto::Prototypical;

/// Deserializer for a sequence of [`ProtoPath`]s.
pub struct ProtoPathListDeserializer<'a, 'ctx, 'load_ctx, T: Prototypical, L: Loader<T>> {
    context: &'a ProtoLoadContext<'ctx, 'load_ctx, T, L>,
}

impl<'a, 'ctx, 'load_ctx, T: Prototypical, L: Loader<T>>
    ProtoPathListDeserializer<'a, 'ctx, 'load_ctx, T, L>
{
    pub fn new(context: &'a ProtoLoadContext<'ctx, 'load_ctx, T, L>) -> Self {
        Self { context }
    }
}

impl<'a, 'ctx, 'load_ctx, 'de, T: Prototypical, L: Loader<T>> DeserializeSeed<'de>
    for ProtoPathListDeserializer<'a, 'ctx, 'load_ctx, T, L>
{
    type Value = Vec<ProtoPath>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PathListVisitor<'a, 'ctx, 'load_ctx, T: Prototypical, L: Loader<T>> {
            context: &'a ProtoLoadContext<'ctx, 'load_ctx, T, L>,
        }

        impl<'a, 'ctx, 'load_ctx, 'de, T: Prototypical, L: Loader<T>> Visitor<'de>
            for PathListVisitor<'a, 'ctx, 'load_ctx, T, L>
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
