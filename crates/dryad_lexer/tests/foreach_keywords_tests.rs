// crates/dryad_lexer/tests/foreach_keywords_tests.rs
use dryad_lexer::{Lexer, token::Token};

#[test]
fn test_in_keyword() {
    let mut lexer = Lexer::new("in");
    let token = lexer.next_token().unwrap();
    assert_eq!(token, Token::Keyword("in".to_string()));
}

#[test]
fn test_for_in_basic_structure() {
    let mut lexer = Lexer::new("for item in lista");
    
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("for".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("item".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("in".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("lista".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn test_for_in_with_braces() {
    let mut lexer = Lexer::new("for x in numbers { print(x); }");
    
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("for".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("in".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("numbers".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol('{'));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("print".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol('('));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol(')'));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol(';'));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol('}'));
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn test_in_not_case_sensitive() {
    let mut lexer = Lexer::new("IN In iN");
    
    // Todas devem ser identificadores, não keywords
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("IN".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("In".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("iN".to_string()));
}

#[test]
fn test_in_as_part_of_identifier() {
    let mut lexer = Lexer::new("inside income input");
    
    // Deve reconhecer como identificadores, não como keywords
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("inside".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("income".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("input".to_string()));
}

#[test]
fn test_for_in_with_array_literal() {
    let mut lexer = Lexer::new("for item in [1, 2, 3]");
    
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("for".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("item".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("in".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol('['));
    assert_eq!(lexer.next_token().unwrap(), Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap(), Token::Number(2.0));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap(), Token::Number(3.0));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol(']'));
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn test_for_in_with_tuple_literal() {
    let mut lexer = Lexer::new("for element in (1, \"test\", true)");
    
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("for".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("element".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("in".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol('('));
    assert_eq!(lexer.next_token().unwrap(), Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap(), Token::String("test".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol(','));
    assert_eq!(lexer.next_token().unwrap(), Token::Boolean(true));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol(')'));
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn test_nested_for_in_keywords() {
    let mut lexer = Lexer::new("for outer in lists { for inner in outer { } }");
    
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("for".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("outer".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("in".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("lists".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol('{'));
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("for".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("inner".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("in".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("outer".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol('{'));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol('}'));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol('}'));
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn test_for_in_with_complex_expressions() {
    let mut lexer = Lexer::new("for i in getItems()");
    
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("for".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("i".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Keyword("in".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Identifier("getItems".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol('('));
    assert_eq!(lexer.next_token().unwrap(), Token::Symbol(')'));
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}
