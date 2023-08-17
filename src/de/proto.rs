use std::fmt::Formatter;

use bevy::asset::Handle;
use serde::de::{DeserializeSeed, Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use bevy_proto_backend::children::Children;
use bevy_proto_backend::load::{Loader, ProtoLoadContext};
use bevy_proto_backend::path::{ProtoPathContext, ProtoPathListDeserializer};
use bevy_proto_backend::schematics::Schematics;
use bevy_proto_backend::templates::Templates;

use crate::de::ProtoChildrenDeserializer;
use crate::prelude::Prototype;
use crate::schematics::SchematicsDeserializer;

const NAME: &str = "name";
const TEMPLATES: &str = "templates";
const SCHEMATICS: &str = "schematics";
const CHILDREN: &str = "children";
const ENTITY: &str = "entity";

#[derive(Deserialize, Debug)]
#[serde(field_identifier, rename_all = "snake_case")]
enum PrototypeField {
    Name,
    Templates,
    Schematics,
    Children,
    Entity,
}

pub struct PrototypeDeserializer<'a, 'ctx, 'load_ctx, L: Loader<Prototype>> {
    pub(crate) context: &'a mut ProtoLoadContext<'ctx, 'load_ctx, Prototype, L>,
}

impl<'a, 'ctx, 'load_ctx, L: Loader<Prototype>> PrototypeDeserializer<'a, 'ctx, 'load_ctx, L> {
    pub fn new(context: &'a mut ProtoLoadContext<'ctx, 'load_ctx, Prototype, L>) -> Self {
        Self { context }
    }
}

impl<'a, 'ctx, 'load_ctx, 'de, L: Loader<Prototype>> DeserializeSeed<'de>
    for PrototypeDeserializer<'a, 'ctx, 'load_ctx, L>
{
    type Value = Prototype;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PrototypeVisitor<'a, 'ctx, 'load_ctx, L: Loader<Prototype>> {
            context: &'a mut ProtoLoadContext<'ctx, 'load_ctx, Prototype, L>,
        }

        impl<'a, 'ctx, 'load_ctx, 'de, L: Loader<Prototype>> Visitor<'de>
            for PrototypeVisitor<'a, 'ctx, 'load_ctx, L>
        {
            type Value = Prototype;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "a `Prototype` struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut templates: Option<Templates> = None;
                let mut schematics: Option<Schematics> = None;
                let mut children: Option<Children<Prototype>> = None;
                let mut requires_entity: Option<bool> = None;

                while let Some(key) = map.next_key::<PrototypeField>()? {
                    match key {
                        PrototypeField::Name => {
                            if id.is_some() {
                                return Err(Error::duplicate_field(NAME));
                            }
                            id = Some(map.next_value::<String>()?)
                        }
                        PrototypeField::Templates => {
                            if templates.is_some() {
                                return Err(Error::duplicate_field(TEMPLATES));
                            }

                            let templates = templates.insert(Templates::default());
                            let paths =
                                map.next_value_seed(ProtoPathListDeserializer::new(self.context))?;

                            for path in paths {
                                let handle: Handle<Prototype> = self.context.get_handle(&path);
                                templates.insert(path, handle);
                            }
                        }
                        PrototypeField::Schematics => {
                            if schematics.is_some() {
                                return Err(Error::duplicate_field(SCHEMATICS));
                            }

                            schematics = Some(map.next_value_seed(SchematicsDeserializer::new(
                                self.context.registry(),
                            ))?);
                        }
                        PrototypeField::Children => {
                            if children.is_some() {
                                return Err(Error::duplicate_field(CHILDREN));
                            }

                            let children = children.insert(Children::default());
                            self.context
                                .with_children::<A::Error, _>(|builder| {
                                    let child_list = map
                                        .next_value_seed(ProtoChildrenDeserializer::new(builder))?;

                                    for child in child_list {
                                        children.insert(child);
                                    }

                                    Ok(())
                                })
                                .map_err(Error::custom)?;
                        }
                        PrototypeField::Entity => {
                            if requires_entity.is_some() {
                                return Err(Error::duplicate_field(ENTITY));
                            }
                            requires_entity = Some(map.next_value::<bool>()?)
                        }
                    }
                }

                Ok(Prototype {
                    id: id.ok_or_else(|| Error::missing_field(NAME))?,
                    path: self.context.base_path().into(),
                    requires_entity: requires_entity.unwrap_or(true),
                    templates,
                    schematics: schematics.unwrap_or_default(),
                    children,
                    dependencies: Default::default(),
                })
            }
        }

        deserializer.deserialize_struct(
            std::any::type_name::<Prototype>(),
            &[NAME, TEMPLATES, SCHEMATICS, CHILDREN, ENTITY],
            PrototypeVisitor {
                context: self.context,
            },
        )
    }
}
