use crate::prelude::Prototype;
use crate::proto::{PrototypeDeserializer, PrototypeError};
use bevy_proto_backend::load::{Loader, ProtoLoadContext};
use bevy_proto_backend::path::ProtoPathContext;
use serde::de::DeserializeSeed;

const RON_FORMATS: &[&str] = &["prototype.ron", "proto.ron"];
const YAML_FORMATS: &[&str] = &["prototype.yaml", "proto.yaml"];

/// The default prototype loader.
///
/// # Supported Formats
///
/// | Format | Feature | Extensions |
/// | ------ | ------- | ---------- |
/// | [RON]    | `ron`   | `.prototype.ron`, `.proto.ron` |
/// | [YAML]   | `yaml`  | `.prototype.yaml`, `.proto.yaml` |
///
/// [RON]: https://github.com/ron-rs/ron
/// [YAML]: https://github.com/dtolnay/serde-yaml
#[derive(Clone)]
pub struct ProtoLoader {
    extensions: Vec<&'static str>,
}

impl Default for ProtoLoader {
    fn default() -> Self {
        let mut extensions = Vec::new();

        if cfg!(feature = "yaml") {
            extensions.extend(YAML_FORMATS);
        }

        if cfg!(feature = "ron") {
            extensions.extend(RON_FORMATS);
        }

        Self { extensions }
    }
}

impl Loader<Prototype> for ProtoLoader {
    type Error = PrototypeError;

    fn deserialize(
        bytes: &[u8],
        ctx: &mut ProtoLoadContext<Prototype, Self>,
    ) -> Result<Prototype, Self::Error> {
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

    fn extensions(&self) -> &[&'static str] {
        &self.extensions
    }
}
