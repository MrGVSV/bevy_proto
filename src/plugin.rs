use crate::{ProtoData, ProtoDataOptions, ProtoDeserializer, Prototype, Prototypical};
use bevy::app::{App, Plugin};

pub struct ProtoPlugin {
	pub options: Option<ProtoDataOptions>,
}

impl ProtoPlugin {
	/// Specify the directory containing the prototype files
	///
	/// # Arguments
	///
	/// * `dir`: The directory path, relative to the project root
	///
	/// returns: ProtoPlugin
	///
	/// # Examples
	///
	/// ```
	/// use bevy_proto::ProtoPlugin;
	///
	/// let plugin = ProtoPlugin::with_dir("assets/config");
	/// ```
	pub fn with_dir(dir: &str) -> Self {
		Self {
			options: Some(ProtoDataOptions {
				directories: vec![dir.to_string()],
				deserializer: Box::new(DefaultProtoDeserializer),
				extensions: Some(vec!["yaml", "json"]),
			}),
		}
	}

	/// Specify a set of directories containing the prototype files
	///
	/// # Arguments
	///
	/// * `dirs`: The directory paths, relative to the project root
	///
	/// returns: ProtoPlugin
	///
	/// # Examples
	///
	/// ```
	/// use bevy_proto::ProtoPlugin;
	///
	/// let plugin = ProtoPlugin::with_dirs(vec![
	///   String::from("assets/config"),
	///   String::from("assets/mods"),
	/// ]);
	/// ```
	pub fn with_dirs(dirs: Vec<String>) -> Self {
		Self {
			options: Some(ProtoDataOptions {
				directories: dirs,
				deserializer: Box::new(DefaultProtoDeserializer),
				extensions: Some(vec!["yaml", "json"]),
			}),
		}
	}
}

impl Default for ProtoPlugin {
	fn default() -> Self {
		Self { options: None }
	}
}

impl Plugin for ProtoPlugin {
	fn build(&self, app: &mut App) {
		if let Some(opts) = &self.options {
			// Insert custom prototype options
			app.insert_resource(opts.clone());
		} else {
			// Insert default options
			app.insert_resource(ProtoDataOptions {
				directories: vec![String::from("assets/prototypes")],
				deserializer: Box::new(DefaultProtoDeserializer),
				extensions: Some(vec!["yaml", "json"]),
			});
		}

		// Initialize prototypes
		app.init_resource::<ProtoData>();
	}
}

#[derive(Clone)]
struct DefaultProtoDeserializer;

impl ProtoDeserializer for DefaultProtoDeserializer {
	fn deserialize(&self, data: &str) -> Option<Box<dyn Prototypical>> {
		if let Ok(value) = serde_yaml::from_str::<Prototype>(data) {
			Some(Box::new(value))
		} else {
			None
		}
	}
}
