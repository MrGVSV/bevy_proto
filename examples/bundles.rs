#![allow(unused_doc_comments)]

use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy::reflect::FromReflect;

use bevy_proto::prelude::*;

#[derive(Reflect, FromReflect, Component)]
#[reflect(ProtoComponent)]
struct SpriteBundleDef {
    pub texture_path: String,
}

impl ProtoComponent for SpriteBundleDef {
    fn apply(&self, entity: &mut EntityMut) {
        // Access the `AssetServer` and load our texture
        let asset_server = entity.world().resource::<AssetServer>();
        let texture = asset_server.load(&self.texture_path);

        entity.insert_bundle(SpriteBundle {
            texture,
            ..Default::default()
        });
    }

    /// This is optional, but we can preload assets we know we'll need
    fn preload_assets(&mut self, preloader: &mut AssetPreloader) {
        // Since we know we'll always need the image pointed to by `texture_path`,
        // we can go ahead and say that this prototype _depends_ on that asset.
        // This will make sure that it is loaded alongside our prototype and will
        // remain loaded at least as long as the prototype.
        preloader.preload_dependency(&self.texture_path);
    }

    fn as_reflect(&self) -> &dyn Reflect {
        self
    }
}

fn load_prototype(asset_server: Res<AssetServer>, mut manager: ProtoManager) {
    let handle = asset_server.load("prototypes/bundles/sprite_test.prototype.yaml");
    manager.add(handle);
}

fn spawn_sprite(mut commands: Commands, manager: ProtoManager, mut has_ran: Local<bool>) {
    if *has_ran {
        return;
    }

    if let Some(proto) = manager.get("Sprite Test") {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        proto.spawn(&mut commands);
        *has_ran = true;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ProtoPlugin::<Prototype>::default())
        // !!! Make sure you register your types !!! //
        .register_type::<SpriteBundleDef>()
        .add_startup_system(load_prototype)
        .add_system(spawn_sprite)
        .run();
}
