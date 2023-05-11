use std::marker::PhantomData;
use std::sync::Arc;

use bevy::app::AppTypeRegistry;
use bevy::asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::{Handle, World};
use parking_lot::RwLock;

use crate::load::{Loader, ProtoLoadContext};
use crate::proto::{Config, Prototypical};
use crate::registration::{LoadQueue, ProtoRegistry};

pub(crate) struct ProtoAssetLoader<T: Prototypical, L: Loader<T>, C: Config<T>> {
    registry: AppTypeRegistry,
    proto_registry: Arc<RwLock<LoadQueue<T>>>,
    loader: L,
    _phantom: PhantomData<C>,
}

impl<T: Prototypical, L: Loader<T>, C: Config<T>> ProtoAssetLoader<T, L, C> {
    pub fn new(loader: L, world: &mut World) -> Self {
        world.init_resource::<AppTypeRegistry>();
        world.init_resource::<ProtoRegistry<T, C>>();

        Self {
            registry: world.resource::<AppTypeRegistry>().clone(),
            proto_registry: world.resource::<ProtoRegistry<T, C>>().load_queue().clone(),
            loader,
            _phantom: Default::default(),
        }
    }
}

impl<T: Prototypical, L: Loader<T>, C: Config<T>> AssetLoader for ProtoAssetLoader<T, L, C> {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async {
            let registry = self.registry.read();
            let mut ctx = ProtoLoadContext::<T, L>::new(&registry, &self.loader, load_context);

            // 1. Deserialize the prototype
            let prototype = L::deserialize(bytes, &mut ctx)?;
            let (prototype, _, mut dependency_paths) = ctx.preprocess_proto(prototype)?;
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
        self.loader.extensions()
    }
}
