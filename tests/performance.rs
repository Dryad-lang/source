// tests/performance.rs
// Testes de performance e benchmarks para Dryad

use dryad::lexer::Lexer;
use dryad::parser::parser::Parser;
use dryad::interpreter::{env::Env, evaluator::Evaluator};
use dryad::cli::DryadCli;
use std::time::{Duration, Instant};
use std::fs;
use tempfile::TempDir;

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_lexer_performance() {
        // Programa grande para testar performance do lexer
        let large_program = (0..1000)
            .map(|i| format!("let var{} = {};", i, i))
            .collect::<Vec<_>>()
            .join("\n");

        let start = Instant::now();
        let mut lexer = Lexer::new(&large_program);
        
        let mut token_count = 0;
        loop {
            let token = lexer.next_token();
            token_count += 1;
            if token == dryad::lexer::token::Token::Eof {
                break;
            }
        }
        
        let duration = start.elapsed();
        
        println!("Lexer processed {} tokens in {:?}", token_count, duration);
        
        // Deve processar tokens rapidamente (menos de 100ms para 1000 variáveis)
        assert!(duration < Duration::from_millis(100));
        assert!(token_count > 4000); // let, var, =, number, ; para cada linha
    }

    #[test]
    fn test_parser_performance() {
        // Programa com expressões matemáticas complexas
        let complex_program = (0..100)
            .map(|i| format!("let result{} = {} + {} * {} - {} / {};", i, i, i+1, i+2, i+3, i+4))
            .collect::<Vec<_>>()
            .join("\n");

        let start = Instant::now();
        let lexer = Lexer::new(&complex_program);
        let mut parser = Parser::new(lexer);
        
        let mut stmt_count = 0;
        while let Some(_stmt) = parser.parse_statement() {
            stmt_count += 1;
        }
        
        let duration = start.elapsed();
        
        println!("Parser processed {} statements in {:?}", stmt_count, duration);
        
        // Deve parsear rapidamente (menos de 200ms para 100 expressões complexas)
        assert!(duration < Duration::from_millis(200));
        assert_eq!(stmt_count, 100);
    }

    #[test]
    fn test_evaluator_performance() {
        let mut env = Env::new();
        let mut evaluator = Evaluator::new();
        
        // Cria programa com muitas operações matemáticas
        let math_program = r#"
            let a = 1;
            let b = 2;
            let c = 3;
            let d = 4;
            let e = 5;
            
            let result1 = a + b * c - d / e;
            let result2 = (a + b) * (c - d) + e;
            let result3 = a * b * c * d * e;
            let result4 = ((a + b) * c + d) * e;
            let result5 = a + b + c + d + e;
        "#;

        let start = Instant::now();
        
        let lexer = Lexer::new(math_program);
        let mut parser = Parser::new(lexer);
        
        while let Some(stmt) = parser.parse_statement() {
            evaluator.eval_stmt(&stmt, &mut env);
        }
        
        let duration = start.elapsed();
        
        println!("Evaluator processed complex math in {:?}", duration);
        
        // Deve avaliar rapidamente (menos de 50ms)
        assert!(duration < Duration::from_millis(50));
        
        // Verifica que os valores foram calculados
        assert!(env.get("result1").is_some());
        assert!(env.get("result2").is_some());
        assert!(env.get("result3").is_some());
        assert!(env.get("result4").is_some());
        assert!(env.get("result5").is_some());
    }

    #[test]
    fn test_large_program_execution() {
        let temp_dir = TempDir::new().unwrap();
        let program_file = temp_dir.path().join("large.dryad");
        
        // Programa bem simples para teste rápido
        let simple_program = r#"
            let x = 1;
            let y = 2;
            let z = x + y;
        "#;
        
        fs::write(&program_file, simple_program).unwrap();
        
        let start = Instant::now();
        
        let mut cli = DryadCli::new();
        let args = vec![
            "dryad".to_string(),
            program_file.to_string_lossy().to_string(),
        ];
        
        let result = cli.run(args);
        let duration = start.elapsed();
        
        println!("Simple program executed in {:?}", duration);
        
        assert!(result.is_ok());
        // Deve executar rapidamente
        assert!(duration < Duration::from_millis(50));
    }

    #[test]
    fn test_memory_usage_stability() {
        // Testa se não há vazamentos de memória óbvios
        let program = r#"
            let x = 42;
            let y = "hello";
            let z = true;
        "#;
        
        // Executa o mesmo programa várias vezes
        for _i in 0..100 {
            let mut env = Env::new();
            let mut evaluator = Evaluator::new();
            
            let lexer = Lexer::new(program);
            let mut parser = Parser::new(lexer);
            
            while let Some(stmt) = parser.parse_statement() {
                evaluator.eval_stmt(&stmt, &mut env);
            }
            
            // Verifica que o ambiente foi criado corretamente
            assert!(env.get("x").is_some());
            assert!(env.get("y").is_some());
            assert!(env.get("z").is_some());
        }
    }

    #[test]
    fn test_string_operations_performance() {
        let mut env = Env::new();
        let mut evaluator = Evaluator::new();
        
        // Programa com muitas operações de string
        let string_program = r#"
            let str1 = "Hello";
            let str2 = " ";
            let str3 = "World";
            let str4 = "!";
            
            let result1 = str1 + str2;
            let result2 = result1 + str3;
            let result3 = result2 + str4;
        "#;

        let start = Instant::now();
        
        let lexer = Lexer::new(string_program);
        let mut parser = Parser::new(lexer);
        
        while let Some(stmt) = parser.parse_statement() {
            evaluator.eval_stmt(&stmt, &mut env);
        }
        
        let duration = start.elapsed();
        
        println!("String operations completed in {:?}", duration);
        
        // Deve processar strings rapidamente (menos de 10ms)
        assert!(duration < Duration::from_millis(10));
        
        // Verifica resultado final
        if let Some(dryad::interpreter::env::Value::String(result)) = env.get("result3") {
            assert_eq!(result, "Hello World!");
        } else {
            panic!("Expected string result");
        }
    }

    #[test]
    fn test_nested_expressions_performance() {
        let mut env = Env::new();
        let mut evaluator = Evaluator::new();
        
        // Expressões profundamente aninhadas
        let nested_program = r#"
            let result = ((((1 + 2) * 3) + 4) * ((5 + 6) - 7)) + (((8 * 9) / 10) + 11);
        "#;

        let start = Instant::now();
        
        let lexer = Lexer::new(nested_program);
        let mut parser = Parser::new(lexer);
        
        if let Some(stmt) = parser.parse_statement() {
            evaluator.eval_stmt(&stmt, &mut env);
        }
        
        let duration = start.elapsed();
        
        println!("Nested expressions evaluated in {:?}", duration);
        
        // Deve avaliar rapidamente mesmo com aninhamento profundo
        assert!(duration < Duration::from_millis(5));
        
        // Verifica que o resultado foi calculado
        assert!(env.get("result").is_some());
    }
}

#[cfg(test)]
mod scalability_tests {
    use super::*;

    #[test]
    fn test_many_variables_performance() {
        let mut env = Env::new();
        let mut evaluator = Evaluator::new();
        
        let start = Instant::now();
        
        // Cria e acessa muitas variáveis
        for i in 0..1000 {
            let program = format!("let var{} = {};", i, i);
            let lexer = Lexer::new(&program);
            let mut parser = Parser::new(lexer);
            
            if let Some(stmt) = parser.parse_statement() {
                evaluator.eval_stmt(&stmt, &mut env);
            }
        }
        
        let duration = start.elapsed();
        
        println!("Created 1000 variables in {:?}", duration);
        
        // Deve criar variáveis rapidamente
        assert!(duration < Duration::from_millis(1000));
        
        // Verifica algumas variáveis
        assert!(env.get("var0").is_some());
        assert!(env.get("var500").is_some());
        assert!(env.get("var999").is_some());
    }

    #[test]
    fn test_deep_call_stack_performance() {
        // Simula muitas operações em sequência (não recursão real ainda)
        let mut env = Env::new();
        let mut evaluator = Evaluator::new();
        
        let start = Instant::now();
        
        // Muitas operações em sequência
        for i in 0..100 {
            let program = format!("let step{} = step{} + 1;", i+1, i);
            
            // Primeira iteração precisa inicializar
            let program = if i == 0 {
                "let step0 = 0; let step1 = step0 + 1;".to_string()
            } else {
                program
            };
            
            let lexer = Lexer::new(&program);
            let mut parser = Parser::new(lexer);
            
            while let Some(stmt) = parser.parse_statement() {
                evaluator.eval_stmt(&stmt, &mut env);
            }
        }
        
        let duration = start.elapsed();
        
        println!("Deep operations completed in {:?}", duration);
        
        // Deve processar operações encadeadas rapidamente
        assert!(duration < Duration::from_millis(200));
    }
}
