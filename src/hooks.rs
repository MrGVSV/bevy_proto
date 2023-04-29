//! Type aliases for common [config] hooks.
//!
//! [config]: crate::config::ProtoConfig

use bevy::asset::Handle;
use bevy::ecs::world::EntityMut;

use bevy_proto_backend::cycles::{Cycle, CycleResponse};
use bevy_proto_backend::schematics::DynamicSchematic;
use bevy_proto_backend::tree::EntityTree;

use crate::proto::Prototype;

pub type OnRegisterPrototype = Box<dyn FnMut(&Prototype, Handle<Prototype>) + Send + Sync>;
pub type OnReloadPrototype = Box<dyn FnMut(&Prototype, Handle<Prototype>) + Send + Sync>;
pub type OnUnregisterPrototype = Box<dyn FnMut(&String, Handle<Prototype>) + Send + Sync>;
pub type OnBeforeApplyPrototype =
    Box<dyn FnMut(&Prototype, &mut EntityMut, &EntityTree) + Send + Sync>;
pub type OnAfterApplyPrototype =
    Box<dyn FnMut(&Prototype, &mut EntityMut, &EntityTree) + Send + Sync>;
pub type OnBeforeRemovePrototype =
    Box<dyn FnMut(&Prototype, &mut EntityMut, &EntityTree) + Send + Sync>;
pub type OnAfterRemovePrototype =
    Box<dyn FnMut(&Prototype, &mut EntityMut, &EntityTree) + Send + Sync>;
pub type OnBeforeApplySchematic =
    Box<dyn FnMut(&DynamicSchematic, &mut EntityMut, &EntityTree) + Send + Sync>;
pub type OnAfterApplySchematic =
    Box<dyn FnMut(&DynamicSchematic, &mut EntityMut, &EntityTree) + Send + Sync>;
pub type OnBeforeRemoveSchematic =
    Box<dyn FnMut(&DynamicSchematic, &mut EntityMut, &EntityTree) + Send + Sync>;
pub type OnAfterRemoveSchematic =
    Box<dyn FnMut(&DynamicSchematic, &mut EntityMut, &EntityTree) + Send + Sync>;
pub type OnCycle = Box<dyn Fn(&Cycle<Prototype>) -> CycleResponse + Send + Sync>;
