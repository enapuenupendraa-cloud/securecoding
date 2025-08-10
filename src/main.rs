use anyhow::Result;
use clap::Parser;

mod cli;
mod backup;
mod restore;
mod delete;
mod logger;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Initialize our logger (writes to logfile.txt)
    logger::init()?;

    // Dispatch to the chosen subcommand
    match cli.command {
        Commands::Backup { src, dest } => {
            backup::run(&src, &dest)?;
            logger::log_action(&format!(
                "Backup: '{}' -> '{}'",
                src.display(),
                dest.display()
            ))?;
        }
        Commands::Restore { backup, target } => {
            restore::run(&backup, &target)?;
            logger::log_action(&format!(
                "Restore: '{}' -> '{}'",
                backup.display(),
                target.display()
            ))?;
        }
        Commands::Delete { file } => {
            delete::run(&file)?;
            logger::log_action(&format!("Delete: '{}'", file.display()))?;
        }
    }

    Ok(())
}
