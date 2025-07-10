// src/parser/parser.rs

use crate::lexer::tokenizer::Lexer;
use crate::lexer::token::Token;
use crate::parser::ast::{Expr, Stmt, BinaryOp, UnaryOp, Visibility, FieldDecl};
use crate::interpreter::errors::{DryadError, ErrorCode};

pub struct Parser {
    lexer: Lexer,
    current: Token,
    peek: Token,
    errors: Vec<DryadError>,
    current_line: usize,
    current_column: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let peek = lexer.next_token();
        Self { 
            lexer, 
            current, 
            peek,
            errors: Vec::new(),
            current_line: 1,
            current_column: 1,
        }
    }
    
    pub fn get_errors(&self) -> &[DryadError] {
        &self.errors
    }
    
    fn add_error(&mut self, code: ErrorCode, message: String) {
        let error = DryadError::with_code(code, message, Some((self.current_line, self.current_column)));
        self.errors.push(error);
    }

    fn advance(&mut self) {
        // Update position tracking based on current token (simplified)
        if let Token::Identifier(_) = &self.current {
            self.current_column += 1; // Simplified tracking
        }
        
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

    fn expect(&mut self, expected_token: &Token, error_code: ErrorCode, error_msg: &str) -> bool {
        if self.check(expected_token) {
            self.advance();
            true
        } else {
            self.add_error(error_code, error_msg.to_string());
            false
        }
    }

    pub fn parse_statement(&mut self) -> Option<Stmt> {
        // Verificar se chegamos ao fim
        if self.check(&Token::Eof) {
            return None;
        }
        
        match &self.current {
            Token::Let => self.parse_let(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::LBrace => self.parse_block(),
            Token::Class => self.parse_class(),
            Token::Fun | Token::Function => self.parse_function(Visibility::Public),
            Token::Namespace => self.parse_namespace(),
            Token::Use => self.parse_use(),
            Token::Using => self.parse_using(),
            Token::Export => self.parse_export(),
            Token::Try => self.parse_try(),
            Token::Throw => self.parse_throw(),
            Token::Public | Token::Private | Token::Protected | Token::Static => {
                let (visibility, is_static) = self.parse_visibility_and_static();
                match &self.current {
                    Token::Fun | Token::Function => self.parse_function_with_modifiers(visibility, is_static),
                    Token::Class => self.parse_class_with_visibility(visibility),
                    _ => {
                        // Se não for fun nem class após modificadores, avança para próximo token
                        self.advance();
                        None
                    }
                }
            },
            Token::Return => self.parse_return(),
            Token::Eof => None,
            _ => {
                // Verificar se é uma atribuição (identifier = expression)
                if let Token::Identifier(name) = &self.current.clone() {
                    let name = name.clone();
                    
                    // Verificar se o próximo token é '=' antes de consumir o identificador
                    if self.peek == Token::Equal {
                        self.advance(); // consume identifier
                        self.advance(); // consume '='
                        
                        // É uma atribuição
                        if let Some(value) = self.parse_expression() {
                            self.matches(&Token::Semicolon); // consume semicolon opcional
                            return Some(Stmt::Assign { name, value });
                        } else {
                            return None;
                        }
                    }
                }
                
                // Tentar parsear como expressão (inclui chamadas de função)
                if let Some(expr) = self.parse_expression() {
                    // Consumir ponto e vírgula opcional
                    self.matches(&Token::Semicolon);
                    Some(Stmt::Expr(expr))
                } else {
                    self.advance(); // Avança para evitar loop infinito
                    None
                }
            }
        }
    }

    fn parse_let(&mut self) -> Option<Stmt> {
        self.parse_let_with_semicolon(true)
    }
    
    fn parse_let_with_semicolon(&mut self, consume_semicolon: bool) -> Option<Stmt> {
        self.advance(); // consume 'let'

        if let Token::Identifier(name) = &self.current {
            let name = name.clone();
            self.advance();

            if !self.expect(&Token::Equal, ErrorCode::E2002, "Expected '=' after variable name in let declaration") {
                return None;
            }

            let expr = match self.parse_expression() {
                Some(expr) => expr,
                None => {
                    self.add_error(ErrorCode::E2007, "Expected expression after '=' in let declaration".to_string());
                    return None;
                }
            };
            
            // Consumir ponto e vírgula apenas se solicitado
            if consume_semicolon {
                self.matches(&Token::Semicolon);
            }
            Some(Stmt::Let { name, value: expr })
        } else {
            self.add_error(ErrorCode::E2011, "Expected identifier after 'let'".to_string());
            None
        }
    }

    fn parse_if(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'if'
        
        if !self.expect(&Token::LParen, ErrorCode::E2005, "Expected '(' after 'if'") {
            return None;
        }
        
        let cond = match self.parse_expression() {
            Some(expr) => expr,
            None => {
                self.add_error(ErrorCode::E2017, "Expected condition expression in if statement".to_string());
                return None;
            }
        };
        
        if !self.expect(&Token::RParen, ErrorCode::E2005, "Expected ')' after if condition") {
            return None;
        }
        
        let then_branch = match self.parse_statement() {
            Some(stmt) => Box::new(stmt),
            None => {
                self.add_error(ErrorCode::E2017, "Expected statement after if condition".to_string());
                return None;
            }
        };
        
        let else_branch = if self.matches(&Token::Else) {
            match self.parse_statement() {
                Some(stmt) => Some(Box::new(stmt)),
                None => {
                    self.add_error(ErrorCode::E2017, "Expected statement after 'else'".to_string());
                    return None;
                }
            }
        } else {
            None
        };

        Some(Stmt::If { cond, then_branch, else_branch })
    }

    fn parse_while(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'while'
        
        if !self.expect(&Token::LParen, ErrorCode::E2005, "Expected '(' after 'while'") {
            return None;
        }
        
        let cond = match self.parse_expression() {
            Some(expr) => expr,
            None => {
                self.add_error(ErrorCode::E2018, "Expected condition expression in while loop".to_string());
                return None;
            }
        };
        
        if !self.expect(&Token::RParen, ErrorCode::E2005, "Expected ')' after while condition") {
            return None;
        }
        
        let body = match self.parse_statement() {
            Some(stmt) => Box::new(stmt),
            None => {
                self.add_error(ErrorCode::E2018, "Expected statement in while loop body".to_string());
                return None;
            }
        };

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
        } else if self.check(&Token::Let) {
            // Para declarações let no for, não consumir o semicolon
            Some(Box::new(self.parse_let_with_semicolon(false)?))
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
            // Verificar se é uma atribuição (identificador seguido de =)
            if let Token::Identifier(name) = &self.current.clone() {
                let name_clone = name.clone();
                let peek_token = self.peek.clone();
                
                if let Token::Equal = peek_token {
                    // É uma atribuição: i = i + 1
                    self.advance(); // consume identificador
                    self.advance(); // consume '='
                    let value = self.parse_expression()?;
                    Some(Expr::Assign {
                        name: name_clone,
                        value: Box::new(value),
                    })
                } else {
                    // Não é uma atribuição, parse como expressão normal
                    Some(self.parse_expression()?)
                }
            } else {
                Some(self.parse_expression()?)
            }
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
                // Skip invalid token and continue
                self.advance();
            }
        }
        
        if !self.expect(&Token::RBrace, ErrorCode::E2004, "Expected '}' to close block") {
            return None;
        }
        
        Some(Stmt::Block(statements))
    }

    fn parse_expression(&mut self) -> Option<Expr> {
        self.parse_or()
    }
    
    fn parse_or(&mut self) -> Option<Expr> {
        let mut expr = self.parse_and()?;
        
        while matches!(self.current, Token::Or) {
            self.advance();
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        
        Some(expr)
    }
    
    fn parse_and(&mut self) -> Option<Expr> {
        let mut expr = self.parse_equality()?;
        
        while matches!(self.current, Token::And) {
            self.advance();
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        
        Some(expr)
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
        let mut expr = self.parse_unary()?;

        while matches!(self.current, Token::Star | Token::Slash) {
            let op = match &self.current {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Mod => BinaryOp::Mod,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }
    
    fn parse_unary(&mut self) -> Option<Expr> {
        if matches!(self.current, Token::Bang | Token::Minus) {
            let op = match &self.current {
                Token::Bang => UnaryOp::Not,
                Token::Minus => UnaryOp::Minus,
                _ => unreachable!(),
            };
            self.advance();
            let expr = self.parse_unary()?;
            Some(Expr::Unary {
                op,
                expr: Box::new(expr),
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        let mut expr = match &self.current.clone() {
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
            Token::True => {
                self.advance();
                Some(Expr::Bool(true))
            }
            Token::False => {
                self.advance();
                Some(Expr::Bool(false))
            }
            Token::Null => {
                self.advance();
                Some(Expr::Null)
            }
            Token::This => {
                self.advance();
                Some(Expr::This)
            }
            Token::New => {
                self.advance(); // consume 'new'
                match &self.current {
                    Token::Identifier(class_name) => {
                        let class = class_name.clone();
                        self.advance();
                        
                        if self.matches(&Token::LParen) {
                            let mut args = Vec::new();
                            
                            if !self.check(&Token::RParen) {
                                loop {
                                    args.push(self.parse_expression()?);
                                    if self.matches(&Token::Comma) {
                                        continue;
                                    } else {
                                        break;
                                    }
                                }
                            }
                            
                            if !self.matches(&Token::RParen) {
                                return None;
                            }
                            
                            Some(Expr::New { class, args })
                        } else {
                            Some(Expr::New { class, args: Vec::new() })
                        }
                    }
                    _ => None,
                }
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
                        self.add_error(ErrorCode::E2005, "Expected ')' after function arguments".to_string());
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
                let expr = match self.parse_expression() {
                    Some(expr) => expr,
                    None => {
                        self.add_error(ErrorCode::E2007, "Expected expression inside parentheses".to_string());
                        return None;
                    }
                };
                if !self.expect(&Token::RParen, ErrorCode::E2005, "Expected ')' after expression") {
                    return None;
                }
                Some(expr)
            }
            Token::LBracket => {
                self.advance(); // consume '['
                let mut elements = Vec::new();
                
                // Se não é ']', então há elementos
                if !self.check(&Token::RBracket) {
                    loop {
                        // Parse a single element using unary level to avoid expression issues
                        if let Some(expr) = self.parse_unary() {
                            elements.push(expr);
                        } else {
                            return None;
                        }
                        
                        // Se há vírgula, continua para próximo elemento
                        if self.matches(&Token::Comma) {
                            // Se há vírgula mas o próximo token é ']', é um trailing comma válido
                            if self.check(&Token::RBracket) {
                                break;
                            }
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                
                // Consume ']'
                if !self.expect(&Token::RBracket, ErrorCode::E2006, "Expected ']' to close array literal") {
                    return None;
                }
                
                Some(Expr::Array(elements))
            }
            _ => None,
        }?;

        // Handle member access (dot notation) and array indexing
        loop {
            if self.matches(&Token::Dot) {
                match &self.current {
                    Token::Identifier(name) => {
                        let member_name = name.clone();
                        self.advance();
                        
                        // Check if it's a method call
                        if self.matches(&Token::LParen) {
                            let mut args = Vec::new();
                            
                            if !self.check(&Token::RParen) {
                                loop {
                                    args.push(self.parse_expression()?);
                                    if self.matches(&Token::Comma) {
                                        continue;
                                    } else {
                                        break;
                                    }
                                }
                            }
                            
                            if !self.matches(&Token::RParen) {
                                return None;
                            }
                            
                            expr = Expr::Call {
                                function: format!("{}.{}", 
                                    match &expr {
                                        Expr::Identifier(n) => n.clone(),
                                        _ => "object".to_string(),
                                    }
                                , member_name),
                                args,
                            };
                        } else {
                            // Usar Get para member access em variáveis
                            expr = Expr::Get {
                                object: Box::new(expr),
                                name: member_name,
                            };
                        }
                    }
                    _ => break,
                }
            } else if self.matches(&Token::LBracket) {
                let index = self.parse_expression()?;
                if !self.matches(&Token::RBracket) {
                    return None;
                }
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }

        Some(expr)
    }

    #[allow(dead_code)]
    fn parse_visibility(&mut self) -> Visibility {
        match &self.current {
            Token::Public => {
                self.advance();
                Visibility::Public
            },
            Token::Private => {
                self.advance();
                Visibility::Private
            },
            Token::Protected => {
                self.advance();
                Visibility::Protected
            },
            _ => Visibility::Public, // default
        }
    }
    
    fn parse_visibility_and_static(&mut self) -> (Visibility, bool) {
        let mut visibility = Visibility::Public;
        let mut is_static = false;
        
        // Pode ter até dois modificadores (visibility + static) em qualquer ordem
        for _ in 0..2 {
            match &self.current {
                Token::Public => {
                    self.advance();
                    visibility = Visibility::Public;
                },
                Token::Private => {
                    self.advance();
                    visibility = Visibility::Private;
                },
                Token::Protected => {
                    self.advance();
                    visibility = Visibility::Protected;
                },
                Token::Static => {
                    self.advance();
                    is_static = true;
                },
                _ => break,
            }
        }
        
        (visibility, is_static)
    }

    fn parse_class(&mut self) -> Option<Stmt> {
        self.parse_class_with_visibility(Visibility::Public)
    }

    fn parse_class_with_visibility(&mut self, visibility: Visibility) -> Option<Stmt> {
        self.advance(); // consume 'class'
        
        let name = match &self.current {
            Token::Identifier(name) => {
                let class_name = name.clone();
                self.advance();
                class_name
            },
            _ => return None,
        };

        if !self.matches(&Token::LBrace) {
            return None;
        }

        let mut methods = Vec::new();
        let mut fields = Vec::new();

        while !self.check(&Token::RBrace) && !self.check(&Token::Eof) {
            // Parse visibility and static modifiers
            let (member_visibility, is_static) = if matches!(self.current, Token::Public | Token::Private | Token::Protected | Token::Static) {
                self.parse_visibility_and_static()
            } else {
                (Visibility::Public, false)
            };

            match &self.current {
                Token::Fun | Token::Function => {
                    if let Some(method) = self.parse_function_with_modifiers(member_visibility, is_static) {
                        methods.push(method);
                    } else {
                        break; // Failed to parse method
                    }
                },
                Token::Identifier(field_name) => {
                    let field = FieldDecl {
                        name: field_name.clone(),
                        visibility: member_visibility,
                    };
                    fields.push(field);
                    self.advance(); // consume identifier
                    
                    if !self.matches(&Token::Semicolon) {
                        return None;
                    }
                },
                _ => {
                    // Unknown token in class body, break out
                    break;
                }
            }
        }

        if !self.matches(&Token::RBrace) {
            return None;
        }

        Some(Stmt::ClassDecl {
            name,
            methods,
            fields,
            visibility,
        })
    }

    fn parse_function(&mut self, visibility: Visibility) -> Option<Stmt> {
        self.advance(); // consume 'fun' or 'function'
        
        let name = match &self.current {
            Token::Identifier(name) => {
                let func_name = name.clone();
                self.advance();
                func_name
            },
            _ => {
                self.add_error(ErrorCode::E2012, "Expected function name after 'function'".to_string());
                return None;
            }
        };

        if !self.expect(&Token::LParen, ErrorCode::E2013, "Expected '(' after function name") {
            return None;
        }

        let mut params = Vec::new();
        
        if !self.check(&Token::RParen) {
            loop {
                match &self.current {
                    Token::Identifier(param) => {
                        params.push(param.clone());
                        self.advance();
                    },
                    _ => {
                        self.add_error(ErrorCode::E2014, "Expected parameter name in function".to_string());
                        return None;
                    }
                }
                
                if self.matches(&Token::Comma) {
                    continue;
                } else {
                    break;
                }
            }
        }

        if !self.expect(&Token::RParen, ErrorCode::E2013, "Expected ')' after function parameters") {
            return None;
        }

        let body = match self.parse_block() {
            Some(body) => Box::new(body),
            None => {
                self.add_error(ErrorCode::E2009, "Expected function body after parameters".to_string());
                return None;
            }
        };

        Some(Stmt::FunDecl {
            name,
            params,
            body,
            visibility,
            is_static: false,
        })
    }
    
    fn parse_function_with_modifiers(&mut self, visibility: Visibility, is_static: bool) -> Option<Stmt> {
        self.advance(); // consume 'fun' or 'function'
        
        let name = match &self.current {
            Token::Identifier(name) => {
                let func_name = name.clone();
                self.advance();
                func_name
            },
            _ => {
                self.add_error(ErrorCode::E2012, "Expected function name after 'function'".to_string());
                return None;
            }
        };

        if !self.expect(&Token::LParen, ErrorCode::E2013, "Expected '(' after function name") {
            return None;
        }

        let mut params = Vec::new();
        
        if !self.check(&Token::RParen) {
            loop {
                match &self.current {
                    Token::Identifier(param) => {
                        params.push(param.clone());
                        self.advance();
                    },
                    _ => {
                        self.add_error(ErrorCode::E2014, "Expected parameter name in function".to_string());
                        return None;
                    }
                }
                
                if self.matches(&Token::Comma) {
                    continue;
                } else {
                    break;
                }
            }
        }

        if !self.expect(&Token::RParen, ErrorCode::E2013, "Expected ')' after function parameters") {
            return None;
        }

        let body = match self.parse_block() {
            Some(body) => Box::new(body),
            None => {
                self.add_error(ErrorCode::E2009, "Expected function body after parameters".to_string());
                return None;
            }
        };

        Some(Stmt::FunDecl {
            name,
            params,
            body,
            visibility,
            is_static,
        })
    }

    fn parse_return(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'return'
        
        let value = if self.check(&Token::Semicolon) {
            None
        } else {
            self.parse_expression()
        };
        
        Some(Stmt::Return(value))
    }

    fn parse_namespace(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'namespace'
        
        // Parse namespace name (pode ser A.B.C)
        let mut namespace_parts = Vec::new();
        
        if let Token::Identifier(name) = &self.current {
            namespace_parts.push(name.clone());
            self.advance();
            
            // Suporte para namespaces aninhados (A.B.C)
            while self.matches(&Token::Dot) {
                if let Token::Identifier(part) = &self.current {
                    namespace_parts.push(part.clone());
                    self.advance();
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }
        
        let namespace_name = namespace_parts.join(".");
        
        // Parse body
        if !self.matches(&Token::LBrace) {
            return None;
        }
        
        let mut body = Vec::new();
        while !self.check(&Token::RBrace) && !self.check(&Token::Eof) {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            } else {
                break;
            }
        }
        
        if !self.matches(&Token::RBrace) {
            return None;
        }
        
        Some(Stmt::NamespaceDecl {
            name: namespace_name,
            body,
        })
    }

    fn parse_using(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'using'
        
        // Parse dotted path: Math.Utils.Something
        let mut module_path = String::new();
        
        if let Token::Identifier(first_part) = &self.current {
            module_path.push_str(first_part);
            self.advance();
            
            // Parse additional dot-separated parts
            while self.current == Token::Dot {
                self.advance(); // consume '.'
                if let Token::Identifier(part) = &self.current {
                    module_path.push('.');
                    module_path.push_str(part);
                    self.advance();
                } else {
                    return None;
                }
            }
            
            // Check for alias: using Math.Utils as MU
            let alias = if self.current == Token::Identifier("as".to_string()) {
                self.advance(); // consume 'as'
                if let Token::Identifier(alias_name) = &self.current {
                    let alias_name = alias_name.clone();
                    self.advance();
                    Some(alias_name)
                } else {
                    return None;
                }
            } else {
                None
            };
            
            // Consumir ponto e vírgula opcional
            self.matches(&Token::Semicolon);
            
            Some(Stmt::Using { module_path, alias })
        } else {
            None
        }
    }

    fn parse_use(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'use'
        
        // Parse file path: './simpleexport.dryad'
        if let Token::String(file_path) = &self.current {
            let file_path = file_path.clone();
            self.advance();
            
            // Consumir ponto e vírgula opcional
            self.matches(&Token::Semicolon);
            
            Some(Stmt::Use { file_path })
        } else {
            None
        }
    }
    
    fn parse_export(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'export'
        
        // Parse the item to export (function, class, or namespace)
        if let Some(item) = self.parse_statement() {
            Some(Stmt::Export { item: Box::new(item) })
        } else {
            None
        }
    }

    fn parse_try(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'try'
        
        // Parse try block
        if let Some(try_block) = self.parse_statement() {
            // Expect 'catch'
            if self.current == Token::Catch {
                self.advance(); // consume 'catch'
                
                // Parse optional catch parameter: catch (e) { ... }
                let catch_param = if self.current == Token::LParen {
                    self.advance(); // consume '('
                    let param = if let Token::Identifier(name) = &self.current {
                        let param_name = name.clone();
                        self.advance();
                        Some(param_name)
                    } else {
                        None
                    };
                    
                    if self.current == Token::RParen {
                        self.advance(); // consume ')'
                    }
                    
                    param
                } else {
                    None
                };
                
                // Parse catch block
                if let Some(catch_block) = self.parse_statement() {
                    Some(Stmt::Try {
                        try_block: Box::new(try_block),
                        catch_param,
                        catch_block: Box::new(catch_block),
                    })
                } else {
                    None
                }
            } else {
                None // Missing catch block
            }
        } else {
            None
        }
    }
    
    fn parse_throw(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'throw'
        
        // Parse the expression to throw
        if let Some(value) = self.parse_expression() {
            // Consume optional semicolon
            self.matches(&Token::Semicolon);
            
            Some(Stmt::Throw { value })
        } else {
            None
        }
    }
}
