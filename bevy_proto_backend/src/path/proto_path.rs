use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::path::{Path, PathBuf, MAIN_SEPARATOR_STR};

use bevy::asset::{AssetPath, HandleId};
use path_clean::PathClean;

use crate::path::{PathError, ProtoPathContext};

/// A wrapper around an [`AssetPath`] that represents a path to a [prototype].
///
/// [prototype]: crate::proto::Prototypical
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ProtoPath(AssetPath<'static>);

impl ProtoPath {
    /// Creates a new [`ProtoPath`] from the given path and a [path context].
    ///
    /// This supports the following path types:
    /// * "Absolute" Asset Paths
    ///   * `/prototypes/Template.prototype.ron`
    /// * Relative Paths
    ///   * `./Template.prototype.ron`
    ///   * `Template.prototype.ron`
    /// * Extensionless Relative Paths
    ///   * `Template`
    ///
    /// Note that "Extensionless Relative Paths" require that the extension is configured
    /// in the respective [`Config`].
    /// Its extension will be chosen based on the first match with the path context's
    /// [base path].
    ///
    /// [path context]: ProtoPathContext
    /// [`Config`]: crate::proto::Config
    /// [base path]: ProtoPathContext::base_path
    pub fn new<P: AsRef<Path>>(path: P, ctx: &dyn ProtoPathContext) -> Result<Self, PathError> {
        // === Paths Types ===
        // 1. Absolute Asset Paths
        //   a. "/prototypes/Template.prototype.ron"
        //   - Note: The leading "/" will need to be stripped to be valid
        // 2. Relative Paths
        //   a. "./Template.prototype.ron"
        // 3. Name-only Relative Paths
        //   a. "Template.prototype.ron"
        // 4. Extensionless Name-only Relative Paths
        //   a. "Template"

        let path = path.as_ref();
        let base_path = ctx.base_path();

        // 1
        if path.has_root() {
            return Ok(ProtoPath::from(
                path.strip_prefix(MAIN_SEPARATOR_STR)
                    .map_err(|_| PathError::MalformedPath(path.to_path_buf()))?
                    .to_path_buf(),
            ));
        }

        let rel_path = base_path
            .parent()
            .ok_or_else(|| PathError::InvalidBase(base_path.to_path_buf()))?
            .join(path)
            .clean();
        let io = ctx.asset_io();

        // 2 & 3
        if io.is_file(rel_path.as_path()) {
            return Ok(ProtoPath::from(rel_path));
        }

        // 4
        for extension in ctx.extensions() {
            let ext = extension.strip_prefix('.').unwrap_or(extension);
            if base_path.to_string_lossy().ends_with(&format!(".{ext}")) {
                let rel_path = rel_path.with_extension(ext);
                return if io.is_file(rel_path.as_path()) {
                    Ok(ProtoPath::from(rel_path))
                } else {
                    Err(PathError::DoesNotExist(rel_path))
                };
            }
        }

        Err(PathError::InvalidExtension(base_path.to_path_buf()))
    }

    /// Get the underlying [`AssetPath`].
    pub fn asset_path(&self) -> &AssetPath<'static> {
        &self.0
    }

    /// Get the [`Path`] of the underlying [`AssetPath`].
    pub fn path(&self) -> &Path {
        self.0.path()
    }

    /// Get the [label] of the underlying [`AssetPath`].
    ///
    /// [label]: AssetPath::label
    pub fn label(&self) -> Option<&str> {
        self.0.label()
    }
}

impl Debug for ProtoPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = self.0.label() {
            write!(f, "{:?}#{}", self.0.path(), label)
        } else {
            write!(f, "{:?}", self.0.path())
        }
    }
}

impl PartialEq<AssetPath<'_>> for ProtoPath {
    fn eq(&self, other: &AssetPath) -> bool {
        &self.0 == other
    }
}

impl PartialEq<&AssetPath<'_>> for ProtoPath {
    fn eq(&self, other: &&AssetPath) -> bool {
        &&self.0 == other
    }
}

impl From<AssetPath<'_>> for ProtoPath {
    fn from(value: AssetPath<'_>) -> Self {
        Self(value.to_owned())
    }
}

impl From<&AssetPath<'_>> for ProtoPath {
    fn from(value: &AssetPath<'_>) -> Self {
        Self(value.to_owned())
    }
}

impl From<PathBuf> for ProtoPath {
    fn from(value: PathBuf) -> Self {
        Self(AssetPath::from(value))
    }
}

impl From<&PathBuf> for ProtoPath {
    fn from(value: &PathBuf) -> Self {
        Self(AssetPath::from(value.clone()))
    }
}

impl From<&Path> for ProtoPath {
    fn from(value: &Path) -> Self {
        Self(AssetPath::from(value.to_path_buf()))
    }
}

impl From<String> for ProtoPath {
    fn from(value: String) -> Self {
        Self(AssetPath::from(value))
    }
}

impl From<&String> for ProtoPath {
    fn from(value: &String) -> Self {
        Self(AssetPath::from(value.clone()))
    }
}

impl From<&str> for ProtoPath {
    fn from(value: &str) -> Self {
        Self(AssetPath::from(value.to_owned()))
    }
}

impl From<ProtoPath> for AssetPath<'static> {
    fn from(value: ProtoPath) -> Self {
        value.0
    }
}

impl From<&ProtoPath> for AssetPath<'static> {
    fn from(value: &ProtoPath) -> Self {
        value.0.clone()
    }
}

impl<'a> From<ProtoPath> for Cow<'a, ProtoPath> {
    fn from(value: ProtoPath) -> Self {
        Cow::Owned(value)
    }
}

impl<'a> From<&'a ProtoPath> for Cow<'a, ProtoPath> {
    fn from(value: &'a ProtoPath) -> Self {
        Cow::Borrowed(value)
    }
}

impl From<ProtoPath> for HandleId {
    fn from(value: ProtoPath) -> Self {
        value.0.get_id().into()
    }
}

impl From<&ProtoPath> for HandleId {
    fn from(value: &ProtoPath) -> Self {
        value.0.get_id().into()
    }
}

impl<'a> AsRef<AssetPath<'a>> for ProtoPath {
    fn as_ref(&self) -> &AssetPath<'a> {
        &self.0
    }
}

impl AsRef<Path> for ProtoPath {
    fn as_ref(&self) -> &Path {
        self.0.path()
    }
}
