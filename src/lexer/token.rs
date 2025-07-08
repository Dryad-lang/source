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
    Class,
    New,
    This,
    Public,
    Private,
    Protected,
    Namespace,
    Try,
    Catch,
    Throw,

    // Literais
    Identifier(String),
    Number(f64),
    String(String),
    True,
    False,
    Null,

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
    And,        // &&
    Or,         // ||

    // Símbolos
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Dot,

    // Fim
    Eof,
}
