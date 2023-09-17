use bevy::app::App;
use bevy::math::Mat4;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::camera::{
    Camera, CameraRenderGraph, OrthographicProjection, PerspectiveProjection, Projection,
};
use bevy::render::mesh::shape::{
    Box, Capsule, Circle, Cube, Cylinder, Icosphere, Plane, Quad, RegularPolygon, Torus, UVSphere,
};
use bevy::render::mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
use bevy::render::mesh::Mesh;
use bevy::render::primitives::Aabb;
use bevy::render::view::{ColorGrading, RenderLayers, Visibility};

use crate::assets::AssetSchematicAppExt;
use bevy_proto_derive::{impl_external_asset_schematic, impl_external_schematic};

use crate::impls::macros::{from_to_default, register_schematic};
use crate::tree::ProtoEntityList;

use super::shapes::*;

pub(crate) fn register(app: &mut App) {
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

    app.register_asset_schematic::<Mesh>()
        .register_asset_schematic::<SkinnedMeshInverseBindposes>()
        .register_type::<BoxInput>()
        .register_type::<CapsuleInput>()
        .register_type::<CapsuleUvProfileInput>()
        .register_type::<CircleInput>()
        .register_type::<CubeInput>()
        .register_type::<CylinderInput>()
        .register_type::<IcosphereInput>()
        .register_type::<PlaneInput>()
        .register_type::<QuadInput>()
        .register_type::<RegularPolygonInput>()
        .register_type::<TorusInput>()
        .register_type::<UVSphereInput>();
}

impl_external_schematic! {
    struct Aabb {}
}

impl_external_schematic! {
    struct Camera {}
}

impl_external_schematic! {
    struct CameraRenderGraph {}
}

impl_external_schematic! {
    #[schematic(from = ColorGradingInput)]
    struct ColorGrading {}
    // ---
    #[derive(Reflect)]
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
    #[derive(Reflect)]
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
    #[derive(Reflect)]
    pub struct RenderLayersInput(u8);
    impl From<RenderLayersInput> for RenderLayers {
        fn from(value: RenderLayersInput) -> Self {
            Self::layer(value.0)
        }
    }
}

impl_external_schematic! {
    pub struct SkinnedMesh {
        #[schematic(asset(inline))]
        pub inverse_bindposes: Handle<SkinnedMeshInverseBindposes>,
        #[schematic(from = ProtoEntityList)]
        pub joints: Vec<Entity>,
    }
}

impl_external_schematic! {
    enum Visibility {}
}

// === Assets === //

impl_external_asset_schematic! {
    #[asset_schematic(from = Vec<Mat4>)]
    struct SkinnedMeshInverseBindposes {}
}

impl_external_asset_schematic! {
    #[asset_schematic(from = MeshInput)]
    struct Mesh {}
}

/// The schematic input type for [`Mesh`].
#[derive(Reflect)]
pub enum MeshInput {
    Box(BoxInput),
    Capsule(CapsuleInput),
    Circle(CircleInput),
    Cube(CubeInput),
    Cylinder(CylinderInput),
    Icosphere(IcosphereInput),
    Plane(PlaneInput),
    Quad(QuadInput),
    RegularPolygon(RegularPolygonInput),
    Torus(TorusInput),
    UvSphere(UVSphereInput),
}

impl From<MeshInput> for Mesh {
    fn from(value: MeshInput) -> Self {
        match value {
            MeshInput::Box(input) => Box::from(input).into(),
            MeshInput::Capsule(input) => Capsule::from(input).into(),
            MeshInput::Circle(input) => Circle::from(input).into(),
            MeshInput::Cube(input) => Cube::from(input).into(),
            MeshInput::Cylinder(input) => Cylinder::from(input).into(),
            MeshInput::Icosphere(input) => Icosphere::from(input).try_into().unwrap(),
            MeshInput::Plane(input) => Plane::from(input).into(),
            MeshInput::Quad(input) => Quad::from(input).into(),
            MeshInput::RegularPolygon(input) => RegularPolygon::from(input).into(),
            MeshInput::Torus(input) => Torus::from(input).into(),
            MeshInput::UvSphere(input) => UVSphere::from(input).into(),
        }
    }
}
