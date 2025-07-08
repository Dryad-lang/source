// src/cli/cli.rs

use std::fs;
use std::path::Path;
use crate::lexer::tokenizer::Lexer;
use crate::parser::parser::Parser;
use crate::interpreter::{env::Env, evaluator::Evaluator};
use crate::interpreter::errors::{DryadError, ErrorSeverity};
use crate::cli::repl::ReplSession;
use crate::oak::cli_integration::{OakCliIntegration, OakCommand};

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionMode {
    RunFile(String),
    Repl,
    Version,
    Help,
    OakInit,
    OakAdd(String),
    OakList,
}

#[derive(Debug, Clone)]
pub struct CliArgs {
    pub mode: ExecutionMode,
    pub verbose: bool,
    pub strict_types: bool,
}

impl CliArgs {
    pub fn parse_from_vec(args: Vec<String>) -> Result<Self, String> {
        let mut mode = ExecutionMode::Help;
        let mut verbose = false;
        let mut strict_types = false;
        let mut file_path = None;
        
        let mut i = 1; // Skip program name
        while i < args.len() {
            match args[i].as_str() {
                "--help" | "-h" => {
                    mode = ExecutionMode::Help;
                    break;
                }
                "--version" | "-v" => {
                    mode = ExecutionMode::Version;
                    break;
                }
                "--repl" | "-r" => {
                    mode = ExecutionMode::Repl;
                }
                "--verbose" => {
                    verbose = true;
                }
                "--strict" => {
                    strict_types = true;
                }
                "oak" => {
                    // Handle oak subcommands
                    i += 1;
                    if i < args.len() {
                        match args[i].as_str() {
                            "init" => {
                                mode = ExecutionMode::OakInit;
                            }
                            "add" => {
                                i += 1;
                                if i < args.len() {
                                    mode = ExecutionMode::OakAdd(args[i].clone());
                                } else {
                                    return Err("oak add requires a package name".to_string());
                                }
                            }
                            "list" => {
                                mode = ExecutionMode::OakList;
                            }
                            _ => {
                                return Err(format!("Unknown oak command: {}", args[i]));
                            }
                        }
                    } else {
                        return Err("oak command requires a subcommand".to_string());
                    }
                }
                arg if arg.starts_with("--") => {
                    return Err(format!("Unknown flag: {}", arg));
                }
                arg => {
                    if file_path.is_none() {
                        file_path = Some(arg.to_string());
                    } else {
                        return Err("Multiple files not supported".to_string());
                    }
                }
            }
            i += 1;
        }
        
        // Se não foi especificado modo explícito e há um arquivo, usar RunFile
        if let ExecutionMode::Help = mode {
            if let Some(file) = file_path {
                mode = ExecutionMode::RunFile(file);
            }
        }
        
        Ok(CliArgs {
            mode,
            verbose,
            strict_types,
        })
    }
    
    pub fn parse_from_env() -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();
        Self::parse_from_vec(args)
    }
}

pub struct DryadCli {
    evaluator: Evaluator,
    args: Option<CliArgs>,
    oak_cli: OakCliIntegration,
}

impl DryadCli {
    pub fn new() -> Self {
        Self {
            evaluator: Evaluator::new(),
            args: None,
            oak_cli: OakCliIntegration::new(),
        }
    }
    
    pub fn with_args(args: CliArgs) -> Self {
        Self {
            evaluator: Evaluator::new(),
            args: Some(args),
            oak_cli: OakCliIntegration::new(),
        }
    }
    
    pub fn run(&mut self, args: Vec<String>) -> Result<(), String> {
        let cli_args = CliArgs::parse_from_vec(args)?;
        self.args = Some(cli_args.clone());
        
        match self.execute(cli_args) {
            Ok(()) => Ok(()),
            Err(error) => Err(error.message),
        }
    }
    
    pub fn execute(&mut self, args: CliArgs) -> Result<(), DryadError> {
        match &args.mode {
            ExecutionMode::Version => {
                println!("Dryad Language v0.1.0");
                Ok(())
            }
            ExecutionMode::Help => {
                self.print_help();
                Ok(())
            }
            ExecutionMode::Repl => {
                self.run_repl(&args)
            }
            ExecutionMode::RunFile(file_path) => {
                self.run_file(file_path, &args)
            }
            ExecutionMode::OakInit => {
                self.oak_init()
            }
            ExecutionMode::OakAdd(package) => {
                self.oak_add(package)
            }
            ExecutionMode::OakList => {
                self.oak_list()
            }
        }
    }
    
    fn print_help(&self) {
        println!("Dryad Language v0.1.0");
        println!("Usage: dryad [OPTIONS] [FILE]");
        println!("       dryad oak <COMMAND>");
        println!();
        println!("OPTIONS:");
        println!("  -h, --help       Show this help message");
        println!("  -v, --version    Show version information");
        println!("  -r, --repl       Start interactive REPL");
        println!("      --verbose    Enable verbose output");
        println!("      --strict     Enable strict type checking");
        println!();
        println!("OAK COMMANDS:");
        println!("  oak init         Initialize a new Oak project");
        println!("  oak add <pkg>    Add a package to the project");
        println!("  oak list         List installed packages");
        println!();
        println!("EXAMPLES:");
        println!("  dryad script.dryad    Run a Dryad script");
        println!("  dryad --repl          Start interactive mode");
        println!("  dryad --strict app.dryad  Run with strict types");
        println!("  dryad oak init        Initialize Oak project");
        println!("  dryad oak add mylib   Add mylib package");
    }
    
    fn run_file(&mut self, file_path: &str, args: &CliArgs) -> Result<(), DryadError> {
        if !Path::new(file_path).exists() {
            return Err(DryadError::new(
                format!("File not found: {}", file_path),
                None,
                ErrorSeverity::Error,
            ));
        }
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| DryadError::new(
                format!("Failed to read file {}: {}", file_path, e),
                None,
                ErrorSeverity::Error,
            ))?;
        
        if args.verbose {
            println!("Executing file: {}", file_path);
            println!("Content length: {} bytes", content.len());
        }
        
        self.execute_code(&content, Some(file_path), args)
    }
    
    fn run_repl(&mut self, args: &CliArgs) -> Result<(), DryadError> {
        if args.verbose {
            println!("Starting REPL mode...");
        }
        
        let mut repl = ReplSession::new(args.verbose, args.strict_types);
        repl.run().map_err(|e| DryadError::new(
            format!("REPL error: {}", e),
            None,
            ErrorSeverity::Error,
        ))
    }
    
    fn execute_code(&mut self, code: &str, _file_name: Option<&str>, args: &CliArgs) -> Result<(), DryadError> {
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);
        let mut env = Env::new();
        
        if args.verbose {
            println!("Parsing code: {}", code);
        }
        
        let mut statement_count = 0;
        
        // Parse and execute statements
        while let Some(stmt) = parser.parse_statement() {
            statement_count += 1;
            if args.verbose {
                println!("Executing statement #{}: {:?}", statement_count, stmt);
            }
            
            let result = self.evaluator.eval_stmt(&stmt, &mut env);
            
            if !result.errors.is_empty() {
                self.evaluator.report_errors(&result.errors);
                if result.errors.iter().any(|e| matches!(e.severity, ErrorSeverity::Error)) {
                    return Err(DryadError::new(
                        "Execution failed with errors".to_string(),
                        None,
                        ErrorSeverity::Error,
                    ));
                }
            }
            
            if let Some(value) = result.value {
                if args.verbose {
                    println!("Result: {:?}", value);
                }
            }
        }
        
        if args.verbose {
            println!("Total statements processed: {}", statement_count);
        }
        
        Ok(())
    }

    // Oak Package Manager Methods

    fn oak_init(&self) -> Result<(), DryadError> {
        let command = OakCommand::Init { 
            name: None, 
            description: None 
        };
        
        let result = self.oak_cli.execute_command(command);
        
        if result.success {
            println!("{}", result.output);
            Ok(())
        } else {
            Err(DryadError::new(
                result.error.unwrap_or("Oak init failed".to_string()),
                None,
                ErrorSeverity::Error,
            ))
        }
    }

    fn oak_add(&self, package: &str) -> Result<(), DryadError> {
        let command = OakCommand::Add { 
            package: package.to_string(), 
            version: None, 
            dev: false 
        };
        
        let result = self.oak_cli.execute_command(command);
        
        if result.success {
            println!("{}", result.output);
            Ok(())
        } else {
            Err(DryadError::new(
                result.error.unwrap_or("Oak add failed".to_string()),
                None,
                ErrorSeverity::Error,
            ))
        }
    }

    fn oak_list(&self) -> Result<(), DryadError> {
        let command = OakCommand::List { 
            dev: false, 
            production: true 
        };
        
        let result = self.oak_cli.execute_command(command);
        
        if result.success {
            println!("{}", result.output);
            Ok(())
        } else {
            Err(DryadError::new(
                result.error.unwrap_or("Oak list failed".to_string()),
                None,
                ErrorSeverity::Error,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_args_parsing() {
        let args = vec!["dryad".to_string(), "test.dryad".to_string()];
        let cli_args = CliArgs::parse_from_vec(args).unwrap();
        
        assert!(matches!(cli_args.mode, ExecutionMode::RunFile(_)));
        assert!(!cli_args.verbose);
        assert!(!cli_args.strict_types);
    }
    
    #[test]
    fn test_flags() {
        let args = vec![
            "dryad".to_string(),
            "--verbose".to_string(),
            "--strict".to_string(),
            "test.dryad".to_string()
        ];
        let cli_args = CliArgs::parse_from_vec(args).unwrap();
        
        assert!(cli_args.verbose);
        assert!(cli_args.strict_types);
    }
}
