//! Configuration for [prototypes].
//!
//! [prototypes]: Prototype

use bevy::asset::Handle;
use bevy::prelude::Resource;

use bevy_proto_backend::cycles::{Cycle, CycleResponse};
use bevy_proto_backend::proto::Config;
use bevy_proto_backend::schematics::{DynamicSchematic, SchematicContext};

use crate::hooks::{
    OnAfterApplyPrototype, OnAfterApplySchematic, OnAfterRemovePrototype, OnAfterRemoveSchematic,
    OnBeforeApplyPrototype, OnBeforeApplySchematic, OnBeforeRemovePrototype,
    OnBeforeRemoveSchematic, OnCycle, OnRegisterPrototype, OnReloadPrototype,
    OnUnregisterPrototype,
};
use crate::proto::Prototype;

/// The config resource for [`Prototype`].
#[derive(Resource, Default)]
pub struct ProtoConfig {
    on_register_prototype: Option<OnRegisterPrototype>,
    on_reload_prototype: Option<OnReloadPrototype>,
    on_unregister_prototype: Option<OnUnregisterPrototype>,
    on_before_apply_prototype: Option<OnBeforeApplyPrototype>,
    on_after_apply_prototype: Option<OnAfterApplyPrototype>,
    on_before_remove_prototype: Option<OnBeforeRemovePrototype>,
    on_after_remove_prototype: Option<OnAfterRemovePrototype>,
    on_before_apply_schematic: Option<OnBeforeApplySchematic>,
    on_after_apply_schematic: Option<OnAfterApplySchematic>,
    on_before_remove_schematic: Option<OnBeforeRemoveSchematic>,
    on_after_remove_schematic: Option<OnAfterRemoveSchematic>,
    on_cycle: Option<OnCycle>,
}

impl ProtoConfig {
    /// Register a callback for [`Config::on_register_prototype`].
    pub fn on_register_prototype(mut self, callback: OnRegisterPrototype) -> Self {
        self.on_register_prototype = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_reload_prototype`].
    pub fn on_reload_prototype(mut self, callback: OnReloadPrototype) -> Self {
        self.on_reload_prototype = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_unregister_prototype`].
    pub fn on_unregister_prototype(mut self, callback: OnUnregisterPrototype) -> Self {
        self.on_unregister_prototype = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_before_apply_prototype`].
    pub fn on_before_apply_prototype(mut self, callback: OnBeforeApplyPrototype) -> Self {
        self.on_before_apply_prototype = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_after_apply_prototype`].
    pub fn on_after_apply_prototype(mut self, callback: OnAfterApplyPrototype) -> Self {
        self.on_after_apply_prototype = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_before_remove_prototype`].
    pub fn on_before_remove_prototype(mut self, callback: OnBeforeRemovePrototype) -> Self {
        self.on_before_remove_prototype = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_after_remove_prototype`].
    pub fn on_after_remove_prototype(mut self, callback: OnAfterRemovePrototype) -> Self {
        self.on_after_remove_prototype = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_before_apply_schematic`].
    pub fn on_before_apply_schematic(mut self, callback: OnBeforeApplySchematic) -> Self {
        self.on_before_apply_schematic = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_after_apply_schematic`].
    pub fn on_after_apply_schematic(mut self, callback: OnAfterApplySchematic) -> Self {
        self.on_after_apply_schematic = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_before_remove_schematic`].
    pub fn on_before_remove_schematic(mut self, callback: OnBeforeRemoveSchematic) -> Self {
        self.on_before_remove_schematic = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_after_remove_schematic`].
    pub fn on_after_remove_schematic(mut self, callback: OnAfterRemoveSchematic) -> Self {
        self.on_after_remove_schematic = Some(callback);
        self
    }

    /// Register a callback for [`Config::on_cycle`].
    pub fn on_cycle(mut self, callback: OnCycle) -> Self {
        self.on_cycle = Some(callback);
        self
    }
}

impl Config<Prototype> for ProtoConfig {
    fn on_register_prototype(&mut self, prototype: &Prototype, handle: Handle<Prototype>) {
        if let Some(on_register_prototype) = &mut self.on_register_prototype {
            on_register_prototype(prototype, handle);
        }
    }

    fn on_reload_prototype(&mut self, prototype: &Prototype, handle: Handle<Prototype>) {
        if let Some(on_reload_prototype) = &mut self.on_reload_prototype {
            on_reload_prototype(prototype, handle);
        }
    }

    fn on_unregister_prototype(&mut self, id: &String, handle: Handle<Prototype>) {
        if let Some(on_unregister_prototype) = &mut self.on_unregister_prototype {
            on_unregister_prototype(id, handle);
        }
    }

    fn on_before_apply_prototype(&mut self, prototype: &Prototype, context: &mut SchematicContext) {
        if let Some(on_before_apply_prototype) = &mut self.on_before_apply_prototype {
            on_before_apply_prototype(prototype, context);
        }
    }

    fn on_after_apply_prototype(&mut self, prototype: &Prototype, context: &mut SchematicContext) {
        if let Some(on_after_apply_prototype) = &mut self.on_after_apply_prototype {
            on_after_apply_prototype(prototype, context);
        }
    }

    fn on_before_remove_prototype(
        &mut self,
        prototype: &Prototype,
        context: &mut SchematicContext,
    ) {
        if let Some(on_before_remove_prototype) = &mut self.on_before_remove_prototype {
            on_before_remove_prototype(prototype, context);
        }
    }

    fn on_after_remove_prototype(&mut self, prototype: &Prototype, context: &mut SchematicContext) {
        if let Some(on_after_remove_prototype) = &mut self.on_after_remove_prototype {
            on_after_remove_prototype(prototype, context);
        }
    }

    fn on_before_apply_schematic(
        &mut self,
        schematic: &DynamicSchematic,
        context: &mut SchematicContext,
    ) {
        if let Some(on_before_apply_schematic) = &mut self.on_before_apply_schematic {
            on_before_apply_schematic(schematic, context);
        }
    }

    fn on_after_apply_schematic(
        &mut self,
        schematic: &DynamicSchematic,
        context: &mut SchematicContext,
    ) {
        if let Some(on_after_apply_schematic) = &mut self.on_after_apply_schematic {
            on_after_apply_schematic(schematic, context);
        }
    }

    fn on_before_remove_schematic(
        &mut self,
        schematic: &DynamicSchematic,
        context: &mut SchematicContext,
    ) {
        if let Some(on_before_remove_schematic) = &mut self.on_before_remove_schematic {
            on_before_remove_schematic(schematic, context);
        }
    }

    fn on_after_remove_schematic(
        &mut self,
        schematic: &DynamicSchematic,
        context: &mut SchematicContext,
    ) {
        if let Some(on_after_remove_schematic) = &mut self.on_after_remove_schematic {
            on_after_remove_schematic(schematic, context);
        }
    }

    fn on_cycle(&self, cycle: &Cycle<Prototype>) -> CycleResponse {
        if let Some(on_cycle) = &self.on_cycle {
            on_cycle(cycle)
        } else if cfg!(debug_assertions) {
            CycleResponse::Panic
        } else {
            CycleResponse::Cancel
        }
    }
}
