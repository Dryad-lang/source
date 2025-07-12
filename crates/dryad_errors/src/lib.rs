// crates/dryad_errors/src/lib.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DryadError {
    Lexer(u16, String),
    Parser(u16, String),
    Runtime(u16, String),
    Type(u16, String),
    Io(u16, String),
    Module(u16, String),
    Syntax(u16, String),
    Warning(u16, String),
    System(u16, String),
}

impl fmt::Display for DryadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DryadError::Lexer(code, msg) => write!(f, "E{}: Erro Léxico - {}", code, msg),
            DryadError::Parser(code, msg) => write!(f, "E{}: Erro Sintático - {}", code, msg),
            DryadError::Runtime(code, msg) => write!(f, "E{}: Erro de Runtime - {}", code, msg),
            DryadError::Type(code, msg) => write!(f, "E{}: Erro de Tipo - {}", code, msg),
            DryadError::Io(code, msg) => write!(f, "E{}: Erro de I/O - {}", code, msg),
            DryadError::Module(code, msg) => write!(f, "E{}: Erro de Módulo - {}", code, msg),
            DryadError::Syntax(code, msg) => write!(f, "E{}: Erro de Sintaxe - {}", code, msg),
            DryadError::Warning(code, msg) => write!(f, "W{}: Aviso - {}", code, msg),
            DryadError::System(code, msg) => write!(f, "E{}: Erro de Sistema - {}", code, msg),
        }
    }
}

impl std::error::Error for DryadError {}

impl DryadError {
    pub fn new(code: u16, msg: &str) -> Self {
        match code {
            1000..=1999 => DryadError::Lexer(code, msg.into()),
            2000..=2999 => DryadError::Parser(code, msg.into()),
            3000..=3999 => DryadError::Runtime(code, msg.into()),
            4000..=4999 => DryadError::Type(code, msg.into()),
            5000..=5999 => DryadError::Io(code, msg.into()),
            6000..=6999 => DryadError::Module(code, msg.into()),
            7000..=7999 => DryadError::Syntax(code, msg.into()),
            8000..=8999 => DryadError::Warning(code, msg.into()),
            _ => DryadError::System(code, msg.into()),
        }
    }

    pub fn code(&self) -> u16 {
        match self {
            DryadError::Lexer(code, _) |
            DryadError::Parser(code, _) |
            DryadError::Runtime(code, _) |
            DryadError::Type(code, _) |
            DryadError::Io(code, _) |
            DryadError::Module(code, _) |
            DryadError::Syntax(code, _) |
            DryadError::Warning(code, _) |
            DryadError::System(code, _) => *code,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            DryadError::Lexer(_, msg) |
            DryadError::Parser(_, msg) |
            DryadError::Runtime(_, msg) |
            DryadError::Type(_, msg) |
            DryadError::Io(_, msg) |
            DryadError::Module(_, msg) |
            DryadError::Syntax(_, msg) |
            DryadError::Warning(_, msg) |
            DryadError::System(_, msg) => msg,
        }
    }
}