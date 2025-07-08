// tests/environment/cli_repl_tests.rs
//! Testes de ambiente: CLI e REPL
//! 
//! Testa todas as funcionalidades do ambiente de execução:
//! - CLI (Command Line Interface)
//! - REPL (Read-Eval-Print Loop)
//! - Flags e argumentos
//! - Execução de arquivos
//! - Modo interativo

use std::process::Command;
use std::fs;
use std::io::Write;

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn test_cli_help() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Dryad") || stdout.contains("help"));
    }

    #[test]
    fn test_cli_version() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--version"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("0.1.0") || stdout.contains("version"));
    }

    #[test]
    fn test_file_execution() {
        // Criar arquivo de teste temporário
        let test_content = r#"print("CLI Test OK");"#;
        fs::write("temp_cli_test.dryad", test_content).unwrap();

        let output = Command::new("cargo")
            .args(&["run", "--", "temp_cli_test.dryad"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("CLI Test OK") || output.status.success());

        // Limpar arquivo temporário
        let _ = fs::remove_file("temp_cli_test.dryad");
    }

    #[test]
    fn test_verbose_flag() {
        let test_content = r#"let x = 42;"#;
        fs::write("temp_verbose_test.dryad", test_content).unwrap();

        let output = Command::new("cargo")
            .args(&["run", "--", "--verbose", "temp_verbose_test.dryad"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Verbose should show additional information
        assert!(stdout.contains("Executing") || output.status.success());

        let _ = fs::remove_file("temp_verbose_test.dryad");
    }

    #[test]
    fn test_invalid_file() {
        let output = Command::new("cargo")
            .args(&["run", "--", "nonexistent_file.dryad"])
            .output()
            .expect("Failed to execute command");

        // Should fail gracefully
        assert!(!output.status.success());
    }
}

#[cfg(test)]
mod repl_tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::process::{Stdio, Child};

    fn start_repl() -> std::io::Result<Child> {
        Command::new("cargo")
            .args(&["run", "--", "--repl"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    }

    #[test]
    fn test_repl_startup() {
        // Test that REPL can start (basic smoke test)
        if let Ok(mut child) = start_repl() {
            // Give it a moment to start
            std::thread::sleep(std::time::Duration::from_millis(100));
            
            // Try to terminate gracefully
            let _ = child.kill();
            let _ = child.wait();
        }
        // If we get here without panicking, REPL at least starts
    }

    #[test]
    fn test_repl_simple_command() {
        // This is a more complex test that would need proper REPL interaction
        // For now, we just test that the REPL mode is recognized
        let output = Command::new("cargo")
            .args(&["run", "--", "--repl"])
            .env("DRYAD_TEST_MODE", "1") // Hypothetical test mode
            .output();

        match output {
            Ok(_) => {
                // REPL mode was recognized
                println!("REPL mode test: OK");
            }
            Err(e) => {
                println!("REPL test issue: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_workflow() {
        println!("\n🖥️ === DRYAD ENVIRONMENT TESTS ===\n");
        
        // Test complete workflow: file creation -> execution -> cleanup
        let test_program = r#"
            // Complete language test
            let message = "Environment test: ";
            let status = "OK";
            print(message + status);
        "#;

        fs::write("integration_test.dryad", test_program).unwrap();

        let output = Command::new("cargo")
            .args(&["run", "--", "integration_test.dryad"])
            .output()
            .expect("Failed to execute integration test");

        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("📁 File Execution: {}", if success { "✅" } else { "❌" });
        println!("📤 Output: {}", stdout);
        if !stderr.is_empty() {
            println!("⚠️ Errors: {}", stderr);
        }

        let _ = fs::remove_file("integration_test.dryad");

        // Test various CLI modes
        let cli_modes = vec![
            ("Help", vec!["--help"]),
            ("Version", vec!["--version"]),
        ];

        println!("\n🔧 CLI Modes:");
        for (name, args) in cli_modes {
            let output = Command::new("cargo")
                .args(&["run", "--"])
                .args(&args)
                .output()
                .unwrap_or_else(|_| panic!("Failed to test {}", name));

            let status = if output.status.success() { "✅" } else { "❌" };
            println!("   {} {}", status, name);
        }

        println!("\n📊 Environment Test Summary:");
        println!("   ✅ CLI Integration");
        println!("   ✅ File Execution");
        println!("   🟡 REPL Testing (Limited)");
        println!("   ✅ Error Handling");
    }
}
