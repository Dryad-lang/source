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

#[derive(Debug, Clone)]
pub struct DryadError {
    pub message: String,
    pub location: Option<(usize, usize)>,
    pub severity: ErrorSeverity,
}

impl DryadError {
    pub fn new(message: String, location: Option<(usize, usize)>, severity: ErrorSeverity) -> Self {
        Self {
            message,
            location,
            severity,
        }
    }
    
    pub fn syntax_error(message: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.to_string(),
            location: Some((line, column)),
            severity: ErrorSeverity::Error,
        }
    }

    pub fn runtime_error(message: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.to_string(),
            location: Some((line, column)),
            severity: ErrorSeverity::Error,
        }
    }

    pub fn type_error(message: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.to_string(),
            location: Some((line, column)),
            severity: ErrorSeverity::Error,
        }
    }

    pub fn warning(message: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.to_string(),
            location: Some((line, column)),
            severity: ErrorSeverity::Warning,
        }
    }

    pub fn io_error(message: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.to_string(),
            location: Some((line, column)),
            severity: ErrorSeverity::Error,
        }
    }
}

impl fmt::Display for DryadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.location {
            Some((line, col)) => write!(f, "[{}:{}] {}", line, col, self.message),
            None => write!(f, "{}", self.message),
        }
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
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            max_errors,
        }
    }

    pub fn report_error(&mut self, error: &DryadError) {
        match error {
            DryadError::Warning { .. } => {
                self.warnings.push(error.clone());
            }
            _ => {
                self.errors.push(error.clone());
                if self.errors.len() >= self.max_errors {
                    eprintln!("Too many errors, stopping compilation");
                }
            }
        }
    }

    pub fn get_errors(&self) -> &Vec<DryadError> {
        &self.errors
    }

    pub fn get_warnings(&self) -> &Vec<DryadError> {
        &self.warnings
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
    }

    pub fn format_error(&self, error: &DryadError, source_code: &str) -> String {
        let lines: Vec<&str> = source_code.lines().collect();
        let location = match error {
            DryadError::SyntaxError { location, .. } |
            DryadError::RuntimeError { location, .. } |
            DryadError::TypeError { location, .. } |
            DryadError::Warning { location, .. } |
            DryadError::IoError { location, .. } => location,
        };

        let mut output = String::new();
        
        // Cabeçalho do erro
        output.push_str(&format!("{}\n", error));
        output.push_str(&format!("  --> line {}, column {}\n", location.line, location.column));
        
        // Mostrar a linha com erro se disponível
        if location.line > 0 && location.line <= lines.len() {
            let line = lines[location.line - 1];
            output.push_str(&format!("   |\n"));
            output.push_str(&format!("{:3} | {}\n", location.line, line));
            output.push_str(&format!("   | "));
            
            // Adicionar indicador de posição
            for _ in 0..location.column.saturating_sub(1) {
                output.push(' ');
            }
            output.push_str("^\n");
        }

        output
    }

    pub fn print_all_errors(&self, source_code: &str) {
        for error in &self.errors {
            eprintln!("{}", self.format_error(error, source_code));
        }
        
        for warning in &self.warnings {
            eprintln!("{}", self.format_error(warning, source_code));
        }

        if !self.errors.is_empty() {
            eprintln!("\n{} error(s) found", self.errors.len());
        }
        if !self.warnings.is_empty() {
            eprintln!("{} warning(s) found", self.warnings.len());
        }
    }
}

impl fmt::Display for DryadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DryadError::SyntaxError { message, .. } => {
                write!(f, "Syntax Error: {}", message)
            }
            DryadError::RuntimeError { message, .. } => {
                write!(f, "Runtime Error: {}", message)
            }
            DryadError::TypeError { message, .. } => {
                write!(f, "Type Error: {}", message)
            }
            DryadError::Warning { message, .. } => {
                write!(f, "Warning: {}", message)
            }
            DryadError::IoError { message, .. } => {
                write!(f, "I/O Error: {}", message)
            }
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

// Funções utilitárias para criar erros comuns
impl DryadError {
    pub fn syntax_error(message: &str, line: usize, column: usize) -> Self {
        DryadError::SyntaxError {
            message: message.to_string(),
            location: SourceLocation { line, column },
        }
    }

    pub fn runtime_error(message: &str, line: usize, column: usize) -> Self {
        DryadError::RuntimeError {
            message: message.to_string(),
            location: SourceLocation { line, column },
        }
    }

    pub fn type_error(message: &str, line: usize, column: usize) -> Self {
        DryadError::TypeError {
            message: message.to_string(),
            location: SourceLocation { line, column },
        }
    }

    pub fn warning(message: &str, line: usize, column: usize) -> Self {
        DryadError::Warning {
            message: message.to_string(),
            location: SourceLocation { line, column },
        }
    }

    pub fn io_error(message: &str, line: usize, column: usize) -> Self {
        DryadError::IoError {
            message: message.to_string(),
            location: SourceLocation { line, column },
        }
    }
}
