// tests/user/test_runner.rs
//! Sistema de execução de testes de usuário
//! 
//! Este sistema executa scripts .dryad para validar
//! toda a sintaxe e funcionalidade da linguagem

use std::fs;
use std::path::Path;
use dryad::{
    lexer::tokenizer::Lexer,
    parser::parser::Parser,
    interpreter::evaluator::Evaluator,
    interpreter::env::Env,
};

struct UserTestResult {
    name: String,
    passed: bool,
    expected_output: Option<String>,
    actual_output: String,
    errors: Vec<String>,
}

pub fn run_user_tests() -> Vec<UserTestResult> {
    let test_dir = "tests/user/scripts";
    let mut results = Vec::new();
    
    if let Ok(entries) = fs::read_dir(test_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("dryad") {
                    let result = run_single_test(&path);
                    results.push(result);
                }
            }
        }
    }
    
    results
}

fn run_single_test(path: &Path) -> UserTestResult {
    let test_name = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    match fs::read_to_string(path) {
        Ok(content) => {
            let expected_output = extract_expected_output(&content);
            let (actual_output, errors) = execute_dryad_code(&content);
            
            let passed = if let Some(expected) = &expected_output {
                actual_output.trim() == expected.trim()
            } else {
                errors.is_empty()
            };
            
            UserTestResult {
                name: test_name,
                passed,
                expected_output,
                actual_output,
                errors,
            }
        }
        Err(e) => UserTestResult {
            name: test_name,
            passed: false,
            expected_output: None,
            actual_output: String::new(),
            errors: vec![format!("Failed to read file: {}", e)],
        }
    }
}

fn extract_expected_output(content: &str) -> Option<String> {
    // Procura por comentários como: // EXPECTED: Hello World
    for line in content.lines() {
        if let Some(output) = line.strip_prefix("// EXPECTED:") {
            return Some(output.trim().to_string());
        }
    }
    None
}

fn execute_dryad_code(code: &str) -> (String, Vec<String>) {
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let mut evaluator = Evaluator::new();
    let mut env = Env::new();
    let mut output = String::new();
    let mut errors = Vec::new();

    while let Some(stmt) = parser.parse_statement() {
        let result = evaluator.eval_stmt(&stmt, &mut env);
        
        if !result.errors.is_empty() {
            for error in result.errors {
                errors.push(format!("{:?}", error));
            }
        }
        
        // Capturar output de print() seria necessário modificar o evaluator
        // Por enquanto, simulamos baseado no resultado
        if let Some(value) = result.value {
            match value {
                dryad::interpreter::env::Value::String(s) => output.push_str(&s),
                dryad::interpreter::env::Value::Number(n) => output.push_str(&n.to_string()),
                dryad::interpreter::env::Value::Bool(b) => output.push_str(&b.to_string()),
                _ => {}
            }
        }
    }
    
    (output, errors)
}

#[cfg(test)]
mod user_tests {
    use super::*;

    #[test]
    fn run_all_user_tests() {
        println!("\n🎭 === DRYAD USER FUNCTIONALITY TESTS ===\n");
        
        let results = run_user_tests();
        let mut passed = 0;
        let mut total = 0;
        
        for result in &results {
            total += 1;
            if result.passed {
                passed += 1;
                println!("✅ {}", result.name);
            } else {
                println!("❌ {}", result.name);
                if !result.errors.is_empty() {
                    println!("   Errors: {:?}", result.errors);
                }
                if let Some(expected) = &result.expected_output {
                    println!("   Expected: {}", expected);
                    println!("   Actual: {}", result.actual_output);
                }
            }
        }
        
        println!("\n📊 SUMMARY: {}/{} tests passed", passed, total);
        
        if total == 0 {
            println!("⚠️  No test scripts found in tests/user/scripts/");
            println!("   Create .dryad files to test language features");
        }
    }
}
