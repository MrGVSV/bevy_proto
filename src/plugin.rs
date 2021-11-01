use crate::{ProtoData, ProtoDataOptions, ProtoDeserializer, Prototype, Prototypical};
use bevy::app::{AppBuilder, Plugin};
use std::ops::Deref;

pub struct ProtoPlugin {
	pub options: Option<ProtoDataOptions>,
}

impl Default for ProtoPlugin {
	fn default() -> Self {
		Self { options: None }
	}
}

impl Plugin for ProtoPlugin {
	fn build(&self, app: &mut AppBuilder) {
		if let Some(opts) = &self.options {
			// Insert custom prototype options
			app.insert_resource(opts.clone());
		} else {
			// Insert default options
			app.insert_resource(ProtoDataOptions {
				directories: vec![String::from("assets/prototypes")],
				deserializer: Box::new(DefaultProtoDeserializer),
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
