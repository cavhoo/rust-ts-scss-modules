use env_logger::Env;
use log::{debug, info};
use std::{path::PathBuf, thread, time::Duration};

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

    #[arg(short, long, default_value_t = 4)]
    /// The number of parallel threads to use for processing
    threads: usize,
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

    info!("Found {} .scss files parsing...", file_count);

    let chunk_size = file_count.div_ceil(args.threads);

    let chunks: Vec<Vec<walkdir::DirEntry>> = result
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut handles = Vec::new();

    for (thread_id, chunk) in chunks.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            let generator = Generator::new();
            for file in &chunk {
                if Args::parse().log_level == "debug" {
                    debug!(
                        "Thread {} processing file: {}",
                        thread_id,
                        file.path().display()
                    );
                } else {
                    info!("Parsing: {}", file.file_name().to_str().unwrap());
                }
                let scss_file = ScssFile::new(file.path());
                debug!("Classes found: {:?}", scss_file.classes());
                if let Err(e) = generator.generate_declaration(&scss_file) {
                    eprintln!(
                        "Error generating declaration for {}: {}",
                        file.file_name().to_str().unwrap(),
                        e
                    );
                }
                thread::sleep(Duration::from_millis(1));
            }
            debug!(
                "Thread {} completed processing {} files.",
                thread_id,
                chunk.len()
            );
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        let Err(_) = handle.join() else { continue };
        eprintln!("A worker thread panicked");
    }

    info!("Parsed {} files successfully.", file_count);
}
