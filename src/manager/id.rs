use crate::Prototypical;
use bevy::asset::{Asset, Handle, HandleId, HandleUntyped};

/// An identifier for [prototypical] assets.
///
/// [prototypical]: crate::Prototypical
#[derive(Debug, Clone)]
pub enum ProtoId {
    /// The name of this prototype.
    Name(String),
    /// The handle that points to this prototype.
    Handle(HandleId),
}

/// An identifier for [prototypical] assets.
///
/// Unlike, [`ProtoId`], this type uses a `&str` and a lifetime to avoid unnecessary allocations.
///
/// [prototypical]: crate::Prototypical
#[derive(Debug, Copy, Clone)]
pub enum ProtoIdRef<'a> {
    /// The name of this prototype.
    Name(&'a str),
    /// The handle that points to this prototype.
    Handle(HandleId),
}

impl ProtoId {
    /// Convert this [`ProtoId`] to a [`ProtoIdRef`].
    pub fn as_ref(&self) -> ProtoIdRef {
        match self {
            Self::Name(name) => ProtoIdRef::Name(name),
            Self::Handle(handle) => ProtoIdRef::Handle(*handle),
        }
    }
}

impl<'a> ProtoIdRef<'a> {
    /// Convert this [`ProtoIdRef`] to an owned [`ProtoId`].
    pub fn to_owned(&self) -> ProtoId {
        match self {
            Self::Name(name) => ProtoId::Name(name.to_string()),
            Self::Handle(handle) => ProtoId::Handle(*handle),
        }
    }
}

impl<'a> From<&'a ProtoId> for ProtoIdRef<'a> {
    fn from(id: &'a ProtoId) -> Self {
        id.as_ref()
    }
}

impl From<ProtoIdRef<'_>> for ProtoId {
    fn from(id: ProtoIdRef<'_>) -> Self {
        id.to_owned()
    }
}

impl<'a> From<&'a str> for ProtoIdRef<'a> {
    fn from(name: &'a str) -> Self {
        Self::Name(name)
    }
}

impl From<&'_ str> for ProtoId {
    fn from(name: &'_ str) -> Self {
        Self::Name(name.to_string())
    }
}

impl From<String> for ProtoId {
    fn from(name: String) -> Self {
        Self::Name(name)
    }
}

impl<'a> From<HandleId> for ProtoIdRef<'a> {
    fn from(handle: HandleId) -> Self {
        Self::Handle(handle)
    }
}

impl<'a> From<&'a HandleId> for ProtoIdRef<'a> {
    fn from(handle: &'a HandleId) -> Self {
        Self::Handle(*handle)
    }
}

impl From<HandleId> for ProtoId {
    fn from(handle: HandleId) -> Self {
        Self::Handle(handle)
    }
}

impl<'a, T: Asset + Prototypical> From<Handle<T>> for ProtoIdRef<'a> {
    fn from(handle: Handle<T>) -> Self {
        Self::Handle(handle.id)
    }
}

impl<'a, T: Asset + Prototypical> From<&'a Handle<T>> for ProtoIdRef<'a> {
    fn from(handle: &'a Handle<T>) -> Self {
        Self::Handle(handle.id)
    }
}

impl<T: Asset + Prototypical> From<Handle<T>> for ProtoId {
    fn from(handle: Handle<T>) -> Self {
        Self::Handle(handle.id)
    }
}

impl<'a> From<HandleUntyped> for ProtoIdRef<'a> {
    fn from(handle: HandleUntyped) -> Self {
        Self::Handle(handle.id)
    }
}

impl<'a> From<&'a HandleUntyped> for ProtoIdRef<'a> {
    fn from(handle: &'a HandleUntyped) -> Self {
        Self::Handle(handle.id)
    }
}

impl From<HandleUntyped> for ProtoId {
    fn from(handle: HandleUntyped) -> Self {
        Self::Handle(handle.id)
    }
}
