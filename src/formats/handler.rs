use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub trait FormatHandlerTrait {
    fn read_content(&self, path: &Path) -> Result<String>;
}

pub struct FormatHandler;

impl FormatHandler {
    pub fn from_path(path: &Path) -> Result<Box<dyn FormatHandlerTrait>> {
        match path.extension().and_then(|e| e.to_str()) {
            Some("txt") | Some("rs") | Some("md") => Ok(Box::new(TextFormat)),
            Some("bin") => Ok(Box::new(BinaryFormat)),
            Some(other) => Err(anyhow::anyhow!("Unsupported format: {}", other)),
            None => Err(anyhow::anyhow!("File has no extension: {:?}", path)),
        }
    }
}

struct TextFormat;

impl FormatHandlerTrait for TextFormat {
    fn read_content(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path)
            .with_context(|| format!("Reading UTF-8 file {}", path.display()))
    }
}

struct BinaryFormat;

impl FormatHandlerTrait for BinaryFormat {
    fn read_content(&self, path: &Path) -> Result<String> {
        let bytes = fs::read(path)
            .with_context(|| format!("Reading binary file {}", path.display()))?;
        // Example: Convert binary data to hex representation
        Ok(hex::encode(bytes))
    }
}
