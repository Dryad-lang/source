// src/lexer/mod.rs
pub mod token;
pub mod tokenizer;

// Re-export para manter a interface
pub use tokenizer::Lexer;
