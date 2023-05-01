use std::borrow::{Borrow, Cow};

use bevy::asset::{AssetServerError, Handle, HandleId, HandleUntyped, LoadState};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{AssetServer, Res, ResMut};
use std::hash::Hash;
use thiserror::Error;
use crate::path::PathError;

use crate::proto::{ProtoStorage, Prototypical};
use crate::registration::ProtoRegistry;

#[derive(Debug, Error)]
pub enum ProtoLoadError {
    /// Indicates that the [`AssetServer`] encountered an error.
    #[error(transparent)]
    AssetServerError(#[from] AssetServerError),
    /// Indicates that there was a path error.
    #[error(transparent)]
    PathError(#[from] PathError),
}

/// A helper [`SystemParam`] for managing [prototypes].
///
/// For the mutable version, see [`PrototypesMut`].
///
/// [prototypes]: Prototypical
#[derive(SystemParam)]
pub struct Prototypes<'w, T: Prototypical> {
    registry: Res<'w, ProtoRegistry<T>>,
    config: Res<'w, <T as Prototypical>::Config>,
    asset_server: Res<'w, AssetServer>,
    storage: Res<'w, ProtoStorage<T>>,
}

/// A helper [`SystemParam`] for managing [prototypes].
///
/// For the immutable version, see [`Prototypes`].
///
/// [prototypes]: Prototypical
#[derive(SystemParam)]
pub struct PrototypesMut<'w, T: Prototypical> {
    registry: Res<'w, ProtoRegistry<T>>,
    config: ResMut<'w, <T as Prototypical>::Config>,
    asset_server: Res<'w, AssetServer>,
    storage: ResMut<'w, ProtoStorage<T>>,
}

impl<'w, T: Prototypical> PrototypesMut<'w, T> {
    /// Load the prototype at the given path.
    ///
    /// This will also store a strong handle to the prototype in order to keep it loaded.
    /// To later remove this handle, call [`PrototypesMut::remove`] with the same path.
    ///
    /// To load without automatically storing the handle, try using [`AssetServer::load`].
    pub fn load<P: Into<Cow<'static, str>>>(&mut self, path: P) -> Handle<T> {
        let path = path.into();
        let handle = self.asset_server.load(path.as_ref());
        self.storage.insert(path, handle.clone());
        handle
    }

    /// Load all the prototypes in the given directory.
    ///
    /// This will also store strong handles to the prototypes in order to keep them loaded.
    ///
    /// To load without automatically storing the handles, try using [`AssetServer::load_folder`].
    pub fn load_folder<P: Into<Cow<'static, str>>>(&mut self, path: P) -> Result<Vec<HandleUntyped>, ProtoLoadError> {
        let path = path.into();
        let handles: Vec<_> = self.asset_server.load_folder(path.as_ref())?;

        for handle in &handles {
            let path = self.asset_server
                .get_handle_path(handle)
                .unwrap()
                .path()
                .to_str()
                .ok_or(PathError::ConversionError)?
                .to_string();

            self.storage.insert(path, handle.clone().typed::<T>());
        }

        Ok(handles)
    }

    /// Remove the stored handle for the given prototype path.
    ///
    /// This allows the asset to be unloaded if the handle is dropped and no other
    /// strong handles exist.
    pub fn remove(&mut self, path: &str) -> Option<Handle<T>> {
        self.storage.remove(path)
    }

    /// Remove all stored handles.
    ///
    /// This allows those assets to be unloaded if no other strong handles exist for them.
    pub fn clear(&mut self) {
        self.storage.clear();
    }

    /// Returns a mutable reference to the [`Config`] resource.
    ///
    /// [`Config`]: crate::proto::Config
    pub fn config_mut(&mut self) -> &mut T::Config {
        &mut self.config
    }
}

macro_rules! impl_prototypes {
    ($ident: ident) => {
        impl<'w, T: Prototypical> $ident<'w, T> {
            /// Returns the [`LoadState`] for the prototype with the given [`HandleId`].
            ///
            /// This method is preferred over [`AssetServer::get_load_state`] as it better
            /// accounts for prototype dependencies and registration.
            pub fn get_load_state<H: Into<HandleId>>(&self, handle: H) -> LoadState {
                let handle_id = handle.into();
                match self.asset_server.get_load_state(handle_id) {
                    LoadState::Loaded => {
                        if self.registry.contains_handle(handle_id) {
                            LoadState::Loaded
                        } else if self.registry.contains_failed_handle(handle_id) {
                            LoadState::Failed
                        } else if self
                            .registry
                            .load_queue()
                            .read()
                            .is_queued_handle(handle_id)
                        {
                            LoadState::Loading
                        } else {
                            LoadState::Failed
                        }
                    }
                    state => state,
                }
            }

            /// Returns true if the prototype with the given [ID] is ready to be spawned.
            ///
            /// This method is preferred over [`AssetServer::get_load_state`] as it better
            /// accounts for prototype dependencies and registration.
            ///
            /// [ID]: Prototypical::id
            pub fn is_ready<I: Hash + Eq + ?Sized>(&self, id: &I) -> bool
            where
                T::Id: Borrow<I>,
            {
                self.registry.contains(id)
            }

            /// Returns true if the prototype with the given handle is ready to be spawned.
            ///
            /// This method is preferred over [`AssetServer::get_load_state`] as it better
            /// accounts for prototype dependencies and registration.
            pub fn is_ready_handle<H: Into<HandleId>>(&self, handle: H) -> bool {
                self.registry.contains_handle(handle)
            }

            /// Returns true if a prototype with the given path is currently stored.
            pub fn contains(&self, path: &str) -> bool {
                self.storage.contains(path)
            }

            /// Get a reference to the strong handle for the prototype at the given path.
            ///
            /// Returns `None` if no matching prototype is currently stored.
            pub fn get(&self, path: &str) -> Option<&Handle<T>> {
                self.storage.get(path)
            }

            /// Returns a reference to the [`Config`] resource.
            ///
            /// [`Config`]: crate::proto::Config
            pub fn config(&self) -> &T::Config {
                &self.config
            }
        }
    };
}

impl_prototypes!(Prototypes);
impl_prototypes!(PrototypesMut);
