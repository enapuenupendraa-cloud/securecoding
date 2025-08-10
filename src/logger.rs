// src/logger.rs

use std::fs::OpenOptions;
use std::io::Write;
use anyhow::{Context, Result};
use chrono::Local;

const LOG_FILE: &str = "logfile.txt";

/// Ensure the log file exists (or create it) before any logging occurs.
pub fn init() -> Result<()> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .with_context(|| format!("Failed to create or open log file '{}'", LOG_FILE))?;
    Ok(())
}

/// Append a timestamped action to the log file.
pub fn log_action(action: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .with_context(|| format!("Failed to open log file '{}'", LOG_FILE))?;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(file, "{} - {}", timestamp, action)
        .with_context(|| format!("Failed to write to log file '{}'", LOG_FILE))?;

    Ok(())
}
