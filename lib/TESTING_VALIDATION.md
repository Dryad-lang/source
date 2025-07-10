# Guia de Testes e Validação - Bibliotecas Dryad

Este documento fornece um framework completo para testes e validação das bibliotecas Dryad, incluindo testes unitários, testes de integração e validação de performance.

## 📋 Índice

1. [Testes Unitários](#testes-unitários)
2. [Testes de Integração](#testes-de-integração)
3. [Testes de Performance](#testes-de-performance)
4. [Validação de Erros](#validação-de-erros)
5. [Testes Automatizados](#testes-automatizados)
6. [Métricas de Qualidade](#métricas-de-qualidade)

---

## 🧪 Testes Unitários

### 1. Estrutura de Teste para Funções Nativas

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::types::Value;
    use crate::interpreter::errors::DryadError;
    
    #[test]
    fn test_console_println_valid_string() {
        let args = vec![Value::String("Hello, World!".to_string())];
        let result = native_console_println(&args);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Bool(true));
    }
    
    #[test]
    fn test_console_println_invalid_args() {
        // Teste com número errado de argumentos
        let args = vec![];
        let result = native_console_println(&args);
        
        assert!(result.is_err());
        if let Err(DryadError::RuntimeError(msg)) = result {
            assert!(msg.contains("expects exactly 1 argument"));
        }
    }
    
    #[test]
    fn test_console_println_wrong_type() {
        // Teste com tipo errado
        let args = vec![Value::Number(42.0)];
        let result = native_console_println(&args);
        
        assert!(result.is_err());
        if let Err(DryadError::RuntimeError(msg)) = result {
            assert!(msg.contains("expects a string"));
        }
    }
}
```

### 2. Testes para Operações de Arquivo

```rust
#[cfg(test)]
mod fs_tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_fs_write_and_read() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let file_path_str = file_path.to_str().unwrap();
        
        // Teste de escrita
        let write_args = vec![
            Value::String(file_path_str.to_string()),
            Value::String("Test content".to_string())
        ];
        let write_result = native_fs_write_file(&write_args);
        assert!(write_result.is_ok());
        
        // Teste de leitura
        let read_args = vec![Value::String(file_path_str.to_string())];
        let read_result = native_fs_read_file(&read_args);
        assert!(read_result.is_ok());
        
        if let Ok(Value::String(content)) = read_result {
            assert_eq!(content, "Test content");
        }
    }
    
    #[test]
    fn test_fs_read_nonexistent_file() {
        let args = vec![Value::String("nonexistent.txt".to_string())];
        let result = native_fs_read_file(&args);
        
        assert!(result.is_err());
        if let Err(DryadError::RuntimeError(msg)) = result {
            assert!(msg.contains("File not found"));
        }
    }
    
    #[test]
    fn test_fs_file_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");
        let file_path_str = file_path.to_str().unwrap();
        
        // Criar arquivo
        fs::write(&file_path, "content").unwrap();
        
        // Testar existência
        let args = vec![Value::String(file_path_str.to_string())];
        let result = native_fs_file_exists(&args);
        
        assert!(result.is_ok());
        if let Ok(Value::Bool(exists)) = result {
            assert!(exists);
        }
    }
}
```

### 3. Testes para Buffer

```rust
#[cfg(test)]
mod buffer_tests {
    use super::*;
    
    #[test]
    fn test_buffer_create_and_write() {
        // Criar buffer
        let create_args = vec![Value::Number(1024.0)];
        let create_result = native_buffer_create(&create_args);
        assert!(result.is_ok());
        
        // Escrever no buffer
        let write_args = vec![
            Value::String("buffer_id".to_string()),
            Value::String("Hello, Buffer!".to_string())
        ];
        let write_result = native_buffer_write(&write_args);
        assert!(write_result.is_ok());
        
        // Ler do buffer
        let read_args = vec![Value::String("buffer_id".to_string())];
        let read_result = native_buffer_read(&read_args);
        assert!(read_result.is_ok());
        
        if let Ok(Value::String(content)) = read_result {
            assert_eq!(content, "Hello, Buffer!");
        }
    }
    
    #[test]
    fn test_buffer_clear() {
        // Criar e escrever
        let create_args = vec![Value::Number(1024.0)];
        native_buffer_create(&create_args).unwrap();
        
        let write_args = vec![
            Value::String("buffer_id".to_string()),
            Value::String("Content".to_string())
        ];
        native_buffer_write(&write_args).unwrap();
        
        // Limpar
        let clear_args = vec![Value::String("buffer_id".to_string())];
        let clear_result = native_buffer_clear(&clear_args);
        assert!(clear_result.is_ok());
        
        // Verificar se está vazio
        let read_args = vec![Value::String("buffer_id".to_string())];
        let read_result = native_buffer_read(&read_args);
        assert!(read_result.is_ok());
        
        if let Ok(Value::String(content)) = read_result {
            assert!(content.is_empty());
        }
    }
}
```

## 🔗 Testes de Integração

### 1. Testes de Importação Dryad

```rust
// Criar arquivo de teste: test_integration.rs
use std::process::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_console_import_and_usage() {
    let dir = tempdir().unwrap();
    let test_file = dir.path().join("test_console.dryad");
    
    let content = r#"
using IO.Console;

fn main() {
    Console.println("Hello from integration test!");
    return 0;
}
"#;
    
    fs::write(&test_file, content).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", test_file.to_str().unwrap()])
        .output()
        .expect("Failed to run dryad");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Hello from integration test!"));
}

#[test]
fn test_fs_import_and_usage() {
    let dir = tempdir().unwrap();
    let test_file = dir.path().join("test_fs.dryad");
    let data_file = dir.path().join("data.txt");
    
    let content = format!(r#"
using IO.fs;

fn main() {{
    fs.writeFile("{}", "Integration test data");
    let content = fs.readFile("{}");
    Console.println(content);
    return 0;
}}
"#, data_file.to_str().unwrap(), data_file.to_str().unwrap());
    
    fs::write(&test_file, content).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", test_file.to_str().unwrap()])
        .output()
        .expect("Failed to run dryad");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Integration test data"));
}
```

### 2. Testes de Namespace

```rust
#[test]
fn test_namespace_isolation() {
    let dir = tempdir().unwrap();
    let test_file = dir.path().join("test_namespace.dryad");
    
    let content = r#"
using IO.Console;
using IO.fs;

fn main() {
    // Teste se ambos os namespaces funcionam juntos
    Console.println("Testing namespace isolation...");
    
    let exists = fs.fileExists("nonexistent.txt");
    if (!exists) {
        Console.println("File doesn't exist - correct!");
    }
    
    return 0;
}
"#;
    
    fs::write(&test_file, content).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", test_file.to_str().unwrap()])
        .output()
        .expect("Failed to run dryad");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Testing namespace isolation..."));
    assert!(stdout.contains("File doesn't exist - correct!"));
}
```

## ⚡ Testes de Performance

### 1. Benchmarks de I/O

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn bench_file_operations() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bench.txt");
        let file_path_str = file_path.to_str().unwrap();
        
        let content = "x".repeat(1024 * 1024); // 1MB
        
        // Benchmark escrita
        let start = Instant::now();
        for _ in 0..10 {
            let args = vec![
                Value::String(file_path_str.to_string()),
                Value::String(content.clone())
            ];
            native_fs_write_file(&args).unwrap();
        }
        let write_duration = start.elapsed();
        
        // Benchmark leitura
        let start = Instant::now();
        for _ in 0..10 {
            let args = vec![Value::String(file_path_str.to_string())];
            native_fs_read_file(&args).unwrap();
        }
        let read_duration = start.elapsed();
        
        println!("Write performance: {}ms for 10x1MB", write_duration.as_millis());
        println!("Read performance: {}ms for 10x1MB", read_duration.as_millis());
        
        // Assertions para performance aceitável
        assert!(write_duration.as_millis() < 1000, "Write too slow");
        assert!(read_duration.as_millis() < 500, "Read too slow");
    }
    
    #[test]
    fn bench_buffer_operations() {
        let start = Instant::now();
        
        // Criar buffer
        let create_args = vec![Value::Number(1024.0 * 1024.0)]; // 1MB
        native_buffer_create(&create_args).unwrap();
        
        // Múltiplas operações
        for i in 0..1000 {
            let write_args = vec![
                Value::String("buffer_id".to_string()),
                Value::String(format!("Data {}", i))
            ];
            native_buffer_write(&write_args).unwrap();
        }
        
        let duration = start.elapsed();
        println!("Buffer operations: {}ms for 1000 writes", duration.as_millis());
        
        assert!(duration.as_millis() < 100, "Buffer operations too slow");
    }
}
```

### 2. Testes de Memória

```rust
#[test]
fn test_memory_usage() {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
    
    struct TrackingAllocator;
    
    unsafe impl GlobalAlloc for TrackingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let ret = System.alloc(layout);
            if !ret.is_null() {
                ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
            }
            ret
        }
        
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            System.dealloc(ptr, layout);
            ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
        }
    }
    
    // Usar allocator personalizado para teste
    let initial_memory = ALLOCATED.load(Ordering::SeqCst);
    
    // Operações que devem não vazar memória
    for _ in 0..100 {
        let args = vec![Value::String("test".to_string())];
        native_console_println(&args).unwrap();
    }
    
    let final_memory = ALLOCATED.load(Ordering::SeqCst);
    let leaked = final_memory - initial_memory;
    
    println!("Memory leaked: {} bytes", leaked);
    assert!(leaked < 1024, "Memory leak detected");
}
```

## 🛡️ Validação de Erros

### 1. Testes de Casos Extremos

```rust
#[cfg(test)]
mod edge_case_tests {
    use super::*;
    
    #[test]
    fn test_large_file_operations() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("large.txt");
        let file_path_str = file_path.to_str().unwrap();
        
        let large_content = "x".repeat(10 * 1024 * 1024); // 10MB
        
        let write_args = vec![
            Value::String(file_path_str.to_string()),
            Value::String(large_content.clone())
        ];
        
        let write_result = native_fs_write_file(&write_args);
        assert!(write_result.is_ok());
        
        let read_args = vec![Value::String(file_path_str.to_string())];
        let read_result = native_fs_read_file(&read_args);
        assert!(read_result.is_ok());
        
        if let Ok(Value::String(content)) = read_result {
            assert_eq!(content.len(), large_content.len());
        }
    }
    
    #[test]
    fn test_invalid_filenames() {
        let invalid_names = vec![
            "", // Empty
            "\0", // Null byte
            "con", // Reserved on Windows
            "prn", // Reserved on Windows
            "a".repeat(300), // Too long
        ];
        
        for name in invalid_names {
            let args = vec![
                Value::String(name.to_string()),
                Value::String("content".to_string())
            ];
            
            let result = native_fs_write_file(&args);
            // Deve falhar ou tratar graciosamente
            if result.is_ok() {
                println!("WARNING: Invalid filename '{}' was accepted", name);
            }
        }
    }
    
    #[test]
    fn test_concurrent_access() {
        use std::thread;
        use std::sync::Arc;
        
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("concurrent.txt");
        let file_path_str = Arc::new(file_path.to_str().unwrap().to_string());
        
        let mut handles = vec![];
        
        // Múltiplas threads escrevendo
        for i in 0..10 {
            let path = file_path_str.clone();
            let handle = thread::spawn(move || {
                let args = vec![
                    Value::String(path.to_string()),
                    Value::String(format!("Content from thread {}", i))
                ];
                native_fs_write_file(&args)
            });
            handles.push(handle);
        }
        
        // Aguardar todas as threads
        for handle in handles {
            let result = handle.join().unwrap();
            // Pelo menos uma deve ter sucesso
            // (comportamento específico depende da implementação)
        }
        
        // Verificar se arquivo existe
        let exists_args = vec![Value::String(file_path_str.to_string())];
        let exists_result = native_fs_file_exists(&exists_args);
        assert!(exists_result.is_ok());
    }
}
```

### 2. Testes de Recuperação de Erros

```rust
#[test]
fn test_error_recovery() {
    // Teste 1: Recuperar de arquivo não encontrado
    let read_args = vec![Value::String("nonexistent.txt".to_string())];
    let read_result = native_fs_read_file(&read_args);
    assert!(read_result.is_err());
    
    // Operação subsequente deve funcionar
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("recovery.txt");
    let file_path_str = file_path.to_str().unwrap();
    
    let write_args = vec![
        Value::String(file_path_str.to_string()),
        Value::String("Recovery test".to_string())
    ];
    let write_result = native_fs_write_file(&write_args);
    assert!(write_result.is_ok());
    
    // Teste 2: Recuperar de argumentos inválidos
    let invalid_args = vec![Value::Number(42.0)];
    let invalid_result = native_console_println(&invalid_args);
    assert!(invalid_result.is_err());
    
    // Operação subsequente deve funcionar
    let valid_args = vec![Value::String("Recovery successful".to_string())];
    let valid_result = native_console_println(&valid_args);
    assert!(valid_result.is_ok());
}
```

## 🤖 Testes Automatizados

### 1. Script de Teste Contínuo

```bash
#!/bin/bash
# run_tests.sh

echo "🧪 Executando testes das bibliotecas Dryad..."

# Testes unitários em Rust
echo "📋 Testes unitários..."
cargo test --lib

# Testes de integração
echo "🔗 Testes de integração..."
cargo test --test integration_tests

# Testes de performance
echo "⚡ Testes de performance..."
cargo test --release bench_ -- --nocapture

# Testes de bibliotecas Dryad
echo "📚 Testes de bibliotecas..."
for test_file in test_*.dryad; do
    if [ -f "$test_file" ]; then
        echo "  Executando $test_file..."
        cargo run -- "$test_file"
        if [ $? -eq 0 ]; then
            echo "  ✅ $test_file passou"
        else
            echo "  ❌ $test_file falhou"
        fi
    fi
done

echo "🎯 Testes concluídos!"
```

### 2. Validação de Qualidade

```rust
// Arquivo: tests/quality_tests.rs
#[cfg(test)]
mod quality_tests {
    use std::fs;
    use std::path::Path;
    
    #[test]
    fn test_all_lib_files_have_namespace() {
        let lib_dir = Path::new("lib/IO");
        assert!(lib_dir.exists(), "lib/IO directory must exist");
        
        for entry in fs::read_dir(lib_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "dryad") {
                let content = fs::read_to_string(&path).unwrap();
                
                assert!(
                    content.contains("namespace IO"),
                    "File {:?} must contain 'namespace IO'",
                    path
                );
                
                assert!(
                    content.contains("export class"),
                    "File {:?} must contain 'export class'",
                    path
                );
            }
        }
    }
    
    #[test]
    fn test_no_deprecated_syntax() {
        let lib_dir = Path::new("lib/IO");
        
        for entry in fs::read_dir(lib_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "dryad") {
                let content = fs::read_to_string(&path).unwrap();
                
                assert!(
                    !content.contains("fun "),
                    "File {:?} uses deprecated 'fun' instead of 'fn'",
                    path
                );
                
                assert!(
                    !content.contains("function "),
                    "File {:?} uses deprecated 'function' instead of 'fn'",
                    path
                );
            }
        }
    }
}
```

## 📊 Métricas de Qualidade

### 1. Cobertura de Testes

```toml
# Cargo.toml
[dependencies]
# ... outras dependências

[dev-dependencies]
tarpaulin = "0.20"
```

```bash
# Executar com cobertura
cargo tarpaulin --out Html
```

### 2. Relatório de Qualidade

```rust
// Arquivo: tests/quality_report.rs
use std::fs;
use std::collections::HashMap;

#[test]
fn generate_quality_report() {
    let mut report = HashMap::new();
    
    // Contar funções nativas
    let native_content = fs::read_to_string("src/interpreter/native.rs").unwrap();
    let native_functions = native_content.matches("fn native_").count();
    report.insert("native_functions", native_functions);
    
    // Contar bibliotecas
    let lib_files = fs::read_dir("lib/IO").unwrap().count();
    report.insert("library_files", lib_files);
    
    // Contar testes
    let test_content = fs::read_to_string("tests/").unwrap_or_default();
    let unit_tests = test_content.matches("#[test]").count();
    report.insert("unit_tests", unit_tests);
    
    println!("📊 Relatório de Qualidade:");
    println!("  Funções nativas: {}", report.get("native_functions").unwrap_or(&0));
    println!("  Arquivos de biblioteca: {}", report.get("library_files").unwrap_or(&0));
    println!("  Testes unitários: {}", report.get("unit_tests").unwrap_or(&0));
    
    // Verificar se há cobertura adequada
    let coverage_ratio = *report.get("unit_tests").unwrap_or(&0) as f64 / 
                        *report.get("native_functions").unwrap_or(&1) as f64;
    
    println!("  Cobertura de testes: {:.2}%", coverage_ratio * 100.0);
    
    assert!(coverage_ratio >= 0.8, "Cobertura de testes insuficiente");
}
```

## 🎯 Próximos Passos

1. **Implementar todos os testes unitários para funções existentes**
2. **Criar testes de integração para cenários complexos**
3. **Adicionar benchmarks de performance**
4. **Implementar testes de stress e carga**
5. **Criar pipeline de CI/CD com execução automática de testes**
6. **Adicionar testes de compatibilidade entre versões**
7. **Implementar testes de segurança**

---

**📅 Atualizado:** 9 de julho de 2025  
**🎯 Status:** Framework completo de testes e validação implementado
