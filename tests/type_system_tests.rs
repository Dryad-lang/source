// tests/type_system_tests.rs

use dryad::interpreter::types::{Type, TypeChecker, TypeError};
use dryad::interpreter::env::{Env, Value};
use dryad::lexer::tokenizer::Lexer;
use dryad::parser::parser::Parser;
use dryad::parser::ast::{Expr, Stmt, BinaryOp};
use dryad::interpreter::evaluator::eval_stmt;

#[test]
fn test_type_inference_numbers() {
    let code = "let x = 42;";
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let mut env = Env::new();
    let type_checker = TypeChecker::new();

    let stmt = parser.parse_statement().unwrap();
    eval_stmt(&stmt, &mut env);
    
    let var_type = type_checker.infer_type(&env.get("x").unwrap());
    assert_eq!(var_type, Type::Number);
}

#[test]
fn test_type_inference_strings() {
    let code = r#"let name = "Dryad";"#;
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let mut env = Env::new();
    let type_checker = TypeChecker::new();

    let stmt = parser.parse_statement().unwrap();
    eval_stmt(&stmt, &mut env);
    
    let var_type = type_checker.infer_type(&env.get("name").unwrap());
    assert_eq!(var_type, Type::String);
}

#[test]
fn test_type_checking_binary_operations() {
    let type_checker = TypeChecker::new_strict(); // Use strict mode for testing
    
    // Operações válidas
    assert!(type_checker.check_binary_op(&Type::Number, &Type::Number, "+").is_ok());
    assert!(type_checker.check_binary_op(&Type::String, &Type::String, "+").is_ok());
    
    // Operações inválidas em modo estrito
    assert!(type_checker.check_binary_op(&Type::Number, &Type::String, "+").is_err());
}

#[test]
fn test_type_coercion() {
    let type_checker = TypeChecker::new();
    
    // Coerção de número para string
    let result = type_checker.coerce(&Value::Number(42.0), &Type::String);
    assert_eq!(result, Some(Value::String("42".to_string())));
    
    // Coerção de string para número
    let result = type_checker.coerce(&Value::String("3.14".to_string()), &Type::Number);
    assert_eq!(result, Some(Value::Number(3.14)));
}

#[test]
fn test_type_error_detection() {
    // First test: simple case that should work
    let mut env = Env::new();
    let type_checker = TypeChecker::new_strict();
    
    // Add variables to environment manually
    eval_stmt(&Stmt::Let { name: "x".to_string(), value: Expr::Number(10.0) }, &mut env);
    eval_stmt(&Stmt::Let { name: "y".to_string(), value: Expr::String("hello".to_string()) }, &mut env);
    
    // Now create an expression that should trigger a type error: x + y
    let binary_expr = Expr::Binary {
        left: Box::new(Expr::Identifier("x".to_string())),
        op: BinaryOp::Add,
        right: Box::new(Expr::Identifier("y".to_string())),
    };
    let stmt = Stmt::Expr(binary_expr);
    
    if let Some(error) = type_checker.check_statement(&stmt, &env) {
        println!("Found expected error: {:?}", error);
        assert!(matches!(error, TypeError::TypeMismatch(_, _)));
        return;
    }
    
    panic!("Expected type error but none was found");
}
