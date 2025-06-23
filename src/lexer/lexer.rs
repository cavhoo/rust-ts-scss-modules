use std::{fmt::Display, iter::Peekable, str::Chars};

use log::debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    LBrace,
    RBrace,
    LParen,
    RParen,
    Colon,
    Semicolon,
    NewLine,
    Plus,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::LBrace => write!(f, "{{"),
            Operator::RBrace => write!(f, "}}"),
            Operator::LParen => write!(f, "("),
            Operator::RParen => write!(f, ")"),
            Operator::Colon => write!(f, ":"),
            Operator::Semicolon => write!(f, ";"),
            Operator::NewLine => write!(f, "\\n"),
            Operator::Plus => write!(f, "+"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Element,          // For HTML elements like div, span
    Import,           // For @import or @use directive
    Include,          // For @include directive
    Class(bool),      // true for nested classes, false for regular classes
    Mixin,            // For @mixin directive
    Variable,         // For variables like $primary
    CssVariable,      // For CSS variables like --primary-color
    Media,            // For @media directive
    Property(String), // For properties like color, font-size
    Comment,          // For comments
    Op(Operator),     // Operators like +, {, }, (, ), :, ;
    Indent(usize),    // Indentation level
    EOF,              // End of file
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Element => write!(f, "<element>"),
            TokenKind::Import => write!(f, "<import>"),
            TokenKind::Include => write!(f, "<include>"),
            TokenKind::Mixin => write!(f, "<mixin>"),
            TokenKind::Media => write!(f, "<media>"),
            TokenKind::Variable => write!(f, "<variable>"),
            TokenKind::CssVariable => write!(f, "<css-variable>"),
            TokenKind::Comment => write!(f, "<comment>"),
            TokenKind::Property(prop) => write!(f, "<property: {prop}>"),
            TokenKind::Class(nested) => write!(f, "<class:{nested}>"),
            TokenKind::Op(operator) => write!(f, "<operator: {operator}>"),
            TokenKind::Indent(indent) => write!(f, "<indent: {indent}>"),
            TokenKind::EOF => write!(f, "<EOF>"),
        }
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_char: Option<char>,
    position: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token {
                kind: TokenKind::EOF,
                value: _,
            } => None,
            token => Some(token),
        }
    }
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

    pub fn next_token(&mut self) -> Token {
        let c = match self.current_char {
            None => {
                return Token {
                    kind: TokenKind::EOF,
                    value: String::new(),
                }
            }
            Some(c) => c,
        };

        let next = match self.peek() {
            None => {
                return Token {
                    kind: TokenKind::EOF,
                    value: String::new(),
                }
            }
            Some(&next) => next,
        };

        self.resolve_char_as_token(c, next)
    }

    fn resolve_char_as_token(&mut self, c: char, next: char) -> Token {
        match (c, next) {
            ('\n', _) => {
                self.advance();
                Token {
                    kind: TokenKind::Op(Operator::NewLine),
                    value: String::from("\\n"),
                }
            }
            ('\r', _) => {
                self.advance();
                Token {
                    kind: TokenKind::Op(Operator::NewLine),
                    value: String::from("\\r"),
                }
            }
            (' ', _) => self.consume_indentation(),
            ('\t', _) => self.consume_indentation(),
            ('/', '/') => self.consume_single_line_comment(),
            ('/', '*') => self.consume_multi_line_comment(),
            ('+', _) => {
                self.advance();
                Token {
                    kind: TokenKind::Op(Operator::Plus),
                    value: String::from(c),
                }
            }
            ('{', _) => {
                self.advance();
                Token {
                    kind: TokenKind::Op(Operator::LBrace),
                    value: String::from(c),
                }
            }
            ('}', _) => {
                self.advance();
                Token {
                    kind: TokenKind::Op(Operator::RBrace),
                    value: String::from(c),
                }
            }
            ('(', _) => {
                self.advance();
                Token {
                    kind: TokenKind::Op(Operator::LParen),
                    value: String::from(c),
                }
            }
            (')', _) => {
                self.advance();
                Token {
                    kind: TokenKind::Op(Operator::RParen),
                    value: String::from(c),
                }
            }
            (':', _) => {
                self.advance();
                Token {
                    kind: TokenKind::Op(Operator::Colon),
                    value: String::from(c),
                }
            }
            (';', _) => {
                self.advance();
                Token {
                    kind: TokenKind::Op(Operator::Semicolon),
                    value: String::from(c),
                }
            }
            ('-', '-') => self.consume_css_variable(),
            ('$', _) => self.consume_variable(),
            ('.', _) => self.consume_class(),
            ('@', 'i') => self.consume_import_or_include(),
            ('@', 'u') => self.consume_use(),
            ('@', 'm') => self.consume_mixin_or_media(),
            ('&', _) if self.peek() == Some(&' ') || self.peek() == Some(&'.') => {
                self.consume_nested_class()
            }
            ('&', ':') => self.consume_pseudo_class(),
            _ if c.is_whitespace() && c != '\n' => self.consume_indentation(),
            _ if c.is_alphabetic() || c == '_' => self.consume_element_or_property(),
            _ => {
                debug!(
                    "Unexpected character at position {}: '{}'",
                    self.position, c
                );
                Token {
                    kind: TokenKind::EOF,
                    value: String::new(),
                }
            } // Handle unexpected characters
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
        self.position += 1;
    }

    fn consume_indentation(&mut self) -> Token {
        let mut indent_level = 0;
        while let Some(c) = self.current_char {
            if c == ' ' || c == '\t' {
                indent_level += 1;
                self.advance();
            } else {
                break;
            }
        }
        Token {
            kind: TokenKind::Indent(indent_level),
            value: String::new(),
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
        Token {
            kind: TokenKind::Comment,
            value: comment,
        }
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
        Token {
            kind: TokenKind::Comment,
            value: comment,
        }
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
        Token {
            kind: TokenKind::Variable,
            value: variable,
        }
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
        Token {
            kind: TokenKind::Class(false),
            value: class,
        }
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
        Token {
            kind: TokenKind::Class(true),
            value: class,
        }
    }

    fn consume_pseudo_class(&mut self) -> Token {
        let mut pseudo_class = String::new();
        self.advance(); // Skip the '&'
        self.advance(); // Skip the ':'
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                pseudo_class.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token {
            kind: TokenKind::Property(pseudo_class), // Pseudo-classes are treated as properties
            value: String::from(""),
        }
    }

    fn consume_import_or_include(&mut self) -> Token {
        let mut import = String::new();
        self.advance(); // Skip the '@'

        let mut is_import = true;

        if self.current_char == Some('i') && self.peek() == Some(&'n') {
            is_import = false; // It's an include, not an import
        }
        while let Some(c) = self.current_char {
            if c.is_alphanumeric()
                || c == '_'
                || c == '-'
                || c == '/'
                || c == '.'
                || c == ','
                || c == '"'
                || c == '\''
                || c == ' '
                || c == '('
                || c == ')'
            {
                import.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token {
            kind: if is_import {
                TokenKind::Import
            } else {
                TokenKind::Include
            },
            value: import,
        }
    }

    fn consume_use(&mut self) -> Token {
        let mut use_statement = String::new();
        self.advance(); // Skip the '@'

        while let Some(c) = self.current_char {
            if c.is_alphanumeric()
                || c == '_'
                || c == '-'
                || c == '/'
                || c == '.'
                || c == ','
                || c == '"'
                || c == '\''
                || c == ' '
                || c == '('
                || c == ')'
            {
                use_statement.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token {
            kind: TokenKind::Include, // Use is treated as an include in this context
            value: use_statement,
        }
    }

    fn consume_mixin_or_media(&mut self) -> Token {
        let mut mixin = String::new();
        self.advance(); // Skip the '@'
        if self.current_char == Some('m') && self.peek() == Some(&'e') {
            return self.consume_media();
        }

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                mixin.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token {
            kind: TokenKind::Mixin,
            value: mixin,
        }
    }

    fn consume_media(&mut self) -> Token {
        while let Some(c) = self.current_char {
            if c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
        Token {
            kind: TokenKind::Media,
            value: String::from("media"),
        }
    }

    fn consume_element_or_property(&mut self) -> Token {
        let mut element = String::new();
        let mut token = Token {
            kind: TokenKind::EOF,
            value: String::new(),
        };
        while let Some(c) = self.current_char {
            if self.peek() == Some(&':') {
                element.push(c);
                self.advance(); // Skip the character
                if self.peek() == Some(&' ') {
                    self.advance(); // Skip the ' '
                    token = Token {
                        kind: TokenKind::Property(element.clone()),
                        value: self.consume_property_value(),
                    };
                    break;
                }
            }
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' {
                element.push(c);
                self.advance();
            } else {
                token = Token {
                    kind: TokenKind::Element,
                    value: element.clone(),
                };
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
                || c == ','
                || c == '!'
                || c == '+'
                || c == '*'
                || c == '.'
                || c == '\n'
                || c == '\t'
                || c == '/'
                || c == '#'
                || c == '!'
            {
                value.push(c);
                self.advance();
            } else {
                panic!("Unexpected character in property value: {}", c);
            }
        }
        value
    }

    fn consume_css_variable(&mut self) -> Token {
        let mut variable = String::new();
        self.advance(); // Skip the first '-'
        self.advance(); // Skip the second '-'
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                variable.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token {
            kind: TokenKind::CssVariable,
            value: variable,
        }
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
    fn test_class() {
        let input = ".class-name { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Class(false),
                value: "class-name".to_string()
            }
        );
    }

    #[test]
    fn test_skip_whitespace() {
        let input = "  div { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Indent(2),
                value: "".to_string()
            }
        );
    }

    #[test]
    fn test_pseudo_class() {
        let input = "&:hover { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Property("hover".to_string()),
                value: "".to_string()
            }
        );
    }

    #[test]
    fn test_single_line_comment() {
        let input = "// This is a comment\ndiv { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Comment,
                value: " This is a comment".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Op(Operator::NewLine),
                value: "\\n".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Element,
                value: "div ".to_string()
            }
        );
    }

    #[test]
    fn test_multi_line_comment() {
        let input = "/* This is a\nmulti-line comment */\ndiv { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Comment,
                value: " This is a\nmulti-line comment ".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Op(Operator::NewLine),
                value: "\\n".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Element,
                value: "div ".to_string()
            }
        );
    }

    #[test]
    fn test_variable_token() {
        let input = "$primary: #ff0000;";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Variable,
                value: "primary".to_string()
            }
        );
    }

    #[test]
    fn test_class_token() {
        let input = ".class-name { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Class(false),
                value: "class-name".to_string()
            }
        );
    }

    #[test]
    fn test_nested_class_token() {
        let input = "&.child { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Class(true),
                value: "child".to_string()
            }
        );
    }

    #[test]
    fn test_import_token() {
        let input = "@import 'styles.css';";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Import,
                value: "import 'styles.css'".to_string()
            }
        );
    }

    #[test]
    fn test_element_and_property_tokens_with_variables() {
        let input = "div { color: $primary; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Element,
                value: "div ".to_string()
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Op(Operator::LBrace),
                value: "{".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Indent(1),
                value: "".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Property("color".to_string()),
                value: "$primary".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Indent(1),
                value: "".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::EOF,
                value: "".to_string()
            }
        );
    }

    #[test]
    fn test_element_and_property_tokens_with_percent() {
        let input = "div { height: 100%; }";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Element,
                value: "div ".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Op(Operator::LBrace),
                value: "{".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Indent(1),
                value: "".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Property("height".to_string()),
                value: "100%".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::Indent(1),
                value: "".to_string()
            }
        );
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::EOF,
                value: "".to_string()
            }
        );
    }

    #[test]
    fn test_css_variable_token() {
        let input = "--primary-color: var(--color) !important;";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next_token(),
            Token {
                kind: TokenKind::CssVariable,
                value: "primary-color".to_string()
            }
        );
    }
}
