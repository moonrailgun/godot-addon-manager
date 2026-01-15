use std::fs;

use colored::Colorize;

use crate::cli::CacheAction;
use crate::error::Result;
use crate::utils::get_cache_dir;

pub fn execute(action: CacheAction) -> Result<()> {
    match action {
        CacheAction::Path => print_path(),
        CacheAction::Clear => clear_cache(),
    }
}

fn print_path() -> Result<()> {
    let cache_dir = get_cache_dir()?;
    println!("{}", cache_dir.display());
    Ok(())
}

fn clear_cache() -> Result<()> {
    let cache_dir = get_cache_dir()?;

    if !cache_dir.exists() {
        println!("Cache directory does not exist.");
        return Ok(());
    }

    // Get size before clearing
    let size = get_dir_size(&cache_dir);

    fs::remove_dir_all(&cache_dir)?;
    fs::create_dir_all(&cache_dir)?;

    println!(
        "{}",
        format!("Cache cleared! Freed {}", format_size(size)).green()
    );
    Ok(())
}

fn get_dir_size(path: &std::path::Path) -> u64 {
    let mut size = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                size += get_dir_size(&path);
            } else if let Ok(meta) = path.metadata() {
                size += meta.len();
            }
        }
    }
    size
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
