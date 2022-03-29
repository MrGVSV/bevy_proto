use crate::prelude::{ProtoLoadError, Prototypical};
use crate::ProtoComponent;
use bevy::utils::HashSet;
use parking_lot::{RwLock, RwLockReadGuard};
use std::any::{Any, TypeId};
use std::sync::Arc;

/// Config used to control how prototypes are processed.
#[derive(Default, Clone)]
pub struct ProtoConfig {
    filter: ProtoFilter,
    extensions: Vec<&'static str>,
    on_register:
        Option<Arc<dyn Fn(&mut dyn Prototypical) -> Result<(), anyhow::Error> + Send + Sync>>,
    on_register_component: Option<Arc<dyn Fn(&mut dyn ProtoComponent) -> bool + Send + Sync>>,
}

#[derive(Clone, Default)]
pub(crate) struct ProtoConfigArc {
    internal: Arc<RwLock<ProtoConfig>>,
}

impl ProtoConfigArc {
    pub fn new(config: ProtoConfig) -> Self {
        Self {
            internal: Arc::new(RwLock::new(config)),
        }
    }
    /// Takes a read lock on the underlying [`ProtoConfig`].
    pub fn read(&self) -> RwLockReadGuard<'_, ProtoConfig> {
        self.internal.read()
    }
}

/// A filter that dictates if some data can be deserialized into a usable
/// [`ProtoComponent`] trait object.
#[derive(Clone)]
pub enum ProtoFilter {
    /// Allows all types.
    All,
    /// Allows only a given set of types.
    Whitelist(HashSet<TypeId>),
    /// Allows all types, except for the given set.
    Blacklist(HashSet<TypeId>),
}

impl Default for ProtoFilter {
    fn default() -> Self {
        Self::All
    }
}

impl ProtoConfig {
    /// The current [filter](ProtoFilter).
    pub fn filter(&self) -> &ProtoFilter {
        &self.filter
    }

    /// Resets the filter to [`ProtoFilter::All`], allowing all types.
    pub fn allow_all(&mut self) {
        self.filter = ProtoFilter::All;
    }

    /// Whitelists the given type.
    ///
    /// Sets the filter to [`ProtoFilter::Whitelist`] if it wasn't already.
    pub fn whitelist<T: Any>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<T>();
        if let ProtoFilter::Whitelist(ref mut list) = self.filter {
            list.insert(type_id);
        } else {
            let mut set = HashSet::new();
            set.insert(type_id);
            self.filter = ProtoFilter::Whitelist(set);
        }
        self
    }

    /// Blacklists the given type.
    ///
    /// Sets the filter to [`ProtoFilter::Blacklist`] if it wasn't already.
    pub fn blacklist<T: Any>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<T>();
        if let ProtoFilter::Blacklist(ref mut list) = self.filter {
            list.insert(type_id);
        } else {
            let mut set = HashSet::new();
            set.insert(type_id);
            self.filter = ProtoFilter::Blacklist(set);
        }
        self
    }

    /// The allowed extensions to be used by prototype files.
    pub fn extensions(&self) -> &Vec<&'static str> {
        &self.extensions
    }

    /// Sets the allowed extensions to be used by prototype files.
    ///
    /// Note: This does not apply to the default [`Prototype`](crate::Prototype) struct.
    pub fn set_extensions(&mut self, extensions: Vec<&'static str>) -> &mut Self {
        self.extensions = extensions;
        self
    }

    /// Checks if the given [`TypeId`] is allowed by the config.
    ///
    /// Returns `Ok(())` if allowed, otherwise returns `Err(ProtoLoadError)`.
    pub(crate) fn assert_allowed(
        &self,
        type_id: TypeId,
        type_name: &str,
    ) -> Result<(), ProtoLoadError> {
        match &self.filter {
            ProtoFilter::All => Ok(()),
            ProtoFilter::Whitelist(list) => {
                if list.contains(&type_id) {
                    Ok(())
                } else {
                    Err(ProtoLoadError::NotWhitelisted {
                        name: type_name.into(),
                    })
                }
            }
            ProtoFilter::Blacklist(list) => {
                if list.contains(&type_id) {
                    Err(ProtoLoadError::Blacklisted {
                        name: type_name.into(),
                    })
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Register a callback for when a prototype is loaded and ready to be inserted as an asset.
    ///
    /// This callback should return `Ok(())` if the prototype should be allowed to be added
    /// as an asset, otherwise it will be discarded.
    ///
    /// Note: This is called as soon as the prototype is loaded, this means that templates
    /// may or may not be loaded by that point.
    pub fn on_register(
        &mut self,
        on_register: Option<
            Arc<dyn Fn(&mut dyn Prototypical) -> Result<(), anyhow::Error> + Send + Sync>,
        >,
    ) {
        self.on_register = on_register;
    }

    /// Register a callback for when a [`ProtoComponent`] is loaded.
    ///
    /// This callback should return `true` if the prototype should be allowed to be added
    /// to the prototype, otherwise it will be discarded.
    pub fn on_register_component(
        &mut self,
        on_register: Option<Arc<dyn Fn(&mut dyn ProtoComponent) -> bool + Send + Sync>>,
    ) {
        self.on_register_component = on_register;
    }

    pub(crate) fn call_on_register(
        &self,
        proto: &mut dyn Prototypical,
    ) -> Result<(), anyhow::Error> {
        if let Some(ref on_register) = self.on_register {
            on_register(proto)
        } else {
            Ok(())
        }
    }

    pub(crate) fn call_on_register_component(&self, component: &mut dyn ProtoComponent) -> bool {
        if let Some(ref on_register) = self.on_register_component {
            on_register(component)
        } else {
            true
        }
    }
}
