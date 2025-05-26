use generator::generator::Generator;
use loader::loader::get_scss_files;
use parser::scss::ScssFile;

mod generator;
mod loader;
mod parser;

fn main() {
    let generator = Generator::new();
    let result = get_scss_files();

    for file in result {
        let scss_file = ScssFile::new(file.path());
        generator.generate_declaration(&scss_file);
    }
}
