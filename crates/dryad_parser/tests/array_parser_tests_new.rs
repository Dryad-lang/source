// crates/dryad_parser/tests/array_parser_tests.rs
use dryad_parser::{Parser, ast::*};
use dryad_lexer::{Lexer, Token};

fn parse_tokens(input: &str) -> Program {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token().unwrap() {
            Token::Eof => break,
            token => tokens.push(token),
        }
    }
    
    let mut parser = Parser::new(tokens);
    parser.parse().unwrap()
}

#[test]
fn test_parse_empty_array() {
    let program = parse_tokens("let arr = [];");
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::VarDeclaration(name, Some(expr)) => {
            assert_eq!(name, "arr");
            match expr {
                Expr::Array(elements) => {
                    assert_eq!(elements.len(), 0);
                },
                _ => panic!("Esperado Array, encontrado {:?}", expr),
            }
        },
        _ => panic!("Esperado VarDeclaration, encontrado {:?}", &program.statements[0]),
    }
}

#[test]
fn test_parse_array_with_numbers() {
    let program = parse_tokens("let numeros = [1, 2, 3];");
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::VarDeclaration(name, Some(expr)) => {
            assert_eq!(name, "numeros");
            match expr {
                Expr::Array(elements) => {
                    assert_eq!(elements.len(), 3);
                    
                    match &elements[0] {
                        Expr::Literal(Literal::Number(n)) => assert_eq!(*n, 1.0),
                        _ => panic!("Esperado número 1"),
                    }
                    
                    match &elements[1] {
                        Expr::Literal(Literal::Number(n)) => assert_eq!(*n, 2.0),
                        _ => panic!("Esperado número 2"),
                    }
                    
                    match &elements[2] {
                        Expr::Literal(Literal::Number(n)) => assert_eq!(*n, 3.0),
                        _ => panic!("Esperado número 3"),
                    }
                },
                _ => panic!("Esperado Array, encontrado {:?}", expr),
            }
        },
        _ => panic!("Esperado VarDeclaration, encontrado {:?}", &program.statements[0]),
    }
}

#[test]
fn test_parse_array_access() {
    let program = parse_tokens("let valor = arr[0];");
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::VarDeclaration(name, Some(expr)) => {
            assert_eq!(name, "valor");
            match expr {
                Expr::Index(array_expr, index_expr) => {
                    match array_expr.as_ref() {
                        Expr::Variable(var_name) => assert_eq!(var_name, "arr"),
                        _ => panic!("Esperado variável arr"),
                    }
                    
                    match index_expr.as_ref() {
                        Expr::Literal(Literal::Number(n)) => assert_eq!(*n, 0.0),
                        _ => panic!("Esperado índice 0"),
                    }
                },
                _ => panic!("Esperado Index, encontrado {:?}", expr),
            }
        },
        _ => panic!("Esperado VarDeclaration, encontrado {:?}", &program.statements[0]),
    }
}

#[test]
fn test_parse_empty_tuple() {
    let program = parse_tokens("let vazio = ();");
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::VarDeclaration(name, Some(expr)) => {
            assert_eq!(name, "vazio");
            match expr {
                Expr::Tuple(elements) => {
                    assert_eq!(elements.len(), 0);
                },
                _ => panic!("Esperado Tuple, encontrado {:?}", expr),
            }
        },
        _ => panic!("Esperado VarDeclaration, encontrado {:?}", &program.statements[0]),
    }
}

#[test]
fn test_parse_tuple_access() {
    let program = parse_tokens("let valor = tupla.1;");
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::VarDeclaration(name, Some(expr)) => {
            assert_eq!(name, "valor");
            match expr {
                Expr::TupleAccess(tuple_expr, index) => {
                    match tuple_expr.as_ref() {
                        Expr::Variable(var_name) => assert_eq!(var_name, "tupla"),
                        _ => panic!("Esperado variável tupla"),
                    }
                    
                    assert_eq!(*index, 1);
                },
                _ => panic!("Esperado TupleAccess, encontrado {:?}", expr),
            }
        },
        _ => panic!("Esperado VarDeclaration, encontrado {:?}", &program.statements[0]),
    }
}
