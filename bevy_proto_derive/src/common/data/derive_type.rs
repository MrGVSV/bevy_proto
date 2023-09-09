use crate::utils::constants::{ASSET_SCHEMATIC_ATTR, SCHEMATIC_ATTR};

/// Indicates which derive macro is being used.
#[derive(Copy, Clone)]
pub(crate) enum DeriveType {
    /// The `Schematic` derive macro.
    Schematic,
    /// The `AssetSchematic` derive macro.
    AssetSchematic,
}

impl DeriveType {
    pub fn attr_name(&self) -> &'static str {
        match self {
            Self::Schematic => SCHEMATIC_ATTR,
            Self::AssetSchematic => ASSET_SCHEMATIC_ATTR,
        }
    }
}
