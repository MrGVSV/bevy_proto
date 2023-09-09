use crate::from_to_default;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::mesh::shape::Cylinder;

/// The schematic input type for [`Cylinder`].
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct CylinderInput {
    /// Radius in the XZ plane.
    pub radius: f32,
    /// Height of the cylinder in the Y axis.
    pub height: f32,
    /// The number of vertices around each horizontal slice of the cylinder. If you are looking at the cylinder from
    /// above, this is the number of points you will see on the circle.
    /// A higher number will make it appear more circular.
    pub resolution: u32,
    /// The number of segments between the two ends. Setting this to 1 will have triangles spanning the full
    /// height of the cylinder. Setting it to 2 will have two sets of triangles with a horizontal slice in the middle of
    /// cylinder. Greater numbers increase triangles/slices in the same way.
    pub segments: u32,
}

from_to_default! {
    Cylinder,
    CylinderInput,
    |value: Input| Self {
        radius: value.radius,
        height: value.height,
        resolution: value.resolution,
        segments: value.segments,
    }
}
