use crate::collector::path_handler::{should_ignore, should_include};
use crate::config::Config;
use crate::output::Writer;
use anyhow::Result;
use ignore::WalkBuilder;
use std::path::Path;
use crate::collector::file_processor::process_file;

pub fn process_directory(dir: &Path, config: &Config, writer: &mut Writer) -> Result<()> {
    let walker = WalkBuilder::new(dir)
        .ignore(false)
        .git_ignore(false)
        .hidden(false)
        .build_parallel();

    walker.run(|| {
        let mut writer = writer.clone();
        Box::new(move |result| {
            match result {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_dir() && should_ignore(path, &config.ignore_paths) {
                        return ignore::WalkState::Skip;
                    } else if path.is_file() && should_include(path, &config.formats, &config.ignore_paths) {
                        if let Err(e) = process_file(path, &mut writer) {
                            eprintln!("Error processing file {}: {:?}", path.display(), e);
                        }
                    }
                }
                Err(e) => eprintln!("Error walking directory: {:?}", e),
            }
            ignore::WalkState::Continue
        })
    });

    Ok(())
}
