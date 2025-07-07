// tests/cli_tests.rs

use dryad::cli::{CliArgs, DryadCli, ExecutionMode};

#[test]
fn test_cli_argument_parsing() {
    let args = vec!["dryad".to_string(), "script.dryad".to_string()];
    let cli_args = CliArgs::parse_from_vec(args).unwrap();
    
    assert_eq!(cli_args.mode, ExecutionMode::RunFile("script.dryad".to_string()));
    assert!(!cli_args.verbose);
    assert!(!cli_args.strict_types);
}

#[test]
fn test_cli_repl_mode() {
    let args = vec!["dryad".to_string(), "--repl".to_string()];
    let cli_args = CliArgs::parse_from_vec(args).unwrap();
    
    assert_eq!(cli_args.mode, ExecutionMode::Repl);
}

#[test]
fn test_cli_verbose_flag() {
    let args = vec!["dryad".to_string(), "--verbose".to_string(), "test.dryad".to_string()];
    let cli_args = CliArgs::parse_from_vec(args).unwrap();
    
    assert!(cli_args.verbose);
}

#[test]
fn test_cli_strict_types_flag() {
    let args = vec!["dryad".to_string(), "--strict".to_string(), "test.dryad".to_string()];
    let cli_args = CliArgs::parse_from_vec(args).unwrap();
    
    assert!(cli_args.strict_types);
}

#[test]
fn test_cli_version_flag() {
    let args = vec!["dryad".to_string(), "--version".to_string()];
    let cli_args = CliArgs::parse_from_vec(args).unwrap();
    
    assert_eq!(cli_args.mode, ExecutionMode::Version);
}

#[test]
fn test_cli_help_flag() {
    let args = vec!["dryad".to_string(), "--help".to_string()];
    let cli_args = CliArgs::parse_from_vec(args).unwrap();
    
    assert_eq!(cli_args.mode, ExecutionMode::Help);
}

#[test]
fn test_cli_invalid_arguments() {
    let args = vec!["dryad".to_string(), "--invalid-flag".to_string()];
    let result = CliArgs::parse_from_vec(args);
    
    assert!(result.is_err());
}

#[test]
fn test_file_execution() {
    // Criar um arquivo temporário para teste
    let test_code = r#"
        let x = 10;
        let y = 20;
        print(x + y);
    "#;
    
    std::fs::write("test_script.dryad", test_code).unwrap();
    
    let args = vec!["dryad".to_string(), "test_script.dryad".to_string()];
    let cli_args = CliArgs::parse_from_vec(args).unwrap();
    let mut cli = DryadCli::new();
    
    let result = cli.execute(cli_args);
    assert!(result.is_ok());
    
    // Limpar arquivo de teste
    std::fs::remove_file("test_script.dryad").unwrap();
}
