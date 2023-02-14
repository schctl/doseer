use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use doseer_core::dirs;
use iced_native::{image, svg};
use mime_guess::Mime;

use crate::mime::ImageOrSvg;

/// Convenience method to plug into maps
fn parse_u16(s: &str) -> Option<u16> {
    s.parse().ok()
}

/// Give a "score" to a dimension path. Such as `48x48` or `64x64@2x`.
fn dimension_score(path: &Path) -> Option<u16> {
    // ignore files
    if path.is_file() {
        return None;
    }

    let comp = path.components().last()?;
    let comp_str = comp.as_os_str().to_string_lossy();

    // prefer svg
    if comp_str == "scalable" {
        return Some(u16::MAX);
    }

    let mut split = comp_str.trim_end_matches('x').split('x');

    // try to match as close as possible to `AxA@Bx`
    if let Some(first) = split.next().and_then(parse_u16) {
        if let Some(second) = split.next() {
            if split.next().is_some() {
                return None;
            }

            let mut mul_split = second.split('@');
            let second = mul_split.next().and_then(parse_u16).unwrap();
            let multiplier = mul_split.next().and_then(parse_u16).unwrap_or(1);

            return Some(first * second * multiplier.pow(2));
        }
    }

    None
}

/// Load the required icon from the icon theme.
fn load_from_icon_theme(mime: &Mime, icon_dir: &Path) -> Option<ImageOrSvg> {
    // we need the absolute path for glob search
    let mut icon_dir = icon_dir.canonicalize().ok()?;

    if !icon_dir.is_dir() {
        return None;
    }

    // find all available icon sizes
    let mut dimensions = icon_dir
        .read_dir()
        .ok()?
        .into_iter()
        .filter_map(|ret| {
            ret.ok().and_then(|entry| {
                let path = entry.path();
                dimension_score(&path).map(|s| (path, s))
            })
        })
        .collect::<Vec<_>>();

    // sort by score so we can select the highest resolution icon
    dimensions.sort_by(|a, b| a.1.cmp(&b.1));

    if let Some(selected) = dimensions.pop().map(|(p, _)| p) {
        icon_dir = selected;
        // if we can't find a size, just continue on and hope
    }

    icon_dir.push("mimetypes");

    // reformat mimetype str
    let mime_str = OsString::from(mime.essence_str().replace('/', "-"));

    // find and load the icon
    for entry in icon_dir.read_dir().ok()?.flatten() {
        if entry.path().file_stem() == Some(&mime_str) {
            return if entry.path().extension() == Some(OsStr::new("svg")) {
                ImageOrSvg::Svg(svg::Handle::from_path(entry.path()))
            } else {
                ImageOrSvg::Image(image::Handle::from_path(entry.path()))
            }
            .into();
        }
    }

    None
}

/// Early return if the expression returns [`Some`].
macro_rules! exists_return {
    ($($expr:expr),*) => {
        $(
            if let Some(__val) = $expr {
                return Some(__val);
            }
        )*
    };
}

/// Try to load an appropriate icon for this mimetype.
pub fn load(mime: &Mime) -> Option<ImageOrSvg> {
    let mut icons_home = dirs::USER.home_dir().to_path_buf();
    icons_home.push(".icons");

    // Check for a `default` icon theme first
    exists_return! {
        load_from_icon_theme(mime, &{
            let mut default = icons_home.clone();
            default.push("default");
            default
        })
    }

    let icons_global = PathBuf::from("/usr/share/icons");

    exists_return! {
        load_from_icon_theme(mime, &{
            let mut default = icons_global.clone();
            default.push("default");
            default
        })
    }

    // Fallback
    fn try_load_all(mime: &Mime, path: &Path) -> Option<ImageOrSvg> {
        for ret in path.read_dir().ok()? {
            exists_return! {
                ret.ok().and_then(|theme| load_from_icon_theme(mime, &theme.path()))
            }
        }

        None
    }

    exists_return! {
        try_load_all(mime, &icons_home),
        try_load_all(mime, &icons_global)
    }

    None
}
