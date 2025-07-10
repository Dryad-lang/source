// src/interpreter/errors.rs

use std::fmt;

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCode {
    // Lexer Errors (1000-1999)
    E1001, // Unexpected character
    E1002, // Unterminated string literal
    E1003, // Unterminated comment
    E1004, // Invalid number format
    E1005, // Invalid escape sequence
    E1006, // Character not recognized
    E1007, // String exceeds maximum length
    E1008, // Invalid unicode sequence
    
    // Parser Errors (2000-2999)
    E2001, // Unexpected token
    E2002, // Expected token not found
    E2003, // Missing semicolon
    E2004, // Missing closing brace
    E2005, // Missing closing parenthesis
    E2006, // Missing closing bracket
    E2007, // Invalid expression
    E2008, // Invalid statement
    E2009, // Invalid function declaration
    E2010, // Invalid class declaration
    E2011, // Invalid variable declaration
    E2012, // Missing function name
    E2013, // Missing parameter list
    E2014, // Invalid parameter
    E2015, // Duplicate parameter name
    E2016, // Invalid return statement
    E2017, // Invalid if statement
    E2018, // Invalid while loop
    E2019, // Invalid for loop
    E2020, // Invalid namespace declaration
    E2021, // Invalid import statement
    E2022, // Invalid export statement
    E2023, // Missing closing quote
    E2024, // Invalid array literal
    E2025, // Invalid object literal
    
    // Runtime/Interpreter Errors (3000-3999)
    E3001, // Undefined variable
    E3002, // Variable already defined
    E3003, // Function not found
    E3004, // Invalid function call
    E3005, // Wrong number of arguments
    E3006, // Type mismatch
    E3007, // Division by zero
    E3008, // Index out of bounds
    E3009, // Null pointer dereference
    E3010, // Invalid assignment
    E3011, // Cannot modify constant
    E3012, // Class not found
    E3013, // Method not found
    E3014, // Property not found
    E3015, // Invalid this context
    E3016, // Constructor error
    E3017, // Inheritance error
    E3018, // Static method access error
    E3019, // Instance method access error
    E3020, // Invalid array operation
    E3021, // Array method not found
    E3022, // Invalid array index
    E3023, // Array callback error
    E3024, // Stack overflow
    E3025, // Memory limit exceeded
    E3026, // Execution timeout
    E3027, // Invalid cast
    E3028, // Circular reference
    E3029, // Resource not available
    E3030, // Permission denied
    
    // Type System Errors (4000-4999)
    E4001, // Invalid type annotation
    E4002, // Type inference failed
    E4003, // Incompatible types
    E4004, // Generic type error
    E4005, // Interface not implemented
    E4006, // Abstract method not implemented
    E4007, // Type constraint violation
    E4008, // Invalid type parameter
    E4009, // Recursive type definition
    E4010, // Union type error
    
    // I/O Errors (5000-5999)
    E5001, // File not found
    E5002, // Permission denied
    E5003, // I/O error
    E5004, // Invalid file format
    E5005, // File size limit exceeded
    E5006, // Network error
    E5007, // Timeout error
    E5008, // Invalid path
    E5009, // Directory not found
    E5010, // Cannot create file
    E5011, // Cannot write to file
    E5012, // Cannot read from file
    
    // Module System Errors (6000-6999)
    E6001, // Module not found
    E6002, // Circular dependency
    E6003, // Invalid module path
    E6004, // Module loading error
    E6005, // Export not found
    E6006, // Import error
    E6007, // Namespace collision
    E6008, // Invalid namespace access
    E6009, // Module version conflict
    E6010, // Missing module dependency
    
    // Syntax Errors (7000-7999)
    E7001, // Missing closing quote in string
    E7002, // Invalid character in identifier
    E7003, // Invalid operator usage
    E7004, // Missing operator
    E7005, // Invalid bracket nesting
    E7006, // Unexpected end of file
    E7007, // Invalid comment syntax
    E7008, // Invalid keyword usage
    E7009, // Reserved word used as identifier
    E7010, // Invalid statement termination
    
    // Warning Codes (W8000-W8999)
    W8001, // Unused variable
    W8002, // Unused function
    W8003, // Unreachable code
    W8004, // Deprecated feature
    W8005, // Performance warning
    W8006, // Style warning
    W8007, // Missing documentation
    W8008, // Potential null pointer
    W8009, // Implicit type conversion
    W8010, // Large file warning
    
    // System Errors (9000-9999)
    E9001, // Internal compiler error
    E9002, // Memory allocation failed
    E9003, // System resource exhausted
    E9004, // Platform not supported
    E9005, // Configuration error
    E9006, // License error
    E9007, // Version incompatibility
    E9008, // Corrupted data
    E9009, // Security violation
    E9010, // Fatal system error
}

impl ErrorCode {
    pub fn to_number(&self) -> u32 {
        match self {
            // Lexer Errors
            ErrorCode::E1001 => 1001,
            ErrorCode::E1002 => 1002,
            ErrorCode::E1003 => 1003,
            ErrorCode::E1004 => 1004,
            ErrorCode::E1005 => 1005,
            ErrorCode::E1006 => 1006,
            ErrorCode::E1007 => 1007,
            ErrorCode::E1008 => 1008,
            
            // Parser Errors
            ErrorCode::E2001 => 2001,
            ErrorCode::E2002 => 2002,
            ErrorCode::E2003 => 2003,
            ErrorCode::E2004 => 2004,
            ErrorCode::E2005 => 2005,
            ErrorCode::E2006 => 2006,
            ErrorCode::E2007 => 2007,
            ErrorCode::E2008 => 2008,
            ErrorCode::E2009 => 2009,
            ErrorCode::E2010 => 2010,
            ErrorCode::E2011 => 2011,
            ErrorCode::E2012 => 2012,
            ErrorCode::E2013 => 2013,
            ErrorCode::E2014 => 2014,
            ErrorCode::E2015 => 2015,
            ErrorCode::E2016 => 2016,
            ErrorCode::E2017 => 2017,
            ErrorCode::E2018 => 2018,
            ErrorCode::E2019 => 2019,
            ErrorCode::E2020 => 2020,
            ErrorCode::E2021 => 2021,
            ErrorCode::E2022 => 2022,
            ErrorCode::E2023 => 2023,
            ErrorCode::E2024 => 2024,
            ErrorCode::E2025 => 2025,
            
            // Runtime Errors
            ErrorCode::E3001 => 3001,
            ErrorCode::E3002 => 3002,
            ErrorCode::E3003 => 3003,
            ErrorCode::E3004 => 3004,
            ErrorCode::E3005 => 3005,
            ErrorCode::E3006 => 3006,
            ErrorCode::E3007 => 3007,
            ErrorCode::E3008 => 3008,
            ErrorCode::E3009 => 3009,
            ErrorCode::E3010 => 3010,
            ErrorCode::E3011 => 3011,
            ErrorCode::E3012 => 3012,
            ErrorCode::E3013 => 3013,
            ErrorCode::E3014 => 3014,
            ErrorCode::E3015 => 3015,
            ErrorCode::E3016 => 3016,
            ErrorCode::E3017 => 3017,
            ErrorCode::E3018 => 3018,
            ErrorCode::E3019 => 3019,
            ErrorCode::E3020 => 3020,
            ErrorCode::E3021 => 3021,
            ErrorCode::E3022 => 3022,
            ErrorCode::E3023 => 3023,
            ErrorCode::E3024 => 3024,
            ErrorCode::E3025 => 3025,
            ErrorCode::E3026 => 3026,
            ErrorCode::E3027 => 3027,
            ErrorCode::E3028 => 3028,
            ErrorCode::E3029 => 3029,
            ErrorCode::E3030 => 3030,
            
            // Type System Errors
            ErrorCode::E4001 => 4001,
            ErrorCode::E4002 => 4002,
            ErrorCode::E4003 => 4003,
            ErrorCode::E4004 => 4004,
            ErrorCode::E4005 => 4005,
            ErrorCode::E4006 => 4006,
            ErrorCode::E4007 => 4007,
            ErrorCode::E4008 => 4008,
            ErrorCode::E4009 => 4009,
            ErrorCode::E4010 => 4010,
            
            // I/O Errors
            ErrorCode::E5001 => 5001,
            ErrorCode::E5002 => 5002,
            ErrorCode::E5003 => 5003,
            ErrorCode::E5004 => 5004,
            ErrorCode::E5005 => 5005,
            ErrorCode::E5006 => 5006,
            ErrorCode::E5007 => 5007,
            ErrorCode::E5008 => 5008,
            ErrorCode::E5009 => 5009,
            ErrorCode::E5010 => 5010,
            ErrorCode::E5011 => 5011,
            ErrorCode::E5012 => 5012,
            
            // Module System Errors
            ErrorCode::E6001 => 6001,
            ErrorCode::E6002 => 6002,
            ErrorCode::E6003 => 6003,
            ErrorCode::E6004 => 6004,
            ErrorCode::E6005 => 6005,
            ErrorCode::E6006 => 6006,
            ErrorCode::E6007 => 6007,
            ErrorCode::E6008 => 6008,
            ErrorCode::E6009 => 6009,
            ErrorCode::E6010 => 6010,
            
            // Syntax Errors
            ErrorCode::E7001 => 7001,
            ErrorCode::E7002 => 7002,
            ErrorCode::E7003 => 7003,
            ErrorCode::E7004 => 7004,
            ErrorCode::E7005 => 7005,
            ErrorCode::E7006 => 7006,
            ErrorCode::E7007 => 7007,
            ErrorCode::E7008 => 7008,
            ErrorCode::E7009 => 7009,
            ErrorCode::E7010 => 7010,
            
            // Warning Codes
            ErrorCode::W8001 => 8001,
            ErrorCode::W8002 => 8002,
            ErrorCode::W8003 => 8003,
            ErrorCode::W8004 => 8004,
            ErrorCode::W8005 => 8005,
            ErrorCode::W8006 => 8006,
            ErrorCode::W8007 => 8007,
            ErrorCode::W8008 => 8008,
            ErrorCode::W8009 => 8009,
            ErrorCode::W8010 => 8010,
            
            // System Errors
            ErrorCode::E9001 => 9001,
            ErrorCode::E9002 => 9002,
            ErrorCode::E9003 => 9003,
            ErrorCode::E9004 => 9004,
            ErrorCode::E9005 => 9005,
            ErrorCode::E9006 => 9006,
            ErrorCode::E9007 => 9007,
            ErrorCode::E9008 => 9008,
            ErrorCode::E9009 => 9009,
            ErrorCode::E9010 => 9010,
        }
    }
    
    pub fn category(&self) -> &'static str {
        match self.to_number() {
            1000..=1999 => "Lexer",
            2000..=2999 => "Parser",
            3000..=3999 => "Runtime",
            4000..=4999 => "Type System",
            5000..=5999 => "I/O",
            6000..=6999 => "Module System",
            7000..=7999 => "Syntax",
            8000..=8999 => "Warning",
            9000..=9999 => "System",
            _ => "Unknown",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}

#[derive(Debug, Clone)]
pub struct DryadError {
    pub code: ErrorCode,
    pub message: String,
    pub location: Option<(usize, usize)>,
    pub severity: ErrorSeverity,
    pub context: Option<String>,
}

impl DryadError {
    pub fn new(message: String, location: Option<(usize, usize)>, severity: ErrorSeverity) -> Self {
        Self {
            code: ErrorCode::E9001, // Default to internal error
            message,
            location,
            severity,
            context: None,
        }
    }
    
    pub fn with_code(code: ErrorCode, message: String, location: Option<(usize, usize)>) -> Self {
        let severity = match code.to_number() {
            8000..=8999 => ErrorSeverity::Warning,
            _ => ErrorSeverity::Error,
        };
        
        Self {
            code,
            message,
            location,
            severity,
            context: None,
        }
    }
    
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
    
    pub fn syntax_error(message: &str, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E7001,
            message.to_string(),
            Some((line, column))
        )
    }

    pub fn runtime_error(message: &str, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E3001,
            message.to_string(),
            Some((line, column))
        )
    }

    pub fn type_error(message: &str, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E4003,
            message.to_string(),
            Some((line, column))
        )
    }

    pub fn warning(message: &str, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::W8001,
            message.to_string(),
            Some((line, column))
        )
    }

    pub fn io_error(message: &str, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E5003,
            message.to_string(),
            Some((line, column))
        )
    }
    
    // Specific error constructors
    pub fn undefined_variable(var_name: &str, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E3001,
            format!("Undefined variable '{}'", var_name),
            Some((line, column))
        )
    }
    
    pub fn function_not_found(func_name: &str, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E3003,
            format!("Function '{}' not found", func_name),
            Some((line, column))
        )
    }
    
    pub fn method_not_found(method_name: &str, obj_type: &str, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E3013,
            format!("Method '{}' not found on type '{}'", method_name, obj_type),
            Some((line, column))
        )
    }
    
    pub fn array_method_not_found(method_name: &str, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E3021,
            format!("Array method '{}' not found", method_name),
            Some((line, column))
        )
    }
    
    pub fn array_index_out_of_bounds(index: i32, size: usize, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E3022,
            format!("Array index {} out of bounds (array size: {})", index, size),
            Some((line, column))
        )
    }
    
    pub fn wrong_argument_count(expected: usize, got: usize, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E3005,
            format!("Expected {} arguments, got {}", expected, got),
            Some((line, column))
        )
    }
    
    pub fn division_by_zero(line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E3007,
            "Division by zero".to_string(),
            Some((line, column))
        )
    }
    
    pub fn unexpected_token(expected: &str, got: &str, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E2001,
            format!("Expected '{}', got '{}'", expected, got),
            Some((line, column))
        )
    }
    
    pub fn unterminated_string(line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E1002,
            "Unterminated string literal".to_string(),
            Some((line, column))
        )
    }
    
    pub fn invalid_character(ch: char, line: usize, column: usize) -> Self {
        Self::with_code(
            ErrorCode::E1001,
            format!("Unexpected character '{}'", ch),
            Some((line, column))
        )
    }
    
    pub fn file_not_found(path: &str) -> Self {
        Self::with_code(
            ErrorCode::E5001,
            format!("File not found: '{}'", path),
            None
        )
    }
    
    pub fn module_not_found(module_name: &str) -> Self {
        Self::with_code(
            ErrorCode::E6001,
            format!("Module '{}' not found", module_name),
            None
        )
    }
}

impl fmt::Display for DryadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity_prefix = match self.severity {
            ErrorSeverity::Error => "ERROR",
            ErrorSeverity::Warning => "WARNING",
            ErrorSeverity::Info => "INFO",
        };
        
        let location_str = match &self.location {
            Some((line, col)) => format!(" at {}:{}", line, col),
            None => String::new(),
        };
        
        let context_str = match &self.context {
            Some(ctx) => format!("\n  Context: {}", ctx),
            None => String::new(),
        };
        
        write!(f, "{} [{}] ({}): {}{}{}", 
            severity_prefix, 
            self.code.to_number(), 
            self.code.category(),
            self.message, 
            location_str,
            context_str
        )
    }
}

pub struct ErrorReporter {
    errors: Vec<DryadError>,
    max_errors: usize,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            max_errors: 10,
        }
    }

    pub fn with_max_errors(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            max_errors,
        }
    }

    pub fn add_error(&mut self, error: DryadError) {
        if self.errors.len() < self.max_errors {
            self.errors.push(error);
        }
    }

    pub fn report_error(&mut self, error: &DryadError) {
        match error.severity {
            ErrorSeverity::Error => eprintln!("ERROR: {}", error),
            ErrorSeverity::Warning => eprintln!("WARNING: {}", error),
            ErrorSeverity::Info => eprintln!("INFO: {}", error),
        }
    }

    pub fn get_errors(&self) -> &[DryadError] {
        &self.errors
    }

    pub fn get_error_count(&self) -> usize {
        self.errors.iter().filter(|e| matches!(e.severity, ErrorSeverity::Error)).count()
    }

    pub fn get_warning_count(&self) -> usize {
        self.errors.iter().filter(|e| matches!(e.severity, ErrorSeverity::Warning)).count()
    }

    pub fn has_errors(&self) -> bool {
        self.get_error_count() > 0
    }

    pub fn has_warnings(&self) -> bool {
        self.get_warning_count() > 0
    }

    pub fn clear(&mut self) {
        self.errors.clear();
    }

    pub fn report_all(&self) {
        for error in &self.errors {
            match error.severity {
                ErrorSeverity::Error => eprintln!("ERROR: {}", error),
                ErrorSeverity::Warning => eprintln!("WARNING: {}", error),
                ErrorSeverity::Info => eprintln!("INFO: {}", error),
            }
        }
    }

    pub fn format_errors(&self) -> String {
        let mut result = String::new();
        for error in &self.errors {
            result.push_str(&format!("{}\n", error));
        }
        result
    }

    pub fn print_summary(&self) {
        let error_count = self.get_error_count();
        let warning_count = self.get_warning_count();
        
        if error_count > 0 || warning_count > 0 {
            println!();
            println!("Summary: {} errors, {} warnings", error_count, warning_count);
        }
    }
}
