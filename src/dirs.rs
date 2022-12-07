//! Directory locations for common usage.

use directories::ProjectDirs;

lazy_static::lazy_static! {
    pub static ref PROJECT: ProjectDirs
                                = ProjectDirs::from("io", "schctl", env!("CARGO_PKG_NAME")).unwrap();
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
