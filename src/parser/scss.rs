use std::{
    collections::HashSet, fmt::{self, Display, Formatter}, fs::File, io::{self, BufRead}, path::Path
};

use regex::Regex;


#[derive(Debug)]
pub struct ScssFile {
    pub class_names: HashSet<String>,
    pub file_path: String,
}

impl ScssFile {
    pub fn new(path: &Path) -> Self {
        let lines = ScssFile::read_lines(path).unwrap();


        Self {
            class_names: ScssFile::extract_class_names(lines),
            file_path: path.display().to_string(),
        }
    }


    fn extract_class_names(lines: io::Lines<io::BufReader<File>>) -> HashSet<String> {

        let mut class_names = HashSet::new();
        for line in lines.map_while(Result::ok) {
            if line.starts_with('@') {
                continue;
            }

            if line.chars().count() == 0 {
                continue;
            }

            let class = ScssFile::parse_line_for_class(&line);

            if class.chars().count() == 0 {
                continue;
            }

            class_names.insert(class);
        }
        class_names
    }

    fn parse_line_for_class(line: &str) -> String {
        let regex_class = Regex::new(r"(?:[\.\#])([A-Za-z]+)(?:\s)").unwrap();

        let Some(result) = regex_class.captures(line) else { return String::from("") };

        String::from(&result[1])

    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}


impl Display for ScssFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.class_names)
    }
}
