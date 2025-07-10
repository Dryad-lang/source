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
        
        // Register only IO functions
        registry.register_io_functions();
        
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
        self.register("native_console_clear".to_string(), native_console_clear);
        
        // IO.fs functions
        self.register("native_fs_read_file".to_string(), native_fs_read_file);
        self.register("native_fs_write_file".to_string(), native_fs_write_file);
        self.register("native_fs_append_file".to_string(), native_fs_append_file);
        self.register("native_fs_file_exists".to_string(), native_fs_file_exists);
        self.register("native_fs_delete_file".to_string(), native_fs_delete_file);
        
        // IO.buffer functions
        self.register("native_buffer_create".to_string(), native_buffer_create);
        self.register("native_buffer_length".to_string(), native_buffer_length);
        
        // Utility functions needed by IO
        self.register("native_types_to_string".to_string(), native_types_to_string);
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
            Value::Array(_) => print!("[Array]"),
            Value::Object(_) => print!("[Object]"),
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
            Value::Array(_) => "array",
            Value::Object(_) => "object",
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
            Value::Array(_) => "[Array]".to_string(),
            Value::Object(_) => "[Object]".to_string(),
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

// Helper functions for JSON conversion
fn convert_json_to_dryad_value(json_value: serde_json::Value) -> Value {
    match json_value {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            let dryad_array: Vec<Value> = arr.into_iter()
                .map(convert_json_to_dryad_value)
                .collect();
            Value::Array(dryad_array)
        }
        serde_json::Value::Object(map) => {
            let mut dryad_object = HashMap::new();
            for (key, value) in map {
                dryad_object.insert(key, convert_json_to_dryad_value(value));
            }
            Value::Object(dryad_object)
        }
    }
}

fn convert_dryad_to_json_value(dryad_value: &Value) -> serde_json::Value {
    match dryad_value {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Number(n) => serde_json::Number::from_f64(*n)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) => {
            let json_array: Vec<serde_json::Value> = arr.iter()
                .map(convert_dryad_to_json_value)
                .collect();
            serde_json::Value::Array(json_array)
        }
        Value::Object(map) => {
            let mut json_object = serde_json::Map::new();
            for (key, value) in map {
                json_object.insert(key.clone(), convert_dryad_to_json_value(value));
            }
            serde_json::Value::Object(json_object)
        }
        _ => serde_json::Value::Null, // For complex types like functions, classes, etc.
    }
}

// JSON native functions
fn native_json_parse(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        match serde_json::from_str::<serde_json::Value>(s) {
            Ok(json_value) => Ok(convert_json_to_dryad_value(json_value)),
            Err(e) => Err(DryadError::new(
                format!("Erro ao fazer parse do JSON: {}", e),
                None,
                ErrorSeverity::Error,
            )),
        }
    } else {
        Err(DryadError::new(
            "json_parse: argumento deve ser uma string".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_json_stringify(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        let json_value = convert_dryad_to_json_value(value);
        match serde_json::to_string(&json_value) {
            Ok(s) => Ok(Value::String(s)),
            Err(e) => Err(DryadError::new(
                format!("Erro ao converter para JSON: {}", e),
                None,
                ErrorSeverity::Error,
            )),
        }
    } else {
        Ok(Value::String("null".to_string()))
    }
}

fn native_json_stringify_pretty(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        let json_value = convert_dryad_to_json_value(value);
        match serde_json::to_string_pretty(&json_value) {
            Ok(s) => Ok(Value::String(s)),
            Err(e) => Err(DryadError::new(
                format!("Erro ao converter para JSON: {}", e),
                None,
                ErrorSeverity::Error,
            )),
        }
    } else {
        Ok(Value::String("null".to_string()))
    }
}

fn native_json_try_parse(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        match serde_json::from_str::<serde_json::Value>(s) {
            Ok(json_value) => {
                // Create a result object with success=true and value
                let mut result_map = HashMap::new();
                result_map.insert("success".to_string(), Value::Bool(true));
                result_map.insert("value".to_string(), convert_json_to_dryad_value(json_value));
                Ok(Value::Object(result_map))
            }
            Err(_) => {
                // Create a result object with success=false
                let mut result_map = HashMap::new();
                result_map.insert("success".to_string(), Value::Bool(false));
                result_map.insert("value".to_string(), Value::Null);
                Ok(Value::Object(result_map))
            }
        }
    } else {
        Err(DryadError::new(
            "json_try_parse: argumento deve ser uma string".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

// Additional JSON functions
fn native_json_set_path(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement JSON path setting
    Err(DryadError::new(
        "json_set_path: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_json_remove_path(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement JSON path removal
    Err(DryadError::new(
        "json_remove_path: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_json_merge(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement JSON merge
    Err(DryadError::new(
        "json_merge: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_json_deep_merge(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement JSON deep merge
    Err(DryadError::new(
        "json_deep_merge: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_json_keys(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::Object(map)) = args.get(0) {
        let keys: Vec<Value> = map.keys()
            .map(|k| Value::String(k.clone()))
            .collect();
        Ok(Value::Array(keys))
    } else {
        Ok(Value::Array(vec![]))
    }
}

fn native_json_values(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::Object(map)) = args.get(0) {
        let values: Vec<Value> = map.values()
            .cloned()
            .collect();
        Ok(Value::Array(values))
    } else {
        Ok(Value::Array(vec![]))
    }
}

fn native_json_size(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        let size = match value {
            Value::Array(arr) => arr.len(),
            Value::Object(map) => map.len(),
            Value::String(s) => s.len(),
            _ => 0,
        };
        Ok(Value::Number(size as f64))
    } else {
        Ok(Value::Number(0.0))
    }
}

fn native_json_is_object(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        Ok(Value::Bool(matches!(value, Value::Object(_))))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_json_is_array(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        Ok(Value::Bool(matches!(value, Value::Array(_))))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_json_filter(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement JSON filter
    Err(DryadError::new(
        "json_filter: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_json_map(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement JSON map
    Err(DryadError::new(
        "json_map: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_json_reduce(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement JSON reduce
    Err(DryadError::new(
        "json_reduce: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

fn native_json_validate_schema(_args: &[Value]) -> Result<Value, DryadError> {
    // TODO: Implement JSON schema validation
    Err(DryadError::new(
        "json_validate_schema: não implementado ainda".to_string(),
        None,
        ErrorSeverity::Error,
    ))
}

// String native functions
fn native_string_length(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        Ok(Value::Number(s.len() as f64))
    } else {
        Err(DryadError::new(
            "string_length: argumento deve ser uma string".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_string_split(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::String(s)), Some(Value::String(delimiter))) = (args.get(0), args.get(1)) {
        let parts: Vec<Value> = s.split(delimiter).map(|p| Value::String(p.to_string())).collect();
        Ok(Value::Array(parts))
    } else {
        Err(DryadError::new(
            "string_split: argumentos devem ser (string, delimiter: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_string_replace_all(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::String(s)), Some(Value::String(old)), Some(Value::String(new))) = 
        (args.get(0), args.get(1), args.get(2)) {
        let new_s = s.replace(old, new);
        Ok(Value::String(new_s))
    } else {
        Err(DryadError::new(
            "string_replace_all: argumentos devem ser (string, old: string, new: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_string_starts_with(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::String(s)), Some(Value::String(prefix))) = (args.get(0), args.get(1)) {
        Ok(Value::Bool(s.starts_with(prefix)))
    } else {
        Err(DryadError::new(
            "string_starts_with: argumentos devem ser (string, prefix: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_string_slice(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::String(s)), Some(Value::Number(start)), Some(Value::Number(end))) = 
        (args.get(0), args.get(1), args.get(2)) {
        let start = *start as usize;
        let end = *end as usize;
        if start <= end && end <= s.len() {
            Ok(Value::String(s[start..end].to_string()))
        } else {
            Err(DryadError::new(
                "string_slice: índices fora do intervalo".to_string(),
                None,
                ErrorSeverity::Error,
            ))
        }
    } else {
        Err(DryadError::new(
            "string_slice: argumentos devem ser (string, start: number, end: number)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_string_is_empty(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        Ok(Value::Bool(s.is_empty()))
    } else {
        Ok(Value::Bool(true))
    }
}

// Additional String functions
fn native_string_contains(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::String(s)), Some(Value::String(substring))) = (args.get(0), args.get(1)) {
        Ok(Value::Bool(s.contains(substring)))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_string_indexof(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::String(s)), Some(Value::String(substring))) = (args.get(0), args.get(1)) {
        match s.find(substring) {
            Some(index) => Ok(Value::Number(index as f64)),
            None => Ok(Value::Number(-1.0)),
        }
    } else {
        Ok(Value::Number(-1.0))
    }
}

fn native_string_to_upper(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        Ok(Value::String(s.to_uppercase()))
    } else {
        Err(DryadError::new(
            "string_to_upper: argumento deve ser uma string".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_string_to_lower(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        Ok(Value::String(s.to_lowercase()))
    } else {
        Err(DryadError::new(
            "string_to_lower: argumento deve ser uma string".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_string_trim(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        Ok(Value::String(s.trim().to_string()))
    } else {
        Err(DryadError::new(
            "string_trim: argumento deve ser uma string".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_string_replace(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::String(s)), Some(Value::String(search)), Some(Value::String(replace))) = 
        (args.get(0), args.get(1), args.get(2)) {
        Ok(Value::String(s.replacen(search, replace, 1)))
    } else {
        Err(DryadError::new(
            "string_replace: argumentos devem ser (string, search: string, replace: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_string_reverse(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        Ok(Value::String(s.chars().rev().collect()))
    } else {
        Err(DryadError::new(
            "string_reverse: argumento deve ser uma string".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_string_is_alphanumeric(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        Ok(Value::Bool(s.chars().all(|c| c.is_alphanumeric())))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_string_is_numeric(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        Ok(Value::Bool(s.chars().all(|c| c.is_numeric())))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_string_is_alpha(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        Ok(Value::Bool(s.chars().all(|c| c.is_alphabetic())))
    } else {
        Ok(Value::Bool(false))
    }
}

// Array native functions
fn native_array_length(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::Array(arr)) = args.get(0) {
        Ok(Value::Number(arr.len() as f64))
    } else {
        Err(DryadError::new(
            "array_length: argumento deve ser um array".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_array_get(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::Array(arr)), Some(Value::Number(index))) = (args.get(0), args.get(1)) {
        let index = *index as usize;
        if index < arr.len() {
            Ok(arr[index].clone())
        } else {
            Ok(Value::Null)
        }
    } else {
        Err(DryadError::new(
            "array_get: argumentos devem ser (array, index: number)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_array_push(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::Array(arr)), Some(value)) = (args.get(0), args.get(1)) {
        let mut new_arr = arr.clone();
        new_arr.push(value.clone());
        Ok(Value::Array(new_arr))
    } else {
        Err(DryadError::new(
            "array_push: argumentos devem ser (array, value)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_array_join(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::Array(arr)), Some(Value::String(separator))) = (args.get(0), args.get(1)) {
        let mut result = String::new();
        for (i, value) in arr.iter().enumerate() {
            if i > 0 {
                result.push_str(separator);
            }
            match value {
                Value::Number(n) => result.push_str(&n.to_string()),
                Value::String(s) => result.push_str(s),
                Value::Bool(b) => result.push_str(&b.to_string()),
                Value::Null => result.push_str("null"),
                _ => {} // Ignore other types for now
            }
        }
        Ok(Value::String(result))
    } else {
        Err(DryadError::new(
            "array_join: argumentos devem ser (array, separator: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

// Object native functions
fn native_object_get(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::Object(obj)), Some(Value::String(key))) = (args.get(0), args.get(1)) {
        if let Some(value) = obj.get(key) {
            Ok(value.clone())
        } else {
            Ok(Value::Null)
        }
    } else {
        Err(DryadError::new(
            "object_get: argumentos devem ser (object, key: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_object_set(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::Object(obj)), Some(Value::String(key)), Some(value)) = 
        (args.get(0), args.get(1), args.get(2)) {
        // Note: This creates a new object since Value::Object contains an owned HashMap
        let mut new_obj = obj.clone();
        new_obj.insert(key.clone(), value.clone());
        Ok(Value::Object(new_obj))
    } else {
        Err(DryadError::new(
            "object_set: argumentos devem ser (object, key: string, value)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_object_has(args: &[Value]) -> Result<Value, DryadError> {
    if let (Some(Value::Object(obj)), Some(Value::String(key))) = (args.get(0), args.get(1)) {
        Ok(Value::Bool(obj.contains_key(key)))
    } else {
        Err(DryadError::new(
            "object_has: argumentos devem ser (object, key: string)".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_object_keys(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::Object(obj)) = args.get(0) {
        let keys: Vec<Value> = obj.keys().cloned().map(Value::String).collect();
        Ok(Value::Array(keys))
    } else {
        Err(DryadError::new(
            "object_keys: argumento deve ser um object".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

// Additional utility functions
fn native_url_encode(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        // Simple URL encoding - encode special characters
        let encoded = s.chars()
            .map(|c| match c {
                ' ' => "%20".to_string(),
                '&' => "%26".to_string(),
                '=' => "%3D".to_string(),
                '?' => "%3F".to_string(),
                '#' => "%23".to_string(),
                _ if c.is_ascii_alphanumeric() || "-_.~".contains(c) => c.to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect::<String>();
        Ok(Value::String(encoded))
    } else {
        Err(DryadError::new(
            "url_encode: argumento deve ser uma string".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_url_decode(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(Value::String(s)) = args.get(0) {
        // Simple URL decoding
        let decoded = s.replace("%20", " ")
            .replace("%26", "&")
            .replace("%3D", "=")
            .replace("%3F", "?")
            .replace("%23", "#");
        Ok(Value::String(decoded))
    } else {
        Err(DryadError::new(
            "url_decode: argumento deve ser uma string".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}

fn native_types_to_string(args: &[Value]) -> Result<Value, DryadError> {
    if let Some(value) = args.get(0) {
        let result = match value {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(_) => "[Array]".to_string(),
            Value::Object(_) => "[Object]".to_string(),
            _ => "[Unknown]".to_string(),
        };
        Ok(Value::String(result))
    } else {
        Ok(Value::String("undefined".to_string()))
    }
}

fn native_console_clear(_args: &[Value]) -> Result<Value, DryadError> {
    // On Windows, use cls command, on Unix use clear
    #[cfg(windows)]
    {
        std::process::Command::new("cmd").args(["/c", "cls"]).output().ok();
    }
    #[cfg(unix)]
    {
        std::process::Command::new("clear").output().ok();
    }
    Ok(Value::Null)
}
