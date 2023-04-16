use bevy::asset::HandleId;
use bevy::prelude::Component;

/// A component used to track existing entities spawned via a [prototype].
///
/// [prototype]: crate::proto::Prototypical
#[derive(Component, Clone, Hash, PartialEq, Eq)]
pub struct ProtoInstance {
    /// Used to identify the prototype.
    handle: HandleId,
    /// Used to indicate the child index within the parent.
    child_index: usize,
}

impl ProtoInstance {
    pub(crate) fn new(handle: HandleId, child_index: usize) -> Self {
        Self {
            handle,
            child_index,
        }
    }
}
