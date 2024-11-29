use clap::{Arg, ArgAction, Command};
use ignore::WalkBuilder;
use std::fs::{self, File, canonicalize};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use std::sync::{Arc, Mutex};

fn main() -> Result<()> {
    let matches = Command::new("CodeCollector")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Collects code from specified directories and files into a single text file.")
        .arg(
            Arg::new("paths")
                .value_name("PATHS")
                .help("Paths (files or directories) to collect code from.")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("formats")
                .short('f')
                .long("formats")
                .value_name("FORMAT")
                .help("File formats to include. If not specified, all files are included.")
                .action(ArgAction::Append)
                .num_args(1..),
        )
        .arg(
            Arg::new("ignore_paths")
                .short('i')
                .long("ignore-paths")
                .value_name("IGNORE_PATH")
                .help("Paths (files or directories) to ignore.")
                .action(ArgAction::Append)
                .num_args(1..),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Output file name.")
                .default_value("collected_code.txt")
                .num_args(1),
        )
        .get_matches();

    let paths: Vec<&str> = matches
        .get_many::<String>("paths")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    let formats: Vec<&str> = matches
        .get_many::<String>("formats")
        .map(|vals| vals.map(|s| s.as_str()).collect())
        .unwrap_or_else(Vec::new);

    let ignore_paths: Vec<String> = matches
        .get_many::<String>("ignore_paths")
        .map(|vals| vals.map(|s| s.to_owned()).collect())
        .unwrap_or_else(Vec::new);

    let ignore_paths: Vec<String> = paths.iter()
        .flat_map(|path| {
            ignore_paths.iter().map(move |ignore| {
                canonicalize(Path::new(path).join(ignore)).unwrap_or(PathBuf::new()).to_string_lossy().into_owned()
            })
        }).filter(|path| !path.is_empty())
        .collect();

    let output_file = matches
        .get_one::<String>("output")
        .expect("Default value is set");

    collect_code(&paths, &formats, &ignore_paths, output_file)
}

fn collect_code(
    paths: &[&str],
    formats: &[&str],
    ignore_paths: &Vec<String>,
    output_file: &str,
) -> Result<()> {
    let file = File::create(output_file).with_context(|| format!("Creating {}", output_file))?;
    let writer = BufWriter::new(file);
    let writer = Arc::new(Mutex::new(Box::new(writer) as Box<dyn Write + Send>));

    // Write Project Structure
    {
        let mut writer = writer.lock().unwrap();
        writeln!(writer, "Project Structure:")?;
        for path in paths.iter().map(|&path| Path::new(path).canonicalize().unwrap()) {
            if path.is_dir() {
                generate_tree(path.as_path(), "", ignore_paths, formats, &mut *writer)?;
            } else if path.is_file() {
                writeln!(writer, "{}", path.display())?;
            }
        }
        writeln!(writer, "\nCode Files:\n================")?;
    }

    // Write Code Files
    for path in paths {
        let path = Path::new(path);
        if path.is_file() {
            if should_include(path, formats, ignore_paths) {
                let mut writer = writer.lock().unwrap();
                process_file(path, &mut *writer)?;
            }
        } else if path.is_dir() {
            if should_ignore(path, ignore_paths) {
                continue;
            }
            process_directory(path, formats, ignore_paths, Arc::clone(&writer))?;
        }
    }

    println!("Code collection completed. Output written to {}", output_file);
    Ok(())
}

fn generate_tree(
    dir: &Path,
    prefix: &str,
    ignore_paths: &Vec<String>,
    formats: &[&str],
    writer: &mut impl Write,
) -> Result<()> {
    let entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .collect();
    let count = entries.len();
    let last_unignored = count - entries.iter().rev().position(|entry| !should_ignore(&entry.path(), ignore_paths)).unwrap_or(0);
    for (i, entry) in entries.into_iter().enumerate() {
        let path = entry.path();
        if should_ignore(&path, ignore_paths) {
            continue;
        }
        let connector = if i == last_unignored - 1 { "└── " } else { "├── " };
        if path.is_dir() {
            writeln!(
                writer,
                "{}{}{}/",
                prefix,
                connector,
                path.file_name().unwrap().to_string_lossy()
            )?;
            let new_prefix = if i == count - 1 {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            generate_tree(&path, &new_prefix, ignore_paths, formats, writer)?;
        } else {
            if formats.is_empty() || formats.iter().any(|ext| path.extension().map_or(false, |e| e == ext.strip_prefix('.').unwrap_or(ext))) {
                writeln!(
                    writer,
                    "{}{}{}",
                    prefix,
                    connector,
                    path.file_name().unwrap().to_string_lossy()
                )?;
            }
        }
    }
    Ok(())
}

fn should_ignore(path: &Path, ignore_paths: &Vec<String>) -> bool {
    let path_str = path.to_string_lossy();
    ignore_paths.iter().any(|ignore| path_str.starts_with(ignore))
        || path
            .file_name()
            .map_or(false, |name| name.to_string_lossy().starts_with('.'))
}

fn should_include(path: &Path, formats: &[&str], ignore_paths: &Vec<String>) -> bool {
    if should_ignore(path, ignore_paths) {
        return false;
    }
    if formats.is_empty() {
        return true;
    }
    path.extension()
        .and_then(|e| e.to_str())
        .map_or(false, |ext| formats.contains(&format!(".{}", ext).as_str()))
}

fn process_file(path: &Path, writer: &mut impl Write) -> Result<()> {
    writeln!(writer, "\n================\nFilepath: {}", path.display())?;
    let content = fs::read_to_string(path)
        .with_context(|| format!("Reading file {}", path.display()))?;
    writeln!(writer, "{}", content)?;
    Ok(())
}

fn process_directory(
    dir: &Path,
    formats: &[&str],
    ignore_paths: &Vec<String>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
) -> Result<()> {
    let walker = WalkBuilder::new(dir)
        .ignore(false)
        .git_ignore(false)
        .hidden(false)
        .build_parallel();

    walker.run(|| {
        let writer = Arc::clone(&writer);
        Box::new(move |result| {
            match result {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() && should_include(path, formats, ignore_paths) {
                        let mut writer = writer.lock().unwrap();
                        if let Err(e) = process_file(path, &mut *writer) {
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
