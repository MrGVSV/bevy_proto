use bevy::reflect::TypeUuid;
use serde::de::DeserializeSeed;

use bevy_proto_backend::children::Children;
use bevy_proto_backend::deps::Dependencies;
use bevy_proto_backend::load::ProtoLoadContext;
use bevy_proto_backend::path::{ProtoPath, ProtoPathContext};
use bevy_proto_backend::proto::Prototypical;
use bevy_proto_backend::schematics::Schematics;
use bevy_proto_backend::templates::Templates;

use crate::config::ProtoConfig;
use crate::proto::{ProtoChild, PrototypeDeserializer, PrototypeError};

/// The core asset type used to create easily-configurable entity trees.
#[derive(Debug, TypeUuid)]
#[uuid = "cbc85a87-723a-4e61-83c7-26e96e54fe9f"]
pub struct Prototype {
    pub(crate) id: String,
    pub(crate) path: ProtoPath,
    pub(crate) schematics: Schematics,
    pub(crate) templates: Option<Templates>,
    pub(crate) dependencies: Dependencies,
    pub(crate) children: Option<Children<Prototype>>,
}

impl Prototypical for Prototype {
    type Id = String;
    type Child = ProtoChild;
    type Config = ProtoConfig;
    type Error = PrototypeError;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn path(&self) -> &ProtoPath {
        &self.path
    }

    fn schematics(&self) -> &Schematics {
        &self.schematics
    }

    fn schematics_mut(&mut self) -> &mut Schematics {
        &mut self.schematics
    }

    fn templates(&self) -> Option<&Templates> {
        self.templates.as_ref()
    }

    fn templates_mut(&mut self) -> Option<&mut Templates> {
        self.templates.as_mut()
    }

    fn dependencies(&self) -> &Dependencies {
        &self.dependencies
    }

    fn dependencies_mut(&mut self) -> &mut Dependencies {
        &mut self.dependencies
    }

    fn children(&self) -> Option<&Children<Self>> {
        self.children.as_ref()
    }

    fn children_mut(&mut self) -> Option<&mut Children<Self>> {
        self.children.as_mut()
    }

    fn deserialize(bytes: &[u8], ctx: &mut ProtoLoadContext<Self>) -> Result<Self, Self::Error> {
        let path = ctx.base_path().to_path_buf();

        let ext = path
            .extension()
            .ok_or_else(|| PrototypeError::MissingExtension(path.clone()))?;

        let ext = ext
            .to_str()
            .ok_or_else(|| PrototypeError::UnsupportedExtension(ext.to_string_lossy().to_string()))?
            .to_lowercase();

        let deserializer = PrototypeDeserializer::new(ctx);

        match ext.as_str() {
            #[cfg(feature = "ron")]
            "ron" => {
                let mut ron_de = ron::Deserializer::from_bytes(bytes)
                    .map_err(|err| PrototypeError::SpannedRonError(path.clone(), err))?;
                deserializer.deserialize(&mut ron_de).map_err(|err| {
                    PrototypeError::SpannedRonError(path.clone(), ron_de.span_error(err))
                })
            }
            #[cfg(feature = "yaml")]
            "yaml" => deserializer
                .deserialize(serde_yaml::Deserializer::from_slice(bytes))
                .map_err(PrototypeError::from),
            other => Err(PrototypeError::UnsupportedExtension(other.to_string())),
        }
    }
}
