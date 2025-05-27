use std::path::PathBuf;

use clap::Parser;
use generator::generator::Generator;
use loader::loader::get_scss_files;
use parser::scss::ScssFile;

mod generator;
mod loader;
mod parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let srcdir = PathBuf::from(args.path);
    let absolute_path = srcdir.canonicalize().unwrap().into_os_string().into_string().expect("Could not resolve path");

    let generator = Generator::new();
    let result = get_scss_files(&absolute_path).collect::<Vec<_>>();
    let file_count = result.len();

    println!("Found {} .scss files parsing...", file_count);
    for file in result {
        println!("Parsing: {}", file.file_name().to_str().unwrap());
        let scss_file = ScssFile::new(file.path());
        generator.generate_declaration(&scss_file);
    }
}
