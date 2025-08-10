use std::{fs, path::Path};
use anyhow::{bail, Context, Result};

/// Deletes the file at `path` if it exists and is a regular file.
pub fn run(path: &Path) -> Result<()> {
    // Verify it’s a file
    if !path.is_file() {
        bail!("Path '{}' is not a regular file", path.display());
    }

    // Perform the deletion
    fs::remove_file(path)
        .with_context(|| format!("Failed to delete file '{}'", path.display()))?;

    Ok(())
}
