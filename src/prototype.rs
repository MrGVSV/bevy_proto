use std::slice::Iter;

use bevy::ecs::prelude::Commands;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{AssetServer, Entity, Res};
use serde::{Deserialize, Serialize};

use crate::{ProtoCommands, ProtoComponent, ProtoData};

/// Allows access to a prototype's name and components so that it can be spawned in
pub trait Prototypical: 'static + Send + Sync {
	/// The name of the prototype
	///
	/// This should be unique amongst all prototypes in the world
	fn name(&self) -> &str;

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
		for component in self.iter_components() {
			component.insert_self(&mut proto_commands, asset_server);
		}

		proto_commands.into()
	}
}

/// The default prototype object, providing the basics for the prototype system
#[derive(Serialize, Deserialize)]
pub struct Prototype {
	/// The name of this prototype
	pub name: String,
	/// The components belonging to this prototype
	pub components: Vec<Box<dyn ProtoComponent>>,
}

impl Prototypical for Prototype {
	fn name(&self) -> &str {
		&self.name
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
