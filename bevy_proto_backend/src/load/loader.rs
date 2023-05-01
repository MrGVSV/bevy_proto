use std::cmp::Reverse;
use std::marker::PhantomData;
use std::sync::Arc;

use anyhow::Error;
use bevy::app::AppTypeRegistry;
use bevy::asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::{FromWorld, Handle, World};
use parking_lot::RwLock;

use crate::load::ProtoLoadContext;
use crate::proto::Config;
use crate::proto::Prototypical;
use crate::registration::{LoadQueue, ProtoRegistry};

pub(crate) struct ProtoLoader<T: Prototypical> {
    registry: AppTypeRegistry,
    proto_registry: Arc<RwLock<LoadQueue<T>>>,
    extensions: Box<[&'static str]>,
    _phantom: PhantomData<T>,
}

impl<T: Prototypical> FromWorld for ProtoLoader<T> {
    fn from_world(world: &mut World) -> Self {
        let mut extensions = world.resource::<T::Config>().extensions();

        // Ensure the extensions are in order of longest to shortest
        extensions.sort_by_key(|ext| Reverse(ext.len()));

        Self {
            registry: world.resource::<AppTypeRegistry>().clone(),
            proto_registry: world.resource::<ProtoRegistry<T>>().load_queue().clone(),
            extensions,
            _phantom: Default::default(),
        }
    }
}

impl<T: Prototypical> AssetLoader for ProtoLoader<T> {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async {
            let registry = self.registry.read();
            let mut ctx = ProtoLoadContext::<T>::new(&registry, load_context, &self.extensions);

            // 1. Deserialize the prototype
            let mut prototype = T::deserialize(bytes, &mut ctx)?;
            let mut dependency_paths = ctx.preprocess_proto(&mut prototype)?;
            dependency_paths.append(ctx.child_paths_mut());

            // 2. Register
            let asset_handle: Handle<T> =
                load_context.get_handle(AssetPath::new_ref(load_context.path(), None));
            self.proto_registry
                .write()
                .queue(prototype.id().clone(), &asset_handle);

            // 3. Finish!
            let asset = LoadedAsset::new(prototype).with_dependencies(dependency_paths);
            load_context.set_default_asset(asset);

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
