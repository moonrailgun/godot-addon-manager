use colored::Colorize;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::utils::ensure_in_godot_project;

pub fn execute() -> Result<()> {
    ensure_in_godot_project()?;

    if !Config::exists() {
        return Err(Error::ConfigNotFound);
    }

    let config = Config::load()?;

    if config.addons.is_empty() {
        println!("No addons installed.");
        return Ok(());
    }

    println!("Installed addons:\n");

    for addon in &config.addons {
        let version_display = if addon.version.is_empty() {
            "unknown".dimmed().to_string()
        } else {
            addon.version.clone()
        };

        println!(
            "  {} {} {}",
            addon.name.green(),
            format!("v{}", version_display).dimmed(),
            format!("({})", &addon.checksum[..8]).dimmed()
        );
        println!("    {}", addon.source.dimmed());
    }

    println!("\n{} addon(s) total.", config.addons.len());

    Ok(())
}
