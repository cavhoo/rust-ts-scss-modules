use std::{fmt::Display, iter::Peekable, str::Chars};


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Element(String),          // e.g., "div",
    Property(String, String), // e.g., "color: $primary"
    Class(String),            // e.g., ".class-name"
    NestedClass { name: String, parent: String },      // e.g., "&.child" or "& .child"
    Variable(String),         // e.g., "$primary"
    Import(String),           // @import or @use
    LBrace,                   // {
    RBrace,                   // }
    Colon,                    // :
    Semicolon,                // ;
    Whitespace,               // spaces, tabs, newlines
    Comment(String),          // // or /* */
    EOF,                      // End of file
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Element(name) => write!(f, "{}", name),
            Token::Property(name, value) => write!(f, "{}: {}", name, value),
            Token::Class(name) => write!(f, "{}", name),
            Token::NestedClass { name, parent } => {
                if parent.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}{}", parent, name)
                }
            }
            Token::Variable(name) => write!(f, "{}", name),
            Token::Import(name) => write!(f, "{}", name),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
            Token::Whitespace => write!(f, "Whitespace"),
            Token::Comment(comment) => write!(f, "Comment({})", comment),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_char: Option<char>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.next();
        Lexer {
            input: chars,
            current_char,
            position: 0,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn consume_single_line_comment(&mut self) -> Token {
        let mut comment = String::new();
        self.advance(); // Skip the '/'
        self.advance(); // Skip the second '/'
        while let Some(c) = self.current_char {
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance();
        }
        Token::Comment(comment)
    }

    fn consume_multi_line_comment(&mut self) -> Token {
        let mut comment = String::new();
        self.advance(); // Skip the '/'
        self.advance(); // Skip the '*'
        while let Some(c) = self.current_char {
            if c == '*' && self.peek() == Some(&'/') {
                self.advance(); // Skip the '*'
                self.advance(); // Skip the '/'
                break;
            }
            comment.push(c);
            self.advance();
        }
        Token::Comment(comment)
    }

    fn consume_variable(&mut self) -> Token {
        let mut variable = String::new();
        self.advance(); // Skip the '$'
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                variable.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Variable(variable)
    }

    fn consume_class(&mut self) -> Token {
        let mut class = String::new();
        self.advance(); // Skip the '.'
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                class.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Class(class)
    }

    fn consume_nested_class(&mut self) -> Token {
        let mut class = String::new();
        self.advance(); // Skip the '&'
        if self.peek() == Some(&' ') {
            self.advance();
        }
        self.advance(); // Skip the '.'
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                class.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::NestedClass {
            name: class,
            parent: String::new(), // Parent is empty for nested classes
        }
    }

    fn consume_import(&mut self) -> Token {
        let mut import = String::new();
        self.advance(); // Skip the '@'
        while let Some(c) = self.current_char {
            if c.is_alphanumeric()
                || c == '_'
                || c == '-'
                || c == '/'
                || c == '.'
                || c == '"'
                || c == '\''
                || c == ' '
            {
                import.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Import(import)
    }

    fn consume_element_or_property(&mut self) -> Token {
        let mut element = String::new();
        let mut token = Token::Element(String::new());
        while let Some(c) = self.current_char {
            if self.peek() == Some(&':') {
                element.push(c);
                self.advance(); // Skip the character
                token = Token::Property(element.clone(), self.consume_property_value());
                break;
            }
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '"' || c == ' ' || c == '%' {
                element.push(c);
                self.advance();
            } else {
                token = Token::Element(element);
                break;
            }
        }
        token
    }

    fn consume_property_value(&mut self) -> String {
        let mut value = String::new();
        while let Some(c) = self.current_char {
            if c == ';' {
                self.advance(); // Skip the ';'
                break;
            } else if c == ':' || c == ' ' {
                self.advance();
            } else if c.is_alphanumeric()
                || c == '-'
                || c == '_'
                || c == '"'
                || c == ' '
                || c == '$'
                || c == '%'
                || c == '('
                || c == ')'
            {
                value.push(c);
                self.advance();
            } else {
                panic!("Unexpected character in property value: {}", c);
            }
        }
        value
    }

    pub fn next_token(&mut self) -> Token {
        match self.current_char {
            None => Token::EOF,
            Some(c) => self.resolve_char_as_token(c),
        }
    }

    fn resolve_char_as_token(&mut self, c: char) -> Token {
        match c {
            ' ' | '\t' | '\n' => {
                self.skip_whitespace();
                Token::Whitespace
            }
            '/' if self.peek() == Some(&'/') => self.consume_single_line_comment(),
            '/' if self.peek() == Some(&'*') => self.consume_multi_line_comment(),
            '{' => {
                self.advance();
                Token::LBrace
            }
            '}' => {
                self.advance();
                Token::RBrace
            }
            ':' => {
                self.advance();
                Token::Colon
            }
            ';' => {
                self.advance();
                Token::Semicolon
            }
            '$' => self.consume_variable(),
            '.' => self.consume_class(),
            '@' => self.consume_import(),
            '&' if self.peek() == Some(&' ') || self.peek() == Some(&'.') => {
                self.consume_nested_class()
            }
            _ if c.is_alphabetic() || c == '_' => self.consume_element_or_property(),
            _ => {
                panic!("Unexpected character: {}", c);
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();

            if token == Token::EOF {
                break;
            }

            match token {
                Token::Class(_)=> tokens.push(token), // Skip whitespace tokens
                Token::NestedClass { name, parent: _ } => {
                    let actual_parent = tokens[tokens.len() - 1].to_string(); // Get the parent class name
                    tokens.push(Token::NestedClass { name, parent: actual_parent }); // Add nested class tokens to the list
                },  // Skip comment tokens
                _ => continue,        // Add other tokens to the list
            }

        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_initialization() {
        let input = "div { color: $primary; }";
        let lexer = Lexer::new(input);
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.current_char, Some('d'));
    }

    #[test]
    fn test_tokenization() {
        let input = ".class-name { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Class("class-name".to_string()));
    }

    #[test]
    fn test_skip_whitespace() {
        let input = "  div { color: $primary; }";
        let mut lexer = Lexer::new(input);
        lexer.skip_whitespace();
        assert_eq!(lexer.next_token(), Token::Element("div ".to_string()));
    }

    #[test]
    fn test_single_line_comment() {
        let input = "// This is a comment\ndiv { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token::Comment(" This is a comment".to_string())
        );
        assert_eq!(lexer.next_token(), Token::Whitespace);
        assert_eq!(lexer.next_token(), Token::Element("div ".to_string()));
    }

    #[test]
    fn test_multi_line_comment() {
        let input = "/* This is a\nmulti-line comment */\ndiv { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token::Comment(" This is a\nmulti-line comment ".to_string())
        );
        assert_eq!(lexer.next_token(), Token::Whitespace);
        assert_eq!(lexer.next_token(), Token::Element("div ".to_string()));
    }

    #[test]
    fn test_variable_token() {
        let input = "$primary: #ff0000;";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Variable("primary".to_string()));
    }

    #[test]
    fn test_class_token() {
        let input = ".class-name { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Class("class-name".to_string()));
    }

    #[test]
    fn test_nested_class_token() {
        let input = "&.child { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::NestedClass("child".to_string()));
    }

    #[test]
    fn test_import_token() {
        let input = "@import 'styles.css';";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Import("import 'styles.css'".to_string()));
    }

    #[test]
    fn test_element_and_property_tokens_with_variables() {
        let input = "div { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Element("div ".to_string()));
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::Whitespace);
        assert_eq!(lexer.next_token(), Token::Property("color".to_string(), "$primary".to_string()));
        assert_eq!(lexer.next_token(), Token::Whitespace);
        assert_eq!(lexer.next_token(), Token::RBrace);
    }

    #[test]
    fn test_element_and_property_tokens_with_percent(){
         let input = "div { height: 100%; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Element("div ".to_string()));
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::Whitespace);
        assert_eq!(lexer.next_token(), Token::Property("height".to_string(), "100%".to_string()));
        assert_eq!(lexer.next_token(), Token::Whitespace);
        assert_eq!(lexer.next_token(), Token::RBrace);

    }

    #[test]
    fn test_element_and_property_tokens_with_pixel(){
         let input = "div { height: 100px; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Element("div ".to_string()));
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::Whitespace);
        assert_eq!(lexer.next_token(), Token::Property("height".to_string(), "100px".to_string()));
        assert_eq!(lexer.next_token(), Token::Whitespace);
        assert_eq!(lexer.next_token(), Token::RBrace);

    }

    #[test]
    fn test_element_and_property_tokens_with_mixin(){
         let input = "div { height: some(1); }";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Element("div ".to_string()));
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::Whitespace);
        assert_eq!(lexer.next_token(), Token::Property("height".to_string(), "some(1)".to_string()));
        assert_eq!(lexer.next_token(), Token::Whitespace);
        assert_eq!(lexer.next_token(), Token::RBrace);
    }
}
