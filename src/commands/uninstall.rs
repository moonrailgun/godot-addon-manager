use std::fs;
use std::path::Path;

use colored::Colorize;

use crate::config::{ADDONS_DIR, Config};
use crate::error::{Error, Result};
use crate::utils::ensure_in_godot_project;

pub fn execute(query: String) -> Result<()> {
    ensure_in_godot_project()?;

    if !Config::exists() {
        return Err(Error::ConfigNotFound);
    }

    let mut config = Config::load()?;

    // Find addon by name or source (exact match)
    let addon = config
        .addons
        .iter()
        .find(|a| a.name == query || a.source == query);

    let addon_name = match addon {
        Some(a) => a.name.clone(),
        None => {
            return Err(Error::Other(format!(
                "Addon '{}' not found in gdam.yaml",
                query
            )));
        }
    };

    // Remove addon directory
    let addon_path = Path::new(ADDONS_DIR).join(&addon_name);
    if addon_path.exists() {
        fs::remove_dir_all(&addon_path)?;
        println!("Removed directory: {}", addon_path.display());
    }

    // Remove from config
    config.addons.retain(|a| a.name != addon_name);
    config.save()?;

    println!(
        "{}",
        format!("Uninstalled '{}' successfully!", addon_name).green()
    );

    Ok(())
}
