// src/interpreter/evaluator.rs

use crate::parser::ast::{Expr, Stmt, BinaryOp};
use crate::interpreter::env::{Env, Value};
use crate::interpreter::io;
use crate::interpreter::types::{TypeChecker};
use crate::interpreter::errors::{DryadError, ErrorReporter, ErrorSeverity};

#[derive(Debug)]
pub struct EvaluationResult {
    pub value: Option<Value>,
    pub errors: Vec<DryadError>,
}

impl EvaluationResult {
    pub fn new(value: Option<Value>) -> Self {
        Self {
            value,
            errors: Vec::new(),
        }
    }
    
    pub fn with_error(error: DryadError) -> Self {
        Self {
            value: None,
            errors: vec![error],
        }
    }
    
    pub fn add_error(&mut self, error: DryadError) {
        self.errors.push(error);
    }
}

pub struct Evaluator {
    type_checker: TypeChecker,
    error_reporter: ErrorReporter,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            type_checker: TypeChecker::new(),
            error_reporter: ErrorReporter::new(),
        }
    }
    
    pub fn eval_stmt(&mut self, stmt: &Stmt, env: &mut Env) -> EvaluationResult {
        // Verificação de tipos antes da avaliação
        if let Some(type_error) = self.type_checker.check_statement(stmt, env) {
            let error = DryadError::new(
                format!("Erro de tipo: {:?}", type_error),
                Some((0, 0)), // TODO: Adicionar posições reais
                ErrorSeverity::Error,
            );
            return EvaluationResult::with_error(error);
        }
        
        match stmt {
            Stmt::Let { name, value } => {
                let result = self.eval_expr(value, env);
                if let Some(val) = result.value {
                    env.set(name.clone(), val.clone());
                    EvaluationResult::new(Some(val))
                } else {
                    result
                }
            }
            Stmt::Expr(expr) => self.eval_expr(expr, env),
            Stmt::Block(stmts) => {
                let mut final_result = EvaluationResult::new(None);
                for s in stmts {
                    let result = self.eval_stmt(s, env);
                    final_result.errors.extend(result.errors);
                    if result.value.is_some() {
                        final_result.value = result.value;
                    }
                }
                final_result
            }
            Stmt::If { cond, then_branch, else_branch } => {
                let cond_result = self.eval_expr(cond, env);
                if !cond_result.errors.is_empty() {
                    return cond_result;
                }
                
                if let Some(val) = cond_result.value {
                    if val.is_truthy() {
                        self.eval_stmt(then_branch, env)
                    } else if let Some(else_b) = else_branch {
                        self.eval_stmt(else_b, env)
                    } else {
                        EvaluationResult::new(None)
                    }
                } else {
                    EvaluationResult::new(None)
                }
            }
            Stmt::While { cond, body } => {
                let mut result = EvaluationResult::new(None);
                loop {
                    let cond_result = self.eval_expr(cond, env);
                    result.errors.extend(cond_result.errors);
                    
                    if let Some(val) = cond_result.value {
                        if val.is_truthy() {
                            let body_result = self.eval_stmt(body, env);
                            result.errors.extend(body_result.errors);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                result
            }
            Stmt::For { init, cond, post, body } => {
                let mut result = EvaluationResult::new(None);
                
                if let Some(init_stmt) = init {
                    let init_result = self.eval_stmt(init_stmt, env);
                    result.errors.extend(init_result.errors);
                }

                loop {
                    let should_continue = if let Some(cond_expr) = cond {
                        let cond_result = self.eval_expr(cond_expr, env);
                        result.errors.extend(cond_result.errors);
                        
                        if let Some(val) = cond_result.value {
                            val.is_truthy()
                        } else {
                            false
                        }
                    } else {
                        true
                    };

                    if !should_continue {
                        break;
                    }

                    let body_result = self.eval_stmt(body, env);
                    result.errors.extend(body_result.errors);
                    
                    if let Some(post_expr) = post {
                        let post_result = self.eval_expr(post_expr, env);
                        result.errors.extend(post_result.errors);
                    }
                }
                result
            }
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr, env: &mut Env) -> EvaluationResult {
        // Verificação de tipos antes da avaliação
        if let Some(type_error) = self.type_checker.check_expression(expr, env) {
            let error = DryadError::new(
                format!("Erro de tipo: {:?}", type_error),
                Some((0, 0)), // TODO: Adicionar posições reais
                ErrorSeverity::Error,
            );
            return EvaluationResult::with_error(error);
        }
        
        match expr {
            Expr::Number(n) => EvaluationResult::new(Some(Value::Number(*n))),
            Expr::String(s) => EvaluationResult::new(Some(Value::String(s.clone()))),
            Expr::Identifier(name) => {
                if let Some(value) = env.get(name) {
                    EvaluationResult::new(Some(value))
                } else {
                    let error = DryadError::new(
                        format!("Variável '{}' não definida", name),
                        Some((0, 0)), // TODO: Adicionar posições reais
                        ErrorSeverity::Error,
                    );
                    EvaluationResult::with_error(error)
                }
            }
            Expr::Binary { left, op, right } => {
                let left_result = self.eval_expr(left, env);
                if !left_result.errors.is_empty() {
                    return left_result;
                }
                
                let right_result = self.eval_expr(right, env);
                if !right_result.errors.is_empty() {
                    return right_result;
                }
                
                if let (Some(lval), Some(rval)) = (left_result.value, right_result.value) {
                    match (lval, rval, op) {
                        (Value::Number(a), Value::Number(b), BinaryOp::Add) => 
                            EvaluationResult::new(Some(Value::Number(a + b))),
                        (Value::Number(a), Value::Number(b), BinaryOp::Sub) => 
                            EvaluationResult::new(Some(Value::Number(a - b))),
                        (Value::Number(a), Value::Number(b), BinaryOp::Mul) => 
                            EvaluationResult::new(Some(Value::Number(a * b))),
                        (Value::Number(a), Value::Number(b), BinaryOp::Div) => {
                            if b == 0.0 {
                                let error = DryadError::new(
                                    "Divisão por zero".to_string(),
                                    Some((0, 0)), // TODO: Adicionar posições reais
                                    ErrorSeverity::Error,
                                );
                                EvaluationResult::with_error(error)
                            } else {
                                EvaluationResult::new(Some(Value::Number(a / b)))
                            }
                        }
                        (Value::Number(a), Value::Number(b), BinaryOp::Equal) => 
                            EvaluationResult::new(Some(Value::Bool(a == b))),
                        (Value::Number(a), Value::Number(b), BinaryOp::NotEqual) => 
                            EvaluationResult::new(Some(Value::Bool(a != b))),
                        (Value::Number(a), Value::Number(b), BinaryOp::Less) => 
                            EvaluationResult::new(Some(Value::Bool(a < b))),
                        (Value::Number(a), Value::Number(b), BinaryOp::LessEqual) => 
                            EvaluationResult::new(Some(Value::Bool(a <= b))),
                        (Value::Number(a), Value::Number(b), BinaryOp::Greater) => 
                            EvaluationResult::new(Some(Value::Bool(a > b))),
                        (Value::Number(a), Value::Number(b), BinaryOp::GreaterEqual) => 
                            EvaluationResult::new(Some(Value::Bool(a >= b))),
                        _ => {
                            let error = DryadError::new(
                                format!("Operação binária não suportada: {:?}", op),
                                Some((0, 0)), // TODO: Adicionar posições reais
                                ErrorSeverity::Error,
                            );
                            EvaluationResult::with_error(error)
                        }
                    }
                } else {
                    EvaluationResult::new(Some(Value::Null))
                }
            }
            Expr::Call { function, args } => self.eval_function_call(function, args, env),
        }
    }
    
    fn eval_function_call(&mut self, function: &str, args: &[Expr], env: &mut Env) -> EvaluationResult {
        match function {
            "print" => {
                for arg in args {
                    let result = self.eval_expr(arg, env);
                    if !result.errors.is_empty() {
                        return result;
                    }
                    
                    if let Some(value) = result.value {
                        match value {
                            Value::Number(n) => print!("{}", n),
                            Value::String(s) => print!("{}", s),
                            Value::Bool(b) => print!("{}", b),
                            Value::Null => print!("null"),
                        }
                    }
                }
                println!(); // Nova linha após print
                EvaluationResult::new(Some(Value::Null))
            }
            "read_file" => {
                if let Some(filename_arg) = args.get(0) {
                    let result = self.eval_expr(filename_arg, env);
                    if !result.errors.is_empty() {
                        return result;
                    }
                    
                    if let Some(Value::String(filename)) = result.value {
                        match io::read_file(&filename) {
                            Ok(content) => EvaluationResult::new(Some(Value::String(content))),
                            Err(e) => {
                                let error = DryadError::new(
                                    format!("Erro ao ler arquivo '{}': {}", filename, e),
                                    Some((0, 0)),
                                    ErrorSeverity::Error,
                                );
                                EvaluationResult::with_error(error)
                            }
                        }
                    } else {
                        let error = DryadError::new(
                            "read_file: argumento deve ser uma string".to_string(),
                            Some((0, 0)),
                            ErrorSeverity::Error,
                        );
                        EvaluationResult::with_error(error)
                    }
                } else {
                    let error = DryadError::new(
                        "read_file: nome do arquivo requerido".to_string(),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    );
                    EvaluationResult::with_error(error)
                }
            }
            "write_file" => {
                if let (Some(filename_arg), Some(content_arg)) = (args.get(0), args.get(1)) {
                    let filename_result = self.eval_expr(filename_arg, env);
                    if !filename_result.errors.is_empty() {
                        return filename_result;
                    }
                    
                    let content_result = self.eval_expr(content_arg, env);
                    if !content_result.errors.is_empty() {
                        return content_result;
                    }
                    
                    if let (Some(Value::String(filename)), Some(Value::String(content))) = 
                        (filename_result.value, content_result.value) {
                        match io::write_file(&filename, &content) {
                            Ok(_) => EvaluationResult::new(Some(Value::Null)),
                            Err(e) => {
                                let error = DryadError::new(
                                    format!("Erro ao escrever arquivo '{}': {}", filename, e),
                                    Some((0, 0)),
                                    ErrorSeverity::Error,
                                );
                                EvaluationResult::with_error(error)
                            }
                        }
                    } else {
                        let error = DryadError::new(
                            "write_file: argumentos devem ser strings".to_string(),
                            Some((0, 0)),
                            ErrorSeverity::Error,
                        );
                        EvaluationResult::with_error(error)
                    }
                } else {
                    let error = DryadError::new(
                        "write_file: nome do arquivo e conteúdo requeridos".to_string(),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    );
                    EvaluationResult::with_error(error)
                }
            }
            "append_file" => {
                if let (Some(filename_arg), Some(content_arg)) = (args.get(0), args.get(1)) {
                    let filename_result = self.eval_expr(filename_arg, env);
                    if !filename_result.errors.is_empty() {
                        return filename_result;
                    }
                    
                    let content_result = self.eval_expr(content_arg, env);
                    if !content_result.errors.is_empty() {
                        return content_result;
                    }
                    
                    if let (Some(Value::String(filename)), Some(Value::String(content))) = 
                        (filename_result.value, content_result.value) {
                        match io::append_file(&filename, &content) {
                            Ok(_) => EvaluationResult::new(Some(Value::Null)),
                            Err(e) => {
                                let error = DryadError::new(
                                    format!("Erro ao anexar ao arquivo '{}': {}", filename, e),
                                    Some((0, 0)),
                                    ErrorSeverity::Error,
                                );
                                EvaluationResult::with_error(error)
                            }
                        }
                    } else {
                        let error = DryadError::new(
                            "append_file: argumentos devem ser strings".to_string(),
                            Some((0, 0)),
                            ErrorSeverity::Error,
                        );
                        EvaluationResult::with_error(error)
                    }
                } else {
                    let error = DryadError::new(
                        "append_file: nome do arquivo e conteúdo requeridos".to_string(),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    );
                    EvaluationResult::with_error(error)
                }
            }
            _ => {
                let error = DryadError::new(
                    format!("Função '{}' não implementada", function),
                    Some((0, 0)),
                    ErrorSeverity::Error,
                );
                EvaluationResult::with_error(error)
            }
        }
    }
    
    pub fn report_errors(&mut self, errors: &[DryadError]) {
        for error in errors {
            self.error_reporter.report_error(error);
        }
    }
}

// Funções de compatibilidade para manter a API antiga
pub fn eval_stmt(stmt: &Stmt, env: &mut Env) -> Option<Value> {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_stmt(stmt, env);
    
    if !result.errors.is_empty() {
        evaluator.report_errors(&result.errors);
        None
    } else {
        result.value
    }
}

pub fn eval_expr(expr: &Expr, env: &mut Env) -> Option<Value> {
    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_expr(expr, env);
    
    if !result.errors.is_empty() {
        evaluator.report_errors(&result.errors);
        None
    } else {
        result.value
    }
}
