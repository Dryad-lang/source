// crates/dryad_parser/tests/statement_tests.rs
use dryad_parser::{Parser, Stmt, Program};
use dryad_lexer::{Lexer, Token};
use dryad_errors::DryadError;

#[cfg(test)]
mod statement_tests {
    use super::*;

    fn parse_program(source: &str) -> Result<Program, DryadError> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        
        loop {
            let token = lexer.next_token()?;
            if matches!(token, Token::Eof) {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    // Testes de Declaração de Variáveis
    #[test]
    fn test_var_declaration_with_value() {
        let program = parse_program("let x = 42;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::VarDeclaration(name, Some(_)) => {
                assert_eq!(name, "x");
            }
            _ => panic!("Esperado declaração de variável"),
        }
    }

    #[test]
    fn test_var_declaration_without_value() {
        let program = parse_program("let y;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::VarDeclaration(name, None) => {
                assert_eq!(name, "y");
            }
            _ => panic!("Esperado declaração de variável sem valor"),
        }
    }

    #[test]
    fn test_var_declaration_with_expression() {
        let program = parse_program("let result = 2 + 3 * 4;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::VarDeclaration(name, Some(_)) => {
                assert_eq!(name, "result");
            }
            _ => panic!("Esperado declaração de variável com expressão"),
        }
    }

    #[test]
    fn test_multiple_statements() {
        let program = parse_program("let x = 10; let y = 20; x + y;").unwrap();
        assert_eq!(program.statements.len(), 3);
        
        // Primeira declaração
        match &program.statements[0] {
            Stmt::VarDeclaration(name, Some(_)) => assert_eq!(name, "x"),
            _ => panic!("Primeira deve ser declaração de x"),
        }
        
        // Segunda declaração
        match &program.statements[1] {
            Stmt::VarDeclaration(name, Some(_)) => assert_eq!(name, "y"),
            _ => panic!("Segunda deve ser declaração de y"),
        }
        
        // Terceira expressão
        match &program.statements[2] {
            Stmt::Expression(_) => {},
            _ => panic!("Terceira deve ser expressão"),
        }
    }

    #[test]
    fn test_var_declaration_different_types() {
        let sources = [
            "let name = \"João\";",
            "let active = true;",
            "let value = null;",
            "let pi = 3.14;",
        ];
        
        for source in &sources {
            let program = parse_program(source).unwrap();
            assert_eq!(program.statements.len(), 1);
            
            match &program.statements[0] {
                Stmt::VarDeclaration(_, Some(_)) => {},
                _ => panic!("Esperado declaração de variável para: {}", source),
            }
        }
    }

    // Testes de Expressões como Statements
    #[test]
    fn test_expression_statement() {
        let program = parse_program("42;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Expression(_) => {},
            _ => panic!("Esperado statement de expressão"),
        }
    }

    #[test]
    fn test_expression_without_semicolon_at_eof() {
        let program = parse_program("2 + 3").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Expression(_) => {},
            _ => panic!("Esperado statement de expressão"),
        }
    }

    // Testes de Erros
    #[test]
    fn test_missing_variable_name() {
        let result = parse_program("let = 42;");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Parser(2011, _) => {}, // Esperado nome da variável
            _ => panic!("Erro esperado: E2011"),
        }
    }

    #[test]
    fn test_missing_semicolon_between_statements() {
        let result = parse_program("let x = 5 let y = 10;");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Parser(2003, _) => {}, // Esperado ';'
            _ => panic!("Erro esperado: E2003"),
        }
    }

    #[test]
    fn test_invalid_variable_name() {
        let result = parse_program("let 123 = 42;");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Parser(2011, _) => {}, // Esperado nome da variável
            _ => panic!("Erro esperado: E2011"),
        }
    }

    // Testes de Programas Complexos
    #[test]
    fn test_complex_program() {
        let source = r#"
            let x = 10;
            let y = 20;
            let result = x + y * 2;
            result;
        "#;
        
        let program = parse_program(source).unwrap();
        assert_eq!(program.statements.len(), 4);
    }

    #[test]
    fn test_nested_expressions() {
        let program = parse_program("let complex = (2 + 3) * (4 - 1);").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::VarDeclaration(name, Some(_)) => {
                assert_eq!(name, "complex");
            }
            _ => panic!("Esperado declaração de variável complexa"),
        }
    }

    #[test]
    fn test_empty_program() {
        let program = parse_program("").unwrap();
        assert_eq!(program.statements.len(), 0);
    }

    #[test]
    fn test_only_semicolons() {
        let program = parse_program(";;;").unwrap();
        // Semicolons sozinhos não criam statements válidos
        assert_eq!(program.statements.len(), 0);
    }

    #[test]
    fn test_whitespace_and_comments() {
        let source = r#"
            // Declaração de variável
            let x = 42; // valor inicial
            
            /* 
               Outra variável
            */
            let y = "hello";
        "#;
        
        let program = parse_program(source).unwrap();
        assert_eq!(program.statements.len(), 2);
    }
}
