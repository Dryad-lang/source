// src/cli/repl.rs

use std::io::{self, Write};
use crate::lexer::tokenizer::Lexer;
use crate::parser::parser::Parser;
use crate::interpreter::{env::Env, evaluator::Evaluator};
use crate::interpreter::errors::{ErrorSeverity};

pub struct ReplSession {
    env: Env,
    evaluator: Evaluator,
    verbose: bool,
    strict_types: bool,
    history: Vec<String>,
}

impl ReplSession {
    pub fn new(verbose: bool, strict_types: bool) -> Self {
        Self {
            env: Env::new(),
            evaluator: Evaluator::new(),
            verbose,
            strict_types,
            history: Vec::new(),
        }
    }
    
    pub fn run(&mut self) -> Result<(), String> {
        println!("Dryad REPL v0.1.0");
        println!("Type 'exit' or press Ctrl+C to quit");
        println!("Type 'help' for available commands");
        if self.strict_types {
            println!("Strict type checking is enabled");
        }
        println!();
        
        loop {
            print!("dryad> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    // EOF reached
                    println!("\nGoodbye!");
                    break;
                }
                Ok(_) => {
                    let input = input.trim();
                    
                    // Handle special commands
                    match input {
                        "" => continue,
                        "exit" | "quit" => {
                            println!("Goodbye!");
                            break;
                        }
                        "help" => {
                            self.print_repl_help();
                            continue;
                        }
                        "clear" => {
                            self.clear_environment();
                            println!("Environment cleared");
                            continue;
                        }
                        "history" => {
                            self.print_history();
                            continue;
                        }
                        "env" => {
                            self.print_environment();
                            continue;
                        }
                        cmd if cmd.starts_with("type ") => {
                            let var_name = cmd.strip_prefix("type ").unwrap().trim();
                            self.print_variable_type(var_name);
                            continue;
                        }
                        _ => {
                            // Try to execute as Dryad code
                            self.history.push(input.to_string());
                            
                            if let Err(err) = self.execute_line(input) {
                                println!("Error: {}", err);
                            }
                        }
                    }
                }
                Err(error) => {
                    println!("Error reading input: {}", error);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    fn execute_line(&mut self, input: &str) -> Result<(), String> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        
        if let Some(stmt) = parser.parse_statement() {
            let result = self.evaluator.eval_stmt(&stmt, &mut self.env);
            
            if !result.errors.is_empty() {
                for error in &result.errors {
                    match error.severity {
                        ErrorSeverity::Error => return Err(error.message.clone()),
                        ErrorSeverity::Warning => println!("Warning: {}", error.message),
                        ErrorSeverity::Info => {
                            if self.verbose {
                                println!("Info: {}", error.message);
                            }
                        }
                    }
                }
            }
            
            if let Some(value) = result.value {
                println!("{:?}", value);
            }
            
            Ok(())
        } else {
            Err("Failed to parse input".to_string())
        }
    }
    
    fn print_repl_help(&self) {
        println!("REPL Commands:");
        println!("  help        Show this help message");
        println!("  exit/quit   Exit the REPL");
        println!("  clear       Clear the environment (reset all variables)");
        println!("  history     Show command history");
        println!("  env         Show current environment variables");
        println!("  type <var>  Show the type of a variable");
        println!();
        println!("Dryad Language Features:");
        println!("  Variables:    let x = 10;");
        println!("  Functions:    fn add(a, b) => a + b;");
        println!("  Expressions:  x + y * 2");
        println!("  Conditionals: if x > 5 then x else 0");
        println!();
    }
    
    fn clear_environment(&mut self) {
        self.env = Env::new();
        self.evaluator = Evaluator::new();
    }
    
    fn print_history(&self) {
        if self.history.is_empty() {
            println!("No command history");
            return;
        }
        
        println!("Command History:");
        for (i, cmd) in self.history.iter().enumerate() {
            println!("  {}: {}", i + 1, cmd);
        }
    }
    
    fn print_environment(&self) {
        println!("Current Environment:");
        // TODO: Implementar inspeção do ambiente
        println!("  (Environment inspection not yet implemented)");
    }
    
    fn print_variable_type(&self, var_name: &str) {
        if let Some(value) = self.env.get(var_name) {
            let type_name = match value {
                crate::interpreter::env::Value::Number(_) => "Number",
                crate::interpreter::env::Value::String(_) => "String", 
                crate::interpreter::env::Value::Bool(_) => "Bool",
                crate::interpreter::env::Value::Null => "Null",
            };
            println!("Type of '{}': {}", var_name, type_name);
        } else {
            println!("Variable '{}' not found", var_name);
        }
    }
}

// Additional helper functions for REPL
impl ReplSession {
    pub fn load_script(&mut self, file_path: &str) -> Result<(), String> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;
        
        println!("Loading script: {}", file_path);
        self.execute_line(&content)
    }
    
    pub fn save_session(&self, file_path: &str) -> Result<(), String> {
        let content = self.history.join("\n");
        std::fs::write(file_path, content)
            .map_err(|e| format!("Failed to save session to {}: {}", file_path, e))?;
        
        println!("Session saved to: {}", file_path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_repl_creation() {
        let repl = ReplSession::new(false, false);
        assert!(!repl.verbose);
        assert!(!repl.strict_types);
    }
    
    #[test]
    fn test_repl_with_strict_types() {
        let repl = ReplSession::new(false, true);
        assert!(repl.strict_types);
    }
    
    #[test]
    fn test_simple_execution() {
        let mut repl = ReplSession::new(false, false);
        let result = repl.execute_line("let x = 42;");
        assert!(result.is_ok());
    }
}
