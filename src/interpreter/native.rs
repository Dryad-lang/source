// src/interpreter/native.rs
// Sistema de funções nativas para interface Rust <-> Dryad

use crate::interpreter::env::Value;
use crate::interpreter::io;
use crate::interpreter::errors::{DryadError, ErrorSeverity};
use std::collections::HashMap;

pub type NativeFunction = fn(&[Value]) -> Result<Value, DryadError>;

pub struct NativeRegistry {
    functions: HashMap<String, NativeFunction>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        
        // Register core native functions
        registry.register_io_functions();
        registry.register_core_functions();
        
        registry
    }
    
    pub fn register(&mut self, name: String, function: NativeFunction) {
        self.functions.insert(name, function);
    }
    
    pub fn call(&self, name: &str, args: &[Value]) -> Option<Result<Value, DryadError>> {
        self.functions.get(name).map(|func| func(args))
    }
    
    pub fn is_native(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
    
    fn register_io_functions(&mut self) {
        // IO.console functions
        self.register("native_console_print".to_string(), native_console_print);
        self.register("native_console_println".to_string(), native_console_println);
        self.register("native_console_input".to_string(), native_console_input);
        
        // IO.fs functions
        self.register("native_fs_read_file".to_string(), native_fs_read_file);
        self.register("native_fs_write_file".to_string(), native_fs_write_file);
        self.register("native_fs_append_file".to_string(), native_fs_append_file);
        self.register("native_fs_file_exists".to_string(), native_fs_file_exists);
        self.register("native_fs_delete_file".to_string(), native_fs_delete_file);
        
        // IO.buffer functions
        self.register("native_buffer_create".to_string(), native_buffer_create);
        self.register("native_buffer_length".to_string(), native_buffer_length);
    }
    
    fn register_core_functions(&mut self) {
        // core.types functions
        self.register("native_core_typeof".to_string(), native_core_typeof);
        self.register("native_core_is_number".to_string(), native_core_is_number);
        self.register("native_core_is_string".to_string(), native_core_is_string);
        self.register("native_core_is_bool".to_string(), native_core_is_bool);
        self.register("native_core_is_null".to_string(), native_core_is_null);
        self.register("native_core_is_function".to_string(), native_core_is_function);
        self.register("native_core_is_class".to_string(), native_core_is_class);
        self.register("native_core_is_instance".to_string(), native_core_is_instance);
        self.register("native_core_is_exception".to_string(), native_core_is_exception);
        self.register("native_core_to_string".to_string(), native_core_to_string);
        self.register("native_core_to_number".to_string(), native_core_to_number);
        self.register("native_core_to_bool".to_string(), native_core_to_bool);
        self.register("native_core_equals".to_string(), native_core_equals);
        self.register("native_core_deep_equals".to_string(), native_core_deep_equals);
        self.register("native_core_string_is_empty".to_string(), native_core_string_is_empty);
        
        // core.meta functions  
        self.register("native_core_get_class_name".to_string(), native_core_get_class_name);
        self.register("native_core_get_class_methods".to_string(), native_core_get_class_methods);
        self.register("native_core_get_class_fields".to_string(), native_core_get_class_fields);
        self.register("native_core_has_method".to_string(), native_core_has_method);
        self.register("native_core_has_field".to_string(), native_core_has_field);
        self.register("native_core_call_method".to_string(), native_core_call_method);
        self.register("native_core_get_field".to_string(), native_core_get_field);
        self.register("native_core_set_field".to_string(), native_core_set_field);
        self.register("native_core_eval".to_string(), native_core_eval);
        self.register("native_core_eval_file".to_string(), native_core_eval_file);
        self.register("native_core_compile".to_string(), native_core_compile);
        self.register("native_core_get_loaded_modules".to_string(), native_core_get_loaded_modules);
        self.register("native_core_get_module_info".to_string(), native_core_get_module_info);
        self.register("native_core_get_module_exports".to_string(), native_core_get_module_exports);
        self.register("native_core_is_module_loaded".to_string(), native_core_is_module_loaded);
        self.register("native_core_reload_module".to_string(), native_core_reload_module);
        self.register("native_core_unload_module".to_string(), native_core_unload_module);
        self.register("native_core_get_environment_vars".to_string(), native_core_get_environment_vars);
        self.register("native_core_get_current_scope".to_string(), native_core_get_current_scope);
        self.register("native_core_get_global_scope".to_string(), native_core_get_global_scope);
        self.register("native_core_get_all_variables".to_string(), native_core_get_all_variables);
        self.register("native_core_get_stack_trace".to_string(), native_core_get_stack_trace);
        self.register("native_core_get_call_stack".to_string(), native_core_get_call_stack);
        self.register("native_core_get_memory_usage".to_string(), native_core_get_memory_usage);
        self.register("native_core_get_performance_info".to_string(), native_core_get_performance_info);
        self.register("native_core_clone_object".to_string(), native_core_clone_object);
        self.register("native_core_deep_clone".to_string(), native_core_deep_clone);
        self.register("native_core_serialize".to_string(), native_core_serialize);
        self.register("native_core_deserialize".to_string(), native_core_deserialize);
    }
}

// IO.console native functions
fn native_console_print(args: &[Value]) -> Result<Value, DryadError> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        match arg {
            Value::Number(n) => print!("{}", n),
            Value::String(s) => print!("{}", s),
            Value::Bool(b) => print!("{}", b),
            Value::Null => print!("null"),
            Value::Class(class) => print!("<class {}>", class.name),
            Value::Instance(instance) => {
                let inst = instance.borrow();
                print!("<instance of {}>", inst.class.name);
            }
            Value::Function { name, .. } => print!("<function {}>", name),
            Value::Exception { message, .. } => print!("Exception: {}", message),
        }
    }
    Ok(Value::Null)
}

fn native_console_println(args: &[Value]) -> Result<Value, DryadError> {
    native_console_print(args)?;
    println!();
    Ok(Value::Null)
}

fn native_console_input(args: &[Value]) -> Result<Value, DryadError> {
    // Prompt if provided
    if let Some(prompt) = args.get(0) {
        native_console_print(&[prompt.clone()])?;
    }
    
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Remove newline
            if input.ends_with('\n') {
                input.pop();
                if input.ends_with('\r') {
                    input.pop();
                }
            }
            Ok(Value::String(input))
        }
        Err(e) => Err(DryadError::new(
            format!("Erro ao ler entrada: {}", e),
            None,
            ErrorSeverity::Error,
        )),
    }
}

// IO.fs native functions
fn native_fs_read_file(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(filename)) = args.get(0) {
        match io::read_file(filename) {
            Ok(content) => Ok(Value::String(content)),
            Err(e) => Err(DryadError::new(
                format!("Erro ao ler arquivo {}: {}", filename, e),
                None,
                ErrorSeverity::Error,
            )),
        }
    } else {
        Err(DryadError::new(
            "read_file: primeiro argumento deve ser uma string (nome do arquivo)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_fs_write_file(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::String(filename)), Some(Value::String(content))) = (args.get(0), args.get(1)) {
        match io::write_file(filename, content) {
            Ok(_) => Ok(Value::Null),
            Err(e) => Err(DryadError::new(
                format!("Erro ao escrever arquivo {}: {}", filename, e),
                None,
                ErrorSeverity::Error,
            )),
        }
    } else {
        Err(DryadError::new(
            "write_file: argumentos devem ser (filename: string, content: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_fs_append_file(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::String(filename)), Some(Value::String(content))) = (args.get(0), args.get(1)) {
        match io::append_file(filename, content) {
            Ok(_) => Ok(Value::Null),
            Err(e) => Err(DryadError::new(
                format!("Erro ao anexar ao arquivo {}: {}", filename, e),
                None,
                ErrorSeverity::Error,
            )),
        }
    } else {
        Err(DryadError::new(
            "append_file: argumentos devem ser (filename: string, content: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_fs_file_exists(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(filename)) = args.get(0) {
        use std::path::Path;
        Ok(Value::Bool(Path::new(filename).exists()))
    } else {
        Err(DryadError::new(
            "file_exists: argumento deve ser uma string (nome do arquivo)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_fs_delete_file(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(filename)) = args.get(0) {
        match std::fs::remove_file(filename) {
            Ok(_) => Ok(Value::Bool(true)),
            Err(e) => Err(DryadError::new(
                format!("Erro ao deletar arquivo {}: {}", filename, e),
                None,
                ErrorSeverity::Error,
            )),
        }
    } else {
        Err(DryadError::new(
            "delete_file: argumento deve ser uma string (nome do arquivo)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

// IO.buffer native functions
fn native_buffer_create(args: &[Value]) -> Result<Value, DryadError> {
    let size = if let Some(Value::Number(n)) = args.get(0) {
        *n as usize
    } else {
        0
    };
    
    // Create a simple string buffer for now
    Ok(Value::String(" ".repeat(size)))
}

fn native_buffer_length(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(buffer)) = args.get(0) {
        Ok(Value::Number(buffer.len() as f64))
    } else {
        Err(DryadError::new(
            "buffer_length: argumento deve ser um buffer (string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

// core.types native functions
fn native_core_typeof(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        let type_name = match value {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Null => "null",
            Value::Class(_) => "class",
            Value::Instance(_) => "instance",
            Value::Function { .. } => "function",
            Value::Exception { .. } => "exception",
        };
        Ok(Value::String(type_name.to_string()))
    } else {
        Err(DryadError::new(
            "typeof: argumento requerido".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_core_is_number(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        Ok(Value::Bool(matches!(value, Value::Number(_))))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_core_is_string(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        Ok(Value::Bool(matches!(value, Value::String(_))))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_core_is_bool(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        Ok(Value::Bool(matches!(value, Value::Bool(_))))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_core_is_null(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        Ok(Value::Bool(matches!(value, Value::Null)))
    } else {
        Ok(Value::Bool(true)) // No argument is considered null
    }
}

fn native_core_is_function(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        Ok(Value::Bool(matches!(value, Value::Function { .. })))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_core_is_class(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        Ok(Value::Bool(matches!(value, Value::Class(_))))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_core_is_instance(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        Ok(Value::Bool(matches!(value, Value::Instance(_))))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_core_is_exception(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        Ok(Value::Bool(matches!(value, Value::Exception { .. })))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_core_to_string(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        let string_repr = match value {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Class(class) => format!("<class {}>", class.name),
            Value::Instance(instance) => {
                let inst = instance.borrow();
                format!("<instance of {}>", inst.class.name)
            }
            Value::Function { name, .. } => format!("<function {}>", name),
            Value::Exception { message, value } => {
                if let Some(v) = value {
                    let inner_string = native_core_to_string(&[*v.clone()])?;
                    if let Value::String(s) = inner_string {
                        format!("Exception {}: {}", message, s)
                    } else {
                        format!("Exception: {}", message)
                    }
                } else {
                    format!("Exception: {}", message)
                }
            }
        };
        Ok(Value::String(string_repr))
    } else {
        Ok(Value::String("".to_string()))
    }
}

fn native_core_to_number(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        match value {
            Value::Number(n) => Ok(Value::Number(*n)),
            Value::String(s) => {
                match s.parse::<f64>() {
                    Ok(n) => Ok(Value::Number(n)),
                    Err(_) => Ok(Value::Number(f64::NAN)),
                }
            }
            Value::Bool(true) => Ok(Value::Number(1.0)),
            Value::Bool(false) => Ok(Value::Number(0.0)),
            _ => Ok(Value::Number(f64::NAN)),
        }
    } else {
        Ok(Value::Number(0.0))
    }
}

fn native_core_to_bool(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        let bool_val = match value {
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Bool(b) => *b,
            Value::Null => false,
            _ => true, // Objects are truthy
        };
        Ok(Value::Bool(bool_val))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_core_equals(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(a), Some(b)) = (args.get(0), args.get(1)) {
        Ok(Value::Bool(a == b))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_core_deep_equals(args: &[Value]) -> Result<Value, DryadError> {
    // For now, same as shallow equals - can be enhanced later
    native_core_equals(args)
}

fn native_core_string_is_empty(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        Ok(Value::Bool(s.is_empty()))
    } else {
        Ok(Value::Bool(true))
    }
}

// core.meta native functions
fn native_core_get_class_name(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        match value {
            Value::Class(class) => Ok(Value::String(class.name.clone())),
            Value::Instance(instance) => {
                let inst = instance.borrow();
                Ok(Value::String(inst.class.name.clone()))
            }
            _ => Err(DryadError::new(
                "get_class_name: argumento deve ser uma classe ou instância".to_string(),
                None,
                ErrorSeverity::Error,
            )),
        }
    } else {
        Err(DryadError::new(
            "get_class_name: argumento requerido".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_core_get_class_methods(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        match value {
            Value::Class(class) => {
                let methods: Vec<String> = class.methods.keys().cloned().collect();
                let method_values: Vec<Value> = methods.into_iter().map(Value::String).collect();
                // For now, return a simple string representation
                Ok(Value::String(format!("{:?}", method_values)))
            }
            Value::Instance(instance) => {
                let inst = instance.borrow();
                let methods: Vec<String> = inst.class.methods.keys().cloned().collect();
                let method_values: Vec<Value> = methods.into_iter().map(Value::String).collect();
                Ok(Value::String(format!("{:?}", method_values)))
            }
            _ => Err(DryadError::new(
                "get_class_methods: argumento deve ser uma classe ou instância".to_string(),
                None,
                ErrorSeverity::Error,
            )),
        }
    } else {
        Err(DryadError::new(
            "get_class_methods: argumento requerido".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_core_get_class_fields(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        match value {
            Value::Class(class) => {
                let fields: Vec<String> = class.fields.iter().map(|f| f.name.clone()).collect();
                let field_values: Vec<Value> = fields.into_iter().map(Value::String).collect();
                Ok(Value::String(format!("{:?}", field_values)))
            }
            Value::Instance(instance) => {
                let inst = instance.borrow();
                let fields: Vec<String> = inst.fields.keys().cloned().collect();
                let field_values: Vec<Value> = fields.into_iter().map(Value::String).collect();
                Ok(Value::String(format!("{:?}", field_values)))
            }
            _ => Err(DryadError::new(
                "get_class_fields: argumento deve ser uma classe ou instância".to_string(),
                None,
                ErrorSeverity::Error,
            )),
        }
    } else {
        Err(DryadError::new(
            "get_class_fields: argumento requerido".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_core_has_method(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(instance), Some(Value::String(method_name))) = (args.get(0), args.get(1)) {
        match instance {
            Value::Instance(inst) => {
                let inst_ref = inst.borrow();
                Ok(Value::Bool(inst_ref.class.methods.contains_key(method_name)))
            }
            Value::Class(class) => {
                Ok(Value::Bool(class.methods.contains_key(method_name)))
            }
            _ => Ok(Value::Bool(false)),
        }
    } else {
        Err(DryadError::new(
            "has_method: argumentos devem ser (instance, method_name: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_core_has_field(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(instance), Some(Value::String(field_name))) = (args.get(0), args.get(1)) {
        match instance {
            Value::Instance(inst) => {
                let inst_ref = inst.borrow();
                Ok(Value::Bool(inst_ref.fields.contains_key(field_name)))
            }
            Value::Class(class) => {
                Ok(Value::Bool(class.fields.iter().any(|f| f.name == *field_name)))
            }
            _ => Ok(Value::Bool(false)),
        }
    } else {
        Err(DryadError::new(
            "has_field: argumentos devem ser (instance, field_name: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_core_call_method(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement dynamic method calling
    Err(DryadError::new(
        "call_method: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_core_get_field(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::Instance(instance)), Some(Value::String(field_name))) = (args.get(0), args.get(1)) {
        let inst = instance.borrow();
        if let Some(value) = inst.fields.get(field_name) {
            Ok(value.clone())
        } else {
            Ok(Value::Null)
        }
    } else {
        Err(DryadError::new(
            "get_field: argumentos devem ser (instance, field_name: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_core_set_field(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::Instance(instance)), Some(Value::String(field_name)), Some(value)) = 
        (args.get(0), args.get(1), args.get(2)) {
        let mut inst = instance.borrow_mut();
        inst.fields.insert(field_name.clone(), value.clone());
        Ok(Value::Null)
    } else {
        Err(DryadError::new(
            "set_field: argumentos devem ser (instance, field_name: string, value)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_core_eval(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement eval functionality
    Err(DryadError::new(
        "eval: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_core_eval_file(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement eval_file functionality
    Err(DryadError::new(
        "eval_file: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_core_compile(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement compile functionality
    Err(DryadError::new(
        "compile: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_core_get_loaded_modules(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_loaded_modules functionality
    Ok(Value::String("[]".to_string())) // Empty array for now
}

fn native_core_get_module_info(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_module_info functionality
    Err(DryadError::new(
        "get_module_info: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_core_get_module_exports(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_module_exports functionality
    Err(DryadError::new(
        "get_module_exports: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_core_is_module_loaded(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement is_module_loaded functionality
    Ok(Value::Bool(false))
}

fn native_core_reload_module(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement reload_module functionality
    Err(DryadError::new(
        "reload_module: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_core_unload_module(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement unload_module functionality
    Err(DryadError::new(
        "unload_module: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_core_get_environment_vars(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_environment_vars functionality
    Ok(Value::String("{}".to_string())) // Empty object for now
}

fn native_core_get_current_scope(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_current_scope functionality
    Ok(Value::String("{}".to_string())) // Empty object for now
}

fn native_core_get_global_scope(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_global_scope functionality
    Ok(Value::String("{}".to_string())) // Empty object for now
}

fn native_core_get_all_variables(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_all_variables functionality
    Ok(Value::String("{}".to_string())) // Empty object for now
}

fn native_core_get_stack_trace(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_stack_trace functionality
    Ok(Value::String("[]".to_string())) // Empty array for now
}

fn native_core_get_call_stack(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_call_stack functionality
    Ok(Value::String("[]".to_string())) // Empty array for now
}

fn native_core_get_memory_usage(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_memory_usage functionality
    Ok(Value::Number(0.0)) // 0 bytes for now
}

fn native_core_get_performance_info(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement get_performance_info functionality
    Ok(Value::String("{}".to_string())) // Empty object for now
}

fn native_core_clone_object(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        // For now, just clone the value (shallow copy)
        Ok(value.clone())
    } else {
        Ok(Value::Null)
    }
}

fn native_core_deep_clone(args: &[Value]) -> Result<Value, DryadError> {
    // For now, same as shallow clone
    native_core_clone_object(args)
}

fn native_core_serialize(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement serialize functionality
    Err(DryadError::new(
        "serialize: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_core_deserialize(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement deserialize functionality
    Err(DryadError::new(
        "deserialize: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}
