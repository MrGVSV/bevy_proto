use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::error::Error;
use std::ffi::OsStr;
use std::ops::Deref;

use bevy::asset::{Asset, HandleUntyped};
use bevy::ecs::prelude::World;
use bevy::prelude::{Assets, ColorMaterial, FromWorld, Handle, Res};
use bevy::reflect::{TypeUuid, Uuid};
use bevy::utils::HashMap;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};

use crate::prototype::Prototype;
use crate::{ProtoComponent, Prototypical};

/// A String newtype for a handle's asset path
#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct HandlePath(pub String);

impl Deref for HandlePath {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub trait PrototypeDataContainer: 'static {
	/// Get a loaded prototype with the given name
	///
	/// # Arguments
	///
	/// * `name`: The name of the prototype
	///
	/// returns: Option<&Prototype>
	fn get_prototype(&self, name: &str) -> Option<&Box<dyn Prototypical>>;

	/// Store a handle
	///
	/// # Arguments
	///
	/// * `protoytpe`: The Prototype this handle belongs to
	/// * `component`: The ProtoComponent this handle belongs to
	/// * `path`: The handle's path
	/// * `handle`: The handle
	///
	/// returns: ()
	///
	/// # Examples
	///
	/// ```
	/// use bevy::prelude::*;
	/// use bevy_proto::{HandlePath, ProtoData, Prototype, PrototypeDataContainer};
	///
	/// struct MyComponent {
	///     texture_path: HandlePath
	/// }
	///
	/// // impl ProtoComponent for MyComponent { ... }
	///
	/// fn some_loader(asset_server: Res<AssetServer>, mut data: ResMut<ProtoData>) {
	///     let comp = MyComponent {
	///         texture_path: HandlePath(String::from("path/to/texture.png"))
	///     };
	///     let proto = Prototype {
	///         name: String::from("My Prototype"),
	///         components: vec![Box::new(comp)]
	///     };
	///
	///     let handle: Handle<Texture> =  asset_server.load(comp.texture_path.0.as_str());
	///
	///     data.insert_handle(&proto, &comp, &comp.texture_path, handle);
	/// }
	/// ```
	fn insert_handle<T: Asset>(
		&mut self,
		protoytpe: &Box<dyn Prototypical>,
		component: &dyn ProtoComponent,
		path: &HandlePath,
		handle: Handle<T>,
	);

	/// Get a cloned handle
	///
	/// # Arguments
	///
	/// * `protoytpe`: The Prototype this handle belongs to
	/// * `component`: The ProtoComponent this handle belongs to
	/// * `path`: The handle's path
	///
	/// returns: Option<Handle<T>>
	fn get_handle<T: Asset>(
		&self,
		protoytpe: &dyn Prototypical,
		component: &dyn ProtoComponent,
		path: &HandlePath,
	) -> Option<Handle<T>> {
		let handle = self.get_untyped_handle(protoytpe, component, path, T::TYPE_UUID)?;
		Some(handle.clone().typed::<T>())
	}

	/// Get a weakly cloned handle
	///
	/// # Arguments
	///
	/// * `protoytpe`: The Prototype this handle belongs to
	/// * `component`: The ProtoComponent this handle belongs to
	/// * `path`: The handle's path
	///
	/// returns: Option<Handle<T>>
	fn get_handle_weak<T: Asset>(
		&self,
		protoytpe: &dyn Prototypical,
		component: &dyn ProtoComponent,
		path: &HandlePath,
	) -> Option<Handle<T>> {
		let handle = self.get_untyped_handle(protoytpe, component, path, T::TYPE_UUID)?;
		Some(handle.clone_weak().typed::<T>())
	}

	/// Get a untyped handle reference
	///
	/// # Arguments
	///
	/// * `protoytpe`: The Prototype this handle belongs to
	/// * `component`: The ProtoComponent this handle belongs to
	/// * `path`: The handle's path
	/// * `asset_type`: The asset type
	///
	/// returns: Option<&HandleUntyped>
	fn get_untyped_handle(
		&self,
		protoytpe: &dyn Prototypical,
		component: &dyn ProtoComponent,
		path: &HandlePath,
		asset_type: Uuid,
	) -> Option<&HandleUntyped>;
}

pub struct ProtoData {
	/// Maps Prototype Name -> Component Type -> HandlePath -> Asset Type -> HandleUntyped
	handles: HashMap<
		String, // Prototype Name
		HashMap<
			TypeId, // Component Type
			HashMap<
				String, // Handle Path
				HashMap<
					Uuid,          // Asset UUID
					HandleUntyped, // Handle
				>,
			>,
		>,
	>,
	prototypes: HashMap<String, Box<dyn Prototypical>>,
}

impl ProtoData {
	pub fn new() -> Self {
		Self {
			handles: HashMap::default(),
			prototypes: HashMap::default(),
		}
	}
}

impl PrototypeDataContainer for ProtoData {
	fn get_prototype(&self, name: &str) -> Option<&Box<dyn Prototypical>> {
		self.prototypes.get(name)
	}

	fn insert_handle<T: Asset>(
		&mut self,
		protoytpe: &Box<dyn Prototypical>,
		component: &dyn ProtoComponent,
		path: &HandlePath,
		handle: Handle<T>,
	) {
		let proto_map = self
			.handles
			.entry(protoytpe.name().to_string())
			.or_insert(HashMap::default());
		let comp_map = proto_map
			.entry(component.type_id())
			.or_insert(HashMap::default());
		let path_map = comp_map
			.entry(path.to_string())
			.or_insert(HashMap::default());
		path_map.insert(T::TYPE_UUID, handle.clone_untyped());
	}

	fn get_untyped_handle(
		&self,
		protoytpe: &dyn Prototypical,
		component: &dyn ProtoComponent,
		path: &HandlePath,
		asset_type: Uuid,
	) -> Option<&HandleUntyped> {
		let proto_map = self.handles.get(protoytpe.name())?;
		let comp_map = proto_map.get(&component.type_id())?;
		let path_map = comp_map.get(path.as_str())?;
		path_map.get(&asset_type)
	}
}

impl FromWorld for ProtoData {
	fn from_world(world: &mut World) -> Self {
		let mut myself = Self {
			handles: Default::default(),
			prototypes: HashMap::default(),
		};

		let options = world
			.get_resource::<ProtoDataOptions>()
			.expect("Expected options for ProtoData")
			.clone();

		for directory in options.directories {
			if let Ok(dir) = std::fs::read_dir(directory) {
				for file_info in dir {
					if file_info.is_err() {
						continue;
					}
					let file_info = file_info.unwrap();

					let path = file_info.path();

					if let Some(filters) = &options.extensions {
						if let Some(ext) = path.extension().and_then(OsStr::to_str) {
							if filters.iter().find(|filter| *filter == &ext).is_none() {
								continue;
							}
						}
					}

					if let Ok(data) = std::fs::read_to_string(path) {
						if let Some(proto) = options.deserializer.deserialize(&data) {
							for component in proto.iter_components() {
								component.prepare(world, &proto, &mut myself);
							}

							myself.prototypes.insert(proto.name().to_string(), proto);
						}
					}
				}
			}
		}

		myself
	}
}

pub trait ProtoDeserializer: DynClone {
	/// Deserializes file input (as a string) into a [`Prototypical`] object
	///
	/// # Arguments
	///
	/// * `data`: The file data as a string
	///
	/// returns: Option<Box<dyn Prototypical, Global>>
	///
	/// # Examples
	///
	/// ```
	/// // The default implementation:
	/// use bevy_proto::{Prototype, Prototypical};
	/// fn example_deserialize(data: &str) -> Option<Box<dyn Prototypical>> {
	/// 	if let Ok(value) = serde_yaml::from_str::<Prototype>(data) {
	///  		Some(Box::new(value))
	///  	} else {
	/// 		None
	///  	}
	/// }
	/// ```
	fn deserialize(&self, data: &str) -> Option<Box<dyn Prototypical>>;
}

dyn_clone::clone_trait_object!(ProtoDeserializer);

/// Options for controlling how prototype data is handled
#[derive(Clone)]
pub struct ProtoDataOptions {
	/// Directories containing prototype data
	pub directories: Vec<String>,
	/// A custom deserializer for prototypes
	pub deserializer: Box<dyn ProtoDeserializer + Send + Sync>,
	/// A collection of extensions to filter the directories by. These do __not__
	/// have a dot ('.') prepended to them.
	///
	/// A value of None allows all files to be read.
	///
	/// # Examples
	///
	/// ```
	/// use bevy_proto::ProtoDataOptions;
	///
	/// let opts = ProtoDataOptions {
	/// 	// Only allow .yaml or .json files
	/// 	extensions: Some(vec!["yaml", "json"]),
	/// 	..Default::default()
	/// };
	/// ```
	pub extensions: Option<Vec<&'static str>>,
}
