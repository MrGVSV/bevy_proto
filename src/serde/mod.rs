mod de;
pub(crate) mod extensions;
mod proto;
mod ser;

pub use de::{
    ComponentListDeserializer, ProtoDeserializable, PrototypeDeserializer, TemplateListDeserializer,
};
pub use ser::{ComponentListSerializer, PrototypeSerializer};

#[cfg(test)]
pub(crate) mod tests {
    use super::{PrototypeDeserializer, PrototypeSerializer};
    use crate::config::ProtoConfig;
    use crate::prelude::{
        ComponentList, ProtoComponent, Prototype, ReflectProtoComponent, TemplateList,
    };
    use crate::serde::extensions::YAML_EXT;
    use crate::serde::ProtoDeserializable;
    use bevy::prelude::{Component, Reflect};
    use bevy::reflect::{FromReflect, TypeRegistry, TypeRegistryArc};
    use serde::de::DeserializeSeed;
    use serde::{Deserialize, Serialize};
    use serde_yaml::Error;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;

    #[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
    #[reflect(ProtoComponent)]
    pub struct MyComponent {
        foo: usize,
        bar: Option<Name>,
    }

    // `Serialize + Deserialize` required since it's stored as an `Option<Name>` in `MyComponent`
    #[derive(Reflect, FromReflect, Clone, Serialize, Deserialize)]
    pub struct Name {
        x: String,
    }

    fn setup() -> (Prototype, ProtoConfig, TypeRegistryArc) {
        let templates = TemplateList::default();
        let prototype = Prototype {
            name: String::from("Foo"),
            templates: TemplateList::with_paths(vec![String::from("IFoo"), String::from("IBar")]),
            components: ComponentList::new(vec![Box::new(MyComponent {
                foo: 123,
                bar: Some(Name {
                    x: String::from("hello"),
                }),
            })]),
        };

        let registry = TypeRegistry::default();
        registry.write().register::<usize>();
        registry.write().register::<i32>();
        registry.write().register::<String>();
        registry.write().register::<MyComponent>();
        registry.write().register::<Name>();
        registry.write().register::<Option<Name>>();

        let mut config = ProtoConfig::default();
        config.whitelist::<MyComponent>();

        (prototype, config, registry)
    }

    fn serialize(proto: &Prototype, registry: &TypeRegistry) -> serde_yaml::Result<String> {
        let serializer = PrototypeSerializer::new(&proto, &registry);
        serde_yaml::to_string(&serializer)
    }

    fn deserialize(
        data: &str,
        config: &ProtoConfig,
        registry: &TypeRegistry,
    ) -> Result<Prototype, Error> {
        let deserializer = PrototypeDeserializer::new(&config, &registry);
        let de = serde_yaml::Deserializer::from_str(&data);
        deserializer.deserialize(de)
    }

    const DATA: &str = r#"---
name: Foo
templates:
  - IFoo
  - IBar
components:
  - type: "bevy_proto::serde::tests::MyComponent"
    struct:
      foo:
        type: usize
        value: 123
      bar:
        type: "core::option::Option<bevy_proto::serde::tests::Name>"
        value:
          x: hello
"#;

    #[test]
    fn should_serialize() -> anyhow::Result<()> {
        let (proto, _, registry) = setup();
        let expected = DATA;
        let output = serialize(&proto, &registry)?;
        assert_eq!(expected, output);
        Ok(())
    }

    #[test]
    fn should_deserialize() -> anyhow::Result<()> {
        let (expected, config, registry) = setup();

        let output = deserialize(DATA, &config, &registry)?;
        assert_eq!(expected, output);

        Ok(())
    }

    #[test]
    fn should_deserialize_asset() -> anyhow::Result<()> {
        let (expected, config, registry) = setup();
        let bytes = DATA.as_bytes();
        let yaml_path = Path::new("foo").join(YAML_EXT);

        let output = Prototype::deserialize(bytes, yaml_path.as_path(), &config, &registry)?;
        assert_eq!(expected, output);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "unknown extension: \"\"")]
    fn should_not_deserialize_unknown_ext() {
        let (expected, config, registry) = setup();
        let bytes = DATA.as_bytes();
        let invalid_path = Path::new("foo").join(".bar");

        let output =
            Prototype::deserialize(bytes, invalid_path.as_path(), &config, &registry).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    #[should_panic(expected = "unsupported filepath: \"�/prototype.yaml\"")]
    fn should_not_deserialize_unsupported_path() {
        let (expected, config, registry) = setup();
        let bytes = DATA.as_bytes();
        let invalid_path = Path::new(OsStr::from_bytes(&[0xff])).join(YAML_EXT);

        let output =
            Prototype::deserialize(bytes, invalid_path.as_path(), &config, &registry).unwrap();
        assert_eq!(expected, output);
    }
}
