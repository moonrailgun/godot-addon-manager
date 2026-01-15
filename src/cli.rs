use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gdam")]
#[command(
    author,
    version,
    about = "Godot Addon Manager - A package manager for Godot addons"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show version information
    Version,

    /// Initialize a new gdam project in current directory
    Init,

    /// Install addons
    Install {
        /// Git repository URL (optional, installs all dependencies if not provided)
        git_url: Option<String>,
    },

    /// List installed addons
    List,

    /// Uninstall an addon
    Uninstall {
        /// Addon name or source URL to uninstall
        name: String,
    },

    /// Upgrade installed addons to latest version
    Upgrade {
        /// Addon name to upgrade (optional, upgrades all if not provided)
        addon_name: Option<String>,
    },

    /// Manage cache
    Cache {
        #[command(subcommand)]
        action: CacheAction,
    },
}

#[derive(Subcommand)]
pub enum CacheAction {
    /// Print cache directory path
    Path,

    /// Clear cache directory
    Clear,
}
