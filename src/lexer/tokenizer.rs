// src/lexer/tokenizer.rs

use crate::lexer::token::Token;
use crate::interpreter::errors::{DryadError, ErrorCode};

pub struct Lexer {
    input: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
    errors: Vec<DryadError>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
            errors: Vec::new(),
        }
    }
    
    pub fn get_errors(&self) -> &[DryadError] {
        &self.errors
    }
    
    fn add_error(&mut self, code: ErrorCode, message: String) {
        let error = DryadError::with_code(code, message, Some((self.line, self.column)));
        self.errors.push(error);
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.is_at_end() {
            return Token::Eof;
        }

        let c = self.advance();

        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '%' => Token::Mod,
            '/' => {
                if self.match_char('/') {
                    // Comentário de linha - pular até o fim da linha
                    self.skip_line_comment();
                    self.next_token()
                } else {
                    Token::Slash
                }
            }
            '=' => {
                if self.match_char('=') {
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                if self.match_char('=') {
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            '<' => {
                if self.match_char('=') {
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if self.match_char('=') {
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '&' => {
                if self.match_char('&') {
                    Token::And
                } else {
                    self.next_token() // ignora & sozinho
                }
            }
            '|' => {
                if self.match_char('|') {
                    Token::Or
                } else {
                    self.next_token() // ignora | sozinho
                }
            }
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '.' => Token::Dot,
            '"' => self.read_string('"'),
            '\'' => self.read_string('\''),
            ch if ch.is_ascii_digit() => self.read_number(ch),
            ch if Self::is_identifier_start(ch) => self.read_identifier(ch),
            ch => {
                self.add_error(
                    ErrorCode::E1001,
                    format!("Unexpected character '{}' (ASCII: {})", ch, ch as u32)
                );
                self.next_token() // Continue parsing
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.input[self.current];
        self.current += 1;
        
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        
        ch
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.current).copied()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.get(self.current) {
            if ch.is_whitespace() {
                self.current += 1;
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while let Some(&ch) = self.input.get(self.current) {
            if ch == '\n' {
                break;
            }
            self.current += 1;
        }
    }

    fn read_number(&mut self, first: char) -> Token {
        let mut number = first.to_string();
        let mut has_dot = false;

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                number.push(self.advance());
            } else if ch == '.' && !has_dot {
                has_dot = true;
                number.push(self.advance());
                
                // Verificar se há dígito após o ponto
                if let Some(next_ch) = self.peek() {
                    if !next_ch.is_ascii_digit() {
                        self.add_error(
                            ErrorCode::E1004,
                            "Invalid number format: decimal point must be followed by digits".to_string()
                        );
                        break;
                    }
                } else {
                    self.add_error(
                        ErrorCode::E1004,
                        "Invalid number format: decimal point at end of input".to_string()
                    );
                    break;
                }
            } else if ch == '.' && has_dot {
                self.add_error(
                    ErrorCode::E1004,
                    "Invalid number format: multiple decimal points".to_string()
                );
                break;
            } else {
                break;
            }
        }

        match number.parse::<f64>() {
            Ok(value) => Token::Number(value),
            Err(_) => {
                self.add_error(
                    ErrorCode::E1004,
                    format!("Invalid number format: '{}'", number)
                );
                Token::Number(0.0)
            }
        }
    }

    fn read_string(&mut self, delimiter: char) -> Token {
        let mut value = String::new();
        let start_line = self.line;
        let start_column = self.column;

        while let Some(ch) = self.peek() {
            if ch == delimiter {
                self.advance();
                return Token::String(value);
            }
            
            if ch == '\n' && delimiter != '`' {
                // String multi-linha só permitida com template literals
                self.add_error(
                    ErrorCode::E1002,
                    format!("Unterminated string literal starting at {}:{}", start_line, start_column)
                );
                return Token::String(value);
            }
            
            if ch == '\\' {
                self.advance(); // Consome a barra
                if let Some(escaped) = self.peek() {
                    match escaped {
                        'n' => { self.advance(); value.push('\n'); }
                        't' => { self.advance(); value.push('\t'); }
                        'r' => { self.advance(); value.push('\r'); }
                        '\\' => { self.advance(); value.push('\\'); }
                        '"' => { self.advance(); value.push('"'); }
                        '\'' => { self.advance(); value.push('\''); }
                        _ => {
                            self.add_error(
                                ErrorCode::E1005,
                                format!("Invalid escape sequence '\\{}'", escaped)
                            );
                            self.advance();
                            value.push(escaped);
                        }
                    }
                } else {
                    self.add_error(
                        ErrorCode::E1005,
                        "Incomplete escape sequence at end of input".to_string()
                    );
                }
            } else {
                value.push(self.advance());
            }
        }

        // Se chegou aqui, a string não foi fechada
        self.add_error(
            ErrorCode::E1002,
            format!("Unterminated string literal starting at {}:{}", start_line, start_column)
        );
        
        Token::String(value)
    }

    fn read_identifier(&mut self, first: char) -> Token {
        let mut ident = first.to_string();

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(self.advance());
            } else {
                break;
            }
        }

        match ident.as_str() {
            "let" => Token::Let,
            "fun" | "fn" => Token::Fun,
            "function" => Token::Function,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "return" => Token::Return,
            "use" => Token::Use,
            "using" => Token::Using,
            "export" => Token::Export,
            "class" => Token::Class,
            "new" => Token::New,
            "this" => Token::This,
            "public" => Token::Public,
            "private" => Token::Private,
            "protected" => Token::Protected,
            "static" => Token::Static,
            "namespace" => Token::Namespace,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "throw" => Token::Throw,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            _ => Token::Identifier(ident),
        }
    }

    fn is_identifier_start(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }
}
