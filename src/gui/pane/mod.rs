//! Pane widget.

pub mod tab;

/// A pane contains many tabs, but displays only one at a time.
pub struct Pane {
    /// A unique ID for this pane.
    pub id: usize,
    /// Tabs held by this pane.
    pub tabs: Vec<usize>,
    /// Currently open tab.
    pub focused: usize,
}
