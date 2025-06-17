use std::path::PathBuf;
use log::{debug, info};
use env_logger::Env;

use clap::Parser;


use generator::generator::Generator;
use loader::loader::get_scss_files;
use parser::scss::ScssFile;

mod generator;
mod lexer;
mod loader;
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

    // Initialize the logger with the specified log level
    let env = Env::default().filter_or("TS_SCSS_LOG_LEVEL", args.log_level);
    env_logger::init_from_env(env);

    let srcdir = PathBuf::from(args.path);
    let absolute_path = srcdir
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .expect("Could not resolve path");

    let result = get_scss_files(&absolute_path).collect::<Vec<_>>();
    let file_count = result.len();

    let generator = Generator::new();
    info!("Found {} .scss files parsing...", file_count);
    for file in result {
        debug!("Parsing: {}", file.file_name().to_str().unwrap());
        let scss_file = ScssFile::new(file.path());
        debug!("Classes found: {:?}", scss_file.classes());
        generator.generate_declaration(&scss_file);
    }

    info!("Parsed {} files successfully.", file_count);
}
