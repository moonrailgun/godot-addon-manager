mod cli;
mod commands;
pub mod config;
mod error;
pub mod git;
pub mod plugin;
pub mod utils;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Version => commands::version::execute(),
        Commands::Init => commands::init::execute(),
        Commands::Install { git_url } => commands::install::execute(git_url),
        Commands::List => commands::list::execute(),
        Commands::Uninstall { name } => commands::uninstall::execute(name),
        Commands::Upgrade { addon_name } => commands::upgrade::execute(addon_name),
        Commands::Cache { action } => commands::cache::execute(action),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
