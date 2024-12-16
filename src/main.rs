use anyhow::Result;
use code_collector::cli::Cli;
use code_collector::config::Config;
use code_collector::collector::Collector;
use code_collector::output::Writer;

fn main() -> Result<()> {
    // Initialize CLI and parse arguments
    let cli = Cli::parse();

    // Load configurations (from system-wide settings and presets)
    let config = Config::initialize(&cli)?;

    // Initialize the writer with output settings
    let mut writer = Writer::new(&config.output_file)?;

    // Initialize the collector with necessary parameters
    let mut collector = Collector::new(&config);

    // Perform the collection
    collector.collect(&mut writer)?;

    println!("Code collection completed. Output written to {}", config.output_file.to_string_lossy());
    Ok(())
}
