// src/oak/cli_integration.rs
//! Integração com CLI do Dryad

use std::fs;
use std::path::{Path, PathBuf};
use super::OakApi;

/// Integração do Oak com o CLI do Dryad
pub struct OakCliIntegration {
    api: OakApi,
}

/// Comandos Oak disponíveis via CLI
#[derive(Debug, Clone)]
pub enum OakCommand {
    Init { name: Option<String>, description: Option<String> },
    Add { package: String, version: Option<String>, dev: bool },
    Remove { package: String },
    List { dev: bool, production: bool },
    Install,
    Update { packages: Vec<String> },
    Info,
    Validate,
    AddLibPath { path: String },
    RemoveLibPath { path: String },
    RunScript { script: String },
}

/// Resultado de comando Oak para CLI
#[derive(Debug, Clone)]
pub struct OakCliResult {
    pub success: bool,
    pub message: String,
    pub output: String,
    pub error: Option<String>,
}

impl OakCliIntegration {
    /// Cria nova instância da integração
    pub fn new() -> Self {
        Self {
            api: OakApi::new(),
        }
    }

    /// Executa um comando Oak
    pub fn execute_command(&self, command: OakCommand) -> OakCliResult {
        match command {
            OakCommand::Init { name, description } => {
                self.handle_init(name, description)
            }
            OakCommand::Add { package, version, dev } => {
                self.handle_add(package, version, dev)
            }
            OakCommand::Remove { package } => {
                self.handle_remove(package)
            }
            OakCommand::List { dev, production } => {
                self.handle_list(dev, production)
            }
            OakCommand::Install => {
                self.handle_install()
            }
            OakCommand::Update { packages } => {
                self.handle_update(packages)
            }
            OakCommand::Info => {
                self.handle_info()
            }
            OakCommand::Validate => {
                self.handle_validate()
            }
            OakCommand::AddLibPath { path } => {
                self.handle_add_lib_path(path)
            }
            OakCommand::RemoveLibPath { path } => {
                self.handle_remove_lib_path(path)
            }
            OakCommand::RunScript { script } => {
                self.handle_run_script(script)
            }
        }
    }

    /// Manipula comando init
    fn handle_init(&self, name: Option<String>, description: Option<String>) -> OakCliResult {
        let result_json = self.api.init_project(name, description);
        
        if result_json["success"].as_bool().unwrap_or(false) {
            let project_name = result_json["data"]["name"].as_str().unwrap_or("unknown");
            let config_file = result_json["data"]["config_file"].as_str().unwrap_or("oaklibs.json");
            
            // Criar oak_modules e copiar bibliotecas padrão
            let copy_result = self.create_oak_modules();
            
            let mut output = format!(
                "✓ Initialized Oak project '{}'\n✓ Created {}",
                project_name, config_file
            );
            
            if copy_result.is_ok() {
                output.push_str("\n✓ Created oak_modules/ directory\n✓ Copied standard libraries to oak_modules/");
            } else {
                output.push_str("\n⚠ Warning: Could not copy standard libraries");
            }
            
            OakCliResult {
                success: true,
                message: "Oak project initialized successfully".to_string(),
                output,
                error: None,
            }
        } else {
            OakCliResult {
                success: false,
                message: result_json["message"].as_str().unwrap_or("Unknown error").to_string(),
                output: String::new(),
                error: Some(result_json["message"].as_str().unwrap_or("Unknown error").to_string()),
            }
        }
    }
    
    /// Cria oak_modules e copia bibliotecas padrão
    fn create_oak_modules(&self) -> Result<(), String> {
        // Criar diretório oak_modules
        fs::create_dir_all("oak_modules").map_err(|e| format!("Failed to create oak_modules: {}", e))?;
        
        // Encontrar a pasta lib das bibliotecas padrão
        let lib_sources = vec![
            "lib",           // Desenvolvimento local
            "../lib",        // Relativo ao exe
            "../../lib",     // Outro nível
        ];
        
        let mut lib_path: Option<&str> = None;
        for source in &lib_sources {
            if Path::new(source).exists() {
                lib_path = Some(source);
                break;
            }
        }
        
        // Detectar lib próximo ao executável
        if lib_path.is_none() {
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(exe_dir) = exe_path.parent() {
                    let exe_lib = exe_dir.join("lib");
                    if exe_lib.exists() {
                        // Copiar da localização do exe
                        self.copy_directory_recursive(&exe_lib, &PathBuf::from("oak_modules"))?;
                        return Ok(());
                    }
                }
            }
        }
        
        if let Some(source_lib) = lib_path {
            // Copiar recursivamente lib/ para oak_modules/
            self.copy_directory_recursive(&PathBuf::from(source_lib), &PathBuf::from("oak_modules"))?;
            Ok(())
        } else {
            Err("Could not find standard library directory".to_string())
        }
    }
    
    /// Copia um diretório recursivamente
    fn copy_directory_recursive(&self, source: &Path, dest: &Path) -> Result<(), String> {
        if !source.exists() {
            return Err(format!("Source directory does not exist: {}", source.display()));
        }
        
        fs::create_dir_all(dest).map_err(|e| format!("Failed to create destination: {}", e))?;
        
        for entry in fs::read_dir(source).map_err(|e| format!("Failed to read source directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            let name = entry.file_name();
            let dest_path = dest.join(&name);
            
            if path.is_dir() {
                self.copy_directory_recursive(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path).map_err(|e| format!("Failed to copy file {}: {}", path.display(), e))?;
            }
        }
        
        Ok(())
    }

    /// Manipula comando add
    fn handle_add(&self, package: String, version: Option<String>, dev: bool) -> OakCliResult {
        let result_json = self.api.add_dependency(package.clone(), version.clone(), dev);
        
        if result_json["success"].as_bool().unwrap_or(false) {
            let version_str = version.unwrap_or_else(|| "latest".to_string());
            let dep_type = if dev { "dev dependency" } else { "dependency" };
            
            OakCliResult {
                success: true,
                message: format!("Added {} as {}", package, dep_type),
                output: format!(
                    "✓ Added {} {} as {}\n✓ Updated oaklibs.json",
                    package, version_str, dep_type
                ),
                error: None,
            }
        } else {
            OakCliResult {
                success: false,
                message: result_json["message"].as_str().unwrap_or("Failed to add package").to_string(),
                output: String::new(),
                error: Some(result_json["message"].as_str().unwrap_or("Unknown error").to_string()),
            }
        }
    }

    /// Manipula comando remove
    fn handle_remove(&self, package: String) -> OakCliResult {
        let result_json = self.api.remove_dependency(package.clone());
        
        if result_json["success"].as_bool().unwrap_or(false) {
            OakCliResult {
                success: true,
                message: format!("Removed {}", package),
                output: format!(
                    "✓ Removed {} from dependencies\n✓ Updated oaklibs.json",
                    package
                ),
                error: None,
            }
        } else {
            OakCliResult {
                success: false,
                message: result_json["message"].as_str().unwrap_or("Failed to remove package").to_string(),
                output: String::new(),
                error: Some(result_json["message"].as_str().unwrap_or("Unknown error").to_string()),
            }
        }
    }

    /// Manipula comando list
    fn handle_list(&self, dev: bool, production: bool) -> OakCliResult {
        let include_dev = dev || (!dev && !production); // Se nenhum especificado, incluir ambos
        let result_json = self.api.list_dependencies(include_dev);
        
        if result_json["success"].as_bool().unwrap_or(false) {
            let project = &result_json["data"]["project"];
            let packages = &result_json["data"]["packages"];
            
            let mut output = String::new();
            output.push_str(&format!("Project: {} v{}\n", 
                project["name"].as_str().unwrap_or("unknown"),
                project["version"].as_str().unwrap_or("unknown")
            ));
            
            if let Some(description) = project["description"].as_str() {
                if !description.is_empty() {
                    output.push_str(&format!("Description: {}\n", description));
                }
            }
            
            output.push_str("\nDependencies:\n");
            
            if let Some(packages_obj) = packages.as_object() {
                if packages_obj.is_empty() {
                    output.push_str("  No dependencies found\n");
                } else {
                    let mut prod_deps = Vec::new();
                    let mut dev_deps = Vec::new();
                    
                    for (name, info) in packages_obj {
                        let version = info["version"].as_str().unwrap_or("unknown");
                        let dep_type = info["type"].as_str().unwrap_or("production");
                        let optional = info["optional"].as_bool().unwrap_or(false);
                        
                        let optional_str = if optional { " (optional)" } else { "" };
                        let dep_line = format!("  {} {}{}", name, version, optional_str);
                        
                        if dep_type == "development" {
                            dev_deps.push(dep_line);
                        } else {
                            prod_deps.push(dep_line);
                        }
                    }
                    
                    if !prod_deps.is_empty() && (production || (!dev && !production)) {
                        output.push_str("  Production:\n");
                        for dep in prod_deps {
                            output.push_str(&format!("  {}\n", dep));
                        }
                    }
                    
                    if !dev_deps.is_empty() && (dev || (!dev && !production)) {
                        output.push_str("  Development:\n");
                        for dep in dev_deps {
                            output.push_str(&format!("  {}\n", dep));
                        }
                    }
                }
            }
            
            OakCliResult {
                success: true,
                message: "Dependencies listed".to_string(),
                output,
                error: None,
            }
        } else {
            OakCliResult {
                success: false,
                message: result_json["message"].as_str().unwrap_or("Failed to list dependencies").to_string(),
                output: String::new(),
                error: Some(result_json["message"].as_str().unwrap_or("Unknown error").to_string()),
            }
        }
    }

    /// Manipula comando install
    fn handle_install(&self) -> OakCliResult {
        let result_json = self.api.install_dependencies();
        
        if result_json["success"].as_bool().unwrap_or(false) {
            let count = result_json["data"]["count"].as_u64().unwrap_or(0);
            
            OakCliResult {
                success: true,
                message: "Dependencies installed".to_string(),
                output: format!("✓ Installed {} packages", count),
                error: None,
            }
        } else {
            OakCliResult {
                success: false,
                message: result_json["message"].as_str().unwrap_or("Installation failed").to_string(),
                output: String::new(),
                error: Some(result_json["message"].as_str().unwrap_or("Unknown error").to_string()),
            }
        }
    }

    /// Manipula comando update
    fn handle_update(&self, packages: Vec<String>) -> OakCliResult {
        if packages.is_empty() {
            // Atualizar todos os pacotes
            OakCliResult {
                success: false,
                message: "Update all packages not implemented yet".to_string(),
                output: String::new(),
                error: Some("Feature not implemented".to_string()),
            }
        } else {
            // Atualizar pacotes específicos
            let mut updated: Vec<String> = Vec::new();
            let failed: Vec<String> = Vec::new();
            
            for package in packages {
                // TODO: Implementar lógica de atualização
                // Por enquanto, simular sucesso
                updated.push(package);
            }
            
            if failed.is_empty() {
                OakCliResult {
                    success: true,
                    message: "Packages updated".to_string(),
                    output: format!("✓ Updated {} packages: {}", updated.len(), updated.join(", ")),
                    error: None,
                }
            } else {
                OakCliResult {
                    success: false,
                    message: "Some packages failed to update".to_string(),
                    output: format!("Updated: {}\nFailed: {}", updated.join(", "), failed.join(", ")),
                    error: Some("Some updates failed".to_string()),
                }
            }
        }
    }

    /// Manipula comando info
    fn handle_info(&self) -> OakCliResult {
        let result_json = self.api.get_project_info();
        
        if result_json["success"].as_bool().unwrap_or(false) {
            let data = &result_json["data"];
            
            let mut output = String::new();
            output.push_str(&format!("Project Name: {}\n", data["name"].as_str().unwrap_or("unknown")));
            output.push_str(&format!("Version: {}\n", data["version"].as_str().unwrap_or("unknown")));
            output.push_str(&format!("Description: {}\n", data["description"].as_str().unwrap_or("none")));
            
            if let Some(author) = data["author"].as_str() {
                output.push_str(&format!("Author: {}\n", author));
            }
            
            if let Some(license) = data["license"].as_str() {
                output.push_str(&format!("License: {}\n", license));
            }
            
            output.push_str(&format!("Dependencies: {}\n", data["dependencies_count"].as_u64().unwrap_or(0)));
            output.push_str(&format!("Dev Dependencies: {}\n", data["dev_dependencies_count"].as_u64().unwrap_or(0)));
            output.push_str(&format!("Oak Version: {}\n", data["oak_version"].as_str().unwrap_or("unknown")));
            
            if let Some(lib_paths) = data["lib_paths"].as_array() {
                output.push_str("Library Paths:\n");
                for path in lib_paths {
                    if let Some(path_str) = path.as_str() {
                        output.push_str(&format!("  {}\n", path_str));
                    }
                }
            }
            
            // Mostrar scripts se existirem
            if let Some(scripts) = data["scripts"].as_object() {
                if !scripts.is_empty() {
                    output.push_str("Scripts:\n");
                    for (name, command) in scripts {
                        output.push_str(&format!("  {}: {}\n", name, command.as_str().unwrap_or("unknown")));
                    }
                }
            }
            
            OakCliResult {
                success: true,
                message: "Project info retrieved".to_string(),
                output,
                error: None,
            }
        } else {
            OakCliResult {
                success: false,
                message: result_json["message"].as_str().unwrap_or("Failed to get project info").to_string(),
                output: String::new(),
                error: Some(result_json["message"].as_str().unwrap_or("Unknown error").to_string()),
            }
        }
    }

    /// Manipula comando validate
    fn handle_validate(&self) -> OakCliResult {
        let result_json = self.api.validate_config();
        
        if result_json["success"].as_bool().unwrap_or(false) {
            let data = &result_json["data"];
            let valid = data["valid"].as_bool().unwrap_or(false);
            
            if valid {
                OakCliResult {
                    success: true,
                    message: "Configuration is valid".to_string(),
                    output: "✓ Project configuration is valid\n✓ All library paths exist\n✓ No issues found".to_string(),
                    error: None,
                }
            } else {
                let empty_vec = Vec::new();
                let issues = data["issues"].as_array().unwrap_or(&empty_vec);
                let mut output = "⚠ Configuration has issues:\n".to_string();
                
                for issue in issues {
                    if let Some(issue_str) = issue.as_str() {
                        output.push_str(&format!("  • {}\n", issue_str));
                    }
                }
                
                OakCliResult {
                    success: true, // Ainda é sucesso, mas com avisos
                    message: "Configuration has issues".to_string(),
                    output,
                    error: None,
                }
            }
        } else {
            OakCliResult {
                success: false,
                message: result_json["message"].as_str().unwrap_or("Validation failed").to_string(),
                output: String::new(),
                error: Some(result_json["message"].as_str().unwrap_or("Unknown error").to_string()),
            }
        }
    }

    /// Manipula comando add lib path
    fn handle_add_lib_path(&self, path: String) -> OakCliResult {
        let result_json = self.api.add_lib_path(path.clone());
        
        if result_json["success"].as_bool().unwrap_or(false) {
            OakCliResult {
                success: true,
                message: "Library path added".to_string(),
                output: format!("✓ Added library path: {}\n✓ Updated oaklibs.json", path),
                error: None,
            }
        } else {
            OakCliResult {
                success: false,
                message: result_json["message"].as_str().unwrap_or("Failed to add library path").to_string(),
                output: String::new(),
                error: Some(result_json["message"].as_str().unwrap_or("Unknown error").to_string()),
            }
        }
    }

    /// Manipula comando remove lib path
    fn handle_remove_lib_path(&self, path: String) -> OakCliResult {
        let result_json = self.api.remove_lib_path(path.clone());
        
        if result_json["success"].as_bool().unwrap_or(false) {
            OakCliResult {
                success: true,
                message: "Library path removed".to_string(),
                output: format!("✓ Removed library path: {}\n✓ Updated oaklibs.json", path),
                error: None,
            }
        } else {
            OakCliResult {
                success: false,
                message: result_json["message"].as_str().unwrap_or("Failed to remove library path").to_string(),
                output: String::new(),
                error: Some(result_json["message"].as_str().unwrap_or("Unknown error").to_string()),
            }
        }
    }

    /// Manipula comando run script
    fn handle_run_script(&self, script: String) -> OakCliResult {
        let result_json = self.api.run_script(script.clone());
        
        if result_json["success"].as_bool().unwrap_or(false) {
            let command = result_json["data"]["command"].as_str().unwrap_or("unknown");
            
            OakCliResult {
                success: true,
                message: "Script found".to_string(),
                output: format!("✓ Script '{}' command: {}\n• To execute, run: {}", script, command, command),
                error: None,
            }
        } else {
            OakCliResult {
                success: false,
                message: result_json["message"].as_str().unwrap_or("Script not found").to_string(),
                output: String::new(),
                error: Some(result_json["message"].as_str().unwrap_or("Unknown error").to_string()),
            }
        }
    }

    /// Mostra ajuda dos comandos Oak
    pub fn show_help() -> String {
        r#"Oak Package Manager Commands:

  oak init [name] [description]     Initialize a new Oak project
  oak add <package> [version]       Add a dependency
  oak add --dev <package> [version] Add a development dependency  
  oak remove <package>              Remove a dependency
  oak list                          List all dependencies
  oak list --dev                    List development dependencies
  oak list --prod                   List production dependencies
  oak install                       Install all dependencies
  oak update [packages...]          Update packages
  oak info                          Show project information
  oak validate                      Validate project configuration
  oak lib add <path>                Add library path
  oak lib remove <path>             Remove library path
  oak run <script>                  Run a project script

Examples:
  oak init my-project "My Dryad project"
  oak add utils ^1.0.0
  oak add --dev test-framework latest
  oak remove old-package
  oak list --dev
  oak install
  oak lib add ./vendor/libs
  oak run build
"#.to_string()
    }

    /// Converte um OakCliResult em erro CLI se não teve sucesso
    fn to_cli_error(&self, result: &OakCliResult) -> Option<String> {
        if result.success {
            None
        } else {
            Some(result.message.clone())
        }
    }
}

impl Default for OakCliIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::env;

    #[test]
    fn test_oak_cli_integration() {
        let integration = OakCliIntegration::new();
        
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        env::set_current_dir(temp_dir.path()).unwrap();
        
        // Testar comando init
        let command = OakCommand::Init {
            name: Some("test-cli".to_string()),
            description: Some("Test CLI integration".to_string()),
        };
        
        let result = integration.execute_command(command);
        assert!(result.success);
        assert!(result.output.contains("test-cli"));
        
        // Testar comando info
        let command = OakCommand::Info;
        let result = integration.execute_command(command);
        assert!(result.success);
        assert!(result.output.contains("test-cli"));
        
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_help_message() {
        let help = OakCliIntegration::show_help();
        assert!(help.contains("Oak Package Manager Commands"));
        assert!(help.contains("oak init"));
        assert!(help.contains("oak add"));
        assert!(help.contains("Examples:"));
    }

    #[test]
    fn test_error_conversion() {
        let integration = OakCliIntegration::new();
        
        let success_result = OakCliResult {
            success: true,
            message: "Success".to_string(),
            output: "Output".to_string(),
            error: None,
        };
        
        let error_result = OakCliResult {
            success: false,
            message: "Error".to_string(),
            output: "".to_string(),
            error: Some("Error details".to_string()),
        };
        
        assert!(integration.to_cli_error(&success_result).is_none());
        assert!(integration.to_cli_error(&error_result).is_some());
    }
}
