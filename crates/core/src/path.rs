//! Path wrapper.

use std::ffi::OsStr;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Immutable wrapper around a path.
///
/// This exists to avoid using more memory than needed since we need to pass around owned [`Path`]s
/// everywhere in the application.
#[derive(Debug, Clone)]
pub struct PathWrap(Arc<Path>);

impl PathWrap {
    /// Create a [`PathWrap`] from a [`Path`].
    #[must_use]
    #[inline]
    pub fn from_path<T>(path: T) -> Self
    where
        T: AsRef<Path>,
    {
        Self::from_into_path(path.as_ref())
    }

    /// Create a new [`PathWrap`] from something that can be converted into an owned [`Path`], potentially
    /// avoiding an extra allocation.
    #[must_use]
    #[inline]
    pub fn from_into_path<T>(path: T) -> Self
    where
        T: Into<Arc<Path>>,
    {
        Self(path.into())
    }

    /// A *good-enough* "display" name for this path.
    #[must_use]
    #[inline]
    pub fn display(&self) -> &OsStr {
        self.components().next_back().unwrap().as_os_str()
    }
}

impl Deref for PathWrap {
    type Target = Path;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for PathWrap {
    #[inline]
    fn as_ref(&self) -> &Path {
        self
    }
}

// --- Serialization impl ---

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

impl Serialize for PathWrap {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("PathWrap", self.as_ref())
    }
}

impl<'de> Deserialize<'de> for PathWrap {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = PathBuf::deserialize(deserializer)?;
        Ok(Self::from_path(inner))
    }
}
