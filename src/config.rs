//! Global configuration options.

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::{resource, resource_make};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    /// Dashbar folder list.
    dash_folders: Vec<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dash_folders: vec![],
        }
    }
}

/// Try to read a configuration file.
pub fn read_config() -> anyhow::Result<Config> {
    let path = resource!(config, "config.toml");

    // Return and write default config if none exists
    if !path.exists() {
        let config = Config::default();
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
