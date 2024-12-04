use anyhow::Result;
use std::path::Path;

// Example utility function to check if a path exists and is accessible
pub fn validate_path(path: &Path) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {:?}", path);
    }
    Ok(())
}

// Add more utility functions as needed
