use crate::from_to_default;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::mesh::shape::Plane;

/// The schematic input type for [`Plane`].
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct PlaneInput {
    /// The total side length of the square.
    pub size: f32,
    /// The number of subdivisions in the mesh.
    ///
    /// 0 - is the original plane geometry, the 4 points in the XZ plane.
    ///
    /// 1 - is split by 1 line in the middle of the plane on both the X axis and the Z axis, resulting in a plane with 4 quads / 8 triangles.
    ///
    /// 2 - is a plane split by 2 lines on both the X and Z axes, subdividing the plane into 3 equal sections along each axis, resulting in a plane with 9 quads / 18 triangles.
    ///
    /// and so on...
    pub subdivisions: u32,
}

from_to_default! {
    Plane,
    PlaneInput,
    |value: Input| Self {
        size: value.size,
        subdivisions: value.subdivisions,
    }
}
