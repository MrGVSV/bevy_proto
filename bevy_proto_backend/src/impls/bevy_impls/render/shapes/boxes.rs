use bevy::math::Vec3;
use bevy::reflect::Reflect;
use bevy::render::mesh::shape::Box;

/// The schematic input type for [`Box`].
#[derive(Reflect)]
pub enum BoxInput {
    Size(Vec3),
    Corners(Vec3, Vec3),
}

impl Default for BoxInput {
    fn default() -> Self {
        Box::default().into()
    }
}

impl From<BoxInput> for Box {
    fn from(value: BoxInput) -> Self {
        match value {
            BoxInput::Size(size) => Box::new(size.x, size.y, size.z),
            BoxInput::Corners(a, b) => Box::from_corners(a, b),
        }
    }
}

impl From<Box> for BoxInput {
    fn from(value: Box) -> Self {
        let Box {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        } = value;
        Self::Corners(
            Vec3::new(min_x, min_y, min_z),
            Vec3::new(max_x, max_y, max_z),
        )
    }
}
