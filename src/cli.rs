use clap::{Arg, ArgAction, Command, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "CodeCollector")]
#[command(version = "1.0")]
#[command(author = "iwannabeacookie")]
#[command(about = "Collects code from specified directories and files into a single text file.")]
#[command(override_usage = "code_collector <PATHS> [OPTIONS]...")]
pub struct Cli {
    /// Paths (files or directories) to collect code from.
    #[arg(required = true, value_name = "PATHS")]
    pub paths: Vec<PathBuf>,

    /// File formats to include. If not specified, all files are included.
    #[arg(short, long, num_args = 1.., value_name = "FORMATS", action = ArgAction::Append)]
    pub formats: Vec<String>,

    /// Paths (files or directories) to ignore.
    #[arg(short, long, num_args = 1.., value_name = "IGNORE_PATHS", action = ArgAction::Append)]
    pub ignore_paths: Vec<PathBuf>,

    /// Output file name.
    #[arg(short, long, value_name = "OUTPUT", default_value = "collected_code.txt")]
    pub output: PathBuf,
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
