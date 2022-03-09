use crate::prelude::{ProtoCommands, ProtoData};
use crate::prototype::Prototypical;
use bevy::prelude::{AssetServer, Reflect, Res, World};
use bevy::reflect::{FromReflect, FromType};

/// A trait that allows components to be used within [`Prototypical`] structs
pub trait ProtoComponent: Reflect + Send + Sync + 'static {
    // fn apply(&self, entity: EntityMut);
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>);
    #[allow(unused_variables)]
    fn prepare(&self, world: &mut World, prototype: &dyn Prototypical, data: &mut ProtoData) {}
    fn as_reflect(&self) -> &dyn Reflect;
    fn name(&self) -> &str {
        self.as_reflect().type_name()
    }
}

#[derive(Clone)]
pub struct ReflectProtoComponent {
    get_boxed: fn(&dyn Reflect) -> Option<Box<dyn ProtoComponent>>,
}

impl ReflectProtoComponent {
    /// Get the underlying [component](ProtoComponent) from the given reflected value
    pub fn get_component(&self, reflect_value: &dyn Reflect) -> Option<Box<dyn ProtoComponent>> {
        (self.get_boxed)(reflect_value)
    }
}

impl<T: ProtoComponent + FromReflect> FromType<T> for ReflectProtoComponent {
    fn from_type() -> Self {
        Self {
            get_boxed: |reflect_value| {
                T::from_reflect(reflect_value)
                    .map(|value| Box::new(value) as Box<dyn ProtoComponent>)
            },
        }
    }
}
