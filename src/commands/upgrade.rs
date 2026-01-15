use std::fs;
use std::path::Path;

use colored::Colorize;

use crate::config::{ADDONS_DIR, Addon, Config};
use crate::error::{Error, Result};
use crate::git::{checkout_latest, ensure_repo, extract_repo_name, get_commit_hash};
use crate::plugin::parse_plugin_cfg;
use crate::utils::{copy_dir_recursive, ensure_in_godot_project, get_cache_dir};

pub fn execute(addon_name: Option<String>) -> Result<()> {
    ensure_in_godot_project()?;

    if !Config::exists() {
        return Err(Error::ConfigNotFound);
    }

    let mut config = Config::load()?;

    if config.addons.is_empty() {
        println!("No addons to upgrade.");
        return Ok(());
    }

    // Filter addons to upgrade
    let addons_to_upgrade: Vec<&Addon> = match &addon_name {
        Some(name) => {
            let addon = config.addons.iter().find(|a| a.name == *name);
            match addon {
                Some(a) => vec![a],
                None => {
                    return Err(Error::Other(format!("Addon '{}' not found", name)));
                }
            }
        }
        None => config.addons.iter().collect(),
    };

    println!(
        "Checking {} addon(s) for updates...",
        addons_to_upgrade.len()
    );

    let cache_dir = get_cache_dir()?;
    let local_addons_dir = Path::new(ADDONS_DIR);
    let mut updated_addons: Vec<(String, String, String)> = Vec::new(); // (name, old_hash, new_hash)

    for addon in &addons_to_upgrade {
        print!("  Checking: {}... ", addon.name);

        let repo_name = extract_repo_name(&addon.source)
            .ok_or_else(|| Error::InvalidUrl(addon.source.clone()))?;

        // Ensure repo exists and checkout latest
        let clone_dir = cache_dir.join(&repo_name);
        ensure_repo(&addon.source, &clone_dir)?;
        checkout_latest(&clone_dir)?;
        let new_hash = get_commit_hash(&clone_dir)?;

        if new_hash == addon.checksum {
            println!("{}", "already up to date".dimmed());
            continue;
        }

        println!("{}", "updating".yellow());

        // Copy new version
        let src_addon_path = clone_dir.join("addons").join(&addon.name);
        if !src_addon_path.exists() {
            println!(
                "{}",
                format!(
                    "    Warning: addon '{}' not found in latest version",
                    addon.name
                )
                .yellow()
            );
            continue;
        }

        let dst_addon_path = local_addons_dir.join(&addon.name);
        if dst_addon_path.exists() {
            fs::remove_dir_all(&dst_addon_path)?;
        }
        copy_dir_recursive(&src_addon_path, &dst_addon_path)?;

        // Get new version from plugin.cfg
        let plugin_cfg_path = dst_addon_path.join("plugin.cfg");
        let plugin_info = parse_plugin_cfg(&plugin_cfg_path);
        let new_version = plugin_info.version.unwrap_or_default();

        updated_addons.push((addon.name.clone(), new_hash.clone(), new_version));
    }

    // Update config with new versions
    for (name, new_hash, new_version) in &updated_addons {
        if let Some(addon) = config.addons.iter_mut().find(|a| a.name == *name) {
            addon.checksum = new_hash.clone();
            addon.version = new_version.clone();
        }
    }

    if !updated_addons.is_empty() {
        config.save()?;
        println!(
            "\n{}",
            format!("Updated {} addon(s):", updated_addons.len()).green()
        );
        for (name, _, version) in &updated_addons {
            if version.is_empty() {
                println!("  - {}", name.green());
            } else {
                println!("  - {} ({})", name.green(), version);
            }
        }
    } else {
        println!("\n{}", "All addons are up to date.".green());
    }

    Ok(())
}
