use anyhow::{Context, Result};
use crate::cli::Cli;
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;
use std::io::Write;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub paths: Vec<PathBuf>,
    pub formats: Vec<String>,
    pub ignore_paths: Vec<PathBuf>,
    pub output_file: PathBuf,
    // Add more configuration fields as needed
}

impl Config {
    pub fn initialize(cli: &Cli) -> Result<Self> {
        let mut config = Config {
            paths: cli.paths.clone(),
            formats: cli.formats.clone(),
            ignore_paths: cli.ignore_paths.clone(),
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
}
