// src/lexer/lexer.rs

use crate::lexer::token::Token;

pub struct Lexer {
    input: Vec<char>,
    current: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        eprintln!("DEBUG: Creating lexer with input: '{}'", input);
        eprintln!("DEBUG: Input length: {}", input.len());
        Self {
            input: input.chars().collect(),
            current: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.is_at_end() {
            eprintln!("DEBUG: Lexer reached end of input");
            return Token::Eof;
        }

        let c = self.advance();
        eprintln!("DEBUG: Lexer processing character: '{}'", c);

        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '%' => Token::Mod,
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
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => {
                eprintln!("DEBUG: Found opening bracket!");
                Token::LBracket
            }
            ']' => Token::RBracket,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '.' => Token::Dot,
            '"' => self.read_string(),
            ch if ch.is_ascii_digit() => self.read_number(ch),
            ch if Self::is_identifier_start(ch) => self.read_identifier(ch),
            _ => {
                eprintln!("DEBUG: Unknown character '{}' at position {}", c, self.current - 1);
                Token::Eof // Retorna EOF em vez de recursão infinita
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.input[self.current];
        self.current += 1;
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

    fn read_number(&mut self, first: char) -> Token {
        let mut number = first.to_string();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                number.push(self.advance());
            } else {
                break;
            }
        }

        Token::Number(number.parse().unwrap_or(0.0))
    }

    fn read_string(&mut self) -> Token {
        let mut value = String::new();

        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                break;
            }
            value.push(self.advance());
        }

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
            "fun" => Token::Fun,
            "fn" => Token::Fun, // "fn" is an alias for "fun"
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "return" => Token::Return,
            "use" => Token::Use,
            "export" => Token::Export,
            "class" => Token::Class,
            "new" => Token::New,
            "this" => Token::This,
            "public" => Token::Public,
            "private" => Token::Private,
            "protected" => Token::Protected,
            _ => Token::Identifier(ident),
        }
    }

    fn is_identifier_start(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }
}
