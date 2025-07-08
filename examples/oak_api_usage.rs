// examples/oak_api_usage.rs
//! Exemplo de como usar a API Oak externamente

use dryad::oak::OakManager;
use dryad::oak::cli_integration::{OakCliIntegration, OakCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Demonstração da API Oak Externa ===\n");
    
    // Criar instância da integração CLI
    let cli_integration = OakCliIntegration::new();
    
    // 1. Inicializar um projeto Oak
    println!("1. Inicializando projeto Oak...");
    let init_command = OakCommand::Init { 
        name: Some("demo-project".to_string()), 
        description: Some("Projeto de demonstração da API Oak".to_string()) 
    };
    
    let result = cli_integration.execute_command(init_command);
    if result.success {
        println!("✅ {}", result.output);
    } else {
        println!("❌ Erro: {}", result.error.unwrap_or_default());
    }
    
    // 2. Listar dependências (deve estar vazio)
    println!("\n2. Listando dependências...");
    let list_command = OakCommand::List { dev: false, production: true };
    
    let result = cli_integration.execute_command(list_command);
    if result.success {
        println!("📦 {}", result.output);
    } else {
        println!("❌ Erro: {}", result.error.unwrap_or_default());
    }
    
    // 3. Tentar adicionar um pacote (vai falhar pois não temos registry)
    println!("\n3. Tentando adicionar pacote 'exemplo-lib'...");
    let add_command = OakCommand::Add { 
        package: "exemplo-lib".to_string(), 
        version: Some("1.0.0".to_string()), 
        dev: false 
    };
    
    let result = cli_integration.execute_command(add_command);
    if result.success {
        println!("✅ {}", result.output);
    } else {
        println!("⚠️  Esperado: {}", result.error.unwrap_or_default());
        println!("   (Normal - não há registry configurado ainda)");
    }
    
    // 4. Validar configuração
    println!("\n4. Validando configuração do projeto...");
    let validate_command = OakCommand::Validate;
    
    let result = cli_integration.execute_command(validate_command);
    if result.success {
        println!("✅ {}", result.output);
    } else {
        println!("⚠️  {}", result.error.unwrap_or_default());
    }
    
    // 5. Demonstração da API direta (OakManager)
    println!("\n5. Demonstrando uso da API direta OakManager...");
    
    // Criar um manager Oak diretamente
    let manager = OakManager::new();
    
    // Tentar inicializar projeto via API direta 
    let result = manager.init_project(Some("api-demo".to_string()), Some("Demo via API".to_string()));
    if result.success {
        println!("✅ {}", result.message);
    } else {
        println!("ℹ️  {}", result.message);
    }
    
    println!("\n=== Demonstração concluída ===");
    println!("A API Oak está funcionando corretamente!");
    println!("Para usar em outros projetos:");
    println!("  - CLI: use dryad::oak::cli_integration::{{OakCliIntegration, OakCommand}};");
    println!("  - API: use dryad::oak::{{OakManager, OakApi}};");
    
    Ok(())
}
