use bevy::app::App;
use bevy::pbr::wireframe::Wireframe;
use bevy::pbr::{
    AlphaMode, CascadeShadowConfig, CascadeShadowConfigBuilder, ClusterConfig, DirectionalLight,
    EnvironmentMapLight, FogFalloff, FogSettings, NotShadowCaster, NotShadowReceiver,
    ParallaxMappingMethod, PointLight, SpotLight, StandardMaterial,
};
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::prelude::Image;
use bevy::render::render_resource::Face;

use crate::assets::{AssetSchematicAppExt, ProtoAsset};
use crate::deps::DependenciesBuilder;
use crate::impls::macros::{from_to_default, register_schematic};
use crate::proto::ProtoColor;
use crate::schematics::{
    FromSchematicInput, FromSchematicPreloadInput, SchematicContext, SchematicId,
};
use bevy_proto_derive::{impl_external_asset_schematic, impl_external_schematic};

pub(super) fn register(app: &mut App) {
    register_schematic!(
        app,
        CascadeShadowConfig,
        ClusterConfig,
        DirectionalLight,
        EnvironmentMapLight,
        FogSettings,
        NotShadowCaster,
        NotShadowReceiver,
        PointLight,
        SpotLight,
        Wireframe,
    );

    app.register_asset_schematic::<StandardMaterial>()
        .register_type::<FaceInput>();
}

impl_external_schematic! {
    #[schematic(from = CascadeShadowConfigInput)]
    struct CascadeShadowConfig {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct CascadeShadowConfigInput {
        pub num_cascades: usize,
        pub minimum_distance: f32,
        pub maximum_distance: f32,
        pub first_cascade_far_bound: f32,
        pub overlap_proportion: f32,
    }
    impl From<CascadeShadowConfigInput> for CascadeShadowConfig {
        fn from(value: CascadeShadowConfigInput) -> Self {
            CascadeShadowConfigBuilder {
                num_cascades: value.num_cascades,
                minimum_distance: value.minimum_distance,
                maximum_distance: value.maximum_distance,
                first_cascade_far_bound: value.first_cascade_far_bound,
                overlap_proportion: value.overlap_proportion,
            }.into()
        }
    }
    impl Default for CascadeShadowConfigInput {
        fn default() -> Self {
            let base = CascadeShadowConfigBuilder::default();
            Self {
                num_cascades: base.num_cascades,
                minimum_distance: base.minimum_distance,
                maximum_distance: base.maximum_distance,
                first_cascade_far_bound: base.first_cascade_far_bound,
                overlap_proportion: base.overlap_proportion,
            }
        }
    }
}

impl_external_schematic! {
    enum ClusterConfig {}
}

impl_external_schematic! {
    struct DirectionalLight {}
}

impl_external_schematic! {
    pub struct EnvironmentMapLight {
        #[schematic(asset)]
        pub diffuse_map: Handle<Image>,
        #[schematic(asset)]
        pub specular_map: Handle<Image>,
    }
}

impl_external_schematic! {
    #[schematic(from = FogSettingsInput)]
    struct FogSettings {}
    // ---
    #[derive(Reflect)]
    #[reflect(Default)]
    pub struct FogSettingsInput {
        pub color: ProtoColor,
        pub directional_light_color: ProtoColor,
        pub directional_light_exponent: f32,
        pub falloff: FogFalloff,
    }
    from_to_default! {
        FogSettings,
        FogSettingsInput,
        |value: Input| Self {
            color: value.color.into(),
            directional_light_color: value.directional_light_color.into(),
            directional_light_exponent: value.directional_light_exponent,
            falloff: value.falloff,
        }
    }
}

impl_external_schematic! {
    struct NotShadowCaster;
}

impl_external_schematic! {
    struct NotShadowReceiver;
}

impl_external_schematic! {
    struct PointLight {}
}

impl_external_schematic! {
    struct SpotLight {}
}

impl_external_schematic! {
    struct Wireframe;
}

impl_external_asset_schematic! {
    #[asset_schematic(from = StandardMaterialInput)]
    struct StandardMaterial {}
}

#[derive(Reflect)]
#[reflect(Default)]
pub struct StandardMaterialInput {
    pub base_color: ProtoColor,
    pub base_color_texture: Option<ProtoAsset<Image>>,
    pub emissive: ProtoColor,
    pub emissive_texture: Option<ProtoAsset<Image>>,
    pub perceptual_roughness: f32,
    pub metallic: f32,
    pub metallic_roughness_texture: Option<ProtoAsset<Image>>,
    pub reflectance: f32,
    pub normal_map_texture: Option<ProtoAsset<Image>>,
    pub flip_normal_map_y: bool,
    pub occlusion_texture: Option<ProtoAsset<Image>>,
    pub double_sided: bool,
    pub cull_mode: Option<FaceInput>,
    pub unlit: bool,
    pub fog_enabled: bool,
    pub alpha_mode: AlphaMode,
    pub depth_bias: f32,
    pub depth_map: Option<ProtoAsset<Image>>,
    pub parallax_depth_scale: f32,
    pub parallax_mapping_method: ParallaxMappingMethod,
    pub max_parallax_layer_count: f32,
}

impl Default for StandardMaterialInput {
    fn default() -> Self {
        let base = StandardMaterial::default();

        Self {
            base_color: base.base_color.into(),
            base_color_texture: base.base_color_texture.map(ProtoAsset::Handle),
            emissive: base.emissive.into(),
            emissive_texture: base.emissive_texture.map(ProtoAsset::Handle),
            perceptual_roughness: base.perceptual_roughness,
            metallic: base.metallic,
            metallic_roughness_texture: base.metallic_roughness_texture.map(ProtoAsset::Handle),
            reflectance: base.reflectance,
            normal_map_texture: base.normal_map_texture.map(ProtoAsset::Handle),
            flip_normal_map_y: base.flip_normal_map_y,
            occlusion_texture: base.occlusion_texture.map(ProtoAsset::Handle),
            double_sided: base.double_sided,
            cull_mode: base.cull_mode.map(Into::into),
            unlit: base.unlit,
            fog_enabled: base.fog_enabled,
            alpha_mode: base.alpha_mode,
            depth_bias: base.depth_bias,
            depth_map: base.depth_map.map(ProtoAsset::Handle),
            parallax_depth_scale: base.parallax_depth_scale,
            parallax_mapping_method: base.parallax_mapping_method,
            max_parallax_layer_count: base.max_parallax_layer_count,
        }
    }
}

impl FromSchematicInput<StandardMaterialInput> for StandardMaterial {
    fn from_input(
        input: StandardMaterialInput,
        id: SchematicId,
        context: &mut SchematicContext,
    ) -> Self {
        Self {
            base_color: input.base_color.into(),
            base_color_texture: input.base_color_texture.map(|value| {
                FromSchematicInput::from_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x0ec0883fecc74db5b30c8e7855f0aeed,
                    )),
                    context,
                )
            }),
            emissive: input.emissive.into(),
            emissive_texture: input.emissive_texture.map(|value| {
                FromSchematicInput::from_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x3f0b895dda574a55887cc32f9c20285a,
                    )),
                    context,
                )
            }),
            perceptual_roughness: input.perceptual_roughness,
            metallic: input.metallic,
            metallic_roughness_texture: input.metallic_roughness_texture.map(|value| {
                FromSchematicInput::from_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0xd063b6d68c33437a9bd341be88d64c06,
                    )),
                    context,
                )
            }),
            reflectance: input.reflectance,
            normal_map_texture: input.normal_map_texture.map(|value| {
                FromSchematicInput::from_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x0ede476ee6994cf986f5a333bd385b62,
                    )),
                    context,
                )
            }),
            flip_normal_map_y: input.flip_normal_map_y,
            occlusion_texture: input.occlusion_texture.map(|value| {
                FromSchematicInput::from_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x19d7d9309c184fb58e31166652c5bee8,
                    )),
                    context,
                )
            }),
            double_sided: input.double_sided,
            cull_mode: input.cull_mode.map(Into::into),
            unlit: input.unlit,
            fog_enabled: input.fog_enabled,
            alpha_mode: input.alpha_mode,
            depth_bias: input.depth_bias,
            depth_map: input.depth_map.map(|value| {
                FromSchematicInput::from_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x833fe20f1b7d4b469e9d6a86f873f7ae,
                    )),
                    context,
                )
            }),
            parallax_depth_scale: input.parallax_depth_scale,
            parallax_mapping_method: input.parallax_mapping_method,
            max_parallax_layer_count: input.max_parallax_layer_count,
        }
    }
}

impl FromSchematicPreloadInput<StandardMaterialInput> for StandardMaterial {
    fn from_preload_input(
        input: StandardMaterialInput,
        id: SchematicId,
        dependencies: &mut DependenciesBuilder,
    ) -> Self {
        Self {
            base_color: input.base_color.into(),
            base_color_texture: input.base_color_texture.map(|value| {
                FromSchematicPreloadInput::from_preload_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x0ec0883fecc74db5b30c8e7855f0aeed,
                    )),
                    dependencies,
                )
            }),
            emissive: input.emissive.into(),
            emissive_texture: input.emissive_texture.map(|value| {
                FromSchematicPreloadInput::from_preload_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x3f0b895dda574a55887cc32f9c20285a,
                    )),
                    dependencies,
                )
            }),
            perceptual_roughness: input.perceptual_roughness,
            metallic: input.metallic,
            metallic_roughness_texture: input.metallic_roughness_texture.map(|value| {
                FromSchematicPreloadInput::from_preload_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0xd063b6d68c33437a9bd341be88d64c06,
                    )),
                    dependencies,
                )
            }),
            reflectance: input.reflectance,
            normal_map_texture: input.normal_map_texture.map(|value| {
                FromSchematicPreloadInput::from_preload_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x0ede476ee6994cf986f5a333bd385b62,
                    )),
                    dependencies,
                )
            }),
            flip_normal_map_y: input.flip_normal_map_y,
            occlusion_texture: input.occlusion_texture.map(|value| {
                FromSchematicPreloadInput::from_preload_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x19d7d9309c184fb58e31166652c5bee8,
                    )),
                    dependencies,
                )
            }),
            double_sided: input.double_sided,
            cull_mode: input.cull_mode.map(Into::into),
            unlit: input.unlit,
            fog_enabled: input.fog_enabled,
            alpha_mode: input.alpha_mode,
            depth_bias: input.depth_bias,
            depth_map: input.depth_map.map(|value| {
                FromSchematicPreloadInput::from_preload_input(
                    value,
                    id.next(bevy::utils::Uuid::from_u128(
                        0x833fe20f1b7d4b469e9d6a86f873f7ae,
                    )),
                    dependencies,
                )
            }),
            parallax_depth_scale: input.parallax_depth_scale,
            parallax_mapping_method: input.parallax_mapping_method,
            max_parallax_layer_count: input.max_parallax_layer_count,
        }
    }
}

#[derive(Reflect)]
pub enum FaceInput {
    Front = 0,
    Back = 1,
}

impl From<FaceInput> for Face {
    fn from(value: FaceInput) -> Self {
        match value {
            FaceInput::Front => Face::Front,
            FaceInput::Back => Face::Back,
        }
    }
}

impl From<Face> for FaceInput {
    fn from(value: Face) -> Self {
        match value {
            Face::Front => FaceInput::Front,
            Face::Back => FaceInput::Back,
        }
    }
}
