// tests/interpreter_tests.rs

use dryad::lexer::tokenizer::Lexer;
use dryad::parser::parser::Parser;
use dryad::interpreter::env::{Env, Value};
use dryad::interpreter::evaluator::eval_stmt;

#[test]
fn test_eval_literal_expression() {
    let lexer = Lexer::new("42");
    let mut parser = Parser::new(lexer);
    let stmt = parser.parse_statement().unwrap();

    let mut env = Env::new();
    let result = eval_stmt(&stmt, &mut env);
    assert_eq!(result, Some(Value::Number(42.0)));
}

#[test]
fn test_eval_binary_expression() {
    let lexer = Lexer::new("1 + 2 * 3");
    let mut parser = Parser::new(lexer);
    let stmt = parser.parse_statement().unwrap();

    let mut env = Env::new();
    let result = eval_stmt(&stmt, &mut env);
    assert_eq!(result, Some(Value::Number(7.0)));
}

#[test]
fn test_eval_let_and_identifier() {
    let lexer = Lexer::new("let x = 10;");
    let mut parser = Parser::new(lexer);
    let stmt = parser.parse_statement().unwrap();

    let mut env = Env::new();
    eval_stmt(&stmt, &mut env);

    let lexer2 = Lexer::new("x + 5");
    let mut parser2 = Parser::new(lexer2);
    let stmt2 = parser2.parse_statement().unwrap();

    let result = eval_stmt(&stmt2, &mut env);
    assert_eq!(result, Some(Value::Number(15.0)));
}

#[test]
fn test_block_execution() {
    let code = r#"
        let x = 1;
        {
            let y = 2;
            let z = x + y;
            print(z);
        }
    "#;
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let mut env = Env::new();

    while let Some(stmt) = parser.parse_statement() {
        eval_stmt(&stmt, &mut env);
    }

    assert_eq!(env.get("x"), Some(Value::Number(1.0)));
}

#[test]
fn test_function_call_print() {
    let code = r#"print("Hello, Dryad");"#;
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let mut env = Env::new();

    let stmt = parser.parse_statement().unwrap();
    let result = eval_stmt(&stmt, &mut env);

    assert_eq!(result, Some(Value::Null));
}