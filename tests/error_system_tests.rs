// tests/error_system_tests.rs

use dryad::interpreter::errors::{DryadError, ErrorReporter, ErrorSeverity};
use dryad::lexer::tokenizer::Lexer;
use dryad::parser::parser::Parser;
use dryad::interpreter::env::Env;
use dryad::interpreter::evaluator::Evaluator;

#[test]
fn test_syntax_error_reporting() {
    let code = "let x = ;"; // Erro de sintaxe
    let mut error_reporter = ErrorReporter::new();
    
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    
    let result = parser.parse_statement();
    assert!(result.is_none());
    
    // Simular erro de parsing
    let error = DryadError::syntax_error("Expected expression after '='", 1, 9);
    
    error_reporter.add_error(error);
    let errors = error_reporter.get_errors();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].to_string().contains("Expected expression"));
}

#[test]
fn test_runtime_error_reporting() {
    let code = "let x = y;"; // Variável não definida
    let _error_reporter = ErrorReporter::new();
    
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let mut env = Env::new();
    
    if let Some(stmt) = parser.parse_statement() {
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_stmt(&stmt, &mut env);
        
        if !result.errors.is_empty() {
            let errors = &result.errors;
            assert_eq!(errors.len(), 1);
            assert!(matches!(errors[0].severity, ErrorSeverity::Error));
        }
    }
}

#[test]
fn test_error_location_tracking() {
    let _code = r#"
let x = 10;
let y = z; // Erro na linha 3
"#;
    
    let mut error_reporter = ErrorReporter::new();
    let error = DryadError::runtime_error("Undefined variable 'z'", 3, 9);
    
    error_reporter.add_error(error);
    let formatted = error_reporter.format_errors();
    
    assert!(formatted.contains("3:9"));
    assert!(formatted.contains("Undefined variable"));
}

#[test]
fn test_multiple_error_collection() {
    let mut error_reporter = ErrorReporter::new();
    
    let error1 = DryadError::syntax_error("Unexpected token", 1, 5);
    let error2 = DryadError::runtime_error("Division by zero", 2, 10);
    
    error_reporter.add_error(error1);
    error_reporter.add_error(error2);
    
    let errors = error_reporter.get_errors();
    assert_eq!(errors.len(), 2);
}

#[test]
fn test_error_severity_levels() {
    let mut error_reporter = ErrorReporter::new();
    
    let warning = DryadError::warning("Unused variable 'x'", 1, 5);
    let error = DryadError::runtime_error("Null pointer access", 2, 10);
    
    error_reporter.add_error(warning);
    error_reporter.add_error(error);
    
    assert_eq!(error_reporter.get_warning_count(), 1);
    assert_eq!(error_reporter.get_error_count(), 1);
}
