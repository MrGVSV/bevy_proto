use bevy::prelude::Handle;

use bevy_proto_backend::children::PrototypicalChild;
use bevy_proto_backend::path::ProtoPath;

use crate::proto::Prototype;

/// The child type of a [`Prototype`].
///
/// This can be deserialized either from a struct or a [`ProtoPath`] string.
pub struct ProtoChild {
    pub(crate) merge_key: Option<String>,
    pub(crate) handle: Handle<Prototype>,
}

impl PrototypicalChild<Prototype> for ProtoChild {
    type Key = String;

    fn handle(&self) -> &Handle<Prototype> {
        &self.handle
    }

    fn merge_key(&self) -> Option<&Self::Key> {
        self.merge_key.as_ref()
    }
}

/// The enum representation of a serialized [`Prototype`] child.
pub enum ProtoChildValue {
    /// The child is the prototype asset at the given path.
    Path(ProtoPath),
    /// The child is the contained prototype.
    Inline(Prototype),
}
