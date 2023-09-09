use crate::from_to_default;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::mesh::shape::{Capsule, CapsuleUvProfile};

/// The schematic input type for [`Capsule`].
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct CapsuleInput {
    /// Radius on the `XZ` plane.
    pub radius: f32,
    /// Number of sections in cylinder between hemispheres.
    pub rings: usize,
    /// Height of the middle cylinder on the `Y` axis, excluding the hemispheres.
    pub depth: f32,
    /// Number of latitudes, distributed by inclination. Must be even.
    pub latitudes: usize,
    /// Number of longitudes, or meridians, distributed by azimuth.
    pub longitudes: usize,
    /// Manner in which UV coordinates are distributed vertically.
    pub uv_profile: CapsuleUvProfileInput,
}

/// The schematic input type for [`CapsuleUvProfile`].
#[derive(Reflect, Copy, Clone)]
pub enum CapsuleUvProfileInput {
    Aspect,
    /// Hemispheres get UV space according to the ratio of latitudes to rings.
    Uniform,
    /// Upper third of the texture goes to the northern hemisphere, middle third to the cylinder
    /// and lower third to the southern one.
    Fixed,
}

from_to_default! {
    CapsuleUvProfile,
    CapsuleUvProfileInput,
    |value: Input| match value {
        Input::Aspect => Self::Aspect,
        Input::Uniform => Self::Uniform,
        Input::Fixed => Self::Fixed,
    }
}

from_to_default! {
    Capsule,
    CapsuleInput,
    |value: Input| Self {
        radius: value.radius,
        rings: value.rings,
        depth: value.depth,
        latitudes: value.latitudes,
        longitudes: value.longitudes,
        uv_profile: value.uv_profile.into(),
    }
}
