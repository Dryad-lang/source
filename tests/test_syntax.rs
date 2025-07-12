#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_number() {
        let mut lexer = Lexer::new("42");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(42.0));
    }
}
