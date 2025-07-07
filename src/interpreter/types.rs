// src/interpreter/types.rs

use crate::interpreter::env::Value;
use crate::parser::ast::{Expr, Stmt, BinaryOp};
use crate::interpreter::env::Env;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Bool,
    Null,
    Any,
}

#[derive(Debug, Clone)]
pub enum TypeError {
    TypeMismatch(Type, Type),
    InvalidOperation(String, Type),
    UndefinedVariable(String),
    InvalidArguments(String, Vec<Type>),
}

pub struct TypeChecker {
    strict_mode: bool,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            strict_mode: false,
        }
    }

    pub fn new_strict() -> Self {
        Self {
            strict_mode: true,
        }
    }

    pub fn infer_type(&self, value: &Value) -> Type {
        match value {
            Value::Number(_) => Type::Number,
            Value::String(_) => Type::String,
            Value::Bool(_) => Type::Bool,
            Value::Null => Type::Null,
        }
    }

    pub fn check_binary_op(&self, left: &Type, right: &Type, op: &str) -> Result<Type, TypeError> {
        match (left, right, op) {
            // Operações numéricas
            (Type::Number, Type::Number, "+") |
            (Type::Number, Type::Number, "-") |
            (Type::Number, Type::Number, "*") |
            (Type::Number, Type::Number, "/") => Ok(Type::Number),
            
            // Concatenação de strings
            (Type::String, Type::String, "+") => Ok(Type::String),
            
            // Comparações
            (Type::Number, Type::Number, "==") |
            (Type::Number, Type::Number, "!=") |
            (Type::Number, Type::Number, "<") |
            (Type::Number, Type::Number, ">") |
            (Type::Number, Type::Number, "<=") |
            (Type::Number, Type::Number, ">=") => Ok(Type::Bool),
            
            (Type::String, Type::String, "==") |
            (Type::String, Type::String, "!=") => Ok(Type::Bool),
            
            // Coerção automática se não estiver em modo estrito
            (Type::Number, Type::String, "+") |
            (Type::String, Type::Number, "+") if !self.strict_mode => Ok(Type::String),
            
            _ => Err(TypeError::TypeMismatch(left.clone(), right.clone())),
        }
    }

    pub fn coerce(&self, value: &Value, target_type: &Type) -> Option<Value> {
        if self.strict_mode {
            return None;
        }

        match (value, target_type) {
            (Value::Number(n), Type::String) => Some(Value::String(n.to_string())),
            (Value::String(s), Type::Number) => {
                s.parse::<f64>().ok().map(Value::Number)
            }
            (Value::Bool(b), Type::String) => Some(Value::String(b.to_string())),
            (Value::String(s), Type::Bool) => {
                match s.as_str() {
                    "true" => Some(Value::Bool(true)),
                    "false" => Some(Value::Bool(false)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn check_statement(&self, stmt: &Stmt, env: &Env) -> Option<TypeError> {
        match stmt {
            Stmt::Let { name: _, value } => self.check_expression(value, env),
            Stmt::Expr(expr) => self.check_expression(expr, env),
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    if let Some(error) = self.check_statement(stmt, env) {
                        return Some(error);
                    }
                }
                None
            }
            Stmt::If { cond, then_branch, else_branch } => {
                if let Some(error) = self.check_expression(cond, env) {
                    return Some(error);
                }
                if let Some(error) = self.check_statement(then_branch, env) {
                    return Some(error);
                }
                if let Some(else_stmt) = else_branch {
                    return self.check_statement(else_stmt, env);
                }
                None
            }
            Stmt::While { cond, body } => {
                if let Some(error) = self.check_expression(cond, env) {
                    return Some(error);
                }
                self.check_statement(body, env)
            }
            Stmt::For { init, cond, post, body } => {
                if let Some(init_stmt) = init {
                    if let Some(error) = self.check_statement(init_stmt, env) {
                        return Some(error);
                    }
                }
                if let Some(cond_expr) = cond {
                    if let Some(error) = self.check_expression(cond_expr, env) {
                        return Some(error);
                    }
                }
                if let Some(post_expr) = post {
                    if let Some(error) = self.check_expression(post_expr, env) {
                        return Some(error);
                    }
                }
                self.check_statement(body, env)
            }
        }
    }

    pub fn check_expression(&self, expr: &Expr, env: &Env) -> Option<TypeError> {
        match expr {
            Expr::Number(_) | Expr::String(_) => None,
            Expr::Identifier(name) => {
                if env.get(name).is_none() {
                    Some(TypeError::UndefinedVariable(name.clone()))
                } else {
                    None
                }
            }
            Expr::Binary { left, op, right } => {
                if let Some(error) = self.check_expression(left, env) {
                    return Some(error);
                }
                if let Some(error) = self.check_expression(right, env) {
                    return Some(error);
                }
                
                let left_type = match self.infer_expression_type(left, env) {
                    Some(t) => t,
                    None => return Some(TypeError::InvalidOperation("Could not infer type".to_string(), Type::Any)),
                };
                let right_type = match self.infer_expression_type(right, env) {
                    Some(t) => t,
                    None => return Some(TypeError::InvalidOperation("Could not infer type".to_string(), Type::Any)),
                };
                
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                    BinaryOp::Equal => "==",
                    BinaryOp::NotEqual => "!=",
                    BinaryOp::Less => "<",
                    BinaryOp::Greater => ">",
                    BinaryOp::LessEqual => "<=",
                    BinaryOp::GreaterEqual => ">=",
                };
                
                self.check_binary_op(&left_type, &right_type, op_str).err()
            }
            Expr::Call { function, args: _ } => {
                // Por enquanto, assumimos que todas as funções são válidas
                // TODO: Implementar verificação de tipos para funções
                match function.as_str() {
                    "print" | "read_file" | "write_file" | "append_file" => None,
                    _ => Some(TypeError::UndefinedVariable(function.clone())),
                }
            }
        }
    }

    fn infer_expression_type(&self, expr: &Expr, env: &Env) -> Option<Type> {
        match expr {
            Expr::Number(_) => Some(Type::Number),
            Expr::String(_) => Some(Type::String),
            Expr::Identifier(name) => {
                env.get(name).map(|value| self.infer_type(&value))
            }
            Expr::Binary { left, op, right } => {
                let left_type = self.infer_expression_type(left, env)?;
                let right_type = self.infer_expression_type(right, env)?;
                
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                    BinaryOp::Equal => "==",
                    BinaryOp::NotEqual => "!=",
                    BinaryOp::Less => "<",
                    BinaryOp::Greater => ">",
                    BinaryOp::LessEqual => "<=",
                    BinaryOp::GreaterEqual => ">=",
                };
                
                self.check_binary_op(&left_type, &right_type, op_str).ok()
            }
            Expr::Call { function, args: _ } => {
                match function.as_str() {
                    "print" => Some(Type::Null),
                    "read_file" => Some(Type::String),
                    "write_file" | "append_file" => Some(Type::Null),
                    _ => None,
                }
            }
        }
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::TypeMismatch(expected, found) => {
                write!(f, "Type mismatch: expected {:?}, found {:?}", expected, found)
            }
            TypeError::InvalidOperation(op, type_) => {
                write!(f, "Invalid operation '{}' for type {:?}", op, type_)
            }
            TypeError::UndefinedVariable(name) => {
                write!(f, "Undefined variable: {}", name)
            }
            TypeError::InvalidArguments(func, types) => {
                write!(f, "Invalid arguments for function '{}': {:?}", func, types)
            }
        }
    }
}
