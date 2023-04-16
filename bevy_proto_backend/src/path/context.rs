use std::path::Path;

use bevy::asset::AssetIo;

/// The context used when processing a [`ProtoPath`];
///
/// [`ProtoPath`]: crate::path::ProtoPath
pub trait ProtoPathContext {
    /// The base path of the [prototype] being loaded.
    ///
    /// [prototype]: crate::proto::Prototypical
    fn base_path(&self) -> &Path;
    /// The current [`AssetIo`].
    fn asset_io(&self) -> &dyn AssetIo;
    /// The allowable extensions as defined in the respective [`Config`].
    ///
    /// [`Config`]: crate::proto::Config
    fn extensions(&self) -> &[&'static str];
}
