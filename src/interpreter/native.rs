// src/interpreter/native_expanded.rs
// Sistema expandido de funções nativas para interface Rust <-> Dryad

use crate::interpreter::env::Value;
use crate::interpreter::io;
use crate::interpreter::errors::{DryadError, ErrorSeverity};
use std::collections::HashMap;
use std::env;
use std::thread;
use std::time::Duration;

pub type NativeFunction = fn(&[Value]) -> Result<Value, DryadError>;

pub struct NativeRegistry {
    functions: HashMap<String, NativeFunction>,
    modules: HashMap<String, Vec<String>>, // module_name -> list of functions
}

impl NativeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
            modules: HashMap::new(),
        };
        
        registry.register_core_functions();
        registry.register_io_functions();
        registry.register_math_functions();
        registry.register_system_functions();
        
        registry
    }
    
    pub fn register(&mut self, name: String, function: NativeFunction) {
        self.functions.insert(name, function);
    }
    
    pub fn register_module_function(&mut self, module: &str, name: &str, function: NativeFunction) {
        let full_name = format!("{}.{}", module, name);
        self.functions.insert(full_name.clone(), function);
        
        self.modules.entry(module.to_string())
            .or_insert_with(Vec::new)
            .push(name.to_string());
    }
    
    pub fn call(&self, name: &str, args: &[Value]) -> Option<Result<Value, DryadError>> {
        self.functions.get(name).map(|f| f(args))
    }
    
    pub fn get_module_functions(&self, module: &str) -> Option<&Vec<String>> {
        self.modules.get(module)
    }
    
    pub fn is_native(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
    
    fn register_core_functions(&mut self) {
        // Core.Types
        self.register_module_function("Core", "typeof", native_core_typeof);
        self.register_module_function("Core", "toString", native_core_to_string);
        self.register_module_function("Core", "isNumber", native_core_is_number);
        self.register_module_function("Core", "isString", native_core_is_string);
        self.register_module_function("Core", "isBool", native_core_is_bool);
        self.register_module_function("Core", "isNull", native_core_is_null);
    }
    
    fn register_io_functions(&mut self) {
        // Console functions
        self.register_module_function("Console", "print", native_console_print);
        self.register_module_function("Console", "println", native_console_println);
        self.register_module_function("Console", "log", native_console_println); // alias
        self.register_module_function("Console", "input", native_console_input);
        self.register_module_function("Console", "clear", native_console_clear);
        
        // File system functions  
        self.register_module_function("Fs", "readFile", native_fs_read_file);
        self.register_module_function("Fs", "writeFile", native_fs_write_file);
        self.register_module_function("Fs", "appendFile", native_fs_append_file);
        self.register_module_function("Fs", "fileExists", native_fs_file_exists);
        self.register_module_function("Fs", "deleteFile", native_fs_delete_file);
        
        // Buffer functions
        self.register_module_function("Buffer", "create", native_buffer_create);
        self.register_module_function("Buffer", "length", native_buffer_length);
    }
    
    fn register_math_functions(&mut self) {
        // Math functions nativas para performance
        self.register_module_function("Math", "sqrt", native_math_sqrt);
        self.register_module_function("Math", "pow", native_math_pow);
        self.register_module_function("Math", "sin", native_math_sin);
        self.register_module_function("Math", "cos", native_math_cos);
        self.register_module_function("Math", "tan", native_math_tan);
        self.register_module_function("Math", "log", native_math_log);
        self.register_module_function("Math", "exp", native_math_exp);
        self.register_module_function("Math", "abs", native_math_abs);
        self.register_module_function("Math", "floor", native_math_floor);
        self.register_module_function("Math", "ceil", native_math_ceil);
        self.register_module_function("Math", "round", native_math_round);
        self.register_module_function("Math", "random", native_math_random);
        self.register_module_function("Math", "pi", native_math_pi);
        self.register_module_function("Math", "e", native_math_e);
    }
    
    fn register_system_functions(&mut self) {
        // System functions
        self.register_module_function("System", "getEnv", native_system_get_env);
        self.register_module_function("System", "setEnv", native_system_set_env);
        self.register_module_function("System", "getArgs", native_system_get_args);
        self.register_module_function("System", "exit", native_system_exit);
        self.register_module_function("System", "currentDir", native_system_current_dir);
        self.register_module_function("System", "sleep", native_system_sleep);
    }
}

// ===== CORE FUNCTIONS =====
fn native_core_typeof(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::String("undefined".to_string()));
    }
    
    let type_str = match &args[0] {
        Value::Number(_) => "number",
        Value::String(_) => "string", 
        Value::Bool(_) => "boolean",
        Value::Null => "null",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        Value::Class(_) => "class",
        Value::Instance(_) => "instance",
        Value::Function { .. } => "function",
        Value::Exception { .. } => "exception",
    };
    
    Ok(Value::String(type_str.to_string()))
}

fn native_core_to_string(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::String("".to_string()));
    }
    
    Ok(Value::String(args[0].to_string()))
}

fn native_core_is_number(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Bool(false));
    }
    Ok(Value::Bool(matches!(args[0], Value::Number(_))))
}

fn native_core_is_string(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Bool(false));
    }
    Ok(Value::Bool(matches!(args[0], Value::String(_))))
}

fn native_core_is_bool(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Bool(false));
    }
    Ok(Value::Bool(matches!(args[0], Value::Bool(_))))
}

fn native_core_is_null(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Bool(true));
    }
    Ok(Value::Bool(matches!(args[0], Value::Null)))
}

// ===== IO FUNCTIONS =====
fn native_console_print(args: &[Value]) -> Result<Value, DryadError> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg.to_string());
    }
    Ok(Value::Null)
}

fn native_console_println(args: &[Value]) -> Result<Value, DryadError> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg.to_string());
    }
    println!();
    Ok(Value::Null)
}

fn native_console_input(args: &[Value]) -> Result<Value, DryadError> {
    use std::io::{self, Write};
    
    if !args.is_empty() {
        print!("{}", args[0].to_string());
        io::stdout().flush().map_err(|e| {
            DryadError::new(format!("IO error: {}", e), None, ErrorSeverity::Error)
        })?;
    }
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| {
        DryadError::new(format!("IO error: {}", e), None, ErrorSeverity::Error)
    })?;
    
    Ok(Value::String(input.trim().to_string()))
}

fn native_console_clear(_args: &[Value]) -> Result<Value, DryadError> {
    print!("\x1B[2J\x1B[1;1H");
    Ok(Value::Null)
}

fn native_fs_read_file(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Err(DryadError::new("Missing file path".to_string(), None, ErrorSeverity::Error));
    }
    
    if let Value::String(path) = &args[0] {
        match io::read_file(path) {
            Ok(content) => Ok(Value::String(content)),
            Err(e) => Err(DryadError::new(format!("Failed to read file: {}", e), None, ErrorSeverity::Error))
        }
    } else {
        Err(DryadError::new("File path must be a string".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_fs_write_file(args: &[Value]) -> Result<Value, DryadError> {
    if args.len() < 2 {
        return Err(DryadError::new("Missing file path or content".to_string(), None, ErrorSeverity::Error));
    }
    
    if let (Value::String(path), content) = (&args[0], &args[1]) {
        match io::write_file(path, &content.to_string()) {
            Ok(_) => Ok(Value::Bool(true)),
            Err(e) => Err(DryadError::new(format!("Failed to write file: {}", e), None, ErrorSeverity::Error))
        }
    } else {
        Err(DryadError::new("File path must be a string".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_fs_append_file(args: &[Value]) -> Result<Value, DryadError> {
    if args.len() < 2 {
        return Err(DryadError::new("Missing file path or content".to_string(), None, ErrorSeverity::Error));
    }
    
    if let (Value::String(path), content) = (&args[0], &args[1]) {
        match io::append_file(path, &content.to_string()) {
            Ok(_) => Ok(Value::Bool(true)),
            Err(e) => Err(DryadError::new(format!("Failed to append to file: {}", e), None, ErrorSeverity::Error))
        }
    } else {
        Err(DryadError::new("File path must be a string".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_fs_file_exists(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Bool(false));
    }
    
    if let Value::String(path) = &args[0] {
        Ok(Value::Bool(std::path::Path::new(path).exists()))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_fs_delete_file(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Err(DryadError::new("Missing file path".to_string(), None, ErrorSeverity::Error));
    }
    
    if let Value::String(path) = &args[0] {
        match std::fs::remove_file(path) {
            Ok(_) => Ok(Value::Bool(true)),
            Err(e) => Err(DryadError::new(format!("Failed to delete file: {}", e), None, ErrorSeverity::Error))
        }
    } else {
        Err(DryadError::new("File path must be a string".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_buffer_create(args: &[Value]) -> Result<Value, DryadError> {
    let size = if args.is_empty() {
        0
    } else if let Value::Number(n) = args[0] {
        n as usize
    } else {
        return Err(DryadError::new("Buffer size must be a number".to_string(), None, ErrorSeverity::Error));
    };
    
    Ok(Value::Array(vec![Value::Number(0.0); size]))
}

fn native_buffer_length(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(0.0));
    }
    
    if let Value::Array(arr) = &args[0] {
        Ok(Value::Number(arr.len() as f64))
    } else if let Value::String(s) = &args[0] {
        Ok(Value::Number(s.len() as f64))
    } else {
        Ok(Value::Number(0.0))
    }
}

// ===== MATH FUNCTIONS =====
fn native_math_sqrt(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(0.0));
    }
    
    if let Value::Number(n) = args[0] {
        Ok(Value::Number(n.sqrt()))
    } else {
        Err(DryadError::new("sqrt requires a number".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_pow(args: &[Value]) -> Result<Value, DryadError> {
    if args.len() < 2 {
        return Ok(Value::Number(0.0));
    }
    
    if let (Value::Number(base), Value::Number(exp)) = (&args[0], &args[1]) {
        Ok(Value::Number(base.powf(*exp)))
    } else {
        Err(DryadError::new("pow requires two numbers".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_sin(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(0.0));
    }
    
    if let Value::Number(n) = args[0] {
        Ok(Value::Number(n.sin()))
    } else {
        Err(DryadError::new("sin requires a number".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_cos(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(1.0));
    }
    
    if let Value::Number(n) = args[0] {
        Ok(Value::Number(n.cos()))
    } else {
        Err(DryadError::new("cos requires a number".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_tan(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(0.0));
    }
    
    if let Value::Number(n) = args[0] {
        Ok(Value::Number(n.tan()))
    } else {
        Err(DryadError::new("tan requires a number".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_log(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(0.0));
    }
    
    if let Value::Number(n) = args[0] {
        Ok(Value::Number(n.ln()))
    } else {
        Err(DryadError::new("log requires a number".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_exp(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(1.0));
    }
    
    if let Value::Number(n) = args[0] {
        Ok(Value::Number(n.exp()))
    } else {
        Err(DryadError::new("exp requires a number".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_abs(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(0.0));
    }
    
    if let Value::Number(n) = args[0] {
        Ok(Value::Number(n.abs()))
    } else {
        Err(DryadError::new("abs requires a number".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_floor(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(0.0));
    }
    
    if let Value::Number(n) = args[0] {
        Ok(Value::Number(n.floor()))
    } else {
        Err(DryadError::new("floor requires a number".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_ceil(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(0.0));
    }
    
    if let Value::Number(n) = args[0] {
        Ok(Value::Number(n.ceil()))
    } else {
        Err(DryadError::new("ceil requires a number".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_round(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Number(0.0));
    }
    
    if let Value::Number(n) = args[0] {
        Ok(Value::Number(n.round()))
    } else {
        Err(DryadError::new("round requires a number".to_string(), None, ErrorSeverity::Error))
    }
}

fn native_math_random(_args: &[Value]) -> Result<Value, DryadError> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    let hash = hasher.finish();
    
    // Simple linear congruential generator
    let normalized = (hash as f64) / (u64::MAX as f64);
    Ok(Value::Number(normalized))
}

fn native_math_pi(_args: &[Value]) -> Result<Value, DryadError> {
    Ok(Value::Number(std::f64::consts::PI))
}

fn native_math_e(_args: &[Value]) -> Result<Value, DryadError> {
    Ok(Value::Number(std::f64::consts::E))
}

// ===== SYSTEM FUNCTIONS =====
fn native_system_get_env(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Null);
    }
    
    if let Value::String(key) = &args[0] {
        match env::var(key) {
            Ok(value) => Ok(Value::String(value)),
            Err(_) => Ok(Value::Null)
        }
    } else {
        Ok(Value::Null)
    }
}

fn native_system_set_env(args: &[Value]) -> Result<Value, DryadError> {
    if args.len() < 2 {
        return Ok(Value::Bool(false));
    }
    
    if let (Value::String(key), value) = (&args[0], &args[1]) {
        env::set_var(key, value.to_string());
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

fn native_system_get_args(_args: &[Value]) -> Result<Value, DryadError> {
    let args: Vec<Value> = env::args()
        .map(|arg| Value::String(arg))
        .collect();
    Ok(Value::Array(args))
}

fn native_system_exit(args: &[Value]) -> Result<Value, DryadError> {
    let code = if args.is_empty() {
        0
    } else if let Value::Number(n) = args[0] {
        n as i32
    } else {
        1
    };
    
    std::process::exit(code);
}

fn native_system_current_dir(_args: &[Value]) -> Result<Value, DryadError> {
    match env::current_dir() {
        Ok(path) => Ok(Value::String(path.to_string_lossy().to_string())),
        Err(e) => Err(DryadError::new(format!("Failed to get current directory: {}", e), None, ErrorSeverity::Error))
    }
}

fn native_system_sleep(args: &[Value]) -> Result<Value, DryadError> {
    if args.is_empty() {
        return Ok(Value::Null);
    }
    
    if let Value::Number(ms) = args[0] {
        let duration = Duration::from_millis(ms as u64);
        thread::sleep(duration);
        Ok(Value::Null)
    } else {
        Err(DryadError::new("sleep requires a number (milliseconds)".to_string(), None, ErrorSeverity::Error))
    }
}
