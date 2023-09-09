use crate::from_to_default;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::mesh::shape::Circle;

/// The schematic input type for [`Circle`].
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct CircleInput {
    /// Inscribed radius in the `XY` plane.
    pub radius: f32,
    /// The number of vertices used.
    pub vertices: usize,
}

from_to_default! {
    Circle,
    CircleInput,
    |value: Input| Self {
        radius: value.radius,
        vertices: value.vertices,
    }
}
