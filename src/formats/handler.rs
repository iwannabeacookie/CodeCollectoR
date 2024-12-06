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
            Some("bin") => Ok(Box::new(BinaryFormat)),
            Some(_other) => Ok(Box::new(BaseFormat)),
            None => Err(anyhow::anyhow!("File has no extension: {:?}", path)),
        }
    }
}

struct BaseFormat;

impl FormatHandlerTrait for BaseFormat {
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
