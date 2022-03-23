use crate::config::ProtoConfig;
use crate::errors::ProtoError;
use crate::prelude::Prototype;
use crate::serde::{extensions, ProtoDeserializable, PrototypeDeserializer};
use anyhow::{anyhow, Error};
use bevy::reflect::TypeRegistryArc;
use serde::de::DeserializeSeed;
use std::path::Path;

impl ProtoDeserializable for Prototype {
    fn deserialize(
        bytes: &[u8],
        path: &Path,
        config: &ProtoConfig,
        type_registry: &TypeRegistryArc,
    ) -> Result<Self, Error> {
        let ext = path
            .extension()
            .ok_or_else(|| ProtoError::UnknownExtension {
                ext: Default::default(),
            })?;
        let path_str = path
            .to_str()
            .ok_or_else(|| anyhow!("unsupported filepath: \"{}\"", path.display()))?;

        if path_str.ends_with(extensions::YAML_EXT) {
            #[cfg(feature = "yaml")]
            {
                let de = serde_yaml::Deserializer::from_slice(bytes);
                let proto_de = PrototypeDeserializer::new(path, config, type_registry);
                return Ok(proto_de.deserialize(de)?);
            }
        } else if path_str.ends_with(extensions::JSON_EXT) {
            #[cfg(feature = "json")]
            {
                let mut de = serde_json::Deserializer::from_slice(bytes);
                let proto_de = PrototypeDeserializer::new(path, config, type_registry);
                return Ok(proto_de.deserialize(&mut de)?);
            }
        } else if path_str.ends_with(extensions::RON_EXT) {
            #[cfg(feature = "ron")]
            {
                let mut de = serde_ron::Deserializer::from_bytes(bytes)?;
                let proto_de = PrototypeDeserializer::new(path, config, type_registry);
                return Ok(proto_de.deserialize(&mut de)?);
            }
        }
        return Err(anyhow!(ProtoError::UnknownExtension {
            ext: ext.to_os_string(),
        }));
    }
}
