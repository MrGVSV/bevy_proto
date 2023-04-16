use bevy::app::App;

pub(crate) fn register_impls(app: &mut App) {
    bevy_impls::asset::register(app);
    bevy_impls::core::register(app);
    #[cfg(feature = "bevy_render")]
    bevy_impls::render::register(app);
    #[cfg(feature = "bevy_sprite")]
    bevy_impls::sprite::register(app);
    bevy_impls::transform::register(app);
    bevy_impls::window::register(app);
}

mod bevy_impls {
    pub mod asset {
        use bevy::app::App;
        use bevy::asset::{Asset, AssetServer, Handle};
        use bevy::ecs::world::EntityMut;

        use bevy_proto_derive::impl_external_schematic;

        use crate::proto::ProtoAsset;
        use crate::schematics::FromSchematicInput;
        use crate::tree::EntityTree;

        #[allow(unused_variables)]
        pub fn register(app: &mut App) {
            #[cfg(feature = "bevy_animation")]
            app.register_type::<Handle<bevy::prelude::AnimationClip>>()
                .register_type_data::<Handle<bevy::prelude::AnimationClip>, crate::schematics::ReflectSchematic>();
            #[cfg(feature = "bevy_audio")]
            app.register_type::<Handle<bevy::prelude::AudioSink>>()
                .register_type::<Handle<bevy::prelude::AudioSource>>()
                .register_type::<Handle<bevy::prelude::SpatialAudioSink>>()
                .register_type_data::<Handle<bevy::prelude::AudioSink>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::prelude::AudioSource>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::prelude::SpatialAudioSink>, crate::schematics::ReflectSchematic>();
            #[cfg(feature = "bevy_gltf")]
            app.register_type::<Handle<bevy::gltf::Gltf>>()
                .register_type::<Handle<bevy::gltf::GltfMesh>>()
                .register_type::<Handle<bevy::gltf::GltfPrimitive>>()
                .register_type::<Handle<bevy::gltf::GltfNode>>()
                .register_type_data::<Handle<bevy::gltf::Gltf>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::gltf::GltfMesh>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::gltf::GltfPrimitive>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::gltf::GltfNode>, crate::schematics::ReflectSchematic>();
            #[cfg(feature = "bevy_pbr")]
            app.register_type::<Handle<bevy::prelude::StandardMaterial>>()
                .register_type_data::<Handle<bevy::prelude::StandardMaterial>, crate::schematics::ReflectSchematic>();
            #[cfg(feature = "bevy_render")]
            app.register_type::<Handle<bevy::prelude::Image>>()
                .register_type::<Handle<bevy::render::mesh::skinning::SkinnedMeshInverseBindposes>>()
                .register_type::<Handle<bevy::prelude::Mesh>>()
                .register_type::<Handle<bevy::prelude::Shader>>()
                .register_type_data::<Handle<bevy::prelude::Image>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::render::mesh::skinning::SkinnedMeshInverseBindposes>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::prelude::Mesh>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::prelude::Shader>, crate::schematics::ReflectSchematic>();
            #[cfg(feature = "bevy_scene")]
            app.register_type::<Handle<bevy::prelude::DynamicScene>>()
                .register_type::<Handle<bevy::prelude::Scene>>()
                .register_type_data::<Handle<bevy::prelude::DynamicScene>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::prelude::Scene>, crate::schematics::ReflectSchematic>();
            #[cfg(feature = "bevy_sprite")]
            app.register_type::<Handle<bevy::prelude::ColorMaterial>>()
                .register_type::<Handle<bevy::prelude::TextureAtlas>>()
                .register_type_data::<Handle<bevy::prelude::ColorMaterial>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::prelude::TextureAtlas>, crate::schematics::ReflectSchematic>();
            #[cfg(feature = "bevy_text")]
            app.register_type::<Handle<bevy::prelude::Font>>()
                .register_type::<Handle<bevy::text::FontAtlasSet>>()
                .register_type_data::<Handle<bevy::prelude::Font>, crate::schematics::ReflectSchematic>()
                .register_type_data::<Handle<bevy::text::FontAtlasSet>, crate::schematics::ReflectSchematic>();
        }

        impl_external_schematic! {
            #[schematic(from = ProtoAsset)]
            struct Handle<T: Asset> {}
            // ---
            impl<T: Asset> FromSchematicInput<ProtoAsset> for Handle<T> {
                fn from_input(input: ProtoAsset, entity: &mut EntityMut, _: &EntityTree) -> Self {
                    match input {
                        ProtoAsset::AssetPath(path) => entity.world().resource::<AssetServer>().get_handle(path)
                    }
                }
            }
        }
    }

    pub mod core {
        use bevy::app::App;
        use bevy::core::Name;
        use bevy::reflect::{FromReflect, Reflect};

        use bevy_proto_derive::impl_external_schematic;

        use crate::schematics::ReflectSchematic;

        pub fn register(app: &mut App) {
            app.register_type::<Name>()
                .register_type_data::<Name, ReflectSchematic>();
        }

        impl_external_schematic! {
            #[schematic(from = NameInput)]
            struct Name {}
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct NameInput(String);
            impl From<NameInput> for Name {
                fn from(input: NameInput) -> Self {
                    Name::new(input.0)
                }
            }
        }
    }

    #[cfg(feature = "bevy_render")]
    pub mod render {
        use bevy::app::App;
        use bevy::render::view::Visibility;

        use bevy_proto_derive::impl_external_schematic;

        use crate::schematics::ReflectSchematic;

        pub fn register(app: &mut App) {
            app.register_type::<Visibility>()
                .register_type_data::<Visibility, ReflectSchematic>();
        }

        impl_external_schematic! {
            struct Visibility {}
        }
    }

    #[cfg(feature = "bevy_sprite")]
    pub mod sprite {
        use bevy::app::App;
        use bevy::sprite::Sprite;

        use bevy_proto_derive::impl_external_schematic;

        use crate::schematics::ReflectSchematic;

        pub fn register(app: &mut App) {
            app.register_type::<Sprite>()
                .register_type_data::<Sprite, ReflectSchematic>();
        }

        impl_external_schematic! {
            struct Sprite {}
        }
    }

    pub mod transform {
        use bevy::app::App;
        use bevy::transform::components::{GlobalTransform, Transform};

        use bevy_proto_derive::impl_external_schematic;

        use crate::schematics::ReflectSchematic;

        pub fn register(app: &mut App) {
            app.register_type::<Transform>()
                .register_type::<GlobalTransform>()
                .register_type_data::<Transform, ReflectSchematic>()
                .register_type_data::<GlobalTransform, ReflectSchematic>();
        }

        // FIXME: `TransformBundle` does not impl `Reflect`
        // impl_external_schematic! {
        //     #[schematic(from = TransformBundleInput)]
        //     struct TransformBundle {}
        //     // ---
        //     #[derive(Reflect, FromReflect)]
        //     pub struct TransformBundleInput {
        //         local: Transform,
        //         global: GlobalTransform
        //     }
        //     impl From<TransformBundleInput> for TransformBundle {
        //         fn from(value: TransformBundleInput) -> Self {
        //             Self {
        //                 local: value.local,
        //                 global: value.global
        //             }
        //         }
        //     }
        // }

        impl_external_schematic! {
            struct Transform {}
        }

        impl_external_schematic! {
            struct GlobalTransform {}
        }
    }

    pub mod window {
        use bevy::app::App;
        use bevy::reflect::{FromReflect, Reflect};
        use bevy::window::{PrimaryWindow, Window};

        use bevy_proto_derive::impl_external_schematic;

        use crate::schematics::ReflectSchematic;

        pub fn register(app: &mut App) {
            app.register_type::<Window>()
                .register_type::<PrimaryWindow>()
                .register_type_data::<Window, ReflectSchematic>()
                .register_type_data::<PrimaryWindow, ReflectSchematic>();
        }

        impl_external_schematic! {
            struct Window {}
        }

        impl_external_schematic! {
            #[schematic(from = PrimaryWindowInput)]
            struct PrimaryWindow;
            // ---
            #[derive(Reflect, FromReflect)]
            pub struct PrimaryWindowInput;
            impl From<PrimaryWindowInput> for PrimaryWindow {
                fn from(_: PrimaryWindowInput) -> Self {
                    Self
                }
            }
        }
    }
}
