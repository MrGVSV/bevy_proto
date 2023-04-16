pub(crate) use derive::DeriveSchematic;
pub(crate) use external::ExternalSchematic;

const ATTRIBUTE: &str = "schematic";

mod container_attributes;
mod data;
mod derive;
mod external;
mod field_attributes;
mod fields;
mod idents;
mod input;
mod self_type;
mod structs;
mod utils;
mod variants;
