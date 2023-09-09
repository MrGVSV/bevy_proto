use crate::from_to_default;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::mesh::shape::Torus;

/// The schematic input type for [`Torus`].
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct TorusInput {
    pub radius: f32,
    pub ring_radius: f32,
    pub subdivisions_segments: usize,
    pub subdivisions_sides: usize,
}

from_to_default! {
    Torus,
    TorusInput,
    |value: Input| Self {
        radius: value.radius,
        ring_radius: value.ring_radius,
        subdivisions_segments: value.subdivisions_segments,
        subdivisions_sides: value.subdivisions_sides,
    }
}
