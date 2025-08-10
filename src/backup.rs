use std::{fs, path::Path};
use anyhow::{bail, Context, Result};

/// Copies the file at `src` to `dest`, creating parent directories as needed.
pub fn run(src: &Path, dest: &Path) -> Result<()> {
    // Ensure source exists and is a file
    if !src.is_file() {
        bail!("Source path '{}' is not a regular file", src.display());
    }

    // Create destination directory if it doesn't exist
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory '{}'", parent.display()))?;
    }

    // Perform the copy
    fs::copy(src, dest)
        .with_context(|| format!("Failed to copy from '{}' to '{}'", src.display(), dest.display()))?;

    Ok(())
}
