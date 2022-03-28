use bevy::asset::HandleId;
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
