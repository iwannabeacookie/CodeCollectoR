use anyhow::{Context, Result};
use crate::cli::Cli;
use std::fs;
use std::path::{PathBuf, Path};
use serde::Deserialize;
use ignore::WalkBuilder;
use std::sync::{Arc, Mutex};
use crate::collector::path_handler::should_ignore;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub paths: Vec<PathBuf>,
    pub formats: Vec<String>,
    pub ignore_paths: Vec<PathBuf>,
    pub ignore_files: Vec<Vec<PathBuf>>,
    pub output_file: PathBuf,
    // Add more configuration fields as needed
}

impl Config {
    pub fn initialize(cli: &Cli) -> Result<Self> {
        let mut config = Config {
            paths: cli.paths.clone(),
            formats: cli.formats.clone(),
            ignore_paths: cli.ignore_paths.clone(),
            ignore_files: vec![],
            output_file: cli.output.clone(),
        };

        // Load system-wide configurations
        if let Some(system_config) = Self::load_system_config()? {
            config.merge(system_config);
        }

        // Load preset configurations
        if let Some(preset_config) = Self::load_preset_config("default_preset")? {
            config.merge(preset_config);
        }

        config.paths = crate::utils::canonicalize_paths(&config.paths)
            .with_context(|| "Canonicalizing paths")?;

        config.ignore_paths = crate::utils::canonicalize_paths(&config.ignore_paths)
            .with_context(|| "Canonicalizing ignore paths")?;

        config.formats = crate::utils::canonicalize_formats(&config.formats);

        Ok(config)
    }

    fn load_system_config() -> Result<Option<Config>> {
        let system_config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("/etc"))
            .join("code_collector")
            .join("config.toml");

        if system_config_path.exists() {
            let contents = fs::read_to_string(&system_config_path)
                .with_context(|| format!("Reading system config from {:?}", system_config_path))?;
            let parsed: Config = toml::from_str(&contents)
                .with_context(|| "Parsing system config")?;
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }

    fn load_preset_config(preset_name: &str) -> Result<Option<Config>> {
        let preset_config_path = PathBuf::from(format!("presets/{}.toml", preset_name));
        if preset_config_path.exists() {
            let contents = fs::read_to_string(&preset_config_path)
                .with_context(|| format!("Reading preset config from {:?}", preset_config_path))?;
            let parsed: Config = toml::from_str(&contents)
                .with_context(|| "Parsing preset config")?;
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }

    fn merge(&mut self, other: Config) {
        self.paths.extend(other.paths);
        self.formats.extend(other.formats);
        self.ignore_paths.extend(other.ignore_paths);
        // Merge other fields as necessary
    }

    pub fn get_ignore(&mut self, dir: &Path) {
        let walker = WalkBuilder::new(dir)
            .ignore(false)
            .git_ignore(false)
            .hidden(false)
            .build_parallel();

        let ignore_paths = Arc::new(Mutex::new(&mut self.ignore_paths));

        walker.run(|| {
            let ignore_paths = ignore_paths.clone();
            Box::new(move |result| {
                match result {
                    Ok(entry) => {
                        // If the entry should be ignored or is not a directory, skip it
                        if should_ignore(&entry.path(), &ignore_paths.lock().unwrap()) {
                            return ignore::WalkState::Skip;
                        }
                        if !entry.path().is_dir() {
                            return ignore::WalkState::Continue;
                        }

                        let path = entry.path();
                        let ignore_file_path = path.join(".collectignore");
                        if ignore_file_path.exists() {
                            let contents = fs::read_to_string(&ignore_file_path)
                                .with_context(|| format!("Reading ignore file from {:?}", ignore_file_path));
                            if let Ok(contents) = contents {
                                let mut data = ignore_paths.lock().unwrap();
                                let ignores: Vec<PathBuf> = contents
                                    .lines()
                                    .map(|line| line.split('#').next().unwrap().trim())
                                    .filter(|line| !line.is_empty())
                                    .map(|line| path.join(line))
                                    .map(|line| PathBuf::from(line))
                                    .collect();
                                data.extend(ignores);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error walking directory: {:?}", e),
                }
                ignore::WalkState::Continue
            })
        });
    }
}
