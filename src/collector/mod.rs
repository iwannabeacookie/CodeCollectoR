pub mod path_handler;
pub mod file_processor;
pub mod directory_processor;

use std::io::Write;

use crate::config::Config;
use crate::output::Writer;
use crate::utils::get_ignore;
use anyhow::Result;

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

        let mut ignore_paths = self.config.ignore_paths.clone();

        // Process each path
        for path in &self.config.paths {
            if path.is_file() {
                if let Some(ignore) = get_ignore(path) {
                    ignore_paths.extend(ignore);
                }

                if path_handler::should_include(path, &self.config.formats, &ignore_paths) {
                    file_processor::process_file(path, writer)?;
                }
            } else if path.is_dir() {
                if !path_handler::should_ignore(path, &ignore_paths) {
                    directory_processor::process_directory(path, &self.config, writer)?;
                }
            }
        }

        Ok(())
    }

    fn write_project_structure(&self, writer: &mut Writer) -> Result<()> {
        writeln!(writer, "Project Structure:")?;
        for path in &self.config.paths {
            // Ignore flag overrides the path flag
            // Should it be this way (???????)

            let mut ignore_paths = self.config.ignore_paths.clone();
            if path.is_dir() {
                ignore_paths.extend(get_ignore(path).unwrap_or(vec![]));
            }

            if path.is_dir() && !path_handler::should_ignore(&path, &ignore_paths) {
                writeln!(writer, "{}/", path.file_name().unwrap().to_string_lossy())?;
                path_handler::generate_tree(
                    &path,
                    "",
                    &mut ignore_paths,
                    &self.config.formats,
                    writer,
                )?;
            }
        }
        writeln!(writer, "\nCode Files:")?;
        writer.flush()?;
        Ok(())
    }
}
