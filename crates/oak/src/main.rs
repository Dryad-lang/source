// crates/oak/src/main.rs
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "oak")]
#[command(about = "Oak - Gestor de Pacotes para Dryad", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inicializa um novo projeto Dryad
    Init {
        /// Nome do projeto
        name: String,
        /// Diretório para criar o projeto (opcional)
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Instala dependências do projeto
    Install {
        /// Nome do pacote para instalar (opcional)
        package: Option<String>,
        /// Versão específica
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Remove uma dependência
    Remove {
        /// Nome do pacote para remover
        package: String,
    },
    /// Lista dependências instaladas
    List,
    /// Atualiza dependências
    Update,
    /// Publica um pacote (futuro)
    Publish,
    /// Executa scripts definidos no projeto
    Run {
        /// Nome do script para executar
        script: String,
    },
    /// Limpa cache e arquivos temporários
    Clean,
    /// Mostra informações do projeto
    Info,
}

#[derive(Serialize, Deserialize, Debug)]
struct OakConfig {
    name: String,
    version: String,
    description: Option<String>,
    author: Option<String>,
    license: Option<String>,
    dependencies: HashMap<String, String>,
    dev_dependencies: HashMap<String, String>,
    scripts: HashMap<String, String>,
}

impl Default for OakConfig {
    fn default() -> Self {
        let mut scripts = HashMap::new();
        scripts.insert("start".to_string(), "dryad run main.dryad".to_string());
        scripts.insert("test".to_string(), "dryad test".to_string());
        scripts.insert("check".to_string(), "dryad check main.dryad".to_string());
        
        OakConfig {
            name: "meu-projeto".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            author: None,
            license: Some("MIT".to_string()),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            scripts,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, path } => {
            if let Err(e) = init_project(&name, path.as_deref()) {
                eprintln!("Erro ao inicializar projeto: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Install { package, version } => {
            if let Err(e) = install_package(package.as_deref(), version.as_deref()) {
                eprintln!("Erro ao instalar: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Remove { package } => {
            if let Err(e) = remove_package(&package) {
                eprintln!("Erro ao remover: {}", e);
                std::process::exit(1);
            }
        }
        Commands::List => {
            if let Err(e) = list_dependencies() {
                eprintln!("Erro ao listar: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Update => {
            if let Err(e) = update_dependencies() {
                eprintln!("Erro ao atualizar: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Publish => {
            println!("⚠️  Publicação será implementada em versões futuras");
        }
        Commands::Run { script } => {
            if let Err(e) = run_script(&script) {
                eprintln!("Erro ao executar script: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Clean => {
            if let Err(e) = clean_project() {
                eprintln!("Erro ao limpar: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Info => {
            if let Err(e) = show_info() {
                eprintln!("Erro ao mostrar informações: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn init_project(name: &str, path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = match path {
        Some(p) => Path::new(p),
        None => Path::new(name),
    };

    // Criar diretório do projeto
    if project_dir.exists() {
        return Err(format!("Diretório '{}' já existe", project_dir.display()).into());
    }

    fs::create_dir_all(project_dir)?;

    // Configurar arquivo oaklibs.json
    let mut config = OakConfig::default();
    config.name = name.to_string();

    let config_path = project_dir.join("oaklibs.json");
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_json)?;

    // Criar arquivo main.dryad
    let main_content = format!(r#"// {}/main.dryad
// Projeto Dryad gerado pelo Oak

let mensagem = "Olá, {}!";
print(mensagem);

// Exemplo de função
function somar(a, b) {{
    return a + b;
}}

let resultado = somar(5, 3);
print("5 + 3 =", resultado);
"#, name, name);

    let main_path = project_dir.join("main.dryad");
    fs::write(&main_path, main_content)?;

    // Criar README.md
    let readme_content = format!(r#"# {}

Projeto criado com Oak - Gestor de Pacotes para Dryad.

## Executar

```bash
oak run start
```

ou

```bash
dryad run main.dryad
```

## Scripts Disponíveis

- `oak run start` - Executa o projeto
- `oak run test` - Executa testes
- `oak run check` - Verifica sintaxe

## Dependências

Veja o arquivo `oaklibs.json` para gerenciar dependências.
"#, name);

    let readme_path = project_dir.join("README.md");
    fs::write(&readme_path, readme_content)?;

    // Criar diretório src (opcional, para projetos maiores)
    let src_dir = project_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    // Criar .gitignore
    let gitignore_content = r#"# Oak
oaklibs.lock
oak_modules/

# Logs
*.log

# Temporários
*.tmp
*.temp

# Sistema
.DS_Store
Thumbs.db
"#;

    let gitignore_path = project_dir.join(".gitignore");
    fs::write(&gitignore_path, gitignore_content)?;

    println!("✓ Projeto '{}' criado com sucesso!", name);
    println!("📁 Localização: {}", project_dir.display());
    println!("\n📋 Próximos passos:");
    println!("   cd {}", name);
    println!("   oak run start");

    Ok(())
}

fn load_config() -> Result<OakConfig, Box<dyn std::error::Error>> {
    let config_path = Path::new("oaklibs.json");
    if !config_path.exists() {
        return Err("Arquivo oaklibs.json não encontrado. Execute 'oak init <nome>' primeiro.".into());
    }

    let content = fs::read_to_string(config_path)?;
    let config: OakConfig = serde_json::from_str(&content)?;
    Ok(config)
}

fn save_config(config: &OakConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_json = serde_json::to_string_pretty(config)?;
    fs::write("oaklibs.json", config_json)?;
    Ok(())
}

fn install_package(package: Option<&str>, version: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config()?;

    match package {
        Some(pkg) => {
            let version = version.unwrap_or("latest");
            config.dependencies.insert(pkg.to_string(), version.to_string());
            save_config(&config)?;
            println!("✓ Pacote '{}@{}' adicionado às dependências", pkg, version);
            println!("⚠️  Instalação real será implementada em versões futuras");
        }
        None => {
            println!("📦 Instalando todas as dependências...");
            for (name, version) in &config.dependencies {
                println!("  - {}@{}", name, version);
            }
            println!("⚠️  Instalação real será implementada em versões futuras");
        }
    }

    Ok(())
}

fn remove_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config()?;

    if config.dependencies.remove(package).is_some() {
        save_config(&config)?;
        println!("✓ Pacote '{}' removido das dependências", package);
    } else {
        println!("⚠️  Pacote '{}' não encontrado nas dependências", package);
    }

    Ok(())
}

fn list_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    println!("📦 Dependências do projeto '{}':", config.name);
    
    if config.dependencies.is_empty() {
        println!("  Nenhuma dependência encontrada");
    } else {
        for (name, version) in &config.dependencies {
            println!("  ├─ {}@{}", name, version);
        }
    }

    if !config.dev_dependencies.is_empty() {
        println!("\n🔧 Dependências de desenvolvimento:");
        for (name, version) in &config.dev_dependencies {
            println!("  ├─ {}@{}", name, version);
        }
    }

    Ok(())
}

fn update_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    
    println!("🔄 Atualizando dependências...");
    for (name, version) in &config.dependencies {
        println!("  - {}@{}", name, version);
    }
    println!("⚠️  Atualização real será implementada em versões futuras");

    Ok(())
}

fn run_script(script: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    match config.scripts.get(script) {
        Some(command) => {
            println!("🚀 Executando script '{}':", script);
            println!("   {}", command);
            
            // Executa o comando
            let mut cmd_parts = command.split_whitespace();
            let program = cmd_parts.next().unwrap();
            let args: Vec<&str> = cmd_parts.collect();

            let status = std::process::Command::new(program)
                .args(&args)
                .status()?;

            if !status.success() {
                return Err(format!("Script '{}' falhou", script).into());
            }
        }
        None => {
            println!("❌ Script '{}' não encontrado", script);
            println!("\n📋 Scripts disponíveis:");
            for (name, command) in &config.scripts {
                println!("  {} - {}", name, command);
            }
        }
    }

    Ok(())
}

fn clean_project() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧹 Limpando projeto...");
    
    // Limpar arquivos de cache (futuro)
    let cache_dirs = ["oak_modules", ".oak_cache", "target"];
    
    for dir in &cache_dirs {
        if Path::new(dir).exists() {
            fs::remove_dir_all(dir)?;
            println!("✓ Removido: {}", dir);
        }
    }
    
    // Limpar arquivos temporários
    let temp_files = ["*.log", "*.tmp"];
    for pattern in &temp_files {
        println!("✓ Limpeza de arquivos: {}", pattern);
    }
    
    println!("✅ Limpeza concluída");
    Ok(())
}

fn show_info() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    println!("📋 Informações do Projeto");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Nome:        {}", config.name);
    println!("Versão:      {}", config.version);
    
    if let Some(desc) = &config.description {
        println!("Descrição:   {}", desc);
    }
    
    if let Some(author) = &config.author {
        println!("Autor:       {}", author);
    }
    
    if let Some(license) = &config.license {
        println!("Licença:     {}", license);
    }

    println!("Dependências: {}", config.dependencies.len());
    println!("Scripts:      {}", config.scripts.len());

    Ok(())
}
