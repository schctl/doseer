//! Global configuration options.

use std::fs::File;
use std::io::{Read, Write};

use crate::dirs;
use crate::path::PathWrap;
use crate::{resource, resource_make};

/// The sidebar configuration.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SideBar {
    /// Bookmark folder list.
    pub bookmarks: Vec<PathWrap>,
}

impl SideBar {
    /// Generate default set of [`SideBar`] configurations.
    pub fn generate() -> Self {
        let bookmarks = [
            Some(dirs::USER.home_dir()),
            dirs::USER.desktop_dir(),
            dirs::USER.document_dir(),
            dirs::USER.download_dir(),
            dirs::USER.picture_dir(),
        ]
        .into_iter()
        .filter_map(|p| p.map(PathWrap::from_path))
        .collect();

        Self { bookmarks }
    }
}

/// Global application configuration.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    /// Side bar configuration.
    pub side_bar: SideBar,
}

impl Config {
    /// Generate default set of application configurations.
    pub fn generate() -> Self {
        Self {
            side_bar: SideBar::generate(),
        }
    }
}

/// Try to read a configuration file.
pub fn read_config() -> anyhow::Result<Config> {
    let path = resource!(config, "config.toml");

    // Return and write default config if none exists
    if !path.exists() {
        let config = Config::generate();
        write_config(&config)?;
        return Ok(config);
    }

    // Read file
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize
    let config = toml::from_str(&contents)?;

    Ok(config)
}

/// Try to write a configuration file.
pub fn write_config(config: &Config) -> anyhow::Result<()> {
    let path = resource_make!(config, "config.toml")?;

    // Serialize

    // TODO: modify contents using `toml_edit` instead. Kind of low-priority anyway
    // since this is a GUI application and we would ideally not have users manually
    // editing the config anyway.
    let contents = toml::to_string_pretty(config)?;

    // Write
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
