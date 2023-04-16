use std::fmt::{Debug, Formatter};

use bevy::ecs::world::EntityMut;
use bevy::prelude::{FromReflect, Reflect};
use bevy::reflect::{FromType, GetTypeRegistration, TypeInfo, TypeRegistration, Typed};

use crate::deps::DependenciesBuilder;
use crate::schematics::schematic::Schematic;
use crate::schematics::SchematicError;
use crate::tree::EntityTree;

/// A dynamic representation of a [`Schematic`].
///
/// Internally, this contains a copy of the schematic's [`ReflectSchematic`]
/// as well as its reflected [input] data.
///
/// This is generated from [`ReflectSchematic::create_dynamic`] and can be used
/// to use a type-erased version of the schematic.
///
/// [input]: Schematic::Input
pub struct DynamicSchematic {
    input: Box<dyn Reflect>,
    reflect_schematic: ReflectSchematic,
}

impl DynamicSchematic {
    /// Get the reflected [schematic input] data.
    ///
    /// [schematic input]: Schematic::Input
    pub fn input(&self) -> &dyn Reflect {
        &*self.input
    }

    /// Dynamically call the corresponding [`Schematic::apply`] method.
    pub fn apply(&self, entity: &mut EntityMut, tree: &EntityTree) -> Result<(), SchematicError> {
        (self.reflect_schematic.apply)(&*self.input, entity, tree)
    }

    /// Dynamically call the corresponding [`Schematic::remove`] method.
    pub fn remove(&self, entity: &mut EntityMut, tree: &EntityTree) -> Result<(), SchematicError> {
        (self.reflect_schematic.remove)(&*self.input, entity, tree)
    }

    /// Dynamically call the corresponding [`Schematic::preload_dependencies`] method.
    pub fn preload_dependencies(
        &mut self,
        dependencies: &mut DependenciesBuilder,
    ) -> Result<(), SchematicError> {
        (self.reflect_schematic.preload_dependencies)(&mut *self.input, dependencies)
    }

    /// The type info of the corresponding [`Schematic`].
    pub fn type_info(&self) -> &'static TypeInfo {
        self.reflect_schematic.type_info()
    }

    /// Generates and returns the [`TypeRegistration`] of the [schematic's input].
    ///
    /// [schematic's input]: Schematic::Input
    pub fn input_registration(&self) -> TypeRegistration {
        self.reflect_schematic.input_registration()
    }

    /// Attempts to clone this [`DynamicSchematic`].
    pub fn try_clone(&self) -> Result<Self, SchematicError> {
        Ok(Self {
            input: (self.reflect_schematic.clone_input)(&*self.input)?,
            reflect_schematic: self.reflect_schematic.clone(),
        })
    }
}

impl Debug for DynamicSchematic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicSchematic")
            .field("type_name", &self.reflect_schematic.type_info.type_name())
            .field("input", &self.input)
            .finish()
    }
}

/// Reflected [type data] for the [`Schematic`] trait.
///
/// [type data]: bevy::reflect::TypeData
#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub struct ReflectSchematic {
    type_info: &'static TypeInfo,
    input_registration: fn() -> TypeRegistration,
    create_dynamic:
        fn(Box<dyn Reflect>, ReflectSchematic) -> Result<DynamicSchematic, SchematicError>,
    apply: fn(
        input: &dyn Reflect,
        entity: &mut EntityMut,
        tree: &EntityTree,
    ) -> Result<(), SchematicError>,
    remove: fn(
        input: &dyn Reflect,
        entity: &mut EntityMut,
        tree: &EntityTree,
    ) -> Result<(), SchematicError>,
    preload_dependencies: fn(
        input: &mut dyn Reflect,
        dependencies: &mut DependenciesBuilder,
    ) -> Result<(), SchematicError>,
    clone_input: fn(input: &dyn Reflect) -> Result<Box<dyn Reflect>, SchematicError>,
}

impl ReflectSchematic {
    /// Create a [`DynamicSchematic`] using the given reflected [schematic input] data.
    ///
    /// [schematic input]: Schematic::Input
    pub fn create_dynamic(
        &self,
        schematic_input: Box<dyn Reflect>,
    ) -> Result<DynamicSchematic, SchematicError> {
        let data = self.clone();
        (self.create_dynamic)(schematic_input, data)
    }

    /// The type info of the corresponding [`Schematic`].
    pub fn type_info(&self) -> &'static TypeInfo {
        self.type_info
    }

    /// Generates and returns the [`TypeRegistration`] of the [schematic's input].
    ///
    /// [schematic's input]: Schematic::Input
    pub fn input_registration(&self) -> TypeRegistration {
        (self.input_registration)()
    }
}

impl<T: Reflect + Typed + Schematic> FromType<T> for ReflectSchematic {
    fn from_type() -> Self {
        Self {
            type_info: <T as Typed>::type_info(),
            input_registration: <T::Input as GetTypeRegistration>::get_type_registration,
            create_dynamic: |reflect_input, data| {
                let input = <T::Input as FromReflect>::take_from_reflect(reflect_input)
                    .map_err(|_| SchematicError::FromReflectFail)?;

                Ok(DynamicSchematic {
                    input: Box::new(input),
                    reflect_schematic: data,
                })
            },
            apply: |reflect_input, entity, tree| {
                let input = reflect_input.downcast_ref::<T::Input>().ok_or_else(|| {
                    SchematicError::TypeMismatch {
                        expected: std::any::type_name::<T::Input>(),
                        found: reflect_input.type_name().to_string(),
                    }
                })?;
                <T as Schematic>::apply(input, entity, tree);
                Ok(())
            },
            remove: |reflect_input, entity, tree| {
                let input = reflect_input.downcast_ref::<T::Input>().ok_or_else(|| {
                    SchematicError::TypeMismatch {
                        expected: std::any::type_name::<T::Input>(),
                        found: reflect_input.type_name().to_string(),
                    }
                })?;
                <T as Schematic>::remove(input, entity, tree);
                Ok(())
            },
            preload_dependencies: |reflect_input, dependencies| {
                let type_name = reflect_input.type_name().to_string();
                let input = reflect_input.downcast_mut::<T::Input>().ok_or_else(|| {
                    SchematicError::TypeMismatch {
                        expected: std::any::type_name::<T::Input>(),
                        found: type_name,
                    }
                })?;
                <T as Schematic>::preload_dependencies(input, dependencies);
                Ok(())
            },
            clone_input: |reflect_input| {
                <T::Input as FromReflect>::from_reflect(reflect_input)
                    .map(|input| Box::new(input) as Box<dyn Reflect>)
                    .ok_or(SchematicError::FromReflectFail)
            },
        }
    }
}
