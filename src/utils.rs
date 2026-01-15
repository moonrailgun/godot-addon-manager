use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

const GODOT_PROJECT_FILE: &str = "project.godot";
const GITIGNORE_FILE: &str = ".gitignore";

/// Check if current directory is a Godot project
pub fn ensure_in_godot_project() -> Result<()> {
    if !is_godot_project(".") {
        return Err(Error::NotInProject);
    }
    Ok(())
}

/// Check if the given path is a Godot project directory
pub fn is_godot_project<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().join(GODOT_PROJECT_FILE).exists()
}

/// Get the gdam cache directory
pub fn get_cache_dir() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| Error::Other("Cannot find system cache directory".to_string()))?;
    let gdam_cache = cache_dir.join("gdam");
    fs::create_dir_all(&gdam_cache)?;
    Ok(gdam_cache)
}

/// Copy a directory recursively
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    if !src.is_dir() {
        return Err(Error::Other(format!(
            "Source is not a directory: {:?}",
            src
        )));
    }

    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// Check if .gitignore has global addons ignore pattern
pub fn has_global_addons_ignore() -> bool {
    let gitignore_path = Path::new(GITIGNORE_FILE);
    if !gitignore_path.exists() {
        return false;
    }

    let file = match fs::File::open(gitignore_path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    for line in BufReader::new(file).lines().flatten() {
        let trimmed = line.trim();
        if trimmed == "addons/" || trimmed == "addons/*" || trimmed == "addons" {
            return true;
        }
    }

    false
}

/// Update .gitignore to add entries if they don't exist
pub fn update_gitignore(entries: &[String]) -> Result<()> {
    let gitignore_path = Path::new(GITIGNORE_FILE);

    // Read existing entries
    let existing: Vec<String> = if gitignore_path.exists() {
        let file = fs::File::open(gitignore_path)?;
        BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .collect()
    } else {
        Vec::new()
    };

    // Find new entries to add
    let new_entries: Vec<&String> = entries.iter().filter(|e| !existing.contains(e)).collect();

    if new_entries.is_empty() {
        return Ok(());
    }

    // Append new entries
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(gitignore_path)?;

    // Add newline if file doesn't end with one
    if !existing.is_empty() {
        writeln!(file)?;
    }

    for entry in new_entries {
        writeln!(file, "{}", entry)?;
    }

    Ok(())
}
