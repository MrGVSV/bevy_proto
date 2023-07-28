use crate::proto::ProtoChild;
use bevy::reflect::{TypePath, TypeUuid};
use bevy_proto_backend::children::Children;
use bevy_proto_backend::deps::Dependencies;
use bevy_proto_backend::path::ProtoPath;
use bevy_proto_backend::proto::Prototypical;
use bevy_proto_backend::schematics::Schematics;
use bevy_proto_backend::templates::Templates;

/// The core asset type used to create easily-configurable entity trees.
#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "cbc85a87-723a-4e61-83c7-26e96e54fe9f"]
pub struct Prototype {
    pub(crate) id: String,
    pub(crate) path: ProtoPath,
    pub(crate) requires_entity: bool,
    pub(crate) schematics: Schematics,
    pub(crate) templates: Option<Templates>,
    pub(crate) dependencies: Dependencies,
    pub(crate) children: Option<Children<Prototype>>,
}

impl Prototypical for Prototype {
    type Id = String;
    type Child = ProtoChild;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn path(&self) -> &ProtoPath {
        &self.path
    }

    fn requires_entity(&self) -> bool {
        self.requires_entity
    }

    fn schematics(&self) -> &Schematics {
        &self.schematics
    }

    fn schematics_mut(&mut self) -> &mut Schematics {
        &mut self.schematics
    }

    fn templates(&self) -> Option<&Templates> {
        self.templates.as_ref()
    }

    fn templates_mut(&mut self) -> Option<&mut Templates> {
        self.templates.as_mut()
    }

    fn dependencies(&self) -> &Dependencies {
        &self.dependencies
    }

    fn dependencies_mut(&mut self) -> &mut Dependencies {
        &mut self.dependencies
    }

    fn children(&self) -> Option<&Children<Self>> {
        self.children.as_ref()
    }

    fn children_mut(&mut self) -> Option<&mut Children<Self>> {
        self.children.as_mut()
    }
}
