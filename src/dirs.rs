//! Directory tools.

use std::path::Path;
use std::time::{Duration, Instant, SystemTime};

use directories::{BaseDirs, ProjectDirs};

use crate::path::PathWrap;

lazy_static::lazy_static! {
    pub static ref PROJECT: ProjectDirs
                                = ProjectDirs::from("io", "schctl", env!("CARGO_PKG_NAME")).unwrap();

    pub static ref BASE: BaseDirs = BaseDirs::new().unwrap();
}

/// Access a specific file or resource, based on required type.
#[macro_export]
macro_rules! resource {
    ($ty:ty, $($path:literal),*) => {
        {
            let mut res = paste::paste! {
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

/// Timestamps useful to indicate when to update contents.
#[derive(Debug, Clone, Copy)]
struct Checked {
    /// Last modified time of the fs.
    modified: SystemTime,
    /// Last read time.
    checked: Instant,
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
    /// Last checked time.
    checked: Checked,
}

impl Contents {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let location = PathWrap::from_path(path)?;

        let mut items = Vec::new();
        Self::read_items_into(&location, &mut items)?;
        let contents = items;

        let modified = match location.metadata()?.modified() {
            Ok(time) => time,
            Err(_) => SystemTime::now(),
        };

        let checked = Checked {
            modified,
            checked: Instant::now(),
        };

        Ok(Self {
            location,
            contents,
            checked,
        })
    }

    /// Get the location this tab points to.
    #[inline]
    pub fn location(&self) -> &Path {
        &self.location
    }

    /// Read the contents of this directory.
    #[inline]
    pub fn contents(&self) -> &[PathWrap] {
        &self.contents
    }

    /// Update contents if needed.
    pub fn update_contents(&mut self) -> anyhow::Result<()> {
        // Check if its been 5 seconds since the last check.
        // This is pretty reasonable since checking over this would be less intensive
        // than fetching fs metadata every call.
        if self.checked.checked.elapsed() > Duration::from_secs(5) {
            // Check the fs metadata for changes
            if let Ok(metadata) = self.location.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if modified > self.checked.modified {
                        // Update contents
                        Self::read_items_into(&self.location, &mut self.contents)?;

                        // Update last checked time
                        self.checked.checked = Instant::now();
                        self.checked.modified = modified;
                    }
                }
            }
        }

        Ok(())
    }

    /// Get items in this location.
    ///
    /// Clears the provided buffer of all its previous contents.
    fn read_items_into(path: &Path, buf: &mut Vec<PathWrap>) -> anyhow::Result<()> {
        buf.clear();

        // TODO: collect_into when its stabilized
        for entry in path.read_dir()?.filter_map(|e| match e {
            Ok(entry) => Some(entry.path()),
            _ => None,
        }) {
            buf.push(PathWrap::from_path(entry)?);
        }

        Ok(())
    }
}
