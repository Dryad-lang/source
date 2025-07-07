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
