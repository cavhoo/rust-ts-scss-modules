use std::{
    collections::HashSet, fmt::{self, Display, Formatter}, fs::{self}, path::Path
};
use crate::lexer::lexer::{Lexer, Token};


#[derive(Debug)]
pub struct ScssFile {
    pub class_names: HashSet<String>,
    pub file_path: String,
}

impl ScssFile {
    pub fn new(path: &Path) -> Self {
        let content = fs::read_to_string(path).unwrap();
        Self {
            class_names: ScssFile::extract_class_names(content),
            file_path: path.display().to_string(),
        }
    }


    fn extract_class_names(lines: String) -> HashSet<String> {
        let mut lexer = Lexer::new(&lines);
        let tokens = lexer.collect::<Vec<Token>>();
        let mut class_names = HashSet::new();
        class_names
    }

}


impl Display for ScssFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.class_names)
    }
}
