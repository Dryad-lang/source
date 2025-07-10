// src/oak/manager.rs
//! Gerenciador principal do Oak - Core API

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde_json::{json, Value};

use crate::interpreter::errors::DryadError;
use super::config::{OakConfig, OakDependency};
use super::registry::{PackageRegistry, LocalRegistry};
use super::resolver::DependencyResolver;

/// Erro específico do Oak
#[derive(Debug, Clone)]
pub struct OakError {
    pub message: String,
    pub code: OakErrorCode,
    pub details: Option<Value>,
}

/// Códigos de erro do Oak
#[derive(Debug, Clone, PartialEq)]
pub enum OakErrorCode {
    ProjectNotFound,
    ProjectAlreadyExists,
    PackageNotFound,
    DependencyConflict,
    InvalidConfiguration,
    RegistryError,
    IoError,
    ParseError,
    ValidationError,
}

/// Resultado de operações Oak
#[derive(Debug, Clone)]
pub struct OakResult {
    pub success: bool,
    pub message: String,
    pub data: Option<Value>,
    pub error: Option<OakError>,
}

/// Opções para operações do Oak
#[derive(Debug, Clone, Default)]
pub struct OakOptions {
    pub config_path: Option<PathBuf>,
    pub registry_path: Option<PathBuf>,
    pub force: bool,
    pub verbose: bool,
    pub dry_run: bool,
}

/// Gerenciador principal do Oak
pub struct OakManager {
    pub config_path: PathBuf,
    pub registry: Box<dyn PackageRegistry>,
    pub resolver: DependencyResolver,
    pub options: OakOptions,
}

impl OakError {
    pub fn new(message: String, code: OakErrorCode) -> Self {
        Self {
            message,
            code,
            details: None,
        }
    }

    pub fn with_details(message: String, code: OakErrorCode, details: Value) -> Self {
        Self {
            message,
            code,
            details: Some(details),
        }
    }

    pub fn from_dryad_error(error: DryadError) -> Self {
        Self {
            message: error.message,
            code: OakErrorCode::ParseError,
            details: Some(json!({
                "location": error.location,
                "severity": format!("{:?}", error.severity)
            })),
        }
    }
}

impl OakResult {
    pub fn success(message: String) -> Self {
        Self {
            success: true,
            message,
            data: None,
            error: None,
        }
    }

    pub fn success_with_data(message: String, data: Value) -> Self {
        Self {
            success: true,
            message,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: OakError) -> Self {
        Self {
            success: false,
            message: error.message.clone(),
            data: error.details.clone(),
            error: Some(error),
        }
    }

    pub fn error_simple(message: String, code: OakErrorCode) -> Self {
        Self::error(OakError::new(message, code))
    }

    /// Converte para JSON para APIs externas
    pub fn to_json(&self) -> Value {
        json!({
            "success": self.success,
            "message": self.message,
            "data": self.data,
            "error": self.error.as_ref().map(|e| json!({
                "code": format!("{:?}", e.code),
                "details": e.details
            }))
        })
    }
}

impl Default for OakManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OakManager {
    /// Cria um novo gerenciador Oak com configurações padrão
    pub fn new() -> Self {
        Self {
            config_path: PathBuf::from("oaklibs.json"),
            registry: Box::new(LocalRegistry::new("./oak_cache".into())),
            resolver: DependencyResolver::new(),
            options: OakOptions::default(),
        }
    }

    /// Cria um gerenciador Oak com opções customizadas
    pub fn with_options(options: OakOptions) -> Self {
        let config_path = options.config_path.clone()
            .unwrap_or_else(|| PathBuf::from("oaklibs.json"));
        
        let registry_path = options.registry_path.clone()
            .unwrap_or_else(|| PathBuf::from("./oak_cache"));

        Self {
            config_path,
            registry: Box::new(LocalRegistry::new(registry_path)),
            resolver: DependencyResolver::new(),
            options,
        }
    }

    /// Verifica se existe um projeto Oak no diretório atual
    pub fn project_exists(&self) -> bool {
        self.config_path.exists()
    }

    /// Inicializa um novo projeto Oak
    pub fn init_project(&self, name: Option<String>, description: Option<String>) -> OakResult {
        if self.project_exists() && !self.options.force {
            return OakResult::error_simple(
                "Oak project already exists. Use --force to overwrite.".to_string(),
                OakErrorCode::ProjectAlreadyExists,
            );
        }

        let mut config = OakConfig::default();
        
        if let Some(n) = name {
            config.name = n;
        }
        
        if let Some(d) = description {
            config.description = d;
        }

        // Salvar configuração
        if let Err(e) = config.save(&self.config_path) {
            return OakResult::error(OakError::from_dryad_error(e));
        }

        // Criar diretórios necessários
        for lib_path in &config.lib_paths {
            if !Path::new(lib_path).exists() {
                if let Err(e) = fs::create_dir_all(lib_path) {
                    return OakResult::error_simple(
                        format!("Failed to create directory {}: {}", lib_path, e),
                        OakErrorCode::IoError,
                    );
                }
            }
        }

        // Inicializar cache do registry
        if let Err(e) = self.registry.initialize() {
            return OakResult::error_simple(
                format!("Failed to initialize package registry: {}", e),
                OakErrorCode::RegistryError,
            );
        }

        OakResult::success_with_data(
            "Oak project initialized successfully".to_string(),
            json!({
                "name": config.name,
                "version": config.version,
                "config_file": self.config_path,
                "lib_paths": config.lib_paths
            })
        )
    }

    /// Carrega a configuração do projeto
    pub fn load_config(&self) -> Result<OakConfig, OakError> {
        if !self.project_exists() {
            return Err(OakError::new(
                "No Oak project found. Run 'oak init' first.".to_string(),
                OakErrorCode::ProjectNotFound,
            ));
        }

        OakConfig::load(&self.config_path)
            .map_err(OakError::from_dryad_error)
    }

    /// Salva a configuração do projeto
    pub fn save_config(&self, config: &OakConfig) -> Result<(), OakError> {
        config.save(&self.config_path)
            .map_err(OakError::from_dryad_error)
    }

    /// Adiciona um pacote às dependências
    pub fn add_package(&self, package_name: String, version: Option<String>, dev: bool) -> OakResult {
        let mut config = match self.load_config() {
            Ok(c) => c,
            Err(e) => return OakResult::error(e),
        };

        let version = version.unwrap_or_else(|| "latest".to_string());
        let mut dependency = OakDependency::new(version.clone());
        dependency.dev = dev;

        // Verificar se o pacote existe no registry
        match self.registry.find_package(&package_name, &version) {
            Ok(Some(package)) => {
                config.add_dependency(package_name.clone(), dependency);
                
                if let Err(e) = self.save_config(&config) {
                    return OakResult::error(e);
                }

                OakResult::success_with_data(
                    format!("Added {} {} to {}", package_name, version, 
                        if dev { "dev dependencies" } else { "dependencies" }),
                    json!({
                        "package": package.to_json(),
                        "dependency_type": if dev { "dev" } else { "production" },
                        "total_dependencies": config.dependencies.len(),
                        "total_dev_dependencies": config.dev_dependencies.len()
                    })
                )
            }
            Ok(None) => {
                OakResult::error_simple(
                    format!("Package {} not found in registry", package_name),
                    OakErrorCode::PackageNotFound,
                )
            }
            Err(e) => {
                OakResult::error_simple(
                    format!("Failed to search registry: {}", e),
                    OakErrorCode::RegistryError,
                )
            }
        }
    }

    /// Remove um pacote das dependências
    pub fn remove_package(&self, package_name: String) -> OakResult {
        let mut config = match self.load_config() {
            Ok(c) => c,
            Err(e) => return OakResult::error(e),
        };

        if config.remove_dependency(&package_name) {
            if let Err(e) = self.save_config(&config) {
                return OakResult::error(e);
            }

            OakResult::success_with_data(
                format!("Removed {} from dependencies", package_name),
                json!({
                    "package": package_name,
                    "total_dependencies": config.dependencies.len(),
                    "total_dev_dependencies": config.dev_dependencies.len()
                })
            )
        } else {
            OakResult::error_simple(
                format!("Package {} not found in dependencies", package_name),
                OakErrorCode::PackageNotFound,
            )
        }
    }

    /// Lista todas as dependências
    pub fn list_packages(&self, dev: bool, production: bool) -> OakResult {
        let config = match self.load_config() {
            Ok(c) => c,
            Err(e) => return OakResult::error(e),
        };

        let mut packages = HashMap::new();

        if production {
            for (name, dep) in &config.dependencies {
                packages.insert(name.clone(), json!({
                    "version": dep.version,
                    "type": "production",
                    "optional": dep.optional,
                    "source": format!("{:?}", dep.source)
                }));
            }
        }

        if dev {
            for (name, dep) in &config.dev_dependencies {
                packages.insert(name.clone(), json!({
                    "version": dep.version,
                    "type": "development",
                    "optional": dep.optional,
                    "source": format!("{:?}", dep.source)
                }));
            }
        }

        OakResult::success_with_data(
            "Dependencies listed successfully".to_string(),
            json!({
                "project": {
                    "name": config.name,
                    "version": config.version,
                    "description": config.description
                },
                "packages": packages,
                "lib_paths": config.lib_paths,
                "scripts": config.scripts
            })
        )
    }

    /// Obtém informações detalhadas sobre o projeto
    pub fn get_project_info(&self) -> OakResult {
        let config = match self.load_config() {
            Ok(c) => c,
            Err(e) => return OakResult::error(e),
        };

        let validation_issues = config.validate();

        OakResult::success_with_data(
            "Project info retrieved".to_string(),
            json!({
                "name": config.name,
                "version": config.version,
                "description": config.description,
                "author": config.author,
                "license": config.license,
                "dependencies_count": config.dependencies.len(),
                "dev_dependencies_count": config.dev_dependencies.len(),
                "lib_paths": config.lib_paths,
                "scripts": config.scripts,
                "oak_version": config.oak_version,
                "validation": {
                    "valid": validation_issues.is_empty(),
                    "issues": validation_issues
                }
            })
        )
    }

    /// Adiciona um caminho de biblioteca
    pub fn add_lib_path(&self, path: String) -> OakResult {
        let mut config = match self.load_config() {
            Ok(c) => c,
            Err(e) => return OakResult::error(e),
        };

        if config.add_lib_path(path.clone()) {
            // Criar diretório se não existir
            if !Path::new(&path).exists() {
                if let Err(e) = fs::create_dir_all(&path) {
                    return OakResult::error_simple(
                        format!("Failed to create directory {}: {}", path, e),
                        OakErrorCode::IoError,
                    );
                }
            }

            if let Err(e) = self.save_config(&config) {
                return OakResult::error(e);
            }

            OakResult::success_with_data(
                format!("Added lib path: {}", path),
                json!({
                    "path": path,
                    "total_paths": config.lib_paths.len()
                })
            )
        } else {
            OakResult::error_simple(
                format!("Path {} already exists", path),
                OakErrorCode::ValidationError,
            )
        }
    }

    /// Remove um caminho de biblioteca
    pub fn remove_lib_path(&self, path: String) -> OakResult {
        let mut config = match self.load_config() {
            Ok(c) => c,
            Err(e) => return OakResult::error(e),
        };

        if config.remove_lib_path(&path) {
            if let Err(e) = self.save_config(&config) {
                return OakResult::error(e);
            }

            OakResult::success_with_data(
                format!("Removed lib path: {}", path),
                json!({
                    "path": path,
                    "total_paths": config.lib_paths.len()
                })
            )
        } else {
            OakResult::error_simple(
                format!("Path {} not found", path),
                OakErrorCode::PackageNotFound,
            )
        }
    }

    /// Instala todas as dependências
    pub fn install(&self) -> OakResult {
        let config = match self.load_config() {
            Ok(c) => c,
            Err(e) => return OakResult::error(e),
        };

        let mut installed = Vec::new();
        let mut failed = Vec::new();

        // Resolver dependências
        let resolved = match self.resolver.resolve(&config) {
            Ok(r) => r,
            Err(e) => return OakResult::error_simple(
                format!("Dependency resolution failed: {}", e),
                OakErrorCode::DependencyConflict,
            ),
        };

        // Instalar pacotes resolvidos
        for (name, package) in resolved {
            match self.registry.install_package(&package) {
                Ok(()) => {
                    installed.push(json!({
                        "name": name,
                        "version": package.version
                    }));
                }
                Err(e) => {
                    failed.push(json!({
                        "name": name,
                        "error": e.to_string()
                    }));
                }
            }
        }

        if failed.is_empty() {
            OakResult::success_with_data(
                format!("Successfully installed {} packages", installed.len()),
                json!({
                    "installed": installed,
                    "count": installed.len()
                })
            )
        } else {
            OakResult::error(OakError::with_details(
                format!("Installation completed with {} failures", failed.len()),
                OakErrorCode::RegistryError,
                json!({
                    "installed": installed,
                    "failed": failed
                })
            ))
        }
    }

    /// Valida a configuração do projeto
    pub fn validate(&self) -> OakResult {
        let config = match self.load_config() {
            Ok(c) => c,
            Err(e) => return OakResult::error(e),
        };

        let issues = config.validate();

        if issues.is_empty() {
            OakResult::success_with_data(
                "Configuration is valid".to_string(),
                json!({
                    "valid": true,
                    "lib_paths_checked": config.lib_paths.len(),
                    "dependencies_count": config.dependencies.len(),
                    "dev_dependencies_count": config.dev_dependencies.len()
                })
            )
        } else {
            OakResult::success_with_data(
                "Configuration has issues".to_string(),
                json!({
                    "valid": false,
                    "issues": issues
                })
            )
        }
    }

    /// Executa um script definido no projeto
    pub fn run_script(&self, script_name: String) -> OakResult {
        let config = match self.load_config() {
            Ok(c) => c,
            Err(e) => return OakResult::error(e),
        };

        if let Some(command) = config.scripts.get(&script_name) {
            OakResult::success_with_data(
                format!("Script '{}' ready to execute", script_name),
                json!({
                    "script": script_name,
                    "command": command
                })
            )
        } else {
            OakResult::error_simple(
                format!("Script '{}' not found", script_name),
                OakErrorCode::PackageNotFound,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::env;

    #[test]
    fn test_oak_manager_creation() {
        let manager = OakManager::new();
        assert_eq!(manager.config_path, PathBuf::from("oaklibs.json"));
    }

    #[test]
    fn test_project_initialization() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        env::set_current_dir(temp_dir.path()).unwrap();
        
        let manager = OakManager::new();
        let result = manager.init_project(
            Some("test-project".to_string()), 
            Some("Test description".to_string())
        );
        
        assert!(result.success);
        assert!(manager.project_exists());
        
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_config_load_save() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        env::set_current_dir(temp_dir.path()).unwrap();
        
        let manager = OakManager::new();
        manager.init_project(None, None);
        
        let config = manager.load_config().unwrap();
        assert_eq!(config.name, "my-dryad-project");
        
        env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_error_handling() {
        let manager = OakManager::new();
        
        // Tentar carregar config sem projeto
        let result = manager.load_config();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, OakErrorCode::ProjectNotFound);
    }
}
