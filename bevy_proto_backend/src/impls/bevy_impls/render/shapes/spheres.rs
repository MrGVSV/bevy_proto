use crate::from_to_default;
use bevy::prelude::shape::Icosphere;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::mesh::shape::UVSphere;

/// The schematic input type for [`UVSphere`].
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct UVSphereInput {
    pub radius: f32,
    pub sectors: usize,
    pub stacks: usize,
}

from_to_default! {
    UVSphere,
    UVSphereInput,
    |value: Input| Self {
        radius: value.radius,
        sectors: value.sectors,
        stacks: value.stacks,
    }
}

/// The schematic input type for [`Icosphere`].
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct IcosphereInput {
    /// The radius of the sphere.
    pub radius: f32,
    /// The number of subdivisions applied.
    pub subdivisions: usize,
}

from_to_default! {
    Icosphere,
    IcosphereInput,
    |value: Input| Self {
        radius: value.radius,
        subdivisions: value.subdivisions,
    }
}
