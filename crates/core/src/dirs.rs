//! Directory tools.

use std::path::Path;

use directories::{BaseDirs, ProjectDirs, UserDirs};

use crate::path::PathWrap;

lazy_static::lazy_static! {
    pub static ref PROJECT: ProjectDirs
                                = ProjectDirs::from("io", "schctl", "doseer").unwrap();
    pub static ref BASE: BaseDirs = BaseDirs::new().unwrap();
    pub static ref USER: UserDirs = UserDirs::new().unwrap();
}

/// Access a specific file or resource, based on required type.
#[macro_export]
macro_rules! resource {
    ($ty:ty, $($path:literal),*) => {
        {
            let mut res = $crate::__paste::paste! {
                $crate::dirs::PROJECT.[<$ty _dir>]().to_owned()
            };

            $(
                res.push($path);
            )*

            res
        }
    };
}

/// Access a resource, while also recursively creating parent directories. See [`resource`]
/// for specifics.
#[macro_export]
macro_rules! resource_make {
    ($ty:ty, $($path:literal),*) => {
        {
            let res = $crate::resource!($ty, $($path),*);

            match res.parent() {
                Some(parent) => std::fs::create_dir_all(parent).map(|_| res),
                _ => Ok(res)
            }
        }
    };
}

/// Reads the contents of a specific directory.
///
/// Designed to be readable and self-update as frequently as possible.
#[derive(Debug)]
pub struct Contents {
    /// The currently open location.
    location: PathWrap,
    /// Items in current location.
    contents: Vec<PathWrap>,
}

impl Contents {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let location = PathWrap::from_path(path);

        let mut items = Vec::new();
        Self::read_items_into(&location, &mut items)?;
        let contents = items;

        Ok(Self { location, contents })
    }

    /// Get the location this tab points to.
    #[inline]
    pub fn location(&self) -> &PathWrap {
        &self.location
    }

    /// Read the contents of this directory.
    #[inline]
    pub fn contents(&self) -> &[PathWrap] {
        &self.contents
    }

    /// Update contents if needed.
    pub fn update_contents(&mut self) -> anyhow::Result<()> {
        Self::read_items_into(&self.location, &mut self.contents)
    }

    /// Get items in this location.
    ///
    /// Clears the provided buffer of all its previous contents.
    fn read_items_into(path: &Path, buf: &mut Vec<PathWrap>) -> anyhow::Result<()> {
        buf.clear();

        // TODO: collect_into when its stabilized
        for entry in path.read_dir()?.filter_map(Result::ok) {
            buf.push(PathWrap::from_into_path(entry.path()));
        }

        Ok(())
    }
}
