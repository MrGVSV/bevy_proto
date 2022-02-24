use std::fmt::Formatter;
use std::iter::Rev;
use std::slice::Iter;

use bevy::ecs::prelude::Commands;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{AssetServer, Entity, Res};
use indexmap::IndexSet;
use serde::{
	de::{self, Error, SeqAccess, Visitor},
	Deserialize, Deserializer, Serialize,
};

use crate::{
	components::ProtoComponent, data::ProtoCommands, data::ProtoData, utils::handle_cycle,
};

/// Allows access to a prototype's name and components so that it can be spawned in
pub trait Prototypical: 'static + Send + Sync {
	/// The name of the prototype
	///
	/// This should be unique amongst all prototypes in the world
	fn name(&self) -> &str;

	/// The names of the parent templates (if any)
	fn templates(&self) -> &[String] {
		&[]
	}

	/// The names of the parent templates (if any) in reverse order
	fn templates_rev(&self) -> Rev<Iter<'_, String>> {
		self.templates().iter().rev()
	}

	/// Returns an iterator of [`ProtoComponent`] objects
	fn iter_components(&self) -> Iter<'_, Box<dyn ProtoComponent>>;

	/// Creates the [`ProtoCommands`] object used for modifying the given entity
	///
	/// # Arguments
	///
	/// * `entity`: The entity commands
	/// * `data`: The prototype data in this world
	///
	/// returns: ProtoCommands
	///
	fn create_commands<'w, 's, 'a, 'p>(
		&'p self,
		entity: EntityCommands<'w, 's, 'a>,
		data: &'p Res<ProtoData>,
	) -> ProtoCommands<'w, 's, 'a, 'p>;

	/// Spawns an entity with this prototype's component structure
	///
	/// # Arguments
	///
	/// * `commands`: The world `Commands`
	/// * `data`: The prototype data in this world
	/// * `asset_server`: The asset server
	///
	/// returns: EntityCommands
	///
	/// # Examples
	///
	/// ```
	/// use bevy::prelude::*;
	/// use bevy_proto::prelude::{ProtoData, Prototype, Prototypical};
	///
	/// fn setup_system(mut commands: Commands, data: Res<ProtoData>, asset_server: &Res<AssetServer>) {
	///     let proto: Prototype = serde_yaml::from_str(r#"
	///     name: My Prototype
	///     components:
	///       - type: SomeMarkerComponent
	///       - type: SomeComponent
	///         value:
	///           - speed: 10.0
	///     "#).unwrap();
	///
	///     let entity = proto.spawn(&mut commands, &data, &asset_server).id();
	///
	///     // ...
	/// }
	///
	/// ```
	fn spawn<'w, 's, 'a, 'p>(
		&'p self,
		commands: &'a mut Commands<'w, 's>,
		data: &Res<ProtoData>,
		asset_server: &Res<AssetServer>,
	) -> EntityCommands<'w, 's, 'a> {
		let entity = commands.spawn();
		self.insert(entity, data, asset_server)
	}

	/// Inserts this prototype's component structure to the given entity
	///
	/// __Note:__ This _will_ override existing components of the same type.
	///
	/// # Arguments
	///
	/// * `entity`: The `EntityCommands` for a given entity
	/// * `data`: The prototype data in this world
	/// * `asset_server`: The asset server
	///
	/// returns: EntityCommands
	///
	/// # Examples
	///
	/// ```
	/// use bevy::prelude::*;
	/// use bevy_proto::prelude::{ProtoData, Prototype, Prototypical};
	///
	/// #[derive(Component, Default)]
	/// struct Player(pub Entity);
	///
	/// fn setup_system(mut commands: Commands, data: Res<ProtoData>, asset_server: &Res<AssetServer>, player: Query<&Player>) {
	///     let proto: Prototype = serde_yaml::from_str(r#"
	///     name: My Prototype
	///     components:
	///       - type: SomeMarkerComponent
	///       - type: SomeComponent
	///         value:
	///           - speed: 10.0
	///     "#).unwrap();
	///
	///     // Get the EntityCommands for the player entity
	/// 	let entity = commands.entity(player.single().0);
	///
	///     // Insert the new components
	///     let entity = proto.insert(entity, &data, &asset_server).id();
	///
	///     // ...
	/// }
	///
	/// ```
	fn insert<'w, 's, 'a, 'p>(
		&'p self,
		entity: EntityCommands<'w, 's, 'a>,
		data: &Res<ProtoData>,
		asset_server: &Res<AssetServer>,
	) -> EntityCommands<'w, 's, 'a> {
		let mut proto_commands = self.create_commands(entity, data);

		spawn_internal(
			self.name(),
			self.templates().iter().rev(),
			self.iter_components(),
			&mut proto_commands,
			data,
			asset_server,
			&mut IndexSet::default(),
		);

		proto_commands.into()
	}
}

/// Internal method used for recursing up the template hierarchy and spawning components
/// from the top to the bottom
fn spawn_internal<'a>(
	name: &'a str,
	templates: Rev<Iter<'a, String>>,
	components: Iter<'a, Box<dyn ProtoComponent>>,
	proto_commands: &mut ProtoCommands,
	data: &'a Res<ProtoData>,
	asset_server: &Res<AssetServer>,
	traversed: &mut IndexSet<&'a str>,
) {
	// We insert first on the off chance that someone made a prototype its own template...
	traversed.insert(name);

	for template in templates {
		if traversed.contains(template.as_str()) {
			// ! === Found Circular Dependency === ! //
			handle_cycle!(
				template,
				traversed,
				"For now, the rest of the spawn has been skipped."
			);

			continue;
		}

		// === Spawn Template === //
		if let Some(parent) = data.get_prototype(template) {
			spawn_internal(
				parent.name(),
				parent.templates_rev(),
				parent.iter_components(),
				proto_commands,
				data,
				asset_server,
				traversed,
			);
		}
	}

	// === Spawn Self === //
	for component in components {
		component.insert_self(proto_commands, asset_server);
	}
}

/// The default prototype object, providing the basics for the prototype system
#[derive(Serialize, Deserialize)]
pub struct Prototype {
	/// The name of this prototype
	pub name: String,
	/// The names of this prototype's templates (if any)
	///
	/// See [`deserialize_templates_list`], for how these names are deserialized.
	#[serde(default)]
	#[serde(alias = "template")]
	#[serde(deserialize_with = "deserialize_templates_list")]
	pub templates: Vec<String>,
	/// The components belonging to this prototype
	#[serde(default)]
	pub components: Vec<Box<dyn ProtoComponent>>,
}

impl Prototypical for Prototype {
	fn name(&self) -> &str {
		&self.name
	}

	fn templates(&self) -> &[String] {
		&self.templates
	}

	fn iter_components(&self) -> Iter<'_, Box<dyn ProtoComponent>> {
		self.components.iter()
	}

	fn create_commands<'w, 's, 'a, 'p>(
		&'p self,
		entity: EntityCommands<'w, 's, 'a>,
		data: &'p Res<ProtoData>,
	) -> ProtoCommands<'w, 's, 'a, 'p> {
		data.get_commands(self, entity)
	}
}

/// A function used to deserialize a list of templates
///
/// A template list can take on the following forms:
///
/// * Inline List:
///   > ```yaml
///   > templates: [ A, B, C ]
///   > ```
/// * Multi-Line List:
///   > ```yaml
///   > templates:
///   >   - A
///   >   - B
///   >   - C
///   > ```
/// * Comma-Separated String:
///   > ```yaml
///   > templates: A, B, C # OR: "A, B, C"
///   > ```
pub fn deserialize_templates_list<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
	D: Deserializer<'de>,
{
	struct TemplatesList;

	impl<'de> Visitor<'de> for TemplatesList {
		type Value = Vec<String>;

		fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
			formatter.write_str("string or vec")
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: Error,
		{
			// Split string by commas
			// Allowing for: "A, B, C" to become [A, B, C]
			Ok(v.split(",").map(|s| s.trim().to_string()).collect())
		}

		fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
		where
			A: SeqAccess<'de>,
		{
			Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
		}
	}

	deserializer.deserialize_any(TemplatesList)
}
