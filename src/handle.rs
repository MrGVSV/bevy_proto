use bevy::asset::{Asset, AssetPath, Handle};
use bevy::reflect::{FromReflect, Reflect, ReflectDeserialize};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

/// A trait that allows an asset handle to be stored.
///
/// This is used by the `#[proto_comp(preload)]` macro to store a preloaded handle.
pub trait StoreHandle {
    /// The asset type of the handle to be stored.
    type AssetType: Asset;
    /// Store the given handle.
    fn store_handle(&mut self, handle: Handle<Self::AssetType>);
}

impl<T: Asset> StoreHandle for HandlePath<T> {
    type AssetType = T;

    fn store_handle(&mut self, handle: Handle<Self::AssetType>) {
        self.set_handle(Some(handle));
    }
}

impl<T: Asset> StoreHandle for Handle<T> {
    type AssetType = T;

    fn store_handle(&mut self, handle: Handle<Self::AssetType>) {
        *self = handle;
    }
}

/// A convenience struct for managing asset dependencies in [`ProtoComponent`] types.
///
/// This object wraps both the asset path and the asset handle, allowing the latter to be
/// loaded using the former during runtime. Additionally, this type also supports
/// deserialization from a string, allowing for more compact usage:
///
/// ```yaml
/// texture: "textures/my_texture.png"
/// ```
///
/// When deserialized, `HandlePath<T>` will be put in a _dehydrated_ state where it only
/// contains the path to an asset. It can then be _hydrated_ by [setting the handle].
///
/// The stored handle will always be _strong_. This is because this object is meant to imply
/// "ownership" of an asset— after all, it contains the key (path) to the asset.
///
/// [`ProtoComponent`]: crate::prelude::ProtoComponent
/// [setting the handle]: HandlePath::set_handle
#[derive(Reflect, FromReflect, Debug)]
#[reflect_value(Serialize, Deserialize, Hash, PartialEq)]
pub struct HandlePath<T: Asset> {
    path: String,
    handle: Option<Handle<T>>,
}

impl<T: Asset> HandlePath<T> {
    /// Creates a new [`HandlePath`].
    pub fn new<P: Into<String>>(path: P) -> Self {
        Self {
            path: path.into(),
            handle: None,
        }
    }

    /// Creates a new, hydrated [`HandlePath`].
    ///
    /// # Panics
    ///
    /// The given handle must be _strong_.
    pub fn with_handle<P: Into<String>>(path: P, handle: Handle<T>) -> Self {
        assert!(handle.is_strong(), "given handle must be strong");
        Self {
            path: path.into(),
            handle: Some(handle),
        }
    }

    /// The path of the asset pointed to by this [`HandlePath`].
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The contained handle, if any.
    ///
    /// This will always be a _strong_ handle.
    pub fn handle(&self) -> Option<&Handle<T>> {
        self.handle.as_ref()
    }

    /// Set the handle.
    ///
    /// # Panics
    ///
    /// The given handle must be _strong_.
    pub fn set_handle(&mut self, handle: Option<Handle<T>>) {
        if let Some(ref h) = handle {
            assert!(h.is_strong(), "given handle must be strong");
        }
        self.handle = handle;
    }

    /// Returns whether this [`HandlePath`] is hydrated or not.
    ///
    /// A _hydrated_ [`HandlePath`] is one that contains a strong handle to an asset.
    pub fn is_hydrated(&self) -> bool {
        self.handle.is_some()
    }
}

impl<T: Asset> Eq for HandlePath<T> {}
impl<T: Asset> PartialEq for HandlePath<T> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.handle == other.handle
    }
}

impl<T: Asset> Clone for HandlePath<T> {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            handle: self.handle.clone(),
        }
    }
}

impl<T: Asset> Hash for HandlePath<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.handle.hash(state);
    }
}

impl<T: Asset> From<String> for HandlePath<T> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<T: Asset> From<HandlePath<T>> for String {
    fn from(value: HandlePath<T>) -> Self {
        value.path
    }
}

impl<T: Asset> From<&'_ str> for HandlePath<T> {
    fn from(value: &'_ str) -> Self {
        Self::new(value)
    }
}

impl<'a, T: Asset> From<&'a HandlePath<T>> for &'a str {
    fn from(value: &'a HandlePath<T>) -> Self {
        value.path()
    }
}

impl<T: Asset> From<AssetPath<'_>> for HandlePath<T> {
    fn from(value: AssetPath<'_>) -> Self {
        let path = value.path();
        Self::new(path.to_string_lossy().to_string())
    }
}

impl<'a, T: Asset> From<&'a HandlePath<T>> for AssetPath<'a> {
    fn from(value: &'a HandlePath<T>) -> Self {
        value.path().into()
    }
}

impl<T: Asset> From<HandlePath<T>> for AssetPath<'static> {
    fn from(value: HandlePath<T>) -> Self {
        AssetPath::new(PathBuf::new().join(value.path), None)
    }
}

impl<T: Asset> Serialize for HandlePath<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.path())
    }
}

impl<'de, T: Asset> Deserialize<'de> for HandlePath<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let path = deserializer.deserialize_string(HandlePathVisitor)?;
        Ok(HandlePath::new(path))
    }
}

struct HandlePathVisitor;

impl<'de> Visitor<'de> for HandlePathVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "an asset path string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Self::Value::from(value))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::handle::HandlePath;
    use bevy::prelude::Image;

    #[test]
    fn should_serialize_handle() {
        let handle = HandlePath::<Image>::new("some/path.yaml");
        let output = serde_yaml::to_string(&handle).unwrap();
        assert_eq!(
            r#"---
some/path.yaml
"#,
            output
        );
    }

    #[test]
    fn should_deserialize_handle() {
        let input = r#"---
some/path.yaml
"#;
        let output: HandlePath<Image> = serde_yaml::from_str(input).unwrap();
        let expected = HandlePath::<Image>::new("some/path.yaml");
        assert_eq!(expected, output);
    }
}
