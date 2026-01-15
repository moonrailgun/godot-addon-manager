use std::path::Path;

use colored::Colorize;

use crate::config::{ADDONS_DIR, Config, get_existing_addons};
use crate::error::Result;
use crate::utils::{ensure_in_godot_project, has_global_addons_ignore, update_gitignore};

pub fn execute() -> Result<()> {
    // Check if in a Godot project
    ensure_in_godot_project()?;

    // Check if gdam.yaml already exists
    if Config::exists() {
        println!(
            "{}",
            "gdam.yaml already exists, skipping initialization.".yellow()
        );
        return Ok(());
    }

    // Create default gdam.yaml
    let config = Config::default();
    config.save()?;
    println!("Created gdam.yaml");

    // If no addons folder exists, add addons/ to gitignore
    let addons_path = Path::new(ADDONS_DIR);
    if !addons_path.exists() && !has_global_addons_ignore() {
        update_gitignore(&["addons/".to_string()])?;
        println!("Added addons/ to .gitignore");
    }

    // Check for existing addons folder
    let existing_addons = get_existing_addons()?;
    if !existing_addons.is_empty() {
        println!(
            "\n{}",
            "Found existing addons that can be migrated to gdam:".yellow()
        );
        for addon in &existing_addons {
            println!("{}", format!("  - {}", addon).yellow());
        }
        println!(
            "\n{}",
            "Use 'gdam install' to add these addons to gdam management.".yellow()
        );
    }

    println!("Initialization complete!");
    Ok(())
}
