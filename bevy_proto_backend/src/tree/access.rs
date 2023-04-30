use std::fmt::{Debug, Formatter};
use std::num::NonZeroIsize;
use std::path::{Component, Path, PathBuf};
use std::slice::Iter;
use std::str::FromStr;

use bevy::prelude::Entity;
use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect, ReflectDeserialize};
use serde::Deserialize;

use crate::schematics::{FromSchematicInput, SchematicContext};

/// A deserializable prototype entity reference.
///
/// [prototype]: crate::proto::Prototypical
#[derive(Clone, Debug, PartialEq, Reflect, FromReflect, Deserialize)]
#[reflect(Deserialize)]
pub enum ProtoEntity {
    /// Access the entity from the given access path.
    EntityPath(PathBuf),
    /// Access a child entity.
    Child(ChildAccess),
    /// Access a sibling entity.
    Sibling(SiblingAccess),
    /// Access the parent entity.
    Parent,
    /// Access an ancestor entity the given number of levels up.
    Ancestor(usize),
    /// Access the root entity.
    Root,
}

/// Determines how a child entity is accessed.
#[derive(Debug, Clone, Eq, PartialEq, Reflect, FromReflect, Deserialize)]
#[reflect(Deserialize)]
pub enum ChildAccess {
    /// Access the child with the given ID.
    ///
    /// The second argument denotes the occurrence.
    /// A negative occurrence begins the search from the last child.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::num::NonZeroIsize;
    /// # use bevy_proto_backend::tree::ChildAccess;
    /// // Access the first occurrence of "Foo" within this entity's children
    /// ChildAccess::Id(String::from("Foo"), NonZeroIsize::new(1).unwrap());
    ///
    /// // Access the second-to-last occurrence of "Foo" within this entity's children
    /// ChildAccess::Id(String::from("Foo"), NonZeroIsize::new(-2).unwrap());
    /// ```
    Id(
        String,
        #[serde(default = "get_one")]
        #[reflect(default = "get_one")]
        NonZeroIsize,
    ),
    /// Access the child at the given index.
    ///
    /// Negative values are offset from the last child.
    At(isize),
}

impl From<String> for ChildAccess {
    fn from(value: String) -> Self {
        Self::Id(value, get_one())
    }
}

impl From<isize> for ChildAccess {
    fn from(value: isize) -> Self {
        Self::At(value)
    }
}

impl From<(String, NonZeroIsize)> for ChildAccess {
    fn from(value: (String, NonZeroIsize)) -> Self {
        Self::Id(value.0, value.1)
    }
}

/// Determines how a sibling entity is accessed.
#[derive(Debug, Clone, Eq, PartialEq, Reflect, FromReflect, Deserialize)]
pub enum SiblingAccess {
    /// Access the sibling with the given ID.
    ///
    /// The second argument denotes the occurrence, offset from the current entity.
    /// A negative occurrence searches from the current entity to the first sibling,
    /// while a positive occurrence searches from the current entity to the last sibling.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::num::NonZeroIsize;
    /// # use bevy_proto_backend::tree::SiblingAccess;
    /// // Access the second occurrence of "Foo" after this entity
    /// SiblingAccess::Id(String::from("Foo"), NonZeroIsize::new(2).unwrap());
    ///
    /// // Access the occurrence of "Foo" right before this entity
    /// SiblingAccess::Id(String::from("Foo"), NonZeroIsize::new(-1).unwrap());
    /// ```
    Id(
        String,
        #[serde(default = "get_one")]
        #[reflect(default = "get_one")]
        NonZeroIsize,
    ),
    /// Access the sibling at the given offset.
    At(NonZeroIsize),
}

fn get_one() -> NonZeroIsize {
    NonZeroIsize::new(1).unwrap()
}

impl From<String> for SiblingAccess {
    fn from(value: String) -> Self {
        Self::Id(value, get_one())
    }
}

impl From<NonZeroIsize> for SiblingAccess {
    fn from(value: NonZeroIsize) -> Self {
        Self::At(value)
    }
}

impl From<(String, NonZeroIsize)> for SiblingAccess {
    fn from(value: (String, NonZeroIsize)) -> Self {
        Self::Id(value.0, value.1)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, FromReflect)]
pub(crate) enum AccessOp {
    /// Access the root entity.
    Root,
    /// Access a parent entity.
    Parent,
    /// Access a child entity.
    Child(ChildAccess),
    /// Access a sibling entity.
    Sibling(SiblingAccess),
}

/// An accessor used to retrieve an [`Entity`] within an [`EntityTree`].
///
/// # Path Format
///
/// This struct can also be created from a [`Path`] (or any type that satisfies `AsRef<Path>`).
///
/// These paths use a custom format to create the different access operations.
/// Here is a list of the various operations:
///
/// | Examples             | Access Type                            |
/// | :------------------- | :------------------------------------- |
/// | `./`                 | Self                                   |
/// | `../`                | [`ProtoEntity::Parent`]                |
/// | `/`                  | [`ProtoEntity::Root`]                  |
/// | `./@2`, `@2`         | [`ChildAccess::At`]                    |
/// | `./foo`, `foo`       | [`ChildAccess::Id`] (1st occurrence)   |
/// | `./@2:foo`, `@2:foo` | [`ChildAccess::Id`] (nth occurrence)   |
/// | `./~2`, `~2`         | [`SiblingAccess::At`]                  |
/// | `./~foo`, `~foo`     | [`SiblingAccess::Id`] (1st occurrence) |
/// | `./~2:foo`, `~2:foo` | [`SiblingAccess::Id`] (nth occurrence) |
///
///
/// [`EntityTree`]: EntityTree
#[derive(Default, Clone, Eq, PartialEq, Reflect, FromReflect, Deserialize)]
#[reflect(Default, Deserialize)]
#[serde(from = "ProtoEntity")]
pub struct EntityAccess {
    ops: Vec<AccessOp>,
}

impl EntityAccess {
    /// Create an [`EntityAccess`] that starts from the root.
    pub fn root() -> Self {
        Self {
            ops: vec![AccessOp::Root],
        }
    }

    /// Add a parent access.
    pub fn parent(mut self) -> Self {
        self.ops.push(AccessOp::Parent);
        self
    }

    /// Add a [child access].
    ///
    /// [child access]: ChildAccess
    pub fn child<C: Into<ChildAccess>>(mut self, child: C) -> Self {
        self.ops.push(AccessOp::Child(child.into()));
        self
    }

    /// Add a [sibling access].
    ///
    /// [sibling access]: SiblingAccess
    pub fn sibling<S: Into<SiblingAccess>>(mut self, sibling: S) -> Self {
        self.ops.push(AccessOp::Sibling(sibling.into()));
        self
    }

    /// Create an access path using the operations in this struct.
    pub fn to_path(&self) -> PathBuf {
        let mut path = if matches!(self.ops.first(), Some(AccessOp::Root)) {
            PathBuf::from("/")
        } else {
            // This is added purely to improve the readability of the path
            PathBuf::from(".")
        };

        for op in self.ops() {
            match op {
                AccessOp::Root => {
                    // Handled in constructor
                    continue;
                }
                AccessOp::Parent => path.push(".."),
                AccessOp::Child(ChildAccess::At(index)) => path.push(format!("@{}", index)),
                AccessOp::Child(ChildAccess::Id(id, occurrence)) => {
                    path.push(if occurrence.get() != 1 {
                        format!("@{}:{}", occurrence, id)
                    } else {
                        id.to_string()
                    })
                }
                AccessOp::Sibling(SiblingAccess::At(index)) => path.push(format!("~{}", index)),
                AccessOp::Sibling(SiblingAccess::Id(id, occurrence)) => {
                    path.push(if occurrence.get() != 1 {
                        format!("~{}:{}", occurrence, id)
                    } else {
                        format!("~{}", id)
                    })
                }
            }
        }

        path
    }

    pub(crate) fn ops(&self) -> Iter<'_, AccessOp> {
        self.ops.iter()
    }
}

impl From<ProtoEntity> for EntityAccess {
    fn from(value: ProtoEntity) -> Self {
        match value {
            ProtoEntity::EntityPath(path) => EntityAccess::from(path),
            ProtoEntity::Child(ChildAccess::Id(id, index)) => {
                EntityAccess::default().child((id, index))
            }
            ProtoEntity::Child(ChildAccess::At(index)) => EntityAccess::default().child(index),
            ProtoEntity::Sibling(SiblingAccess::Id(id, index)) => {
                EntityAccess::default().sibling((id, index))
            }
            ProtoEntity::Sibling(SiblingAccess::At(index)) => {
                EntityAccess::default().sibling(index)
            }
            ProtoEntity::Parent => EntityAccess::default().parent(),
            ProtoEntity::Ancestor(depth) => {
                let mut access = EntityAccess::default();
                access.ops.extend((0..depth).map(|_| AccessOp::Parent));
                access
            }
            ProtoEntity::Root => EntityAccess::root(),
        }
    }
}

impl<T: AsRef<Path>> From<T> for EntityAccess {
    fn from(value: T) -> Self {
        let path = value.as_ref();

        let mut access = if path.is_absolute() {
            Self::root()
        } else {
            Self::default()
        };

        for component in path.components() {
            match component {
                Component::Prefix(_) => panic!("prefix path operation not supported"),
                Component::RootDir => {
                    // Handled in constructor
                    continue;
                }
                Component::CurDir => {
                    // Do nothing
                    continue;
                }
                Component::ParentDir => {
                    access.ops.push(AccessOp::Parent);
                }
                Component::Normal(path) => {
                    let path = path.to_string_lossy();

                    if let Some(index) = path.strip_prefix('@') {
                        if let Some((index, id)) = index.split_once(':') {
                            // "foo/bar/@3:baz"
                            let occurrence = NonZeroIsize::from_str(index.trim()).unwrap();
                            access.ops.push(AccessOp::Child(ChildAccess::Id(
                                id.trim().to_string(),
                                occurrence,
                            )));
                        } else {
                            // "foo/bar/@3"
                            let index = isize::from_str(index.trim()).unwrap();
                            access.ops.push(AccessOp::Child(ChildAccess::At(index)));
                        }
                    } else if let Some(index) = path.strip_prefix('~') {
                        if let Some((index, id)) = index.split_once(':') {
                            // "foo/bar/~3:baz"
                            let occurrence = NonZeroIsize::from_str(index.trim()).unwrap();
                            access.ops.push(AccessOp::Sibling(SiblingAccess::Id(
                                id.trim().to_string(),
                                occurrence,
                            )));
                        } else {
                            // "foo/bar/~3"
                            let offset = NonZeroIsize::from_str(index.trim()).unwrap();
                            access
                                .ops
                                .push(AccessOp::Sibling(SiblingAccess::At(offset)));
                        }
                    } else {
                        // "foo/bar/baz"
                        access.ops.push(AccessOp::Child(ChildAccess::Id(
                            path.to_string(),
                            get_one(),
                        )))
                    }
                }
            }
        }

        access
    }
}

impl FromSchematicInput<EntityAccess> for Entity {
    fn from_input(input: EntityAccess, context: &mut SchematicContext) -> Self {
        context
            .find_entity(&input)
            .unwrap_or_else(|| panic!("entity should exist at path {:?}", input.to_path()))
    }
}

impl FromSchematicInput<ProtoEntity> for Entity {
    fn from_input(input: ProtoEntity, context: &mut SchematicContext) -> Self {
        let access: EntityAccess = input.into();
        context
            .find_entity(&access)
            .unwrap_or_else(|| panic!("entity should exist at path {:?}", access.to_path()))
    }
}

impl FromSchematicInput<EntityAccess> for Option<Entity> {
    fn from_input(input: EntityAccess, context: &mut SchematicContext) -> Self {
        context.find_entity(&input)
    }
}

impl FromSchematicInput<ProtoEntity> for Option<Entity> {
    fn from_input(input: ProtoEntity, context: &mut SchematicContext) -> Self {
        context.find_entity(&input.into())
    }
}

/// A helper struct to deserialize `Vec<Entity>`.
///
/// # Example
///
/// ```ignore
/// # use bevy::prelude::{Entity, FromReflect, Reflect};
/// # use bevy_proto_backend::tree::ProtoEntityList;
/// # use bevy_proto_backend::schematics::{Schematic, ReflectSchematic};
/// #[derive(Reflect, FromReflect, Schematic)]
/// #[reflect(Schematic)]
/// struct EntityGroup {
///   #[from = ProtoEntityList]
///   entities: Vec<Entity>
/// }
/// ```
///
#[derive(Default, Clone, PartialEq, Reflect, FromReflect, Deserialize)]
#[reflect(Default, Deserialize)]
#[serde(transparent)]
pub struct ProtoEntityList(pub Vec<ProtoEntity>);

impl FromSchematicInput<ProtoEntityList> for Vec<Entity> {
    fn from_input(input: ProtoEntityList, context: &mut SchematicContext) -> Self {
        input
            .0
            .into_iter()
            .map(|entity| {
                let access: EntityAccess = entity.into();
                context
                    .find_entity(&access)
                    .unwrap_or_else(|| panic!("entity should exist at path {:?}", access.to_path()))
            })
            .collect()
    }
}

impl Debug for EntityAccess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_path())
    }
}
