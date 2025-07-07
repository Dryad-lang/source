// src/parser/parser.rs

use crate::lexer::tokenizer::Lexer;
use crate::lexer::token::Token;
use crate::parser::ast::{Expr, Stmt, BinaryOp};

pub struct Parser {
    lexer: Lexer,
    current: Token,
    peek: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let peek = lexer.next_token();
        Self { lexer, current, peek }
    }

    fn advance(&mut self) {
        self.current = std::mem::replace(&mut self.peek, self.lexer.next_token());
    }

    fn check(&self, token: &Token) -> bool {
        &self.current == token
    }

    fn matches(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn parse_statement(&mut self) -> Option<Stmt> {
        match &self.current {
            Token::Let => self.parse_let(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::LBrace => self.parse_block(),
            _ => self.parse_expression().map(Stmt::Expr),
        }
    }

    fn parse_let(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'let'

        if let Token::Identifier(name) = &self.current {
            let name = name.clone();
            self.advance();

            if self.current != Token::Equal {
                return None;
            }
            self.advance(); // consume '='

            let expr = self.parse_expression()?;
            Some(Stmt::Let { name, value: expr })
        } else {
            None
        }
    }

    fn parse_if(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'if'
        
        if !self.check(&Token::LParen) {
            return None;
        }
        self.advance(); // consume '('
        
        let cond = self.parse_expression()?;
        
        if !self.check(&Token::RParen) {
            return None;
        }
        self.advance(); // consume ')'
        
        let then_branch = Box::new(self.parse_statement()?);
        
        let else_branch = if self.matches(&Token::Else) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Some(Stmt::If { cond, then_branch, else_branch })
    }

    fn parse_while(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'while'
        
        if !self.check(&Token::LParen) {
            return None;
        }
        self.advance(); // consume '('
        
        let cond = self.parse_expression()?;
        
        if !self.check(&Token::RParen) {
            return None;
        }
        self.advance(); // consume ')'
        
        let body = Box::new(self.parse_statement()?);

        Some(Stmt::While { cond, body })
    }

    fn parse_for(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'for'
        
        if !self.check(&Token::LParen) {
            return None;
        }
        self.advance(); // consume '('
        
        let init = if self.check(&Token::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };
        
        if !self.check(&Token::Semicolon) {
            return None;
        }
        self.advance(); // consume ';'
        
        let cond = if !self.check(&Token::Semicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if !self.check(&Token::Semicolon) {
            return None;
        }
        self.advance(); // consume ';'
        
        let post = if !self.check(&Token::RParen) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if !self.check(&Token::RParen) {
            return None;
        }
        self.advance(); // consume ')'
        
        let body = Box::new(self.parse_statement()?);

        Some(Stmt::For { init, cond, post, body })
    }

    fn parse_block(&mut self) -> Option<Stmt> {
        self.advance(); // consume '{'
        
        let mut statements = Vec::new();
        
        while !self.check(&Token::RBrace) && !self.check(&Token::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                break;
            }
        }
        
        if !self.check(&Token::RBrace) {
            return None;
        }
        self.advance(); // consume '}'
        
        Some(Stmt::Block(statements))
    }

    fn parse_expression(&mut self) -> Option<Expr> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Option<Expr> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.current, Token::EqualEqual | Token::BangEqual) {
            let op = match &self.current {
                Token::EqualEqual => BinaryOp::Equal,
                Token::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut expr = self.parse_term()?;

        while matches!(
            self.current,
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual
        ) {
            let op = match &self.current {
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_factor()?;

        while matches!(self.current, Token::Plus | Token::Minus) {
            let op = match &self.current {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        while matches!(self.current, Token::Star | Token::Slash) {
            let op = match &self.current {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_primary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match &self.current.clone() {
            Token::Number(n) => {
                let num = *n;
                self.advance();
                Some(Expr::Number(num))
            }
            Token::String(s) => {
                let string = s.clone();
                self.advance();
                Some(Expr::String(string))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                
                // Check for function call
                if self.current == Token::LParen {
                    self.advance(); // consume '('
                    let mut args = Vec::new();
                    
                    if self.current != Token::RParen {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.current == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    
                    if self.current != Token::RParen {
                        return None;
                    }
                    self.advance(); // consume ')'
                    
                    Some(Expr::Call {
                        function: name,
                        args,
                    })
                } else {
                    Some(Expr::Identifier(name))
                }
            }
            Token::LParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                if self.current != Token::RParen {
                    return None;
                }
                self.advance(); // consume ')'
                Some(expr)
            }
            _ => None,
        }
    }
}
