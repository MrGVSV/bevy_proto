use std::fmt::Formatter;

use bevy::asset::Handle;
use serde::de::{DeserializeSeed, Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use bevy_proto_backend::children::ProtoChildBuilder;
use bevy_proto_backend::path::ProtoPath;

use crate::prelude::Prototype;
use crate::proto::child::de::value::ProtoChildValueDeserializer;
use crate::proto::child::de::{PROTO_CHILD, PROTO_CHILD_MERGE_KEY, PROTO_CHILD_VALUE};
use crate::proto::{ProtoChild, ProtoChildValue};

#[derive(Deserialize, Debug)]
#[serde(field_identifier, rename_all = "snake_case")]
enum ProtoChildField {
    MergeKey,
    Value,
}

pub struct ProtoChildDeserializer<'a, 'ctx, 'load_ctx> {
    builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype>,
}

impl<'a, 'ctx, 'load_ctx> ProtoChildDeserializer<'a, 'ctx, 'load_ctx> {
    pub fn new(builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype>) -> Self {
        Self { builder }
    }
}

impl<'a, 'ctx, 'load_ctx, 'de> DeserializeSeed<'de>
    for ProtoChildDeserializer<'a, 'ctx, 'load_ctx>
{
    type Value = ProtoChild;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProtoChildVisitor<'a, 'ctx, 'load_ctx> {
            pub(crate) builder: &'a mut ProtoChildBuilder<'ctx, 'load_ctx, Prototype>,
        }
        impl<'a, 'ctx, 'load_ctx, 'de> Visitor<'de> for ProtoChildVisitor<'a, 'ctx, 'load_ctx> {
            type Value = ProtoChild;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "a `{}` struct", PROTO_CHILD)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let path = ProtoPath::new(value, self.builder).map_err(Error::custom)?;
                let handle = self.builder.add_child_path(path).map_err(Error::custom)?;
                Ok(ProtoChild {
                    handle,
                    merge_key: None,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut merge_key: Option<String> = None;
                let mut handle: Option<Handle<Prototype>> = None;

                while let Some(key) = map.next_key::<ProtoChildField>()? {
                    match key {
                        ProtoChildField::MergeKey => {
                            if merge_key.is_some() {
                                return Err(Error::duplicate_field(PROTO_CHILD_MERGE_KEY));
                            }
                            merge_key = map.next_value::<Option<String>>()?;
                        }
                        ProtoChildField::Value => {
                            if handle.is_some() {
                                return Err(Error::duplicate_field(PROTO_CHILD_VALUE));
                            }

                            let value = map
                                .next_value_seed(ProtoChildValueDeserializer::new(self.builder))?;
                            handle = match value {
                                ProtoChildValue::Path(path) => {
                                    Some(self.builder.add_child_path(path).map_err(Error::custom)?)
                                }
                                ProtoChildValue::Inline(prototype) => {
                                    Some(self.builder.add_child(prototype).map_err(Error::custom)?)
                                }
                            };
                        }
                    }
                }

                Ok(ProtoChild {
                    merge_key,
                    handle: handle.ok_or_else(|| Error::missing_field(PROTO_CHILD_VALUE))?,
                })
            }
        }

        deserializer.deserialize_any(ProtoChildVisitor {
            builder: self.builder,
        })
    }
}
