use crate::from_to_default;
use bevy::math::Vec2;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::mesh::shape::Quad;

/// The schematic input type for [`Quad`].
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct QuadInput {
    /// Full width and height of the rectangle.
    pub size: Vec2,
    /// Horizontally-flip the texture coordinates of the resulting mesh.
    pub flip: bool,
}

from_to_default! {
    Quad,
    QuadInput,
    |value: Input| Self {
        size: value.size,
        flip: value.flip,
    }
}
