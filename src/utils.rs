use anyhow::Result;
use std::path::{Path, PathBuf};

// Example utility function to check if a path exists and is accessible
pub fn validate_path(path: &Path) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {:?}", path);
    }
    Ok(())
}

pub fn canonicalize_paths(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    paths.iter().map(|path| path.canonicalize().map_err(|e| anyhow::Error::new(e))).collect()
}

pub fn canonicalize_formats(extensions: &[String]) -> Vec<String> {
    extensions.iter().map(|ext| ext.trim_start_matches('.').to_owned()).collect()
}
