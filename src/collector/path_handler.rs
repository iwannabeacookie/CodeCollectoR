use std::io::Write;
use crate::output::Writer;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;

pub fn should_ignore(path: &Path, ignore_paths: &[PathBuf]) -> bool {
    ignore_paths.iter().any(|ignore| path.eq(ignore.as_path()))
        || path
            .file_name()
            .map_or(false, |name| name.to_string_lossy().starts_with('.'))
}

pub fn should_include(path: &Path, formats: &[String], ignore_paths: &[PathBuf]) -> bool {
    if should_ignore(path, ignore_paths) {
        return false;
    } else if path.is_dir() {
        return true;
    }
    if formats.is_empty() {
        return true;
    }
    path.extension()
        .and_then(|e| e.to_str())
        .map_or(false, |ext| formats.contains(&ext.to_string()))
}

pub fn generate_tree(
    dir: &Path,
    prefix: &str,
    ignore_paths: &Vec<PathBuf>,
    formats: &[String],
    writer: &mut Writer,
) -> Result<()> {
    let entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .collect();
    let count = entries.len();
    let last_unignored = entries.iter().rev().position(|entry| {
        let path = entry.path();
        should_include(&path, formats, &ignore_paths)
    }).map(|pos| count - pos).unwrap_or(count);

    for (i, entry) in entries.into_iter().enumerate() {
        let path = entry.path();
        if should_ignore(&path, &ignore_paths) {
            continue;
        }
        let connector = if i == last_unignored - 1 { "└── " } else { "├── " };
        if path.is_dir() {
            writeln!(
                writer,
                "{}{}{}/",
                prefix,
                connector,
                path.file_name().unwrap().to_string_lossy()
            )?;
            let new_prefix = if i == last_unignored - 1 {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            generate_tree(&path, &new_prefix, ignore_paths, formats, writer)?;
        } else {
            if should_include(&path, formats, &ignore_paths) {
                writeln!(
                    writer,
                    "{}{}{}",
                    prefix,
                    connector,
                    path.file_name().unwrap().to_string_lossy()
                )?;
            }
        }
    }
    Ok(())
}
