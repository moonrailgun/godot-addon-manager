use std::path::Path;
use std::process::Command;

use crate::error::{Error, Result};

/// Ensure repository exists and is up to date. Clone if not exists, fetch if exists.
pub fn ensure_repo(url: &str, target_dir: &Path) -> Result<()> {
    if target_dir.join(".git").exists() {
        // Repository exists, fetch latest
        fetch_repo(target_dir)?;
    } else {
        // Clone new repository
        clone_repo(url, target_dir)?;
    }
    Ok(())
}

/// Clone a git repository
pub fn clone_repo(url: &str, target_dir: &Path) -> Result<()> {
    let output = Command::new("git")
        .args(["clone", url])
        .arg(target_dir)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::GitError(format!("Failed to clone: {}", stderr)));
    }

    Ok(())
}

/// Fetch latest changes from remote
pub fn fetch_repo(repo_dir: &Path) -> Result<()> {
    let output = Command::new("git")
        .args(["fetch", "origin"])
        .current_dir(repo_dir)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::GitError(format!("Failed to fetch: {}", stderr)));
    }

    Ok(())
}

/// Checkout a specific commit or branch
pub fn checkout(repo_dir: &Path, ref_name: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["checkout", ref_name])
        .current_dir(repo_dir)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::GitError(format!("Failed to checkout: {}", stderr)));
    }

    Ok(())
}

/// Checkout and pull latest from default branch
pub fn checkout_latest(repo_dir: &Path) -> Result<()> {
    // Get default branch name
    let output = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD", "--short"])
        .current_dir(repo_dir)
        .output()?;

    let default_branch = if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        branch.strip_prefix("origin/").unwrap_or(&branch).to_string()
    } else {
        "main".to_string() // fallback
    };

    // Checkout default branch
    checkout(repo_dir, &default_branch)?;

    // Pull latest
    let output = Command::new("git")
        .args(["pull", "origin", &default_branch])
        .current_dir(repo_dir)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::GitError(format!("Failed to pull: {}", stderr)));
    }

    Ok(())
}

/// Get the current commit hash of a repository
pub fn get_commit_hash(repo_dir: &Path) -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo_dir)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::GitError(format!(
            "Failed to get commit hash: {}",
            stderr
        )));
    }

    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(hash)
}

/// Validate if a string is a valid git URL
pub fn is_valid_git_url(url: &str) -> bool {
    url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("git@")
        || url.starts_with("git://")
}

/// Extract repository name from git URL
pub fn extract_repo_name(url: &str) -> Option<String> {
    let url = url.trim_end_matches('/').trim_end_matches(".git");
    url.rsplit('/').next().map(|s| s.to_string())
}
