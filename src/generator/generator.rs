use std::{fs::{self}, path::Path};


use crate::parser::scss::ScssFile;


#[derive(Debug)]
pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        Generator {}
    }

    pub fn generate_declaration(&self, scss_file: &ScssFile) {
		if scss_file.class_names.is_empty() {
			return
		}
		let declaration_file_path = Path::new(&scss_file.file_path);
		let mut declaration_file_path_formatted = String::from("");
		declaration_file_path_formatted.push_str(declaration_file_path.parent().unwrap().to_str().unwrap());
		declaration_file_path_formatted.push('/');
		declaration_file_path_formatted.push_str(declaration_file_path.file_name().unwrap().to_str().unwrap());
		declaration_file_path_formatted.push_str(".d.ts");

        let mut content_string = String::from("");

        // First line create Style type
        content_string.push_str("export type Styles = {\n");

        // create entries for every class
        for class in &scss_file.class_names {
            content_string.push_str(&format!("\t{}: string;\n", class));
        }
        content_string.push_str("}\n");

		// Write declarationr
        content_string.push_str("export type ClassNames = keyof Styles;\n\n");
        content_string.push_str("declare const styles: Styles;\n\n");
        content_string.push_str("export default styles;\n\n");

		fs::write(declaration_file_path_formatted, content_string).expect("Could not write stylesheet file");
    }
}
