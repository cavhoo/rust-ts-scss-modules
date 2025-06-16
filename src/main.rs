use std::path::PathBuf;

use clap::Parser;
use generator::generator::Generator;
use loader::loader::get_scss_files;
use logger::logger::{LogLevel, Logger};
use parser::scss::ScssFile;

mod generator;
mod lexer;
mod loader;
mod logger;
mod parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the root directory of your app
    #[arg(short, long)]
    path: String,

    /// Log level for the application
    #[arg(short, long, default_value = "info", value_enum)]
    log_level: String,
}

fn main() {
    let args = Args::parse();
    let srcdir = PathBuf::from(args.path);
    let log_level = match args.log_level.as_str() {
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warning" => LogLevel::Warning,
        "error" => LogLevel::Error,
        _ => {
            eprintln!("Invalid log level specified. Defaulting to 'warning'.");
            LogLevel::Warning
        }
    };

    let logger = Logger::new(log_level);
    let absolute_path = srcdir
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .expect("Could not resolve path");

    let result = get_scss_files(&absolute_path).collect::<Vec<_>>();
    let file_count = result.len();

    let generator = Generator::new();
    logger.info(format!("Found {} .scss files parsing...", file_count).as_str());
    for file in result {
        logger.debug(format!("Parsing: {}", file.file_name().to_str().unwrap()).as_str());
        let scss_file = ScssFile::new(file.path());
        logger.debug(format!("Classes found: {:?}", scss_file.classes()).as_str());
        generator.generate_declaration(&scss_file);
    }

    logger.info(format!("Parsed {} files successfully.", file_count).as_str());
}
