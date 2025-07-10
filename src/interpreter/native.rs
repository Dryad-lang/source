// src/interpreter/native.rs
// Sistema de funções nativas para interface Rust <-> Dryad - Versão IO Only

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
            Value::Array(arr) => {
                print!("[");
                for (i, elem) in arr.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    match elem {
                        Value::Number(n) => print!("{}", n),
                        Value::String(s) => print!("\"{}\"", s),
                        Value::Bool(b) => print!("{}", b),
                        Value::Null => print!("null"),
                        _ => print!("{:?}", elem),
                    }
                }
                print!("]");
            }
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
    if args.is_empty() {
        println!();
    } else {
        native_console_print(args)?;
        println!();
    }
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

// Utility functions
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
