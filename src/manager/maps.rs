use crate::Prototypical;
use bevy::asset::{Asset, Handle, HandleId};
use bevy::utils::HashMap;
use parking_lot::RwLock;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct NameToHandle(Arc<RwLock<HashMap<String, HandleId>>>);

impl Deref for NameToHandle {
    type Target = Arc<RwLock<HashMap<String, HandleId>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Clone)]
pub struct HandleToName(Arc<RwLock<HashMap<HandleId, String>>>);

impl Deref for HandleToName {
    type Target = Arc<RwLock<HashMap<HandleId, String>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ProtoHandles<T: Asset + Prototypical>(Arc<RwLock<HashMap<HandleId, Handle<T>>>>);

impl<T: Asset + Prototypical> Default for ProtoHandles<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Asset + Prototypical> Clone for ProtoHandles<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Asset + Prototypical> Deref for ProtoHandles<T> {
    type Target = Arc<RwLock<HashMap<HandleId, Handle<T>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
