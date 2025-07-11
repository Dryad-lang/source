// tests/critical.rs
// Testes críticos para componentes essenciais do Dryad

use dryad::lexer::Lexer;
use dryad::parser::parser::Parser;
use dryad::parser::ast::{Stmt, Expr, BinaryOp};
use dryad::interpreter::{env::Env, evaluator::Evaluator};
use dryad::interpreter::env::Value;
use std::collections::HashMap;

#[cfg(test)]
mod lexer_tests {
    use super::*;
    use dryad::lexer::token::Token;

    fn dummyFunc() {
        // Função dummy para evitar warnings de unused code
        // Expr warning
        Expr::Number(0.0);
    }

    #[test]
    fn test_basic_tokenization() {
        let mut lexer = Lexer::new("let x = 42;");
        
        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Equal);
        assert_eq!(lexer.next_token(), Token::Number(42.0));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_string_literals() {
        let mut lexer = Lexer::new(r#"let msg = "Hello, World!";"#);
        
        lexer.next_token(); // let
        lexer.next_token(); // msg
        lexer.next_token(); // =
        assert_eq!(lexer.next_token(), Token::String("Hello, World!".to_string()));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * / == != < > <= >=");
        
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Minus);
        assert_eq!(lexer.next_token(), Token::Star);
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::EqualEqual);
        assert_eq!(lexer.next_token(), Token::BangEqual);
        assert_eq!(lexer.next_token(), Token::Less);
        assert_eq!(lexer.next_token(), Token::Greater);
        assert_eq!(lexer.next_token(), Token::LessEqual);
        assert_eq!(lexer.next_token(), Token::GreaterEqual);
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("if else while for function class using");
        
        assert_eq!(lexer.next_token(), Token::If);
        assert_eq!(lexer.next_token(), Token::Else);
        assert_eq!(lexer.next_token(), Token::While);
        assert_eq!(lexer.next_token(), Token::For);
        assert_eq!(lexer.next_token(), Token::Function);
        assert_eq!(lexer.next_token(), Token::Class);
        assert_eq!(lexer.next_token(), Token::Using);
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use dryad::parser::ast::{Stmt, Expr, BinaryOp};

    #[test]
    fn test_variable_declaration() {
        let lexer = Lexer::new("let x = 42;");
        let mut parser = Parser::new(lexer);
        
        if let Some(Stmt::Let { name, value }) = parser.parse_statement() {
            assert_eq!(name, "x");
            assert_eq!(value, Expr::Number(42.0));
        } else {
            panic!("Expected Let statement");
        }
    }

    #[test]
    fn test_binary_expression() {
        let lexer = Lexer::new("2 + 3 * 4;");
        let mut parser = Parser::new(lexer);
        
        if let Some(Stmt::Expr(expr)) = parser.parse_statement() {
            // Deve ser 2 + (3 * 4) devido à precedência
            match expr {
                Expr::Binary { left, op, right } => {
                    assert_eq!(*left, Expr::Number(2.0));
                    assert_eq!(op, BinaryOp::Add);
                    match *right {
                        Expr::Binary { left, op, right } => {
                            assert_eq!(*left, Expr::Number(3.0));
                            assert_eq!(op, BinaryOp::Mul);
                            assert_eq!(*right, Expr::Number(4.0));
                        }
                        _ => panic!("Expected binary expression on right side"),
                    }
                }
                _ => panic!("Expected binary expression"),
            }
        } else {
            panic!("Expected expression statement");
        }
    }

    #[test]
    fn test_if_statement() {
        let lexer = Lexer::new("if (x > 0) { return true; }");
        let mut parser = Parser::new(lexer);
        
        if let Some(Stmt::If { cond, then_branch, else_branch }) = parser.parse_statement() {
            // Verifica a condição
            match cond {
                Expr::Binary { left, op, right } => {
                    assert_eq!(*left, Expr::Identifier("x".to_string()));
                    assert_eq!(op, BinaryOp::Greater);
                    assert_eq!(*right, Expr::Number(0.0));
                }
                _ => panic!("Expected binary condition"),
            }
            
            // Verifica que tem then_branch
            match *then_branch {
                Stmt::Block(_) => {}, // OK
                _ => panic!("Expected block statement"),
            }
            
            // Verifica que não tem else
            assert!(else_branch.is_none());
        } else {
            panic!("Expected If statement");
        }
    }

    #[test]
    fn test_function_call() {
        let lexer = Lexer::new("Console.println(\"Hello\");");
        let mut parser = Parser::new(lexer);
        
        if let Some(Stmt::Expr(expr)) = parser.parse_statement() {
            match expr {
                Expr::Call { function, args } => {
                    assert_eq!(function, "Console.println");
                    assert_eq!(args.len(), 1);
                    assert_eq!(args[0], Expr::String("Hello".to_string()));
                }
                _ => panic!("Expected function call"),
            }
        } else {
            panic!("Expected expression statement");
        }
    }
}

#[cfg(test)]
mod evaluator_tests {
    use super::*;

    #[test]
    fn test_arithmetic_evaluation() {
        let mut env = Env::new();
        let mut evaluator = Evaluator::new();
        
        // Testa expressão simples: 2 + 3 * 4 = 14
        let lexer = Lexer::new("2 + 3 * 4");
        let mut parser = Parser::new(lexer);
        
        if let Some(Stmt::Expr(expr)) = parser.parse_statement() {
            let result = evaluator.eval_expr(&expr, &mut env);
            if let Some(Value::Number(n)) = result.value {
                assert_eq!(n, 14.0);
            } else {
                panic!("Expected numeric result: {:?}", result);
            }
        }
    }

    #[test]
    fn test_variable_assignment_and_retrieval() {
        let mut env = Env::new();
        let mut evaluator = Evaluator::new();
        
        // Atribui variável: let x = 42;
        let lexer = Lexer::new("let x = 42;");
        let mut parser = Parser::new(lexer);
        
        if let Some(stmt) = parser.parse_statement() {
            evaluator.eval_stmt(&stmt, &mut env);
        }
        
        // Recupera variável: x
        assert_eq!(env.get("x"), Some(Value::Number(42.0)));
    }

    #[test]
    fn test_string_operations() {
        let mut env = Env::new();
        let mut evaluator = Evaluator::new();
        
        // Testa concatenação: "Hello" + " " + "World"
        env.set("greeting".to_string(), Value::String("Hello".to_string()));
        env.set("space".to_string(), Value::String(" ".to_string()));
        env.set("world".to_string(), Value::String("World".to_string()));
        
        let lexer = Lexer::new("greeting + space + world");
        let mut parser = Parser::new(lexer);
        
        if let Some(Stmt::Expr(expr)) = parser.parse_statement() {
            let result = evaluator.eval_expr(&expr, &mut env);
            if let Some(Value::String(s)) = result.value {
                assert_eq!(s, "Hello World");
            } else {
                panic!("Expected string result");
            }
        }
    }

    #[test]
    fn test_boolean_logic() {
        let mut env = Env::new();
        let mut evaluator = Evaluator::new();
        
        // Testa expressão lógica: true && false
        let lexer = Lexer::new("true && false");
        let mut parser = Parser::new(lexer);
        
        if let Some(Stmt::Expr(expr)) = parser.parse_statement() {
            let result = evaluator.eval_expr(&expr, &mut env);
            if let Some(Value::Bool(b)) = result.value {
                assert_eq!(b, false);
            } else {
                panic!("Expected boolean result");
            }
        }
    }

    #[test]
    fn test_comparison_operations() {
        let mut env = Env::new();
        let mut evaluator = Evaluator::new();
        
        // Testa comparação: 5 > 3
        let lexer = Lexer::new("5 > 3");
        let mut parser = Parser::new(lexer);
        
        if let Some(Stmt::Expr(expr)) = parser.parse_statement() {
            let result = evaluator.eval_expr(&expr, &mut env);
            if let Some(Value::Bool(b)) = result.value {
                assert_eq!(b, true);
            } else {
                panic!("Expected boolean result");
            }
        }
    }
}

#[cfg(test)]
mod environment_tests {
    use super::*;

    #[test]
    fn test_environment_operations() {
        let mut env = Env::new();
        
        // Testa set e get
        env.set("x".to_string(), Value::Number(42.0));
        assert_eq!(env.get("x"), Some(Value::Number(42.0)));
        
        // Testa variável inexistente
        assert_eq!(env.get("nonexistent"), None);
        
        // Testa sobrescrita
        env.set("x".to_string(), Value::String("hello".to_string()));
        assert_eq!(env.get("x"), Some(Value::String("hello".to_string())));
    }

    #[test]
    fn test_value_types() {
        let mut env = Env::new();
        
        // Testa diferentes tipos de valores
        env.set("num".to_string(), Value::Number(3.14));
        env.set("text".to_string(), Value::String("hello".to_string()));
        env.set("flag".to_string(), Value::Bool(true));
        env.set("empty".to_string(), Value::Null);
        
        // Verifica tipos
        assert_eq!(env.get("num"), Some(Value::Number(3.14)));
        assert_eq!(env.get("text"), Some(Value::String("hello".to_string())));
        assert_eq!(env.get("flag"), Some(Value::Bool(true)));
        assert_eq!(env.get("empty"), Some(Value::Null));
    }

    #[test]
    fn test_array_values() {
        let mut env = Env::new();
        
        let array = Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
        
        env.set("numbers".to_string(), array.clone());
        
        // Verifica se o array foi armazenado (teste de tipo mais direto)
        if let Some(Value::Array(retrieved)) = env.get("numbers") {
            assert_eq!(retrieved.len(), 3);
            if let Value::Number(n) = &retrieved[0] {
                assert_eq!(*n, 1.0);
            }
            if let Value::Number(n) = &retrieved[1] {
                assert_eq!(*n, 2.0);
            }
            if let Value::Number(n) = &retrieved[2] {
                assert_eq!(*n, 3.0);
            }
        } else {
            panic!("Expected array value");
        }
    }

    #[test]
    fn test_object_values() {
        let mut env = Env::new();
        
        let mut object = HashMap::new();
        object.insert("name".to_string(), Value::String("John".to_string()));
        object.insert("age".to_string(), Value::Number(30.0));
        
        let obj_value = Value::Object(object.clone());
        env.set("person".to_string(), obj_value.clone());
        
        // Verifica se o objeto foi armazenado (teste de tipo mais direto)
        if let Some(Value::Object(retrieved)) = env.get("person") {
            assert_eq!(retrieved.len(), 2);
            
            if let Some(Value::String(name)) = retrieved.get("name") {
                assert_eq!(name, "John");
            } else {
                panic!("Expected name field");
            }
            
            if let Some(Value::Number(age)) = retrieved.get("age") {
                assert_eq!(*age, 30.0);
            } else {
                panic!("Expected age field");
            }
        } else {
            panic!("Expected object value");
        }
    }
}
