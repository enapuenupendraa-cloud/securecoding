use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A secure file backup utility in Rust
#[derive(Parser)]
#[command(name = "safe_backup")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Which operation to perform
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Copy a file to a .bak file
    Backup {
        /// Path of the source file
        #[arg(value_name = "SRC")]
        src: PathBuf,
        /// Path where the backup file will be created
        #[arg(value_name = "DEST")]
        dest: PathBuf,
    },
    /// Restore a file from its .bak file
    Restore {
        /// Path of the backup file
        #[arg(value_name = "BACKUP")]
        backup: PathBuf,
        /// Directory to restore into
        #[arg(value_name = "TARGET")]
        target: PathBuf,
    },
    /// Delete a file if it is a regular file
    Delete {
        /// Path of the file to delete
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}
