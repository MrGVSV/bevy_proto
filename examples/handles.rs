#![allow(unused_doc_comments)]

use bevy::prelude::*;
use bevy::reflect::FromReflect;

use bevy_proto::prelude::*;

/// A definition struct used to construct a `SpriteBundle`.
///
/// We use `#[proto_comp(into_bundle = "SpriteBundle")]` to tell the derive macro
/// that we should convert this `ProtoComponent` into a `SpriteBundle`.
#[derive(Reflect, FromReflect, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
#[proto_comp(into_bundle = "SpriteBundle")]
struct SpriteBundleDef {
    /// This tells the derive macro that we should preload this texture.
    ///
    /// Internally, this generates the `preload_assets` method, loads the texture,
    /// and stores it in the `texture` field (since `HandlePath` can be used to store handles)
    #[proto_comp(preload(type = "Image", dest = "texture"))]
    texture: HandlePath<Image>,
}

impl From<SpriteBundleDef> for SpriteBundle {
    fn from(def: SpriteBundleDef) -> Self {
        Self {
            texture: def.texture.handle().unwrap().clone(),
            ..Default::default()
        }
    }
}

fn load_prototype(asset_server: Res<AssetServer>, mut manager: ProtoManager<Prototype>) {
    let handle = asset_server.load("prototypes/handles/sprite_test.prototype.yaml");
    manager.add(handle);
}

fn spawn_sprite(
    mut commands: Commands,
    manager: ProtoManager<Prototype>,
    mut has_ran: Local<bool>,
) {
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
        .register_type::<HandlePath<Image>>()
        .add_startup_system(load_prototype)
        .add_system(spawn_sprite)
        .run();
}
