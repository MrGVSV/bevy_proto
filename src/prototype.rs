use std::ops::Add;
use std::slice::Iter;

use bevy::ecs::prelude::Commands;
use bevy::ecs::system::EntityCommands;
use bevy::log::warn;
use bevy::prelude::{AssetServer, Entity, Res};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::{ProtoCommands, ProtoComponent, ProtoData};

/// Allows access to a prototype's name and components so that it can be spawned in
pub trait Prototypical: 'static + Send + Sync {
	/// The name of the prototype
	///
	/// This should be unique amongst all prototypes in the world
	fn name(&self) -> &str;

	/// The name of the template (if any)
	fn template(&self) -> Option<&str> {
		None
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
	fn create_commands<'a, 'b, 'c>(
		&'c self,
		entity: EntityCommands<'a, 'b>,
		data: &'c Res<ProtoData>,
	) -> ProtoCommands<'a, 'b, 'c>;

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
	/// use bevy_proto::{ProtoData, Prototype, Prototypical};
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
	fn spawn<'a, 'b, 'c>(
		&'c self,
		commands: &'b mut Commands<'a>,
		data: &Res<ProtoData>,
		asset_server: &Res<AssetServer>,
	) -> EntityCommands<'a, 'b> {
		let mut entity = commands.spawn();
		let mut proto_commands = self.create_commands(entity, data);

		spawn_internal(
			self.name(),
			self.template(),
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
	template: Option<&'a str>,
	components: Iter<'a, Box<dyn ProtoComponent>>,
	proto_commands: &mut ProtoCommands,
	data: &'a Res<ProtoData>,
	asset_server: &Res<AssetServer>,
	traversed: &mut IndexSet<&'a str>,
) {
	// We insert first on the off chance that someone made a prototype its own template...
	traversed.insert(name);

	match template {
		Some(template_name) if traversed.contains(template_name) => {
			// ! === Found Circular Dependency === ! //
			let tree: String = traversed
				.iter()
				.map(|n| format!("'{}' -> ", n))
				.collect::<String>()
				.add(&format!("'{}'", template_name));
			warn!(
				"{} ({})\n\t{}",
				"Found a circular dependency when trying to spawn a prototype!",
				tree,
				"The rest of the spawn has been skipped, but make sure you remove any template that might call itself!"
			);
		}
		Some(template_name) => {
			// === Spawn Template === //
			if let Some(parent) = data.get_prototype(template_name) {
				spawn_internal(
					parent.name(),
					parent.template(),
					parent.iter_components(),
					proto_commands,
					data,
					asset_server,
					traversed,
				);
			}
		}
		_ => (),
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
	/// The name of this prototype's template (if any)
	#[serde(default)]
	pub template: Option<String>,
	/// The components belonging to this prototype
	#[serde(default)]
	pub components: Vec<Box<dyn ProtoComponent>>,
}

impl Prototypical for Prototype {
	fn name(&self) -> &str {
		&self.name
	}

	fn template(&self) -> Option<&str> {
		self.template.as_deref()
	}

	fn iter_components(&self) -> Iter<'_, Box<dyn ProtoComponent>> {
		self.components.iter()
	}

	fn create_commands<'a, 'b, 'c>(
		&'c self,
		entity: EntityCommands<'a, 'b>,
		data: &'c Res<ProtoData>,
	) -> ProtoCommands<'a, 'b, 'c> {
		data.get_commands(self, entity)
	}
}
