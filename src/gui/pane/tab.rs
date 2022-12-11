//! Tab widget.

use std::path::{Path, PathBuf};

use crate::dirs;

/// Preserved scroll state of this tab.
#[derive(Debug, Default)]
struct ScrollableState {
    offset: f32,
}

/// A single tab displays a single open location.
#[derive(Debug)]
pub struct Tab {
    /// The currently open location.
    location: PathBuf,
    /// Preserved scroll state.
    scroll: ScrollableState,
}

impl Tab {
    /// Open a new tab with the user's home directory.
    #[inline]
    pub fn new() -> Self {
        Self::new_with(dirs::BASE.home_dir())
    }

    /// Open a new tab with a specified location.
    #[inline]
    pub fn new_with(location: &Path) -> Self {
        Self {
            location: location.to_owned(),
            scroll: ScrollableState::default(),
        }
    }

    /// Get the location this tab points to.
    #[inline]
    pub fn location(&self) -> &Path {
        &self.location
    }
}
