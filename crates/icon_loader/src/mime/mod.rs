//! Load icons associated with mime-types.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use iced_native::{image, svg};
use mime_guess::Mime;
use parking_lot::RwLock;

mod platform;

/// Handle to either an image or an svg.
#[derive(Debug, Clone)]
pub enum ImageOrSvg {
    Image(image::Handle),
    Svg(svg::Handle),
}

lazy_static::lazy_static! {
    static ref MIME_CACHE: RwLock<HashMap<Mime, Arc<ImageOrSvg>>> = RwLock::new(HashMap::new());
}

/// Try to load an appropriate icon for this file.
pub fn load<P: AsRef<Path>>(path: P) -> Option<Arc<ImageOrSvg>> {
    let mime = mime_guess::from_path(path).first_or_text_plain();
    // TODO: handle generating thumbnails for media types

    if let Some(icon) = MIME_CACHE.read().get(&mime) {
        return Some(icon.clone());
    }

    platform::load(&mime).map(|icon| {
        let icon_ptr = Arc::new(icon);
        MIME_CACHE.write().insert(mime.clone(), icon_ptr.clone());
        icon_ptr
    })
}
