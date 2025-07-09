// src/interpreter/evaluator.rs

use crate::parser::ast::{Expr, Stmt, BinaryOp, UnaryOp};
use crate::interpreter::env::{Env, Value, Class, Instance};
use crate::interpreter::types::TypeChecker;
use crate::interpreter::errors::{DryadError, ErrorReporter, ErrorSeverity};
use crate::interpreter::module_loader::ModuleLoader;
use crate::interpreter::native::NativeRegistry;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct EvaluationResult {
    pub value: Option<Value>,
    pub errors: Vec<DryadError>,
    pub exception: Option<Value>,
}

impl EvaluationResult {
    pub fn new(value: Option<Value>) -> Self {
        Self {
            value,
            errors: Vec::new(),
            exception: None,
        }
    }
    
    pub fn with_error(error: DryadError) -> Self {
        Self {
            value: None,
            errors: vec![error],
            exception: None,
        }
    }
    
    pub fn with_exception(exception: Value) -> Self {
        Self {
            value: None,
            errors: Vec::new(),
            exception: Some(exception),
        }
    }
    
    pub fn add_error(&mut self, error: DryadError) {
        self.errors.push(error);
    }
    
    pub fn is_exception(&self) -> bool {
        self.exception.is_some()
    }
}

pub struct Evaluator {
    type_checker: TypeChecker,
    error_reporter: ErrorReporter,
    module_loader: ModuleLoader,
    native_registry: NativeRegistry,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            type_checker: TypeChecker::new(),
            error_reporter: ErrorReporter::new(),
            module_loader: ModuleLoader::new(),
            native_registry: NativeRegistry::new(),
        }
    }
    
    pub fn eval_stmt(&mut self, stmt: &Stmt, env: &mut Env) -> EvaluationResult {
        // Verificação de tipos desabilitada temporariamente para funções nativas
        // if let Some(type_error) = self.type_checker.check_statement(stmt, env) {
        //     let error = DryadError::new(
        //         format!("Erro de tipo: {:?}", type_error),
        //         Some((0, 0)), // TODO: Adicionar posições reais
        //         ErrorSeverity::Error,
        //     );
        //     return EvaluationResult::with_error(error);
        // }
        
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
            Stmt::Assign { name, value } => {
                // Reatribuição - a variável deve já existir
                let result = self.eval_expr(value, env);
                if let Some(val) = result.value {
                    env.set(name.clone(), val.clone()); // Atualizar valor existente
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
                    
                    // If there's an exception, propagate it immediately
                    if result.is_exception() {
                        final_result.exception = result.exception;
                        final_result.errors.extend(result.errors);
                        return final_result;
                    }
                    
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
                
                // Criar um novo escopo para o loop for
                let mut loop_env = env.clone();
                
                if let Some(init_stmt) = init {
                    let init_result = self.eval_stmt(init_stmt, &mut loop_env);
                    result.errors.extend(init_result.errors);
                }

                loop {
                    let should_continue = if let Some(cond_expr) = cond {
                        let cond_result = self.eval_expr(cond_expr, &mut loop_env);
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

                    let body_result = self.eval_stmt(body, &mut loop_env);
                    result.errors.extend(body_result.errors);
                    
                    if let Some(post_expr) = post {
                        let post_result = self.eval_expr(post_expr, &mut loop_env);
                        result.errors.extend(post_result.errors);
                    }
                }
                result
            }
            Stmt::ClassDecl { name, methods, fields, visibility: _ } => {
                let mut class = Class::new(name.clone(), fields.clone());
                
                // Adicionar métodos à classe
                for method in methods {
                    if let Stmt::FunDecl { name: method_name, params, body, visibility, is_static } = method {
                        // Validate static methods don't access instance variables
                        if *is_static {
                            let field_names: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();
                            if let Err(validation_error) = self.validate_static_method(body, &field_names) {
                                return EvaluationResult::with_error(validation_error);
                            }
                        }
                        
                        let method_value = Value::Function {
                            name: method_name.clone(),
                            params: params.clone(),
                            body: (**body).clone(),
                            visibility: visibility.clone(),
                            is_static: *is_static,
                        };
                        
                        if *is_static {
                            class.add_static_method(method_name.clone(), method_value);
                        } else {
                            class.add_method(method_name.clone(), method_value);
                        }
                    }
                }
                
                let class_value = Value::Class(Rc::new(class));
                env.set(name.clone(), class_value.clone());
                EvaluationResult::new(Some(class_value))
            }
            
            Stmt::FunDecl { name, params, body, visibility, is_static } => {
                let func_value = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: (**body).clone(),
                    visibility: visibility.clone(),
                    is_static: *is_static,
                };
                env.set(name.clone(), func_value.clone());
                EvaluationResult::new(Some(func_value))
            }
            
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.eval_expr(expr, env)
                } else {
                    EvaluationResult::new(Some(Value::Null))
                }
            }
            
            Stmt::NamespaceDecl { name, body } => {
                // Processar declarações dentro do namespace
                let mut final_result = EvaluationResult::new(None);
                
                for stmt in body {
                    match stmt {
                        Stmt::ClassDecl { name: class_name, methods, fields, visibility: _ } => {
                            // Criar classe com nome completo incluindo namespace
                            let full_class_name = format!("{}.{}", name, class_name);
                            
                            let mut class = Class::new(full_class_name.clone(), fields.clone());
                            
                            // Processar métodos da classe
                            for method in methods {
                                if let Stmt::FunDecl { name: method_name, params, body, visibility, is_static } = method {
                                    let method_value = Value::Function {
                                        name: method_name.clone(),
                                        params: params.clone(),
                                        body: *body.clone(),
                                        visibility: visibility.clone(),
                                        is_static: *is_static,
                                    };
                                    
                                    if *is_static {
                                        class.add_static_method(method_name.clone(), method_value);
                                    } else {
                                        class.add_method(method_name.clone(), method_value);
                                    }
                                }
                            }
                            
                            let class_value = Value::Class(Rc::new(class));
                            env.define_in_namespace(name, class_name, class_value);
                        }
                        
                        Stmt::FunDecl { name: func_name, params, body, visibility, is_static } => {
                            // Criar função com nome completo incluindo namespace
                            let func_value = Value::Function {
                                name: format!("{}.{}", name, func_name),
                                params: params.clone(),
                                body: *body.clone(),
                                visibility: visibility.clone(),
                                is_static: *is_static,
                            };
                            env.define_in_namespace(name, func_name, func_value);
                        }
                        
                        Stmt::Export { item } => {
                            // Process export within namespace
                            let export_result = self.eval_stmt(item, env);
                            final_result.errors.extend(export_result.errors);
                            
                            // Extract exported item and add to namespace
                            match item.as_ref() {
                                Stmt::FunDecl { name: func_name, params, body, visibility, is_static } => {
                                    let func_value = Value::Function {
                                        name: format!("{}.{}", name, func_name),
                                        params: params.clone(),
                                        body: *body.clone(),
                                        visibility: visibility.clone(),
                                        is_static: *is_static,
                                    };
                                    env.define_in_namespace(name, func_name, func_value.clone());
                                    env.export_item(format!("{}.{}", name, func_name), func_value);
                                }
                                Stmt::ClassDecl { name: class_name, methods, fields, visibility: _ } => {
                                    let full_class_name = format!("{}.{}", name, class_name);
                                    let mut class = Class::new(full_class_name.clone(), fields.clone());
                                    
                                    for method in methods {
                                        if let Stmt::FunDecl { name: method_name, params, body, visibility, is_static } = method {
                                            let method_value = Value::Function {
                                                name: method_name.clone(),
                                                params: params.clone(),
                                                body: *body.clone(),
                                                visibility: visibility.clone(),
                                                is_static: *is_static,
                                            };
                                            
                                            if *is_static {
                                                class.add_static_method(method_name.clone(), method_value);
                                            } else {
                                                class.add_method(method_name.clone(), method_value);
                                            }
                                        }
                                    }
                                    
                                    let class_value = Value::Class(Rc::new(class));
                                    env.define_in_namespace(name, class_name, class_value.clone());
                                    env.export_item(format!("{}.{}", name, class_name), class_value);
                                }
                                _ => {}
                            }
                        }
                        
                        _ => {
                            // Para outros tipos de statement, executar normalmente
                            let result = self.eval_stmt(stmt, env);
                            final_result.errors.extend(result.errors);
                        }
                    }
                }
                
                final_result
            }
            
            Stmt::Using { module_path, alias } => {
                // Try to load external module first
                let module_result = {
                    self.module_loader.load_module(module_path)
                };
                
                match module_result {
                    Ok(statements) => {
                        // Execute module statements in a new environment
                        let mut module_env = Env::new();
                        for stmt in &statements {
                            let result = self.eval_stmt(stmt, &mut module_env);
                            if !result.errors.is_empty() {
                                return result;
                            }
                        }
                        
                        // Import all exported items from the module
                        let exported_items = module_env.get_all_exported();
                        for (name, value) in &exported_items {
                            let import_name = if let Some(alias_name) = alias {
                                format!("{}.{}", alias_name, name.split('.').last().unwrap_or(name))
                            } else {
                                name.clone()
                            };
                            env.set(import_name, value.clone());
                        }
                        
                        // Import namespace items directly (for classes like Text.JSON)
                        let all_variables = module_env.get_all_variables();
                        for (var_name, var_value) in &all_variables {
                            if var_name.contains('.') {
                                // Import full namespace path (e.g., Text.JSON)
                                env.set(var_name.clone(), var_value.clone());
                                
                                // Also import just the class name if no alias specified
                                if alias.is_none() {
                                    let parts: Vec<&str> = var_name.split('.').collect();
                                    if parts.len() >= 2 {
                                        let class_name = parts.last().unwrap();
                                        env.set(class_name.to_string(), var_value.clone());
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // Fallback to namespace alias (for same-file namespaces)
                        if let Some(alias_name) = alias {
                            env.add_alias(alias_name.clone(), module_path.clone());
                        } else {
                            let parts: Vec<&str> = module_path.split('.').collect();
                            if let Some(last_part) = parts.last() {
                                env.add_alias(last_part.to_string(), module_path.clone());
                            }
                        }
                    }
                }
                
                EvaluationResult::new(Some(Value::Null))
            }

            Stmt::Use { file_path } => {
                // Load file relative to current script
                let module_result = {
                    self.module_loader.load_file(file_path)
                };
                
                match module_result {
                    Ok(statements) => {
                        // Execute module statements in current environment
                        let mut module_env = Env::new();
                        for stmt in &statements {
                            let result = self.eval_stmt(stmt, &mut module_env);
                            if !result.errors.is_empty() {
                                return result;
                            }
                        }
                        
                        // Import all exported items from the module directly into current scope
                        let exported_items = module_env.get_all_exported();
                        for (name, value) in &exported_items {
                            env.set(name.clone(), value.clone());
                        }
                    }
                    Err(e) => {
                        let error = DryadError::new(
                            format!("Failed to load file '{}': {}", file_path, e),
                            None,
                            crate::interpreter::errors::ErrorSeverity::Error,
                        );
                        return EvaluationResult::with_error(error);
                    }
                }
                
                EvaluationResult::new(Some(Value::Null))
            }
            
            Stmt::Export { item } => {
                // Evaluate the item and export it
                let result = self.eval_stmt(item, env);
                
                // Extract the name and value for export
                match item.as_ref() {
                    Stmt::FunDecl { name, .. } => {
                        if let Some(value) = env.get(name) {
                            env.export_item(name.clone(), value);
                        }
                    }
                    Stmt::ClassDecl { name, .. } => {
                        if let Some(value) = env.get(name) {
                            env.export_item(name.clone(), value);
                        }
                    }
                    Stmt::NamespaceDecl { name, .. } => {
                        // Export all items from namespace
                        // This is simplified - in practice, you'd need to track namespace contents
                        env.export_item(name.clone(), Value::Null);
                    }
                    _ => {
                        // Other statements can't be exported
                    }
                }
                
                result
            }
            
            Stmt::Try { try_block, catch_param, catch_block } => {
                // Execute try block
                let try_result = self.eval_stmt(try_block, env);
                
                // If there's an exception, handle it with catch block
                if try_result.is_exception() {
                    if let Some(exception) = try_result.exception {
                        // Create new environment for catch block with exception parameter
                        if let Some(param_name) = catch_param {
                            env.set(param_name.clone(), exception);
                        }
                        
                        // Execute catch block
                        let catch_result = self.eval_stmt(catch_block, env);
                        
                        // Clean up exception parameter if it was set
                        if let Some(param_name) = catch_param {
                            env.remove(param_name);
                        }
                        
                        catch_result
                    } else {
                        try_result
                    }
                } else {
                    try_result
                }
            }
            
            Stmt::Throw { value } => {
                // Evaluate the expression to throw
                let eval_result = self.eval_expr(value, env);
                
                if let Some(throw_value) = eval_result.value {
                    // Create exception
                    let exception = match throw_value {
                        Value::String(msg) => Value::Exception {
                            message: msg,
                            value: None,
                        },
                        other => Value::Exception {
                            message: format!("{:?}", other),
                            value: Some(Box::new(other)),
                        },
                    };
                    
                    EvaluationResult::with_exception(exception)
                } else {
                    // If evaluation failed, return that error
                    eval_result
                }
            }
            
            // ...existing code...
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr, env: &mut Env) -> EvaluationResult {
        // Verificação de tipos desabilitada temporariamente para funções nativas
        // if let Some(type_error) = self.type_checker.check_expression(expr, env) {
        //     let error = DryadError::new(
        //         format!("Erro de tipo: {:?}", type_error),
        //         Some((0, 0)), // TODO: Adicionar posições reais
        //         ErrorSeverity::Error,
        //     );
        //     return EvaluationResult::with_error(error);
        // }
        
        match expr {
            Expr::Number(n) => EvaluationResult::new(Some(Value::Number(*n))),
            Expr::String(s) => EvaluationResult::new(Some(Value::String(s.clone()))),
            Expr::Bool(b) => EvaluationResult::new(Some(Value::Bool(*b))),
            Expr::Null => EvaluationResult::new(Some(Value::Null)),
            Expr::Identifier(name) => {
                // Primeiro tenta buscar normalmente
                if let Some(value) = env.get(name) {
                    EvaluationResult::new(Some(value))
                } else if self.native_registry.is_native(name) {
                    // Se é uma função nativa, criar um placeholder para chamadas
                    EvaluationResult::new(Some(Value::Function {
                        name: name.clone(),
                        params: vec![], // Native functions handle their own params
                        body: Stmt::Block(vec![]), // Empty body for native functions
                        visibility: crate::parser::ast::Visibility::Public,
                        is_static: false, // Native functions are not static by default
                    }))
                } else if name.contains('.') {
                    // Se contém ponto, pode ser um caminho de namespace ou alias
                    if let Some(value) = env.resolve_with_alias(name) {
                        EvaluationResult::new(Some(value))
                    } else if let Some(value) = env.resolve_namespace_path(name) {
                        EvaluationResult::new(Some(value))
                    } else {
                        let error = DryadError::new(
                            format!("Caminho '{}' não encontrado", name),
                            Some((0, 0)), // TODO: Adicionar posições reais
                            ErrorSeverity::Error,
                        );
                        EvaluationResult::with_error(error)
                    }
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
                        (Value::String(a), Value::String(b), BinaryOp::Add) => 
                            EvaluationResult::new(Some(Value::String(a + &b))),
                        // Concatenação string + número
                        (Value::String(a), Value::Number(b), BinaryOp::Add) => 
                            EvaluationResult::new(Some(Value::String(a + &b.to_string()))),
                        // Concatenação número + string
                        (Value::Number(a), Value::String(b), BinaryOp::Add) => 
                            EvaluationResult::new(Some(Value::String(a.to_string() + &b))),
                        // Concatenação string + bool
                        (Value::String(a), Value::Bool(b), BinaryOp::Add) => 
                            EvaluationResult::new(Some(Value::String(a + &b.to_string()))),
                        // Concatenação bool + string
                        (Value::Bool(a), Value::String(b), BinaryOp::Add) => 
                            EvaluationResult::new(Some(Value::String(a.to_string() + &b))),
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
                        
                        // Operadores lógicos
                        (a, b, BinaryOp::And) => {
                            if a.is_truthy() && b.is_truthy() {
                                EvaluationResult::new(Some(Value::Bool(true)))
                            } else {
                                EvaluationResult::new(Some(Value::Bool(false)))
                            }
                        }
                        (a, b, BinaryOp::Or) => {
                            if a.is_truthy() || b.is_truthy() {
                                EvaluationResult::new(Some(Value::Bool(true)))
                            } else {
                                EvaluationResult::new(Some(Value::Bool(false)))
                            }
                        }
                        
                        // Comparações de string
                        (Value::String(a), Value::String(b), BinaryOp::Equal) => 
                            EvaluationResult::new(Some(Value::Bool(a == b))),
                        (Value::String(a), Value::String(b), BinaryOp::NotEqual) => 
                            EvaluationResult::new(Some(Value::Bool(a != b))),
                        
                        // Comparações de bool
                        (Value::Bool(a), Value::Bool(b), BinaryOp::Equal) => 
                            EvaluationResult::new(Some(Value::Bool(a == b))),
                        (Value::Bool(a), Value::Bool(b), BinaryOp::NotEqual) => 
                            EvaluationResult::new(Some(Value::Bool(a != b))),
                        
                        // Comparações mistas
                        (Value::Null, Value::Null, BinaryOp::Equal) => 
                            EvaluationResult::new(Some(Value::Bool(true))),
                        (Value::Null, _, BinaryOp::Equal) | (_, Value::Null, BinaryOp::Equal) => 
                            EvaluationResult::new(Some(Value::Bool(false))),
                        (Value::Null, Value::Null, BinaryOp::NotEqual) => 
                            EvaluationResult::new(Some(Value::Bool(false))),
                        (Value::Null, _, BinaryOp::NotEqual) | (_, Value::Null, BinaryOp::NotEqual) => 
                            EvaluationResult::new(Some(Value::Bool(true))),
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
            Expr::Unary { op, expr } => {
                let expr_result = self.eval_expr(expr, env);
                if !expr_result.errors.is_empty() {
                    return expr_result;
                }
                
                if let Some(value) = expr_result.value {
                    match (op, value) {
                        (UnaryOp::Not, val) => {
                            EvaluationResult::new(Some(Value::Bool(!val.is_truthy())))
                        }
                        (UnaryOp::Minus, Value::Number(n)) => {
                            EvaluationResult::new(Some(Value::Number(-n)))
                        }
                        (UnaryOp::Minus, _) => {
                            let error = DryadError::new(
                                "Operador unário '-' só pode ser aplicado a números".to_string(),
                                Some((0, 0)),
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
            Expr::This => {
                if let Some(this_value) = env.get("this") {
                    EvaluationResult::new(Some(this_value))
                } else {
                    let error = DryadError::new(
                        "Uso de 'this' fora de um contexto de método".to_string(),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    );
                    EvaluationResult::with_error(error)
                }
            }
            
            Expr::New { class, args } => {
                if let Some(Value::Class(class_ref)) = env.get(class) {
                    let instance = Instance::new(class_ref.clone());
                    let instance_value = Value::Instance(Rc::new(RefCell::new(instance)));
                    
                    // Verificar se existe método init e chamá-lo
                    if let Some(init_method) = class_ref.get_method("init") {
                        let mut method_env = Env::with_this(instance_value.clone());
                        
                        // Avaliar argumentos
                        let mut arg_values = Vec::new();
                        for arg in args {
                            let arg_result = self.eval_expr(arg, env);
                            if !arg_result.errors.is_empty() {
                                return arg_result;
                            }
                            if let Some(val) = arg_result.value {
                                arg_values.push(val);
                            }
                        }
                        
                        // Chamar método init se existir
                        if let Value::Function { params, body, .. } = init_method {
                            // Bind parâmetros
                            for (i, param) in params.iter().enumerate() {
                                if let Some(arg_val) = arg_values.get(i) {
                                    method_env.set(param.clone(), arg_val.clone());
                                }
                            }
                            
                            // Executar corpo do método init
                            let _init_result = self.eval_stmt(&body, &mut method_env);
                        }
                    }
                    
                    EvaluationResult::new(Some(instance_value))
                } else {
                    let error = DryadError::new(
                        format!("Classe '{}' não definida", class),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    );
                    EvaluationResult::with_error(error)
                }
            }
            
            Expr::Get { object, name } => {
                let obj_result = self.eval_expr(object, env);
                if !obj_result.errors.is_empty() {
                    return obj_result;
                }
                
                if let Some(Value::Instance(instance_ref)) = obj_result.value {
                    let instance = instance_ref.borrow();
                    
                    // Primeiro tenta acessar campo
                    if let Some(field_value) = instance.get_field(name) {
                        EvaluationResult::new(Some(field_value))
                    } else if let Some(method) = instance.get_method(name) {
                        // Retorna método bound à instância
                        EvaluationResult::new(Some(method))
                    } else {
                        let error = DryadError::new(
                            format!("Propriedade '{}' não encontrada", name),
                            Some((0, 0)),
                            ErrorSeverity::Error,
                        );
                        EvaluationResult::with_error(error)
                    }
                } else {
                    let error = DryadError::new(
                        "Tentativa de acessar propriedade em valor não-objeto".to_string(),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    );
                    EvaluationResult::with_error(error)
                }
            }
            
            Expr::Set { object, name, value } => {
                let obj_result = self.eval_expr(object, env);
                if !obj_result.errors.is_empty() {
                    return obj_result;
                }
                
                let val_result = self.eval_expr(value, env);
                if !val_result.errors.is_empty() {
                    return val_result;
                }
                
                if let (Some(Value::Instance(instance_ref)), Some(new_value)) = 
                    (obj_result.value, val_result.value) {
                    let mut instance = instance_ref.borrow_mut();
                    instance.set_field(name.clone(), new_value.clone());
                    EvaluationResult::new(Some(new_value))
                } else {
                    let error = DryadError::new(
                        "Tentativa de definir propriedade em valor não-objeto".to_string(),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    );
                    EvaluationResult::with_error(error)
                }
            }
            
            Expr::Assign { name, value } => {
                let val_result = self.eval_expr(value, env);
                if !val_result.errors.is_empty() {
                    return val_result;
                }
                
                if let Some(val) = val_result.value {
                    env.set(name.clone(), val.clone());
                    EvaluationResult::new(Some(val))
                } else {
                    EvaluationResult::new(Some(Value::Null))
                }
            }
            
            // ...existing code...
        }
    }
    
    fn eval_function_call(&mut self, function: &str, args: &[Expr], env: &mut Env) -> EvaluationResult {
        // Verificar se é uma chamada com pontos (obj.method ou namespace.class.method)
        if function.contains('.') {
            let parts: Vec<&str> = function.split('.').collect();
            
            // Caso de 3 partes: namespace.class.method
            if parts.len() == 3 {
                let namespace_name = parts[0];
                let class_name = parts[1];
                let method_name = parts[2];
                
                // Tentar buscar a classe no namespace
                let full_class_name = format!("{}.{}", namespace_name, class_name);
                if let Some(class_value) = env.get(&full_class_name) {
                    if let Value::Class(class_ref) = class_value {
                        if let Some(static_method) = class_ref.get_static_method(method_name) {
                            // Avaliar argumentos
                            let mut arg_values = Vec::new();
                            for arg in args {
                                let arg_result = self.eval_expr(arg, env);
                                if !arg_result.errors.is_empty() {
                                    return arg_result;
                                }
                                if let Some(val) = arg_result.value {
                                    arg_values.push(val);
                                }
                            }
                            
                            // Chamar método estático
                            if let Value::Function { params, body, is_static, .. } = static_method {
                                if !is_static {
                                    let error = DryadError::new(
                                        format!("Método '{}' não é estático", method_name),
                                        Some((0, 0)),
                                        ErrorSeverity::Error,
                                    );
                                    return EvaluationResult::with_error(error);
                                }
                                
                                let mut method_env = Env::new(); // Sem this binding
                                
                                // Bind parâmetros
                                for (i, param) in params.iter().enumerate() {
                                    if let Some(arg_val) = arg_values.get(i) {
                                        method_env.set(param.clone(), arg_val.clone());
                                    }
                                }
                                
                                // Executar corpo do método
                                return self.eval_stmt(&body, &mut method_env);
                            }
                        }
                        
                        let error = DryadError::new(
                            format!("Método estático '{}' não encontrado na classe '{}.{}'", method_name, namespace_name, class_name),
                            Some((0, 0)),
                            ErrorSeverity::Error,
                        );
                        return EvaluationResult::with_error(error);
                    }
                }
                
                let error = DryadError::new(
                    format!("Classe '{}.{}' não encontrada", namespace_name, class_name),
                    Some((0, 0)),
                    ErrorSeverity::Error,
                );
                return EvaluationResult::with_error(error);
            }
            
            // Caso de 2 partes: obj.method
            if parts.len() == 2 {
                let obj_name = parts[0];
                let method_name = parts[1];
                
                // Verificar se obj_name é uma variável (não um namespace)
                if let Some(obj_value) = env.get(obj_name) {
                    match obj_value {
                        // Chamada de método estático em uma classe
                        Value::Class(class_ref) => {
                            if let Some(static_method) = class_ref.get_static_method(method_name) {
                                // Avaliar argumentos
                                let mut arg_values = Vec::new();
                                for arg in args {
                                    let arg_result = self.eval_expr(arg, env);
                                    if !arg_result.errors.is_empty() {
                                        return arg_result;
                                    }
                                    if let Some(val) = arg_result.value {
                                        arg_values.push(val);
                                    }
                                }
                                
                                // Chamar método estático (sem this binding)
                                if let Value::Function { params, body, is_static, .. } = static_method {
                                    if !is_static {
                                        let error = DryadError::new(
                                            format!("Método '{}' não é estático", method_name),
                                            Some((0, 0)),
                                            ErrorSeverity::Error,
                                        );
                                        return EvaluationResult::with_error(error);
                                    }
                                    
                                    let mut method_env = Env::new(); // Sem this binding
                                    
                                    // Bind parâmetros
                                    for (i, param) in params.iter().enumerate() {
                                        if let Some(arg_val) = arg_values.get(i) {
                                            method_env.set(param.clone(), arg_val.clone());
                                        }
                                    }
                                    
                                    // Executar corpo do método
                                    return self.eval_stmt(&body, &mut method_env);
                                }
                            }
                            
                            let error = DryadError::new(
                                format!("Método estático '{}' não encontrado na classe '{}'", method_name, obj_name),
                                Some((0, 0)),
                                ErrorSeverity::Error,
                            );
                            return EvaluationResult::with_error(error);
                        }
                        
                        // Chamada de método em uma instância
                        Value::Instance(instance_ref) => {
                            let instance = instance_ref.borrow();
                            
                            if let Some(method_value) = instance.get_method(method_name) {
                                // Avaliar argumentos
                                let mut arg_values = Vec::new();
                                for arg in args {
                                    let arg_result = self.eval_expr(arg, env);
                                    if !arg_result.errors.is_empty() {
                                        return arg_result;
                                    }
                                    if let Some(val) = arg_result.value {
                                        arg_values.push(val);
                                    }
                                }
                                
                                // Chamar método com this binding
                                if let Value::Function { params, body, is_static, .. } = method_value {
                                    if is_static {
                                        let error = DryadError::new(
                                            format!("Não é possível chamar método estático '{}' em uma instância", method_name),
                                            Some((0, 0)),
                                            ErrorSeverity::Error,
                                        );
                                        return EvaluationResult::with_error(error);
                                    }
                                    
                                    let mut method_env = Env::with_this(Value::Instance(instance_ref.clone()));
                                    
                                    // Bind parâmetros
                                    for (i, param) in params.iter().enumerate() {
                                        if let Some(arg_val) = arg_values.get(i) {
                                            method_env.set(param.clone(), arg_val.clone());
                                        }
                                    }
                                    
                                    // Executar corpo do método
                                    return self.eval_stmt(&body, &mut method_env);
                                }
                            } else {
                                let error = DryadError::new(
                                    format!("Método '{}' não encontrado", method_name),
                                    Some((0, 0)),
                                    ErrorSeverity::Error,
                                );
                                return EvaluationResult::with_error(error);
                            }
                        }
                        
                        _ => {
                            // Para outros tipos, tratar como antes
                        }
                    }
                }
            }
            
            // Se não é um método em variável, tentar resolver como namespace ou alias
            if let Some(func_value) = env.resolve_with_alias(function) {
                return self.call_resolved_function(func_value, args, env);
            }
            
            if let Some(func_value) = env.resolve_namespace_path(function) {
                return self.call_resolved_function(func_value, args, env);
            }
        }
        
        // Evaluate arguments first
        let mut arg_values = Vec::new();
        for arg in args {
            let arg_result = self.eval_expr(arg, env);
            if !arg_result.errors.is_empty() {
                return arg_result;
            }
            if let Some(val) = arg_result.value {
                arg_values.push(val);
            }
        }
        
        // Check if it's a native function first
        if self.native_registry.is_native(function) {
            match self.native_registry.call(function, &arg_values) {
                Some(Ok(value)) => return EvaluationResult::new(Some(value)),
                Some(Err(error)) => return EvaluationResult::with_error(error),
                None => {} // Fall through to legacy handling
            }
        }
        
        // Legacy built-in functions (for backward compatibility)
        match function {
            "print" => {
                // Redirect to native function
                match self.native_registry.call("native_console_println", &arg_values) {
                    Some(Ok(value)) => EvaluationResult::new(Some(value)),
                    Some(Err(error)) => EvaluationResult::with_error(error),
                    None => {
                        let error = DryadError::new(
                            "Função 'print' não encontrada".to_string(),
                            Some((0, 0)),
                            ErrorSeverity::Error,
                        );
                        EvaluationResult::with_error(error)
                    }
                }
            }
            "read_file" => {
                // Redirect to native function
                match self.native_registry.call("native_fs_read_file", &arg_values) {
                    Some(Ok(value)) => EvaluationResult::new(Some(value)),
                    Some(Err(error)) => EvaluationResult::with_error(error),
                    None => {
                        let error = DryadError::new(
                            "Função 'read_file' não encontrada".to_string(),
                            Some((0, 0)),
                            ErrorSeverity::Error,
                        );
                        EvaluationResult::with_error(error)
                    }
                }
            }
            "write_file" => {
                // Redirect to native function
                match self.native_registry.call("native_fs_write_file", &arg_values) {
                    Some(Ok(value)) => EvaluationResult::new(Some(value)),
                    Some(Err(error)) => EvaluationResult::with_error(error),
                    None => {
                        let error = DryadError::new(
                            "Função 'write_file' não encontrada".to_string(),
                            Some((0, 0)),
                            ErrorSeverity::Error,
                        );
                        EvaluationResult::with_error(error)
                    }
                }
            }
            "append_file" => {
                // Redirect to native function
                match self.native_registry.call("native_fs_append_file", &arg_values) {
                    Some(Ok(value)) => EvaluationResult::new(Some(value)),
                    Some(Err(error)) => EvaluationResult::with_error(error),
                    None => {
                        let error = DryadError::new(
                            "Função 'append_file' não encontrada".to_string(),
                            Some((0, 0)),
                            ErrorSeverity::Error,
                        );
                        EvaluationResult::with_error(error)
                    }
                }
            }
            _ => {
                // Try to find user-defined function in environment
                if let Some(func_value) = env.get(function) {
                    return self.call_resolved_function(func_value, args, env);
                }
                
                // Tentar resolver função com namespace (fallback)
                if let Some(func_value) = env.resolve_namespace_path(function) {
                    self.call_resolved_function(func_value, args, env)
                } else {
                    let error = DryadError::new(
                        format!("Função '{}' não encontrada", function),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    );
                    EvaluationResult::with_error(error)
                }
            }
        }
    }
    
    fn call_resolved_function(&mut self, func_value: Value, args: &[Expr], env: &mut Env) -> EvaluationResult {
        match func_value {
            Value::Function { params, body, .. } => {
                // Avaliar argumentos
                let mut arg_values = Vec::new();
                for arg in args {
                    let arg_result = self.eval_expr(arg, env);
                    if !arg_result.errors.is_empty() {
                        return arg_result;
                    }
                    if let Some(val) = arg_result.value {
                        arg_values.push(val);
                    }
                }
                
                // Criar novo ambiente para execução da função
                let mut func_env = Env::new();
                
                // Bind parâmetros
                for (i, param) in params.iter().enumerate() {
                    if let Some(arg_val) = arg_values.get(i) {
                        func_env.set(param.clone(), arg_val.clone());
                    }
                }
                
                // Executar corpo da função
                self.eval_stmt(&body, &mut func_env)
            }
            Value::Class(class) => {
                // Instanciar classe (construtor)
                let instance = Instance::new(class.clone());
                let instance_value = Value::Instance(Rc::new(RefCell::new(instance)));
                
                // Verificar se existe método init e chamá-lo
                if let Some(init_method) = class.get_method("init") {
                    let mut method_env = Env::with_this(instance_value.clone());
                    
                    // Avaliar argumentos
                    let mut arg_values = Vec::new();
                    for arg in args {
                        let arg_result = self.eval_expr(arg, env);
                        if !arg_result.errors.is_empty() {
                            return arg_result;
                        }
                        if let Some(val) = arg_result.value {
                            arg_values.push(val);
                        }
                    }
                    
                    // Chamar método init se existir
                    if let Value::Function { params, body, .. } = init_method {
                        // Bind parâmetros
                        for (i, param) in params.iter().enumerate() {
                            if let Some(arg_val) = arg_values.get(i) {
                                method_env.set(param.clone(), arg_val.clone());
                            }
                        }
                        
                        // Executar init
                        let _init_result = self.eval_stmt(&body, &mut method_env);
                    }
                }
                
                EvaluationResult::new(Some(instance_value))
            }
            _ => {
                let error = DryadError::new(
                    "Valor não é uma função ou classe".to_string(),
                    Some((0, 0)),
                    ErrorSeverity::Error,
                );
                EvaluationResult::with_error(error)
            }
        }
    }

    /// Validates that a static method does not access instance variables or `this`
    fn validate_static_method(&self, stmt: &Stmt, class_fields: &[String]) -> Result<(), DryadError> {
        self.validate_static_in_stmt(stmt, class_fields)
    }
    
    fn validate_static_in_stmt(&self, stmt: &Stmt, class_fields: &[String]) -> Result<(), DryadError> {
        match stmt {
            Stmt::Block(statements) => {
                for stmt in statements {
                    self.validate_static_in_stmt(stmt, class_fields)?;
                }
            }
            Stmt::Let { value, .. } => {
                self.validate_static_in_expr(value, class_fields)?;
            }
            Stmt::Assign { value, .. } => {
                self.validate_static_in_expr(value, class_fields)?;
            }
            Stmt::If { cond, then_branch, else_branch } => {
                self.validate_static_in_expr(cond, class_fields)?;
                self.validate_static_in_stmt(then_branch, class_fields)?;
                if let Some(else_branch) = else_branch {
                    self.validate_static_in_stmt(else_branch, class_fields)?;
                }
            }
            Stmt::While { cond, body } => {
                self.validate_static_in_expr(cond, class_fields)?;
                self.validate_static_in_stmt(body, class_fields)?;
            }
            Stmt::Return(Some(expr)) => {
                self.validate_static_in_expr(expr, class_fields)?;
            }
            Stmt::Expr(expr) => {
                self.validate_static_in_expr(expr, class_fields)?;
            }
            _ => {} // Other statements are fine
        }
        Ok(())
    }
    
    fn validate_static_in_expr(&self, expr: &Expr, class_fields: &[String]) -> Result<(), DryadError> {
        match expr {
            Expr::This => {
                return Err(DryadError::new(
                    "Métodos estáticos não podem acessar 'this'".to_string(),
                    Some((0, 0)),
                    ErrorSeverity::Error,
                ));
            }
            Expr::Identifier(name) => {
                if class_fields.contains(name) {
                    return Err(DryadError::new(
                        format!("Método estático não pode acessar a variável de instância '{}'", name),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    ));
                }
            }
            Expr::Get { object, name } => {
                // Check if accessing this.field
                if let Expr::This = **object {
                    return Err(DryadError::new(
                        format!("Método estático não pode acessar 'this.{}'", name),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    ));
                }
                self.validate_static_in_expr(object, class_fields)?;
            }
            Expr::Set { object, value, .. } => {
                // Check if setting this.field
                if let Expr::This = **object {
                    return Err(DryadError::new(
                        "Método estático não pode modificar campos de instância".to_string(),
                        Some((0, 0)),
                        ErrorSeverity::Error,
                    ));
                }
                self.validate_static_in_expr(object, class_fields)?;
                self.validate_static_in_expr(value, class_fields)?;
            }
            Expr::Binary { left, right, .. } => {
                self.validate_static_in_expr(left, class_fields)?;
                self.validate_static_in_expr(right, class_fields)?;
            }
            Expr::Unary { expr, .. } => {
                self.validate_static_in_expr(expr, class_fields)?;
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.validate_static_in_expr(arg, class_fields)?;
                }
            }
            Expr::New { args, .. } => {
                for arg in args {
                    self.validate_static_in_expr(arg, class_fields)?;
                }
            }
            _ => {} // Other expressions are fine
        }
        Ok(())
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
