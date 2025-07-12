// crates/dryad_parser/tests/native_directive_parser_tests.rs

use dryad_parser::{Parser, ast::{Stmt, Program}};
use dryad_lexer::{Lexer, Token};

fn parse_input(input: &str) -> Result<Program, dryad_errors::DryadError> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token().unwrap() {
            Token::Eof => break,
            token => tokens.push(token),
        }
    }
    
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn test_parse_single_native_directive() {
    let input = "#<console_io>";
    let program = parse_input(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::NativeDirective(module) => {
            assert_eq!(module, "console_io");
        }
        _ => panic!("Esperado NativeDirective"),
    }
}

#[test]
fn test_parse_multiple_native_directives() {
    let input = r#"
        #<console_io>
        #<file_io>
        #<debug>
    "#;
    let program = parse_input(input).unwrap();
    
    assert_eq!(program.statements.len(), 3);
    
    match &program.statements[0] {
        Stmt::NativeDirective(module) => assert_eq!(module, "console_io"),
        _ => panic!("Esperado NativeDirective"),
    }
    
    match &program.statements[1] {
        Stmt::NativeDirective(module) => assert_eq!(module, "file_io"),
        _ => panic!("Esperado NativeDirective"),
    }
    
    match &program.statements[2] {
        Stmt::NativeDirective(module) => assert_eq!(module, "debug"),
        _ => panic!("Esperado NativeDirective"),
    }
}

#[test]
fn test_parse_native_directive_with_code() {
    let input = r#"
        #<console_io>
        let x = 5;
        native_print(x);
    "#;
    let program = parse_input(input).unwrap();
    
    assert_eq!(program.statements.len(), 3);
    
    match &program.statements[0] {
        Stmt::NativeDirective(module) => assert_eq!(module, "console_io"),
        _ => panic!("Esperado NativeDirective"),
    }
    
    match &program.statements[1] {
        Stmt::VarDeclaration(name, _) => assert_eq!(name, "x"),
        _ => panic!("Esperado VarDeclaration"),
    }
    
    match &program.statements[2] {
        Stmt::Expression(_) => {}, // Chamada de função
        _ => panic!("Esperado Expression"),
    }
}

#[test]
fn test_parse_native_directive_at_beginning() {
    let input = r#"
        #<debug>
        function test() {
            return 42;
        }
        let result = test();
    "#;
    let program = parse_input(input).unwrap();
    
    assert_eq!(program.statements.len(), 3);
    
    match &program.statements[0] {
        Stmt::NativeDirective(module) => assert_eq!(module, "debug"),
        _ => panic!("Esperado NativeDirective"),
    }
    
    match &program.statements[1] {
        Stmt::FunctionDeclaration(name, _, _) => assert_eq!(name, "test"),
        _ => panic!("Esperado FunctionDeclaration"),
    }
}

#[test]
fn test_parse_native_directive_mixed_positions() {
    let input = r#"
        let x = 10;
        #<console_io>
        native_print(x);
        #<debug>
        let type_x = native_typeof(x);
    "#;
    let program = parse_input(input).unwrap();
    
    assert_eq!(program.statements.len(), 5);
    
    match &program.statements[0] {
        Stmt::VarDeclaration(name, _) => assert_eq!(name, "x"),
        _ => panic!("Esperado VarDeclaration"),
    }
    
    match &program.statements[1] {
        Stmt::NativeDirective(module) => assert_eq!(module, "console_io"),
        _ => panic!("Esperado NativeDirective"),
    }
    
    match &program.statements[2] {
        Stmt::Expression(_) => {}, // native_print call
        _ => panic!("Esperado Expression"),
    }
    
    match &program.statements[3] {
        Stmt::NativeDirective(module) => assert_eq!(module, "debug"),
        _ => panic!("Esperado NativeDirective"),
    }
    
    match &program.statements[4] {
        Stmt::VarDeclaration(name, _) => assert_eq!(name, "type_x"),
        _ => panic!("Esperado VarDeclaration"),
    }
}

#[test]
fn test_parse_native_directive_with_underscore() {
    let input = "#<terminal_ansi>";
    let program = parse_input(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::NativeDirective(module) => {
            assert_eq!(module, "terminal_ansi");
        }
        _ => panic!("Esperado NativeDirective"),
    }
}

#[test]
fn test_parse_native_directive_with_numbers() {
    let input = "#<crypto123>";
    let program = parse_input(input).unwrap();
    
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::NativeDirective(module) => {
            assert_eq!(module, "crypto123");
        }
        _ => panic!("Esperado NativeDirective"),
    }
}

#[test]
fn test_parse_complex_program_with_directives() {
    let input = r#"
        #<console_io>
        #<debug>
        #<file_io>
        
        function calculate(a, b) {
            native_print("Calculating...");
            let result = a + b;
            native_log(result);
            return result;
        }
        
        let x = 10;
        let y = 20;
        let sum = calculate(x, y);
        
        if sum > 25 {
            native_print("Sum is greater than 25");
        }
    "#;
    let program = parse_input(input).unwrap();
    
    // Verifica se tem pelo menos as 3 diretivas + função + 3 variáveis + if = 8 statements
    assert!(program.statements.len() >= 8);
    
    // Verifica as primeiras 3 diretivas
    match &program.statements[0] {
        Stmt::NativeDirective(module) => assert_eq!(module, "console_io"),
        _ => panic!("Esperado NativeDirective"),
    }
    
    match &program.statements[1] {
        Stmt::NativeDirective(module) => assert_eq!(module, "debug"),
        _ => panic!("Esperado NativeDirective"),
    }
    
    match &program.statements[2] {
        Stmt::NativeDirective(module) => assert_eq!(module, "file_io"),
        _ => panic!("Esperado NativeDirective"),
    }
}
