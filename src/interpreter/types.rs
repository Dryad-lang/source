// src/interpreter/types.rs

use crate::interpreter::env::{Value, Env};
use crate::parser::ast::{Expr, Stmt, BinaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Bool,
    Null,
    Class,
    Instance,
    Function,
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
            Value::Class(_) => Type::Class,
            Value::Instance(_) => Type::Instance,
            Value::Function { .. } => Type::Function,
            Value::Exception { .. } => Type::String, // Exceptions are treated as strings for type checking
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
            Stmt::Assign { name: _, value } => self.check_expression(value, env),
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
            Stmt::FunDecl { .. } => {
                // TODO: Implementar verificação de tipos para declarações de função
                None
            }
            Stmt::ClassDecl { .. } => {
                // TODO: Implementar verificação de tipos para declarações de classe
                None
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.check_expression(expr, env)
                } else {
                    None
                }
            }
            Stmt::NamespaceDecl { .. } => {
                // TODO: Implementar verificação de tipos para namespaces
                None
            }
            Stmt::Using { module_path: _, alias: _ } => {
                // Using statements don't need type checking by themselves
                None
            }
            Stmt::Export { item } => {
                // Check the exported item
                self.check_statement(item, env)
            }
            Stmt::Try { try_block, catch_block, .. } => {
                // Check both try and catch blocks
                if let Some(error) = self.check_statement(try_block, env) {
                    return Some(error);
                }
                self.check_statement(catch_block, env)
            }
            Stmt::Throw { value } => {
                // Check the expression being thrown
                self.check_expression(value, env)
            }
        }
    }

    pub fn check_expression(&self, expr: &Expr, env: &Env) -> Option<TypeError> {
        match expr {
            Expr::Number(_) | Expr::String(_) | Expr::Bool(_) | Expr::Null => None,
            Expr::Identifier(name) => {
                // Primeiro tenta busca normal, depois namespace path
                if env.get(name).is_none() && env.resolve_namespace_path(name).is_none() {
                    Some(TypeError::UndefinedVariable(name.clone()))
                } else {
                    None
                }
            }
            Expr::Unary { op: _, expr } => {
                // Verificar a expressão interna
                self.check_expression(expr, env)
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
                    BinaryOp::And => "&&",
                    BinaryOp::Or => "||",
                };
                
                self.check_binary_op(&left_type, &right_type, op_str).err()
            }
            Expr::Call { function, args: _ } => {
                // Por enquanto, assumimos que todas as funções são válidas
                // TODO: Implementar verificação de tipos para funções
                match function.as_str() {
                    "print" | "read_file" | "write_file" | "append_file" => None,
                    _ => {
                        // Verificar se é uma função ou classe válida (incluindo namespace paths)
                        // Ou se é um método em um objeto existente
                        if env.get(function).is_some() || env.resolve_namespace_path(function).is_some() {
                            None
                        } else if function.contains('.') {
                            // Pode ser um método - verificar se o objeto existe
                            let parts: Vec<&str> = function.split('.').collect();
                            if parts.len() == 2 && env.get(parts[0]).is_some() {
                                None // Objeto existe, assumir que método é válido
                            } else {
                                Some(TypeError::UndefinedVariable(function.clone()))
                            }
                        } else {
                            Some(TypeError::UndefinedVariable(function.clone()))
                        }
                    }
                }
            }
            Expr::This => {
                // TODO: Verificar se estamos em contexto de método
                None
            }
            Expr::Get { object, name: _ } => {
                // Verificar se o objeto é uma instância
                self.check_expression(object, env)
            }
            Expr::Set { object, name: _, value } => {
                // Verificar objeto e valor
                if let Some(error) = self.check_expression(object, env) {
                    return Some(error);
                }
                self.check_expression(value, env)
            }
            Expr::New { class: _, args } => {
                // Verificar argumentos
                for arg in args {
                    if let Some(error) = self.check_expression(arg, env) {
                        return Some(error);
                    }
                }
                None
            }
        }
    }

    fn infer_expression_type(&self, expr: &Expr, env: &Env) -> Option<Type> {
        match expr {
            Expr::Number(_) => Some(Type::Number),
            Expr::String(_) => Some(Type::String),
            Expr::Bool(_) => Some(Type::Bool),
            Expr::Null => Some(Type::Null),
            Expr::Unary { op: _, expr } => {
                // Operadores unários geralmente retornam bool ou o mesmo tipo da expressão
                match self.infer_expression_type(expr, env) {
                    Some(Type::Number) => Some(Type::Number), // para operador -
                    _ => Some(Type::Bool), // para operador !
                }
            }
            Expr::Identifier(name) => {
                if let Some(value) = env.get(name) {
                    Some(self.infer_type(&value))
                } else if let Some(value) = env.resolve_namespace_path(name) {
                    Some(self.infer_type(&value))
                } else {
                    None
                }
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
                    BinaryOp::And => "&&",
                    BinaryOp::Or => "||",
                };
                
                self.check_binary_op(&left_type, &right_type, op_str).ok()
            }
            Expr::Call { function, args: _ } => {
                match function.as_str() {
                    "print" => Some(Type::Null),
                    "read_file" => Some(Type::String),
                    "write_file" | "append_file" => Some(Type::Null),
                    _ => {
                        // Para funções/classes em namespace, retornar tipo baseado no valor
                        if let Some(value) = env.get(function).or_else(|| env.resolve_namespace_path(function)) {
                            match value {
                                Value::Function { .. } => Some(Type::Any), // Tipo de retorno da função
                                Value::Class(_) => Some(Type::Instance), // Instanciação de classe
                                _ => Some(Type::Any),
                            }
                        } else {
                            None
                        }
                    }
                }
            }
            Expr::This => {
                // TODO: Inferir tipo baseado no contexto da classe
                Some(Type::Instance)
            }
            Expr::Get { object: _, name: _ } => {
                // TODO: Inferir tipo baseado no campo acessado
                Some(Type::Any)
            }
            Expr::Set { object: _, name: _, value } => {
                self.infer_expression_type(value, env)
            }
            Expr::New { class: _, args: _ } => {
                Some(Type::Instance)
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
