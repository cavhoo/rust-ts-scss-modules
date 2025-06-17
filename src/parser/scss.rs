use std::{
    collections::HashSet, fs::{self}, path::Path
};
use crate::lexer::lexer::{Lexer, Token, TokenKind};


#[derive(Debug)]
pub struct ScssFile {
    pub tokens: Vec<Token>,
    pub file_path: String,
}

impl ScssFile {
    pub fn new(path: &Path) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let lexer = Lexer::new(&content);
        let tokens = lexer.collect::<Vec<Token>>();
        Self {
            tokens,
            file_path: path.display().to_string(),
        }
    }

    pub fn classes(&self) -> HashSet<String> {
        let mut classes = HashSet::new();
        for token in &self.tokens {
            if let TokenKind::Class(false) = token.kind {
                classes.insert(token.value.to_string());
            }

            if let TokenKind::Class(true) = token.kind {
                // If the class is nested, we still consider it a class
                classes.insert(token.value.to_string());
            }
        }
        classes
    }
}
