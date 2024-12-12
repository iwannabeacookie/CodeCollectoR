use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::fs;

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

pub fn get_ignore(dir: &Path) -> Option<Vec<PathBuf>> {
    let ignore_file_path = dir.join(".collectignore");
    if ignore_file_path.exists() {
        let contents = fs::read_to_string(&ignore_file_path)
            .with_context(|| format!("Reading ignore file from {:?}", ignore_file_path));
        if let Ok(contents) = contents {
            let ignore_paths = contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|path| dir.join(path))
            .collect();
            return Some(ignore_paths);
        }
    } 
    return None;
}
