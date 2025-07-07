// src/interpreter/evaluator.rs

use crate::parser::ast::{Expr, Stmt, BinaryOp};
use crate::interpreter::env::{Env, Value};
use crate::interpreter::io;
use crate::lexer::tokenizer::Lexer;
use crate::parser::parser::Parser;

pub fn eval_stmt(stmt: &Stmt, env: &mut Env) -> Option<Value> {
    match stmt {
        Stmt::Let { name, value } => {
            let val = eval_expr(value, env)?;
            env.set(name.clone(), val.clone());
            Some(val)
        }
        Stmt::Expr(expr) => eval_expr(expr, env),
        Stmt::Block(stmts) => {
            let mut result = None;
            for s in stmts {
                result = eval_stmt(s, env);
            }
            result
        }
        Stmt::If { cond, then_branch, else_branch } => {
            let val = eval_expr(cond, env)?;
            if val.is_truthy() {
                eval_stmt(then_branch, env)
            } else if let Some(else_b) = else_branch {
                eval_stmt(else_b, env)
            } else {
                None
            }
        }
        Stmt::While { cond, body } => {
            while eval_expr(cond, env)?.is_truthy() {
                eval_stmt(body, env);
            }
            None
        }
        Stmt::For { init, cond, post, body } => {
            if let Some(init_stmt) = init {
                eval_stmt(init_stmt, env);
            }

            while cond.as_ref().map(|c| eval_expr(c, env)?.is_truthy()).unwrap_or(true) {
                eval_stmt(body, env);
                if let Some(post_expr) = post {
                    eval_expr(post_expr, env);
                }
            }
            None
        }
        _ => {
            // Não implementado ainda
            None
        }
    }
}

pub fn eval_expr(expr: &Expr, env: &mut Env) -> Option<Value> {
    match expr {
        Expr::Number(n) => Some(Value::Number(*n)),
        Expr::String(s) => Some(Value::String(s.clone())),
        Expr::Identifier(name) => env.get(name),
        Expr::Binary { left, op, right } => {
            let lval = eval_expr(left, env)?;
            let rval = eval_expr(right, env)?;
            match (lval, rval, op) {
                (Value::Number(a), Value::Number(b), BinaryOp::Add) => Some(Value::Number(a + b)),
                (Value::Number(a), Value::Number(b), BinaryOp::Sub) => Some(Value::Number(a - b)),
                (Value::Number(a), Value::Number(b), BinaryOp::Mul) => Some(Value::Number(a * b)),
                (Value::Number(a), Value::Number(b), BinaryOp::Div) => Some(Value::Number(a / b)),
                (Value::Number(a), Value::Number(b), BinaryOp::Equal) => Some(Value::Bool(a == b)),
                (Value::Number(a), Value::Number(b), BinaryOp::NotEqual) => Some(Value::Bool(a != b)),
                (Value::Number(a), Value::Number(b), BinaryOp::Less) => Some(Value::Bool(a < b)),
                (Value::Number(a), Value::Number(b), BinaryOp::LessEqual) => Some(Value::Bool(a <= b)),
                (Value::Number(a), Value::Number(b), BinaryOp::Greater) => Some(Value::Bool(a > b)),
                (Value::Number(a), Value::Number(b), BinaryOp::GreaterEqual) => Some(Value::Bool(a >= b)),
                _ => Some(Value::Null),
            }
        }
        Expr::Call { function, args } => match function.as_str() {
            "print" => {
                for arg in args {
                    if let Some(value) = eval_expr(arg, env) {
                        match value {
                            Value::Number(n) => print!("{}", n),
                            Value::String(s) => print!("{}", s),
                            Value::Bool(b) => print!("{}", b),
                            Value::Null => print!("null"),
                        }
                    }
                }
                println!(); // Nova linha após print
                Some(Value::Null)
            }
            "read_file" => {
                if let Some(filename_arg) = args.get(0) {
                    if let Some(filename_value) = eval_expr(filename_arg, env) {
                        if let Value::String(filename) = filename_value {
                            match io::read_file(&filename) {
                                Ok(content) => Some(Value::String(content)),
                                Err(_) => {
                                    println!("Erro ao ler arquivo: {}", filename);
                                    Some(Value::Null)
                                }
                            }
                        } else {
                            println!("read_file: argumento deve ser uma string");
                            Some(Value::Null)
                        }
                    } else {
                        Some(Value::Null)
                    }
                } else {
                    println!("read_file: nome do arquivo requerido");
                    Some(Value::Null)
                }
            }
            "write_file" => {
                if let (Some(filename_arg), Some(content_arg)) = (args.get(0), args.get(1)) {
                    if let (Some(filename_value), Some(content_value)) = 
                        (eval_expr(filename_arg, env), eval_expr(content_arg, env)) {
                        if let (Value::String(filename), Value::String(content)) = 
                            (filename_value, content_value) {
                            match io::write_file(&filename, &content) {
                                Ok(_) => Some(Value::Null),
                                Err(_) => {
                                    println!("Erro ao escrever arquivo: {}", filename);
                                    Some(Value::Null)
                                }
                            }
                        } else {
                            println!("write_file: argumentos devem ser strings");
                            Some(Value::Null)
                        }
                    } else {
                        Some(Value::Null)
                    }
                } else {
                    println!("write_file: nome do arquivo e conteúdo requeridos");
                    Some(Value::Null)
                }
            }
            "append_file" => {
                if let (Some(filename_arg), Some(content_arg)) = (args.get(0), args.get(1)) {
                    if let (Some(filename_value), Some(content_value)) = 
                        (eval_expr(filename_arg, env), eval_expr(content_arg, env)) {
                        if let (Value::String(filename), Value::String(content)) = 
                            (filename_value, content_value) {
                            match io::append_file(&filename, &content) {
                                Ok(_) => Some(Value::Null),
                                Err(_) => {
                                    println!("Erro ao anexar ao arquivo: {}", filename);
                                    Some(Value::Null)
                                }
                            }
                        } else {
                            println!("append_file: argumentos devem ser strings");
                            Some(Value::Null)
                        }
                    } else {
                        Some(Value::Null)
                    }
                } else {
                    println!("append_file: nome do arquivo e conteúdo requeridos");
                    Some(Value::Null)
                }
            }
            _ => {
                println!("Função '{}' não implementada", function);
                Some(Value::Null)
            }
        }
    }
}
