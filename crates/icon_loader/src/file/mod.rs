//! Load icons associated with filesystem paths.

use std::path::Path;
use std::sync::Arc;

use iced_core::{image, svg};

mod platform;

/// Handle to either an image or an svg.
#[derive(Debug, Clone)]
pub enum ImageOrSvg {
    Image(image::Handle),
    Svg(svg::Handle),
}

/// Shorthand.
pub type Icon = Arc<ImageOrSvg>;

/// Icon loader for filesystem paths.
pub struct Loader(platform::Loader);

impl Loader {
    /// Create a new icon loader.
    #[inline]
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(platform::Loader::new())
    }

    /// Try to load an appropriate icon for this file.
    #[inline]
    #[must_use]
    pub fn load(&self, path: &Path) -> Option<Icon> {
        self.0.load(path)
    }
}
