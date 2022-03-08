use crate::prelude::Prototype;
use crate::serde::de::ComponentListDeserializer;
use crate::serde::ser::ComponentListSerializer;
use bevy::reflect::TypeRegistry;
use serde::de::{DeserializeSeed, Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

pub struct ProtoSerializer<'a> {
	prototype: &'a Prototype,
	registry: &'a TypeRegistry,
}

pub struct ProtoDeserializer<'a> {
	registry: &'a TypeRegistry,
}

impl<'a> ProtoSerializer<'a> {
	pub fn new(prototype: &'a Prototype, registry: &'a TypeRegistry) -> Self {
		Self {
			prototype,
			registry,
		}
	}
}

impl<'a> ProtoDeserializer<'a> {
	pub fn new(registry: &'a TypeRegistry) -> Self {
		Self { registry }
	}
}

impl<'a> Serialize for ProtoSerializer<'a> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let total = if self.prototype.templates.is_empty() {
			2
		} else {
			3
		};
		let mut state = serializer.serialize_map(Some(total))?;
		state.serialize_entry("name", &self.prototype.name)?;
		if !self.prototype.templates.is_empty() {
			state.serialize_entry("templates", &self.prototype.templates)?;
		}

		let comp_list = &self.prototype.components;
		let comp_serializer = ComponentListSerializer::new(&comp_list, self.registry);

		state.serialize_entry("components", &comp_serializer);
		state.end()
	}
}

impl<'a, 'de> DeserializeSeed<'de> for ProtoDeserializer<'a> {
	type Value = Prototype;

	fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_map(ProtoVisitor {
			registry: self.registry,
		})
	}
}

pub struct ProtoVisitor<'a> {
	registry: &'a TypeRegistry,
}

impl<'a, 'de> Visitor<'de> for ProtoVisitor<'a> {
	type Value = Prototype;

	fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
		write!(formatter, "a Prototype definition")
	}

	fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
	where
		A: MapAccess<'de>,
	{
		let mut name = None;
		let mut templates = None;
		let mut components = None;

		while let Some(key) = map.next_key::<String>()? {
			match key.as_str() {
				"name" => name = Some(map.next_value()?),
				"template" | "templates" => templates = Some(map.next_value()?),
				"components" => {
					components =
						Some(map.next_value_seed(ComponentListDeserializer::new(self.registry))?)
				},
				invalid => return Err(A::Error::custom(format!("Invalid entry: {}", invalid))),
			}
		}

		Ok(Prototype {
			name: name.ok_or_else(|| A::Error::custom("Missing `name` property"))?,
			templates: templates.unwrap_or_default(),
			components: components.unwrap_or_default(),
		})
	}
}

#[cfg(test)]
pub(crate) mod tests {
	use crate::components::{ComponentList, ReflectProtoComponent};
	use crate::prelude::ProtoComponent;
	use crate::prototype::Prototype;
	use crate::serde::proto::{ProtoDeserializer, ProtoSerializer};
	use bevy::prelude::{Component, Reflect};
	use bevy::reflect::ReflectDeserialize;
	use bevy::reflect::{TypeRegistry, TypeRegistryArc};
	use serde::de::DeserializeSeed;
	use serde::{Deserialize, Serialize};
	use serde_yaml::Deserializer;
	use std::any::{Any, TypeId};

	#[derive(Reflect, Component, ProtoComponent, Clone, Debug, Serialize, Deserialize)]
	#[reflect(ProtoComponent, Serialize, Deserialize)]
	pub struct MyComponent {
		foo: usize,
		// bar: Option<Name>,
	}

	#[derive(Reflect, Clone, Serialize, Deserialize)]
	pub struct Name {
		x: String,
	}

	fn setup() -> (Prototype, TypeRegistryArc) {
		let prototype = Prototype {
			name: String::from("Foo"),
			templates: vec![String::from("IFoo"), String::from("IBar")],
			components: ComponentList::new(vec![Box::new(MyComponent {
				foo: 123,
				// bar: Some(Name {
				// 	x: String::from("hello"),
				// }),
			})]),
		};

		let registry = TypeRegistry::default();
		registry.write().register::<usize>();
		registry.write().register::<i32>();
		registry.write().register::<String>();
		registry.write().register::<MyComponent>();
		registry.write().register::<Name>();
		registry.write().register::<Option<Name>>();

		println!(
			"goot: {:?}",
			TypeId::of::<crate::serde::proto::tests::MyComponent>(),
		);

		(prototype, registry)
	}

	#[test]
	fn should_serialize() -> anyhow::Result<()> {
		let (mut proto, registry) = setup();

		let serializer = ProtoSerializer::new(&proto, &registry);
		let output = serde_yaml::to_string(&serializer)?;

		let expected = r#"---
name: Foo
templates:
  - IFoo
  - IBar
components:
  - type: "bevy_proto::serde::proto::tests::MyComponent"
    struct:
      foo:
        type: usize
        value: 123
      bar:
        type: "core::option::Option<bevy_proto::serde::proto::tests::Name>"
        value:
          x: hello
"#;

		assert_eq!(expected, output);
		Ok(())
	}

	#[test]
	fn should_deserialize() -> anyhow::Result<()> {
		let (_, registry) = setup();

		let input = r#"---
name: Foo
templates:
  - IFoo
  - IBar
components:
  - type: "bevy_proto::serde::proto::tests::MyComponent"
    struct:
      foo:
        type: usize
        value: 123
"#;

		{
			let x = registry.read();
			let d = x
				.get_with_name("bevy_proto::serde::proto::tests::MyComponent")
				.unwrap();
			println!("T: {:?}", d.type_id());
			// for r in x.iter() {
			// 	println!("Name: {}", r.short_name());
			// 	println!("Data: {:?}", r.data::<ReflectProtoComponent>().is_some());
			// }
		}

		let deserializer = ProtoDeserializer::new(&registry);
		let mut de = serde_yaml::Deserializer::from_str(&input);
		let value = deserializer.deserialize(de)?;
		println!("{:?}", value);

		Ok(())
	}
}
