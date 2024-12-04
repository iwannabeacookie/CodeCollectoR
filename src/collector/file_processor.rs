use crate::formats::handler::FormatHandler;
use crate::output::Writer;
use anyhow::{Context, Result};
use std::io::Write;
use std::path::Path;

pub fn process_file(path: &Path, writer: &mut Writer) -> Result<()> {
    writeln!(writer, "\n================\nFilepath: {}\n################", path.display())?;
    let handler = FormatHandler::from_path(path)?;
    let content = handler.read_content(path)
        .with_context(|| format!("Reading file {}", path.display()))?;
    writeln!(writer, "{}", content)?;
    Ok(())
}
