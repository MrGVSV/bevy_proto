//! A collection of custom schematics (requires the `custom_schematics` feature).
//!
//! The types in this module are meant to provide [`Schematic`] implementations of
//! common types.
//! For example, many bundles in Bevy do not meet the requirements to implement
//! `Schematic` themselves, so they have equivalent types defined here as a stopgap.

use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{Component, GlobalTransform, Transform};
use bevy::reflect::{FromReflect, Reflect};

use bevy_proto_backend::schematics::{ReflectSchematic, Schematic};

pub(crate) fn register_custom_schematics(app: &mut App) {
    app.register_type::<TransformBundle>();
    #[cfg(feature = "bevy_sprite")]
    app.register_type::<SpriteBundle>();
    #[cfg(feature = "bevy_render")]
    app.register_type::<VisibilityBundle>();
}

/// A [`Schematic`] implementation of [`SpriteBundle`].
///
/// [`SpriteBundle`]: bevy::prelude::SpriteBundle
#[cfg(feature = "bevy_sprite")]
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::prelude::SpriteBundle)]
pub struct SpriteBundle {
    #[reflect(default)]
    pub sprite: bevy::prelude::Sprite,
    #[reflect(default)]
    pub transform: Transform,
    #[reflect(default)]
    pub global_transform: GlobalTransform,
    #[schematic(asset(lazy))]
    pub texture: Handle<bevy::prelude::Image>,
    #[reflect(default)]
    pub visibility: bevy::prelude::Visibility,
    #[reflect(ignore, default)]
    pub computed_visibility: bevy::prelude::ComputedVisibility,
}

#[cfg(feature = "bevy_sprite")]
impl From<SpriteBundle> for bevy::prelude::SpriteBundle {
    fn from(value: SpriteBundle) -> Self {
        Self {
            sprite: value.sprite,
            transform: value.transform,
            global_transform: value.global_transform,
            texture: value.texture,
            visibility: value.visibility,
            computed_visibility: value.computed_visibility,
        }
    }
}

/// A [`Schematic`] implementation of [`TransformBundle`].
///
/// [`TransformBundle`]: bevy::prelude::TransformBundle
#[derive(Component, Schematic, Reflect, FromReflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::prelude::TransformBundle)]
pub struct TransformBundle {
    #[reflect(default)]
    pub local: Transform,
    #[reflect(default)]
    pub global: GlobalTransform,
}

impl From<TransformBundle> for bevy::prelude::TransformBundle {
    fn from(value: TransformBundle) -> Self {
        Self {
            local: value.local,
            global: value.global,
        }
    }
}

/// A [`Schematic`] implementation of [`VisibilityBundle`].
///
/// [`VisibilityBundle`]: bevy::prelude::VisibilityBundle
#[cfg(feature = "bevy_render")]
#[derive(Component, Schematic, Reflect, FromReflect)]
#[reflect(Schematic)]
#[schematic(into = bevy::prelude::VisibilityBundle)]
pub struct VisibilityBundle {
    #[reflect(default)]
    pub visibility: bevy::prelude::Visibility,
    #[reflect(ignore, default)]
    pub computed: bevy::prelude::ComputedVisibility,
}

#[cfg(feature = "bevy_render")]
impl From<VisibilityBundle> for bevy::prelude::VisibilityBundle {
    fn from(value: VisibilityBundle) -> Self {
        Self {
            visibility: value.visibility,
            computed: value.computed,
        }
    }
}
