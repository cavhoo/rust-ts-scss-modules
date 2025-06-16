use std::{fs::File, path::Path};


use handlebars::{to_json, Handlebars};
use serde_json::Map;

use crate::parser::scss::ScssFile;


#[derive(Debug)]
pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        Generator {}
    }

    pub fn generate_declaration(&self, scss_file: &ScssFile) {
		if scss_file.classes().is_empty() {
			return
		}

		let mut handlebars = Handlebars::new();
		handlebars.register_template_file("default", "./templates/default.hbs").unwrap();

		let declaration_file_path = Path::new(&scss_file.file_path);
		let mut declaration_file_path_formatted = String::from("");
		declaration_file_path_formatted.push_str(declaration_file_path.parent().unwrap().to_str().unwrap());
		declaration_file_path_formatted.push('/');
		declaration_file_path_formatted.push_str(declaration_file_path.file_name().unwrap().to_str().unwrap());
		declaration_file_path_formatted.push_str(".d.ts");


		let mut outfile = File::create(declaration_file_path_formatted).expect("Could not create file handle.");

		let mut output_data = Map::new();

		output_data.insert("class".to_string(), to_json(Vec::from_iter(&scss_file.classes())));

		handlebars.render_to_write("default", &to_json(output_data), &mut outfile).expect("Could not write to output file.");

    }
}
