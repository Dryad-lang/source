// crates/dryad_parser/tests/function_parser_tests.rs
use dryad_parser::{Parser, ast::{Stmt, Expr, Literal}};
use dryad_lexer::{Lexer, Token};

fn parse_tokens(input: &str) -> Vec<Stmt> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token().unwrap() {
            Token::Eof => break,
            token => tokens.push(token),
        }
    }
    
    let mut parser = Parser::new(tokens);
    parser.parse().unwrap().statements
}

#[test]
fn test_simple_function_declaration() {
    let statements = parse_tokens("function test() { return 42; }");
    
    assert_eq!(statements.len(), 1);
    if let Stmt::FunctionDeclaration(name, params, body) = &statements[0] {
        assert_eq!(name, "test");
        assert_eq!(params.len(), 0);
        
        if let Stmt::Block(block_stmts) = body.as_ref() {
            assert_eq!(block_stmts.len(), 1);
            if let Stmt::Return(Some(expr)) = &block_stmts[0] {
                if let Expr::Literal(Literal::Number(n)) = expr {
                    assert_eq!(*n, 42.0);
                } else {
                    panic!("Expected number literal");
                }
            } else {
                panic!("Expected return statement");
            }
        } else {
            panic!("Expected block");
        }
    } else {
        panic!("Expected function declaration");
    }
}

#[test]
fn test_function_with_parameters() {
    let statements = parse_tokens("function saudacao(nome) { return \"Olá, \" + nome; }");
    
    assert_eq!(statements.len(), 1);
    if let Stmt::FunctionDeclaration(name, params, _) = &statements[0] {
        assert_eq!(name, "saudacao");
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], "nome");
    } else {
        panic!("Expected function declaration");
    }
}

#[test]
fn test_function_with_multiple_parameters() {
    let statements = parse_tokens("function calcular(x, y, z) { return x + y + z; }");
    
    assert_eq!(statements.len(), 1);
    if let Stmt::FunctionDeclaration(name, params, _) = &statements[0] {
        assert_eq!(name, "calcular");
        assert_eq!(params.len(), 3);
        assert_eq!(params[0], "x");
        assert_eq!(params[1], "y");
        assert_eq!(params[2], "z");
    } else {
        panic!("Expected function declaration");
    }
}

#[test]
fn test_function_without_return() {
    let statements = parse_tokens("function cumprimentar(nome) { let msg = \"Oi, \" + nome; }");
    
    assert_eq!(statements.len(), 1);
    if let Stmt::FunctionDeclaration(name, params, body) = &statements[0] {
        assert_eq!(name, "cumprimentar");
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], "nome");
        
        if let Stmt::Block(block_stmts) = body.as_ref() {
            assert_eq!(block_stmts.len(), 1);
            // Deve ter uma declaração de variável
            assert!(matches!(block_stmts[0], Stmt::VarDeclaration(_, _)));
        }
    } else {
        panic!("Expected function declaration");
    }
}

#[test]
fn test_return_without_value() {
    let statements = parse_tokens("function vazia() { return; }");
    
    assert_eq!(statements.len(), 1);
    if let Stmt::FunctionDeclaration(_, _, body) = &statements[0] {
        if let Stmt::Block(block_stmts) = body.as_ref() {
            assert_eq!(block_stmts.len(), 1);
            if let Stmt::Return(value) = &block_stmts[0] {
                assert!(value.is_none());
            } else {
                panic!("Expected return statement");
            }
        }
    } else {
        panic!("Expected function declaration");
    }
}

#[test]
fn test_function_call_parsing() {
    let statements = parse_tokens("saudacao(\"Maria\");");
    
    assert_eq!(statements.len(), 1);
    if let Stmt::Expression(Expr::Call(name, args)) = &statements[0] {
        assert_eq!(name, "saudacao");
        assert_eq!(args.len(), 1);
        if let Expr::Literal(Literal::String(s)) = &args[0] {
            assert_eq!(s, "Maria");
        } else {
            panic!("Expected string argument");
        }
    } else {
        panic!("Expected function call expression");
    }
}

#[test]
fn test_function_call_with_multiple_arguments() {
    let statements = parse_tokens("calcular(1, 2, 3);");
    
    assert_eq!(statements.len(), 1);
    if let Stmt::Expression(Expr::Call(name, args)) = &statements[0] {
        assert_eq!(name, "calcular");
        assert_eq!(args.len(), 3);
        
        for (i, arg) in args.iter().enumerate() {
            if let Expr::Literal(Literal::Number(n)) = arg {
                assert_eq!(*n, (i + 1) as f64);
            } else {
                panic!("Expected number argument");
            }
        }
    } else {
        panic!("Expected function call expression");
    }
}

#[test]
fn test_nested_function_calls() {
    let statements = parse_tokens("print(saudacao(\"João\"));");
    
    assert_eq!(statements.len(), 1);
    if let Stmt::Expression(Expr::Call(outer_name, outer_args)) = &statements[0] {
        assert_eq!(outer_name, "print");
        assert_eq!(outer_args.len(), 1);
        
        if let Expr::Call(inner_name, inner_args) = &outer_args[0] {
            assert_eq!(inner_name, "saudacao");
            assert_eq!(inner_args.len(), 1);
            
            if let Expr::Literal(Literal::String(s)) = &inner_args[0] {
                assert_eq!(s, "João");
            } else {
                panic!("Expected string argument");
            }
        } else {
            panic!("Expected nested function call");
        }
    } else {
        panic!("Expected function call expression");
    }
}

#[test]
fn test_function_call_with_expressions() {
    let statements = parse_tokens("calcular(x + 1, y * 2);");
    
    assert_eq!(statements.len(), 1);
    if let Stmt::Expression(Expr::Call(name, args)) = &statements[0] {
        assert_eq!(name, "calcular");
        assert_eq!(args.len(), 2);
        
        // Primeiro argumento: x + 1
        if let Expr::Binary(_, op, _) = &args[0] {
            assert_eq!(op, "+");
        } else {
            panic!("Expected binary expression");
        }
        
        // Segundo argumento: y * 2
        if let Expr::Binary(_, op, _) = &args[1] {
            assert_eq!(op, "*");
        } else {
            panic!("Expected binary expression");
        }
    } else {
        panic!("Expected function call expression");
    }
}
