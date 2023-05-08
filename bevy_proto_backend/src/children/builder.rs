use std::path::Path;

use bevy::asset::{AssetIo, Handle, LoadedAsset};

use crate::load::{Loader, ProtoLoadContext};
use crate::path::{ProtoPath, ProtoPathContext};
use crate::proto::Prototypical;

/// A helper struct for properly building out a [prototype's] children.
///
/// [prototype's]: Prototypical
pub struct ProtoChildBuilder<'ctx, 'load_ctx, T: Prototypical, L: Loader<T>> {
    pub(crate) context: ProtoLoadContext<'ctx, 'load_ctx, T, L>,
    child_count: usize,
}

impl<'ctx, 'load_ctx, T: Prototypical, L: Loader<T>> ProtoChildBuilder<'ctx, 'load_ctx, T, L> {
    pub(crate) fn new(context: ProtoLoadContext<'ctx, 'load_ctx, T, L>) -> Self {
        Self {
            context,
            child_count: 0,
        }
    }

    /// Add the given child to the parent.
    pub fn add_child(&mut self, mut child: T) -> Result<Handle<T>, L::Error> {
        let deps = self.context.preprocess_proto(&mut child)?;

        let child_handle = self.context.set_labeled_asset(
            &format!("{:0>3}--{:0>3}", self.context.depth(), self.child_count),
            LoadedAsset::new(child).with_dependencies(deps),
        );

        self.child_count += 1;

        Ok(child_handle)
    }

    /// Add the child with the given path to the parent.
    pub fn add_child_path(&mut self, child_path: ProtoPath) -> Result<Handle<T>, L::Error> {
        self.context
            .child_paths_mut()
            .push(child_path.asset_path().to_owned());

        self.child_count += 1;

        Ok(self.context.get_handle(child_path))
    }

    /// Access the current [`ProtoLoadContext`].
    pub fn context(&self) -> &ProtoLoadContext<'ctx, 'load_ctx, T, L> {
        &self.context
    }

    /// Access the current [`ProtoLoadContext`] mutably.
    pub fn context_mut(&mut self) -> &mut ProtoLoadContext<'ctx, 'load_ctx, T, L> {
        &mut self.context
    }
}

impl<'ctx, 'load_ctx, T: Prototypical, L: Loader<T>> ProtoPathContext
    for ProtoChildBuilder<'ctx, 'load_ctx, T, L>
{
    fn base_path(&self) -> &Path {
        self.context.base_path()
    }

    fn asset_io(&self) -> &dyn AssetIo {
        self.context.load_context().asset_io()
    }

    fn extensions(&self) -> &[&'static str] {
        self.context.extensions()
    }
}
