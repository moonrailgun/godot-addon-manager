use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::error::{Error, Result};

pub const CONFIG_FILE: &str = "gdam.yaml";
pub const ADDONS_DIR: &str = "addons";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub addons: Vec<Addon>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Addon {
    pub name: String,
    pub version: String,
    pub source: String,
    pub checksum: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            addons: Vec::new(),
        }
    }
}

impl Config {
    /// Check if config file exists in current directory
    pub fn exists() -> bool {
        Path::new(CONFIG_FILE).exists()
    }

    /// Load config from file
    pub fn load() -> Result<Self> {
        let content = fs::read_to_string(CONFIG_FILE)?;
        serde_yaml::from_str(&content)
            .map_err(|e| Error::Other(format!("Failed to parse config: {}", e)))
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| Error::Other(format!("Failed to serialize config: {}", e)))?;
        fs::write(CONFIG_FILE, content)?;
        Ok(())
    }
}

/// Get list of existing addon folders
pub fn get_existing_addons() -> Result<Vec<String>> {
    let addons_path = Path::new(ADDONS_DIR);
    if !addons_path.exists() {
        return Ok(Vec::new());
    }

    let mut addons = Vec::new();
    for entry in fs::read_dir(addons_path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                addons.push(name.to_string());
            }
        }
    }
    Ok(addons)
}
