pub mod path_handler;
pub mod file_processor;
pub mod directory_processor;

use std::io::Write;

use crate::config::Config;
use crate::output::Writer;
use anyhow::{Context, Result};

pub struct Collector {
    config: Config,
}

impl Collector {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub fn collect(&self, writer: &mut Writer) -> Result<()> {
        // Write Project Structure
        self.write_project_structure(writer)?;

        // Process each path
        for path in &self.config.paths {
            if path.is_file() {
                if path_handler::should_include(path, &self.config.formats, &self.config.ignore_paths) {
                    file_processor::process_file(path, writer)?;
                }
            } else if path.is_dir() {
                if !path_handler::should_ignore(path, &self.config.ignore_paths) {
                    directory_processor::process_directory(path, &self.config, writer)?;
                }
            }
        }

        Ok(())
    }

    fn write_project_structure(&self, writer: &mut Writer) -> Result<()> {
        writeln!(writer, "Project Structure:")?;
        for path in &self.config.paths {
            let canonical_path = path.canonicalize()
                .with_context(|| format!("Canonicalizing path {:?}", path))?;
            if canonical_path.is_dir() {
                path_handler::generate_tree(
                    &canonical_path,
                    "",
                    &self.config.ignore_paths,
                    &self.config.formats,
                    writer,
                )?;
            } else if canonical_path.is_file() {
                writeln!(writer, "{}", canonical_path.display())?;
            }
        }
        writeln!(writer, "\nCode Files:")?;
        Ok(())
    }
}
