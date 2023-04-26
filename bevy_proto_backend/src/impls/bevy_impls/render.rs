use bevy::app::App;
use bevy::prelude::{Camera, Entity, OrthographicProjection, PerspectiveProjection, Projection};
use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};
use bevy::render::camera::CameraRenderGraph;
use bevy::render::mesh::skinning::SkinnedMesh;
use bevy::render::primitives::Aabb;
use bevy::render::view::{ColorGrading, RenderLayers, Visibility};

use crate::impls::macros::{from_to_default, register_schematic};
use crate::tree::ProtoEntityList;
use bevy_proto_derive::impl_external_schematic;

pub(super) fn register(app: &mut App) {
    register_schematic!(
        app,
        Aabb,
        Camera,
        CameraRenderGraph,
        ColorGrading,
        OrthographicProjection,
        PerspectiveProjection,
        Projection,
        RenderLayers,
        SkinnedMesh,
        Visibility,
    );
}

impl_external_schematic! {
    struct Aabb {}
}

impl_external_schematic! {
    struct Camera {}
}

impl_external_schematic! {
    #[schematic(from = CameraRenderGraphInput)]
    struct CameraRenderGraph();
    // ---
    #[derive(Reflect, FromReflect)]
    pub struct CameraRenderGraphInput(String);
    impl From<CameraRenderGraphInput> for CameraRenderGraph {
        fn from(value: CameraRenderGraphInput) -> Self {
            Self::new(value.0)
        }
    }
}

impl_external_schematic! {
    #[schematic(from = ColorGradingInput)]
    struct ColorGrading {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub struct ColorGradingInput {
        pub exposure: f32,
        pub gamma: f32,
        pub pre_saturation: f32,
        pub post_saturation: f32,
    }
    from_to_default! {
        ColorGrading,
        ColorGradingInput,
        |value: Input| Self {
            exposure: value.exposure,
            gamma: value.gamma,
            pre_saturation: value.pre_saturation,
            post_saturation: value.post_saturation,
        }
    }
}

impl_external_schematic! {
    struct OrthographicProjection {}
}

impl_external_schematic! {
    struct PerspectiveProjection {}
}

impl_external_schematic! {
    #[schematic(from = ProjectionInput)]
    enum Projection {}
    // ---
    #[derive(Reflect, FromReflect)]
    #[reflect(Default)]
    pub enum ProjectionInput {
        Perspective(PerspectiveProjection),
        Orthographic(OrthographicProjection),
    }
    from_to_default! {
        Projection,
        ProjectionInput,
        |value: Input| match value {
            Input::Perspective(projection) => Self::Perspective(projection),
            Input::Orthographic(projection) => Self::Orthographic(projection),
        }
    }
}

impl_external_schematic! {
    #[schematic(from = RenderLayersInput)]
    struct RenderLayers();
    // ---
    #[derive(Reflect, FromReflect)]
    pub struct RenderLayersInput(u8);
    impl From<RenderLayersInput> for RenderLayers {
        fn from(value: RenderLayersInput) -> Self {
            Self::layer(value.0)
        }
    }
}

impl_external_schematic! {
    pub struct SkinnedMesh {
        #[schematic(asset)]
        pub inverse_bindposes: Handle<SkinnedMeshInverseBindposes>,
        #[schematic(from = ProtoEntityList)]
        pub joints: Vec<Entity>,
    }
}

impl_external_schematic! {
    enum Visibility {}
}
