//! Icon loader based on the [freedesktop icon spec].
//!
//! [freedesktop icon spec]: https://specifications.freedesktop.org/icon-theme-spec

use std::borrow::Cow;
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};

use doseer_core::dirs;
use doseer_core::path::PathWrap;
use iced_core::{image, svg};
use ini::Ini;
use moka::sync::Cache;

use crate::file::{Icon, ImageOrSvg};

mod desktop;

/// Push a path temporarily while evaluating an expression.
macro_rules! pop_out {
    ($path:expr, $pop:expr, $val:expr) => {{
        let mut __path = $path;
        __path.push($pop);
        let __val = ($val)(&mut __path);
        __path.pop();
        __val
    }};
}

/// Loader implementation.
#[derive(Debug)]
pub struct Loader {
    /// Icon cache for mimetype.
    mime_cache: Cache<String, Icon>,
    /// Icon cache for specific places such as home directory.
    place_cache: Cache<&'static str, Icon>,
    /// User themes in order of priority.
    themes: Vec<Theme>,
}

impl Loader {
    #[inline]
    pub fn new() -> Self {
        Self {
            themes: load_user_themes(),
            mime_cache: Cache::builder().max_capacity(100).build(),
            place_cache: Cache::builder().max_capacity(36).build(),
        }
    }

    pub fn load(&self, path: &Path) -> Option<Icon> {
        if path.is_dir() {
            self.load_place(path)
        } else {
            self.load_mime(path)
        }
    }

    fn load_mime(&self, path: &Path) -> Option<Icon> {
        let mime = mime_guess::from_path(path).first_or_text_plain();
        let essence = mime.essence_str().replace('/', "-");

        if let Some(icon) = self.mime_cache.get(&essence) {
            return Some(icon);
        }

        if let Some(icon) = self._raw_load(|theme| theme.load_mime(&essence)) {
            self.mime_cache.insert(essence, icon.clone());
            return Some(icon);
        }

        None
    }

    fn load_place(&self, path: &Path) -> Option<Icon> {
        let essence = if path == dirs::BASE.home_dir() {
            "folder-home"
        } else if Some(path) == dirs::USER.desktop_dir() {
            "folder-desktop"
        } else if Some(path) == dirs::USER.picture_dir() {
            "folder-pictures"
        } else {
            "folder"
        };
        // TODO: ...

        if let Some(icon) = self.place_cache.get(essence) {
            return Some(icon);
        }

        if let Some(icon) = self._raw_load(|theme| theme.load_place(essence)) {
            self.place_cache.insert(essence, icon.clone());
            return Some(icon);
        }

        None
    }

    /// Load a resource from an appropriate theme.
    fn _raw_load<F>(&self, loader: F) -> Option<Icon>
    where
        F: Fn(&Theme) -> Option<Icon>,
    {
        for theme in &self.themes {
            if let Some(icon) = (loader)(theme) {
                return Some(icon);
            }

            for inherit in &theme.inherits {
                if let Ok(inherited) = self.themes.binary_search_by(|probe| {
                    if probe.name.eq_ignore_ascii_case(inherit) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }) {
                    if let Some(icon) = (loader)(&self.themes[inherited]) {
                        return Some(icon);
                    }
                }
            }
        }

        None
    }
}

/// A theme index with the highest resolution of each useful directory.
#[derive(Debug, Clone)]
struct Theme {
    /// Name of the theme.
    name: String,
    /// Path to the mimetype icons.
    mimes: Vec<PathWrap>,
    /// Path to the filesystem icons.
    places: Vec<PathWrap>,
    /// Names of inherited themes.
    inherits: Vec<String>,
}

impl Theme {
    /// Load a mimetype icon from this theme.
    #[inline]
    fn load_mime(&self, essence: &str) -> Option<Icon> {
        let essence = OsStr::new(essence);

        for mime in &self.mimes {
            if let Some(icon) = Self::_raw_load(mime, essence) {
                return Some(icon);
            }
        }

        None
    }

    /// Try to load the icon for a "place" from this theme.
    #[inline]
    fn load_place(&self, essence: &str) -> Option<Icon> {
        let essence = OsStr::new(essence);

        for mime in &self.places {
            if let Some(icon) = Self::_raw_load(mime, essence) {
                return Some(icon);
            }
        }

        None
    }

    /// Load an icon from its "essence" given an icon directory.
    fn _raw_load(icon_dir: &Path, essence: &OsStr) -> Option<Icon> {
        for entry in icon_dir.read_dir().ok()?.flatten() {
            if entry.path().file_stem() == Some(essence) {
                return Some(Arc::new(
                    if entry.path().extension() == Some(OsStr::new("svg")) {
                        ImageOrSvg::Svg(svg::Handle::from_path(entry.path()))
                    } else {
                        ImageOrSvg::Image(image::Handle::from_path(entry.path()))
                    },
                ));
            }
        }

        None
    }

    /// Try to parse an `index.theme` file.
    #[inline]
    fn parse_index<P>(path: P) -> Option<Self>
    where
        P: AsRef<Path>,
    {
        Self::_parse_index(path.as_ref())
    }

    /// Try to load an `index.theme` file in the given directory.
    #[inline]
    fn parse_index_at(path: &mut PathBuf) -> Option<Self> {
        // FIXME: handle index.theme being in a subdirectory also?
        pop_out!(path, "index.theme", Self::parse_index)
    }

    /// Parse implementation.
    #[tracing::instrument]
    fn _parse_index(path: &Path) -> Option<Self> {
        tracing::trace!("indexing icon theme...");

        let path = path.canonicalize().ok()?;
        let parent = path.parent().unwrap();

        let index = Ini::load_from_file(&path).ok()?;

        let main_section = index.section(Some("Icon Theme"))?;

        let name = main_section
            .get("Name")
            .map_or_else(|| parent.to_string_lossy(), Cow::Borrowed)
            .into_owned();

        let inherits = main_section
            .get("Inherits")
            .map(|s| s.split(',').map(ToOwned::to_owned).collect::<Vec<_>>())
            .unwrap_or_default();

        // --- Parse icon directories ---

        let directories = main_section.get("Directories")?;

        // "Score" a directory based on how suitable it is for our use.
        // Prefer scalable icons, then higher resolution ones.
        let score_directory = |dir: &str| -> u32 {
            // Load the directory description
            // For example [48x48@2x/mimetypes]
            if let Some(dir_section) = index.section(Some(dir)) {
                // resolve scalable icon
                if dir_section.get("Type") == Some("Scalable") {
                    // Get the max size of a scalable icon
                    if let Some(max_size) = dir_section.get("MaxSize") {
                        if let Ok(parsed) = max_size.parse::<u32>() {
                            return parsed.pow(2);
                        }
                    }
                }

                // resolve fixed icon
                if let Some(size) = dir_section.get("Size") {
                    if let Ok(parsed) = size.parse::<u32>() {
                        // scale factor
                        let scale = dir_section
                            .get("Scale")
                            .and_then(|s| s.parse::<u32>().ok())
                            .unwrap_or(1);
                        return parsed.pow(2) * scale.pow(2);
                    }
                }
            }

            0
        };

        let mut mimes = Vec::with_capacity(2);
        let mut places = Vec::with_capacity(2);

        for dir in directories.split(',') {
            if dir.contains("mimetypes") {
                mimes.push((dir, (score_directory)(dir)));
            } else if dir.contains("places") {
                places.push((dir, (score_directory)(dir)));
            }
        }

        if mimes.is_empty() && places.is_empty() {
            return None;
        }

        // Sort and remap
        mimes.sort_unstable_by_key(|a| a.1);
        mimes.reverse();
        places.sort_unstable_by_key(|a| a.1);
        places.reverse();

        let mimes = mimes
            .into_iter()
            .map(|a| PathWrap::from_into_path(parent.join(a.0)))
            .collect();
        let places = places
            .into_iter()
            .map(|a| PathWrap::from_into_path(parent.join(a.0)))
            .collect();

        // ---     ---        ---     ---

        tracing::debug!(name = name, "indexed icon theme");

        Some(Self {
            name,
            mimes,
            places,
            inherits,
        })
    }
}

/// Try to load the user's current theme.
#[tracing::instrument]
fn load_user_themes() -> Vec<Theme> {
    /// Spawn a theme loading task with a channel.
    fn spawn_load<F>(f: F) -> Receiver<Option<Theme>>
    where
        F: FnOnce() -> Option<Theme> + Send + Sync,
    {
        let (tx, rx) = mpsc::channel();

        rayon::scope(move |s| {
            s.spawn(move |_| {
                let ret = (f)();
                let _ = tx.send(ret);
            });
        });

        rx
    }

    let mut jobs = Vec::with_capacity(3);

    // Theme search directories
    let mut home_icon_dir = dirs::USER.home_dir().to_path_buf();
    home_icon_dir.push(".icons");

    let mut xdg_icon_dir = dirs::BASE.data_dir().to_path_buf();
    xdg_icon_dir.push("icons");

    let global_icon_dir = PathBuf::from("/usr/share/icons");

    // Load themes in thread pool
    for lookup in [home_icon_dir, xdg_icon_dir, global_icon_dir] {
        if let Ok(contents) = lookup.read_dir() {
            for child in contents.filter_map(Result::ok) {
                let mut path = child.path();
                jobs.push(spawn_load(|| Theme::parse_index_at(&mut path)));
            }
        }
    }

    // Join all jobs
    let mut all_themes = jobs
        .into_iter()
        .filter_map(|rx| match rx.recv().ok().flatten() {
            Some(t) => {
                tracing::info!(name = t.name, "loaded icon theme");
                Some(t)
            }
            None => None,
        })
        .collect::<Vec<_>>();

    // Check if there is a desktop environment configured theme, and move that to the front
    if let Some(desktop_theme) = desktop::current_theme() {
        if let Ok(idx) = all_themes.binary_search_by(|t| {
            if t.name.eq_ignore_ascii_case(&desktop_theme) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }) {
            all_themes.swap(0, idx);

            tracing::info!(name = desktop_theme, "loaded desktop icon theme");
        }
    }

    all_themes
}
