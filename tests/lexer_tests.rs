// tests/lexer_tests.rs

use dryad::lexer::{Lexer, token::Token};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_tokens() {
        let mut lexer = Lexer::new("+-*/= == ! != < <= > >= ;");

        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Minus);
        assert_eq!(lexer.next_token(), Token::Star);
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::Equal);
        assert_eq!(lexer.next_token(), Token::EqualEqual);
        assert_eq!(lexer.next_token(), Token::Bang);
        assert_eq!(lexer.next_token(), Token::BangEqual);
        assert_eq!(lexer.next_token(), Token::Less);
        assert_eq!(lexer.next_token(), Token::LessEqual);
        assert_eq!(lexer.next_token(), Token::Greater);
        assert_eq!(lexer.next_token(), Token::GreaterEqual);
        assert_eq!(lexer.next_token(), Token::Semicolon);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let mut lexer = Lexer::new("let fun if else use export return test123");

        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Fun);
        assert_eq!(lexer.next_token(), Token::If);
        assert_eq!(lexer.next_token(), Token::Else);
        assert_eq!(lexer.next_token(), Token::Use);
        assert_eq!(lexer.next_token(), Token::Export);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::Identifier("test123".to_string()));
    }

    #[test]
    fn test_numbers_and_strings() {
        let mut lexer = Lexer::new("42 3.14 \"hello\"");

        assert_eq!(lexer.next_token(), Token::Number(42.0));
        assert_eq!(lexer.next_token(), Token::Number(3.14));
        assert_eq!(lexer.next_token(), Token::String("hello".to_string()));
    }
}
