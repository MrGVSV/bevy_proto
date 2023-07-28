use std::fmt::Formatter;

use bevy::reflect::serde::{TypeRegistrationDeserializer, TypedReflectDeserializer};
use bevy::reflect::TypeRegistryInternal;
use serde::de::{DeserializeSeed, Error, MapAccess, Visitor};
use serde::Deserializer;

use bevy_proto_backend::schematics::{ReflectSchematic, Schematics};

pub(crate) struct SchematicsDeserializer<'a> {
    registry: &'a TypeRegistryInternal,
}

impl<'a> SchematicsDeserializer<'a> {
    pub fn new(registry: &'a TypeRegistryInternal) -> Self {
        Self { registry }
    }
}

impl<'de, 'a> DeserializeSeed<'de> for SchematicsDeserializer<'a> {
    type Value = Schematics;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SchematicsVisitor<'a> {
            registry: &'a TypeRegistryInternal,
        }
        impl<'de, 'a> Visitor<'de> for SchematicsVisitor<'a> {
            type Value = Schematics;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "map of schematics")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let size_hint = map.size_hint().unwrap_or_default();
                let mut schematics = Schematics::with_capacity(size_hint);

                while let Some(registration) =
                    map.next_key_seed(TypeRegistrationDeserializer::new(self.registry))?
                {
                    if schematics.contains_by_name(registration.type_name()) {
                        return Err(Error::custom(format_args!(
                            "duplicate schematic: `{}`",
                            registration.type_name()
                        )));
                    }

                    let reflect_schematic =
                        registration.data::<ReflectSchematic>().ok_or_else(|| {
                            Error::custom(format_args!(
                                "missing `ReflectSchematic` registration for schematic: `{}`",
                                registration.type_name()
                            ))
                        })?;

                    let input_registration = reflect_schematic.input_registration();

                    let input = map.next_value_seed(TypedReflectDeserializer::new(
                        &input_registration,
                        self.registry,
                    ))?;

                    let schematic = reflect_schematic
                        .create_dynamic(input)
                        .map_err(Error::custom)?;

                    schematics.insert_dynamic(schematic);
                }

                Ok(schematics)
            }
        }

        deserializer.deserialize_map(SchematicsVisitor {
            registry: self.registry,
        })
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Component;
    use bevy::reflect::{Reflect, TypeRegistryInternal};

    use bevy_proto_backend::schematics::Schematic;

    use super::*;

    #[derive(Reflect, Component, Schematic, Eq, PartialEq, Debug)]
    struct MySchematic {
        foo: usize,
    }

    #[test]
    fn should_deserialize_schematics() {
        let mut registry = TypeRegistryInternal::new();
        registry.register::<MySchematic>();
        registry.register_type_data::<MySchematic, ReflectSchematic>();

        let input = r#"
{
    "bevy_proto::schematics::tests::MySchematic": (
        foo: 123
    )
}"#;

        let deserializer = SchematicsDeserializer::new(&registry);
        let schematics = deserializer
            .deserialize(&mut ron::de::Deserializer::from_str(input).unwrap())
            .unwrap();

        assert_eq!(1, schematics.len());
        assert_eq!(
            &MySchematic { foo: 123 },
            schematics
                .get::<MySchematic>()
                .unwrap()
                .input()
                .downcast_ref::<MySchematic>()
                .unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "missing `ReflectSchematic` registration for schematic")]
    fn should_not_deserialize_schematics() {
        let mut registry = TypeRegistryInternal::new();
        registry.register::<MySchematic>();

        let input = r#"
{
    "bevy_proto::schematics::tests::MySchematic": (
        foo: 123
    )
}"#;

        let deserializer = SchematicsDeserializer::new(&registry);
        deserializer
            .deserialize(&mut ron::de::Deserializer::from_str(input).unwrap())
            .unwrap();
    }
}
