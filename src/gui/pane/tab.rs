//! Tab widget.

use std::path::PathBuf;

/// Preserved scroll state of this tab.
#[derive(Debug, Default)]
pub struct ScrollableState {
    pub offset: f32,
}

/// A single tab displays a single open location.
#[derive(Debug)]
pub struct Tab {
    /// Unique identifier for this tab.
    pub id: usize,
    /// The currently open location.
    pub location: PathBuf,
    /// Preserved scroll state.
    pub scroll: ScrollableState,
}
