// tests/parser_tests.rs

use dryad::lexer::tokenizer::Lexer;
use dryad::parser::parser::Parser;
use dryad::parser::ast::{Expr, Stmt, BinaryOp};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_expression() {
        let lexer = Lexer::new("42");
        let mut parser = Parser::new(lexer);

        let stmt = parser.parse_statement().unwrap();
        assert_eq!(stmt, Stmt::Expr(Expr::Number(42.0)));
    }

    #[test]
    fn test_parse_let_statement() {
        let lexer = Lexer::new("let x = 5;");
        let mut parser = Parser::new(lexer);

        let stmt = parser.parse_statement().unwrap();
        assert_eq!(stmt, Stmt::Let {
            name: "x".into(),
            value: Expr::Number(5.0)
        });
    }

    #[test]
    fn test_parse_binary_expression() {
        let lexer = Lexer::new("1 + 2");
        let mut parser = Parser::new(lexer);

        let stmt = parser.parse_statement().unwrap();
        assert_eq!(stmt, Stmt::Expr(
            Expr::Binary {
                left: Box::new(Expr::Number(1.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(2.0)),
            }
        ));
    }
}