use crate::from_to_default;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::mesh::shape::RegularPolygon;

/// The schematic input type for [`RegularPolygon`].
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct RegularPolygonInput {
    /// Circumscribed radius in the `XY` plane.
    ///
    /// In other words, the vertices of this polygon will all touch a circle of this radius.
    pub radius: f32,
    /// Number of sides.
    pub sides: usize,
}

from_to_default! {
    RegularPolygon,
    RegularPolygonInput,
    |value: Input| Self {
        radius: value.radius,
        sides: value.sides,
    }
}
