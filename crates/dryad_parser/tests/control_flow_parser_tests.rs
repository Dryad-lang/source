use dryad_parser::{Parser, ast::{Stmt, Expr}};
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
    let program = parser.parse().unwrap();
    program.statements
}

#[test]
fn test_parse_simple_if_statement() {
    let input = "if idade >= 18 { x = 1; }";
    let statements = parse_tokens(input);
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::If(condition, body) => {
            // Verificar que a condição é uma expressão binária
            match condition {
                Expr::Binary(left, op, right) => {
                    assert!(matches!(**left, Expr::Variable(_)));
                    assert_eq!(op, ">=");
                    assert!(matches!(**right, Expr::Literal(_)));
                }
                _ => panic!("Condição deveria ser uma expressão binária"),
            }
            // Verificar que o corpo é um bloco
            assert!(matches!(**body, Stmt::Block(_)));
        }
        _ => panic!("Deveria ser um Statement::If"),
    }
}

#[test]
fn test_parse_if_else_statement() {
    let input = "if nota >= 7.0 { status = \"aprovado\"; } else { status = \"reprovado\"; }";
    let statements = parse_tokens(input);
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::IfElse(condition, then_block, else_block) => {
            // Verificar condição
            assert!(matches!(condition, Expr::Binary(_, _, _)));
            // Verificar ambos os blocos
            assert!(matches!(**then_block, Stmt::Block(_)));
            assert!(matches!(**else_block, Stmt::Block(_)));
        }
        _ => panic!("Deveria ser um Statement::IfElse"),
    }
}

#[test]
fn test_parse_nested_if_else_chain() {
    let input = r#"
    if pontuacao >= 90 {
        resultado = "excelente";
    } else if pontuacao >= 80 {
        resultado = "bom";
    } else {
        resultado = "regular";
    }
    "#;
    let statements = parse_tokens(input);
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::IfElse(condition1, then_block1, else_block1) => {
            // Primeiro if
            assert!(matches!(condition1, Expr::Binary(_, _, _)));
            assert!(matches!(**then_block1, Stmt::Block(_)));
            
            // O else deveria ser outro IfElse
            match &**else_block1 {
                Stmt::IfElse(condition2, then_block2, else_block2) => {
                    assert!(matches!(condition2, Expr::Binary(_, _, _)));
                    assert!(matches!(**then_block2, Stmt::Block(_)));
                    assert!(matches!(**else_block2, Stmt::Block(_)));
                }
                _ => panic!("Else deveria conter outro IfElse"),
            }
        }
        _ => panic!("Deveria ser um Statement::IfElse"),
    }
}

#[test]
fn test_parse_nested_if_statements() {
    let input = r#"
    if x > 0 {
        if y > 0 {
            resultado = "ambos_positivos";
        }
    }
    "#;
    let statements = parse_tokens(input);
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::If(_, body) => {
            match &**body {
                Stmt::Block(block_stmts) => {
                    assert_eq!(block_stmts.len(), 1);
                    assert!(matches!(block_stmts[0], Stmt::If(_, _)));
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Deveria ser um Statement::If"),
    }
}

#[test]
fn test_parse_if_with_complex_condition() {
    let input = "if (idade >= 18 && idade <= 65) && ativo == true { elegivel = true; }";
    let statements = parse_tokens(input);
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::If(condition, _) => {
            // Verificar que é uma expressão binária com &&
            match condition {
                Expr::Binary(_, op, _) => {
                    assert_eq!(op, "&&");
                }
                _ => panic!("Condição deveria ser uma expressão binária"),
            }
        }
        _ => panic!("Deveria ser um Statement::If"),
    }
}

#[test]
fn test_parse_if_without_braces_error() {
    // Dryad requer chaves para blocos
    let input = "if idade >= 18 x = 1;";
    
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token().unwrap() {
            Token::Eof => break,
            token => tokens.push(token),
        }
    }
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_parse_if_with_single_statement_block() {
    let input = "if true { x = 42; }";
    let statements = parse_tokens(input);
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::If(condition, body) => {
            assert!(matches!(condition, Expr::Literal(_)));
            match &**body {
                Stmt::Block(block_stmts) => {
                    assert_eq!(block_stmts.len(), 1);
                    assert!(matches!(block_stmts[0], Stmt::Assignment(_, _)));
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Deveria ser um Statement::If"),
    }
}

#[test]
fn test_parse_if_with_multiple_statements() {
    let input = r#"
    if valor > 100 {
        let bonus = valor * 0.1;
        total = total + bonus;
        aplicado = true;
    }
    "#;
    let statements = parse_tokens(input);
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::If(_, body) => {
            match &**body {
                Stmt::Block(block_stmts) => {
                    assert_eq!(block_stmts.len(), 3);
                    assert!(matches!(block_stmts[0], Stmt::VarDeclaration(_, _)));
                    assert!(matches!(block_stmts[1], Stmt::Assignment(_, _)));
                    assert!(matches!(block_stmts[2], Stmt::Assignment(_, _)));
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Deveria ser um Statement::If"),
    }
}

#[test]
fn test_exact_syntax_md_examples() {
    // If simples - usando assignment em vez de print
    let input1 = r#"let idade = 18;
if idade >= 18 {
    status = "maior_de_idade";
}"#;
    let statements1 = parse_tokens(input1);
    assert_eq!(statements1.len(), 2); // let + if
    assert!(matches!(statements1[1], Stmt::If(_, _)));
    
    // If-else - usando assignment em vez de print
    let input2 = r#"let nota = 7.5;
if nota >= 7.0 {
    resultado = "aprovado";
} else {
    resultado = "reprovado";
}"#;
    let statements2 = parse_tokens(input2);
    assert_eq!(statements2.len(), 2); // let + if-else
    assert!(matches!(statements2[1], Stmt::IfElse(_, _, _)));
    
    // If-else encadeado - usando assignment em vez de print
    let input3 = r#"let pontuacao = 85;
if pontuacao >= 90 {
    classificacao = "excelente";
} else if pontuacao >= 80 {
    classificacao = "bom";
} else if pontuacao >= 70 {
    classificacao = "regular";
} else {
    classificacao = "insuficiente";
}"#;
    let statements3 = parse_tokens(input3);
    assert_eq!(statements3.len(), 2); // let + if-else encadeado
    assert!(matches!(statements3[1], Stmt::IfElse(_, _, _)));
}

#[test]
fn test_parse_if_else_precedence() {
    let input = "if a { if b { x = 1; } } else { y = 2; }";
    let statements = parse_tokens(input);
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::IfElse(_, then_block, else_block) => {
            // Then block deveria conter um if aninhado
            match &**then_block {
                Stmt::Block(block_stmts) => {
                    assert_eq!(block_stmts.len(), 1);
                    assert!(matches!(block_stmts[0], Stmt::If(_, _)));
                }
                _ => panic!("Then block deveria ser um bloco"),
            }
            // Else block deveria ser simples
            assert!(matches!(**else_block, Stmt::Block(_)));
        }
        _ => panic!("Deveria ser um Statement::IfElse"),
    }
}

#[test]
fn test_parse_if_with_variable_assignment() {
    let input = "if resultado { sucesso = true; }";
    let statements = parse_tokens(input);
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::If(condition, body) => {
            assert!(matches!(condition, Expr::Variable(_)));
            match &**body {
                Stmt::Block(block_stmts) => {
                    assert_eq!(block_stmts.len(), 1);
                    assert!(matches!(block_stmts[0], Stmt::Assignment(_, _)));
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Deveria ser um Statement::If"),
    }
}
