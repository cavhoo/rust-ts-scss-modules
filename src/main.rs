use std::{fs, path::PathBuf};

use clap::Parser;
use generator::generator::Generator;
use lexer::lexer::Lexer;
use loader::loader::get_scss_files;
use parser::scss::ScssFile;

mod generator;
mod lexer;
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
    let absolute_path = srcdir
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .expect("Could not resolve path");

    let generator = Generator::new();
    let result = get_scss_files(&absolute_path).collect::<Vec<_>>();
    let file_count = result.len();

    let mut file = &result[0];
    println!("Parsing: {}", file.file_name().to_str().unwrap());
    let mut scssContent = fs::read_to_string(file.path()).unwrap();
    println!("{}", scssContent);
    println!("Found {} .scss files parsing...", file_count);
    let mut scss_file = ScssFile::new(file.path());
    println!("{:?}", scss_file.class_names);
    generator.generate_declaration(&scss_file);
    // for file in result {
    //     println!("Parsing: {}", file.file_name().to_str().unwrap());
    //     let scss_file = ScssFile::new(file.path());
    //     generator.generate_declaration(&scss_file);
    // }
}
