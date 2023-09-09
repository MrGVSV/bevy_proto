use crate::from_to_default;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::mesh::shape::Cube;

/// The schematic input type for [`Cube`].
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct CubeInput {
    pub size: f32,
}

from_to_default! {
    Cube,
    CubeInput,
    |value: Input| Self {
        size: value.size,
    }
}
