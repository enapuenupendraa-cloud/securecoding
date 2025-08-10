// src/restore.rs

use std::{fs, path::Path};
use anyhow::{bail, Context, Result};

/// Restores the backup file at `backup` into the directory `target_dir`,
/// stripping a `.bak` extension if present.
pub fn run(backup: &Path, target_dir: &Path) -> Result<()> {
    // Verify backup exists and is a file
    if !backup.is_file() {
        bail!("Backup path '{}' is not a regular file", backup.display());
    }

    // Determine the output file name
    let original_name = backup
        .file_name()
        .with_context(|| format!("Cannot extract file name from '{}'", backup.display()))?;

    // Strip ".bak" extension if present, else keep original name
    let output_name = match backup.extension() {
        Some(ext) if ext == "bak" => {
            backup
                .file_stem()
                .with_context(|| format!("Cannot extract file stem from '{}'", backup.display()))?
        }
        _ => original_name,
    };

    // Ensure the target directory exists (create if needed)
    fs::create_dir_all(target_dir)
        .with_context(|| format!("Failed to create target directory '{}'", target_dir.display()))?;

    // Build the destination path
    let dest_path = target_dir.join(output_name);

    // Copy the backup file to the destination
    fs::copy(backup, &dest_path)
        .with_context(|| {
            format!(
                "Failed to restore '{}' into '{}'",
                backup.display(),
                dest_path.display()
            )
        })?;

    Ok(())
}
