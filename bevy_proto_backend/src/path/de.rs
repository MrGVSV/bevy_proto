use std::fmt::Formatter;

use serde::de::{DeserializeSeed, Error, Visitor};
use serde::Deserializer;

use crate::path::{ProtoPath, ProtoPathContext};

/// Deserializer for a [`ProtoPath`].
pub struct ProtoPathDeserializer<'a> {
    context: &'a dyn ProtoPathContext,
}

impl<'a> ProtoPathDeserializer<'a> {
    pub fn new(context: &'a dyn ProtoPathContext) -> Self {
        Self { context }
    }
}

impl<'a, 'ctx, 'load_ctx, 'de> DeserializeSeed<'de> for ProtoPathDeserializer<'a> {
    type Value = ProtoPath;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PathVisitor<'a> {
            context: &'a dyn ProtoPathContext,
        }

        impl<'a, 'ctx, 'load_ctx, 'de> Visitor<'de> for PathVisitor<'a> {
            type Value = ProtoPath;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "a path to a prototype")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                ProtoPath::new(value, self.context).map_err(Error::custom)
            }
        }

        deserializer.deserialize_str(PathVisitor {
            context: self.context,
        })
    }
}
