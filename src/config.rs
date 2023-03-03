//! Global configuration options.

use std::fs::File;
use std::io::{Read, Write};

use doseer_core::dirs;
use doseer_core::path::PathWrap;
use doseer_core::{resource, resource_make};

/// Global application configuration.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    /// Bookmark folder list.
    pub bookmarks: Vec<PathWrap>,
}

impl Config {
    /// Generate default set of application configurations.
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

    /// Try to load the configuration file.
    pub fn load() -> anyhow::Result<Self> {
        let path = resource!(config, "config.toml");
        // Return and write default config if none exists
        if !path.exists() {
            let config = Config::generate();
            config.flush()?;
            return Ok(config);
        }
        // Read
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // Deserialize
        Ok(toml::from_str(&contents)?)
    }

    /// Asynchronously try to write to the configuration file.
    pub fn flush(&self) -> anyhow::Result<()> {
        let path = resource_make!(config, "config.toml")?;
        // Serialize
        let contents = toml::to_string_pretty(&self)?;
        // Write
        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}
