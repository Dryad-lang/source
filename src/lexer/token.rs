// src/lexer/token.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Palavras-chave
    Let,
    Fun,
    If,
    Else,
    For,
    While,
    Return,
    Export,
    Use,

    // Literais
    Identifier(String),
    Number(f64),
    String(String),

    // Operadores
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Símbolos
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    // Fim
    Eof,
}
