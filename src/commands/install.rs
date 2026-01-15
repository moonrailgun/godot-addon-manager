use std::fs;
use std::path::Path;

use colored::Colorize;

use crate::config::{ADDONS_DIR, Addon, Config};
use crate::error::{Error, Result};
use crate::git::{checkout, ensure_repo, extract_repo_name, get_commit_hash, is_valid_git_url};
use crate::plugin::parse_plugin_cfg;
use crate::utils::{
    copy_dir_recursive, ensure_in_godot_project, get_cache_dir, has_global_addons_ignore,
    update_gitignore,
};

pub fn execute(git_url: Option<String>) -> Result<()> {
    ensure_in_godot_project()?;

    match git_url {
        Some(url) => install_from_url(&url),
        None => install_all_dependencies(),
    }
}

/// Install addon from a git URL
fn install_from_url(url: &str) -> Result<()> {
    if !is_valid_git_url(url) {
        return Err(Error::InvalidUrl(url.to_string()));
    }

    let repo_name = extract_repo_name(url).ok_or_else(|| Error::InvalidUrl(url.to_string()))?;

    println!("Installing addon from: {}", url);

    // Ensure repository in cache directory
    let cache_dir = get_cache_dir()?;
    let clone_dir = cache_dir.join(&repo_name);

    println!("Fetching repository...");
    ensure_repo(url, &clone_dir)?;

    // Get commit hash before processing
    let commit_hash = get_commit_hash(&clone_dir)?;

    // Check for addons directory
    let source_addons_dir = clone_dir.join("addons");
    if !source_addons_dir.exists() {
        return Err(Error::AddonNotFound(
            "No 'addons' directory found in repository".to_string(),
        ));
    }

    // Load or create config
    let mut config = if Config::exists() {
        Config::load()?
    } else {
        return Err(Error::ConfigNotFound);
    };

    // Create local addons directory if needed
    let local_addons_dir = Path::new(ADDONS_DIR);
    fs::create_dir_all(local_addons_dir)?;

    // Process each addon in the repository
    let mut gitignore_entries = Vec::new();
    let mut installed_count = 0;

    for entry in fs::read_dir(&source_addons_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let addon_name = entry.file_name().to_string_lossy().to_string();
        let src_addon_path = entry.path();
        let dst_addon_path = local_addons_dir.join(&addon_name);

        println!("  Installing: {}", addon_name.green());

        // Copy addon to local addons directory
        if dst_addon_path.exists() {
            fs::remove_dir_all(&dst_addon_path)?;
        }
        copy_dir_recursive(&src_addon_path, &dst_addon_path)?;

        // Parse plugin.cfg for version
        let plugin_cfg_path = dst_addon_path.join("plugin.cfg");
        let plugin_info = parse_plugin_cfg(&plugin_cfg_path);
        let version = plugin_info.version.unwrap_or_default();

        // Add to gitignore entries
        gitignore_entries.push(format!("addons/{}", addon_name));

        // Update config - remove existing entry if present
        config.addons.retain(|a| a.name != addon_name);
        config.addons.push(Addon {
            name: addon_name,
            version,
            source: url.to_string(),
            checksum: commit_hash.clone(),
        });

        installed_count += 1;
    }

    // Update .gitignore (skip if global addons ignore exists)
    if !gitignore_entries.is_empty() && !has_global_addons_ignore() {
        update_gitignore(&gitignore_entries)?;
    }

    // Save config
    config.save()?;

    println!(
        "{}",
        format!("Installed {} addon(s) successfully!", installed_count).green()
    );

    Ok(())
}

/// Install all dependencies from gdam.yaml
fn install_all_dependencies() -> Result<()> {
    if !Config::exists() {
        return Err(Error::ConfigNotFound);
    }

    let config = Config::load()?;

    if config.addons.is_empty() {
        println!("No addons to install.");
        return Ok(());
    }

    println!("Installing {} addon(s)...", config.addons.len());

    let cache_dir = get_cache_dir()?;
    let local_addons_dir = Path::new(ADDONS_DIR);
    fs::create_dir_all(local_addons_dir)?;

    let mut gitignore_entries = Vec::new();

    for addon in &config.addons {
        println!("  Installing: {} ({})", addon.name.green(), addon.version);

        let repo_name = extract_repo_name(&addon.source)
            .ok_or_else(|| Error::InvalidUrl(addon.source.clone()))?;

        let clone_dir = cache_dir.join(&repo_name);

        // Ensure repo exists and checkout specific commit
        ensure_repo(&addon.source, &clone_dir)?;
        checkout(&clone_dir, &addon.checksum)?;

        // Copy addon from cache
        let src_addon_path = clone_dir.join("addons").join(&addon.name);
        if !src_addon_path.exists() {
            println!(
                "{}",
                format!("  Warning: addon '{}' not found in cache", addon.name).yellow()
            );
            continue;
        }

        let dst_addon_path = local_addons_dir.join(&addon.name);
        if dst_addon_path.exists() {
            fs::remove_dir_all(&dst_addon_path)?;
        }
        copy_dir_recursive(&src_addon_path, &dst_addon_path)?;

        gitignore_entries.push(format!("addons/{}", addon.name));
    }

    // Update .gitignore (skip if global addons ignore exists)
    if !gitignore_entries.is_empty() && !has_global_addons_ignore() {
        update_gitignore(&gitignore_entries)?;
    }

    println!("{}", "All addons installed successfully!".green());

    Ok(())
}
