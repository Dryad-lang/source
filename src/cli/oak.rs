// src/cli/oak.rs

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde_json::{json, Value};
use crate::interpreter::errors::{DryadError, ErrorSeverity};

/// Configuração de um projeto Oak
#[derive(Debug, Clone)]
pub struct OakConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: HashMap<String, String>,
    pub lib_paths: Vec<String>,
}

impl OakConfig {
    /// Cria uma configuração padrão
    pub fn default() -> Self {
        Self {
            name: "my-dryad-project".to_string(),
            version: "1.0.0".to_string(),
            description: "A Dryad project using Oak package manager".to_string(),
            dependencies: HashMap::new(),
            lib_paths: vec!["./lib".to_string()],
        }
    }

    /// Carrega configuração do arquivo oaklibs.json
    pub fn load() -> Result<Self, DryadError> {
        if !Path::new("oaklibs.json").exists() {
            return Err(DryadError::new(
                "No Oak project found. oaklibs.json not found.".to_string(),
                None,
                ErrorSeverity::Error,
            ));
        }

        let content = fs::read_to_string("oaklibs.json")
            .map_err(|e| DryadError::new(
                format!("Failed to read oaklibs.json: {}", e),
                None,
                ErrorSeverity::Error,
            ))?;

        let config: Value = serde_json::from_str(&content)
            .map_err(|e| DryadError::new(
                format!("Failed to parse oaklibs.json: {}", e),
                None,
                ErrorSeverity::Error,
            ))?;

        let mut dependencies = HashMap::new();
        if let Some(deps) = config["dependencies"].as_object() {
            for (name, version) in deps {
                dependencies.insert(name.clone(), version.as_str().unwrap_or("latest").to_string());
            }
        }

        let mut lib_paths = vec!["./lib".to_string()];
        if let Some(paths) = config["lib_paths"].as_array() {
            lib_paths = paths.iter()
                .filter_map(|p| p.as_str())
                .map(|s| s.to_string())
                .collect();
        }

        Ok(Self {
            name: config["name"].as_str().unwrap_or("unknown").to_string(),
            version: config["version"].as_str().unwrap_or("1.0.0").to_string(),
            description: config["description"].as_str().unwrap_or("").to_string(),
            dependencies,
            lib_paths,
        })
    }

    /// Salva configuração no arquivo oaklibs.json
    pub fn save(&self) -> Result<(), DryadError> {
        let config = json!({
            "name": self.name,
            "version": self.version,
            "description": self.description,
            "dependencies": self.dependencies,
            "lib_paths": self.lib_paths
        });

        let json_string = serde_json::to_string_pretty(&config)
            .map_err(|e| DryadError::new(
                format!("Failed to serialize config: {}", e),
                None,
                ErrorSeverity::Error,
            ))?;

        fs::write("oaklibs.json", json_string)
            .map_err(|e| DryadError::new(
                format!("Failed to write oaklibs.json: {}", e),
                None,
                ErrorSeverity::Error,
            ))?;

        Ok(())
    }
}

/// Resultado de operações Oak
#[derive(Debug, Clone)]
pub struct OakResult {
    pub success: bool,
    pub message: String,
    pub data: Option<Value>,
}

impl OakResult {
    pub fn success(message: String) -> Self {
        Self {
            success: true,
            message,
            data: None,
        }
    }

    pub fn success_with_data(message: String, data: Value) -> Self {
        Self {
            success: true,
            message,
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
            data: None,
        }
    }
}

/// Oak Package Manager - API Principal
pub struct OakManager {
    config_path: String,
}

impl OakManager {
    /// Cria um novo gerenciador Oak
    pub fn new() -> Self {
        Self {
            config_path: "oaklibs.json".to_string(),
        }
    }

    /// Cria um novo gerenciador Oak com caminho customizado
    pub fn with_config_path(config_path: String) -> Self {
        Self {
            config_path,
        }
    }

    /// Verifica se existe um projeto Oak no diretório atual
    pub fn project_exists(&self) -> bool {
        Path::new(&self.config_path).exists()
    }

    /// Inicializa um novo projeto Oak
    pub fn init_project(&self, name: Option<String>, description: Option<String>) -> OakResult {
        if self.project_exists() {
            return OakResult::error("Oak project already initialized".to_string());
        }

        let mut config = OakConfig::default();
        if let Some(n) = name {
            config.name = n;
        }
        if let Some(d) = description {
            config.description = d;
        }

        match config.save() {
            Ok(()) => {
                // Criar diretório lib se não existir
                if !Path::new("lib").exists() {
                    if let Err(e) = fs::create_dir("lib") {
                        return OakResult::error(format!("Failed to create lib directory: {}", e));
                    }
                }

                OakResult::success_with_data(
                    "Oak project initialized successfully".to_string(),
                    json!({
                        "name": config.name,
                        "version": config.version,
                        "config_file": self.config_path
                    })
                )
            }
            Err(e) => OakResult::error(e.message),
        }
    }

    /// Adiciona um pacote às dependências
    pub fn add_package(&self, package_name: String, version: Option<String>) -> OakResult {
        if !self.project_exists() {
            return OakResult::error("No Oak project found. Run init first.".to_string());
        }

        match OakConfig::load() {
            Ok(mut config) => {
                let version = version.unwrap_or_else(|| "latest".to_string());
                config.dependencies.insert(package_name.clone(), version.clone());

                match config.save() {
                    Ok(()) => OakResult::success_with_data(
                        format!("Added {} to dependencies", package_name),
                        json!({
                            "package": package_name,
                            "version": version,
                            "total_dependencies": config.dependencies.len()
                        })
                    ),
                    Err(e) => OakResult::error(e.message),
                }
            }
            Err(e) => OakResult::error(e.message),
        }
    }

    /// Remove um pacote das dependências
    pub fn remove_package(&self, package_name: String) -> OakResult {
        if !self.project_exists() {
            return OakResult::error("No Oak project found. Run init first.".to_string());
        }

        match OakConfig::load() {
            Ok(mut config) => {
                if config.dependencies.remove(&package_name).is_some() {
                    match config.save() {
                        Ok(()) => OakResult::success_with_data(
                            format!("Removed {} from dependencies", package_name),
                            json!({
                                "package": package_name,
                                "total_dependencies": config.dependencies.len()
                            })
                        ),
                        Err(e) => OakResult::error(e.message),
                    }
                } else {
                    OakResult::error(format!("Package {} not found in dependencies", package_name))
                }
            }
            Err(e) => OakResult::error(e.message),
        }
    }

    /// Lista todas as dependências
    pub fn list_packages(&self) -> OakResult {
        if !self.project_exists() {
            return OakResult::error("No Oak project found. Run init first.".to_string());
        }

        match OakConfig::load() {
            Ok(config) => {
                OakResult::success_with_data(
                    "Dependencies listed successfully".to_string(),
                    json!({
                        "project": {
                            "name": config.name,
                            "version": config.version,
                            "description": config.description
                        },
                        "dependencies": config.dependencies,
                        "lib_paths": config.lib_paths
                    })
                )
            }
            Err(e) => OakResult::error(e.message),
        }
    }

    /// Obtém informações sobre o projeto
    pub fn get_project_info(&self) -> OakResult {
        if !self.project_exists() {
            return OakResult::error("No Oak project found".to_string());
        }

        match OakConfig::load() {
            Ok(config) => {
                OakResult::success_with_data(
                    "Project info retrieved".to_string(),
                    json!({
                        "name": config.name,
                        "version": config.version,
                        "description": config.description,
                        "dependencies_count": config.dependencies.len(),
                        "lib_paths": config.lib_paths
                    })
                )
            }
            Err(e) => OakResult::error(e.message),
        }
    }

    /// Adiciona um caminho de biblioteca
    pub fn add_lib_path(&self, path: String) -> OakResult {
        if !self.project_exists() {
            return OakResult::error("No Oak project found. Run init first.".to_string());
        }

        match OakConfig::load() {
            Ok(mut config) => {
                if !config.lib_paths.contains(&path) {
                    config.lib_paths.push(path.clone());
                    match config.save() {
                        Ok(()) => OakResult::success_with_data(
                            format!("Added lib path: {}", path),
                            json!({
                                "path": path,
                                "total_paths": config.lib_paths.len()
                            })
                        ),
                        Err(e) => OakResult::error(e.message),
                    }
                } else {
                    OakResult::error(format!("Path {} already exists", path))
                }
            }
            Err(e) => OakResult::error(e.message),
        }
    }

    /// Remove um caminho de biblioteca
    pub fn remove_lib_path(&self, path: String) -> OakResult {
        if !self.project_exists() {
            return OakResult::error("No Oak project found. Run init first.".to_string());
        }

        match OakConfig::load() {
            Ok(mut config) => {
                if let Some(pos) = config.lib_paths.iter().position(|x| *x == path) {
                    config.lib_paths.remove(pos);
                    match config.save() {
                        Ok(()) => OakResult::success_with_data(
                            format!("Removed lib path: {}", path),
                            json!({
                                "path": path,
                                "total_paths": config.lib_paths.len()
                            })
                        ),
                        Err(e) => OakResult::error(e.message),
                    }
                } else {
                    OakResult::error(format!("Path {} not found", path))
                }
            }
            Err(e) => OakResult::error(e.message),
        }
    }

    /// Valida a configuração do projeto
    pub fn validate_config(&self) -> OakResult {
        if !self.project_exists() {
            return OakResult::error("No Oak project found".to_string());
        }

        match OakConfig::load() {
            Ok(config) => {
                let mut issues = Vec::new();

                // Verificar se caminhos de lib existem
                for path in &config.lib_paths {
                    if !Path::new(path).exists() {
                        issues.push(format!("Library path does not exist: {}", path));
                    }
                }

                // Verificar nome do projeto
                if config.name.trim().is_empty() {
                    issues.push("Project name is empty".to_string());
                }

                if issues.is_empty() {
                    OakResult::success_with_data(
                        "Configuration is valid".to_string(),
                        json!({
                            "valid": true,
                            "lib_paths_checked": config.lib_paths.len(),
                            "dependencies_count": config.dependencies.len()
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
            Err(e) => OakResult::error(e.message),
        }
    }
}

/// API Externa para integração com outras ferramentas
pub mod api {
    use super::*;
    use std::ffi::{CString, CStr};
    use std::os::raw::c_char;

    /// Versão C-style para integração externa
    /// Inicializa um projeto Oak
    #[no_mangle]
    pub extern "C" fn oak_init_project(name: *const c_char, description: *const c_char) -> *mut c_char {
        let manager = OakManager::new();
        
        let name_opt = if name.is_null() {
            None
        } else {
            unsafe { CStr::from_ptr(name) }.to_str().ok().map(|s| s.to_string())
        };

        let desc_opt = if description.is_null() {
            None
        } else {
            unsafe { CStr::from_ptr(description) }.to_str().ok().map(|s| s.to_string())
        };

        let result = manager.init_project(name_opt, desc_opt);
        let json_result = serde_json::to_string(&json!({
            "success": result.success,
            "message": result.message,
            "data": result.data
        })).unwrap_or_else(|_| "{}".to_string());

        CString::new(json_result).unwrap().into_raw()
    }

    /// Adiciona um pacote
    #[no_mangle]
    pub extern "C" fn oak_add_package(package_name: *const c_char, version: *const c_char) -> *mut c_char {
        let manager = OakManager::new();
        
        let package = unsafe { CStr::from_ptr(package_name) }.to_str()
            .unwrap_or("unknown").to_string();

        let version_opt = if version.is_null() {
            None
        } else {
            unsafe { CStr::from_ptr(version) }.to_str().ok().map(|s| s.to_string())
        };

        let result = manager.add_package(package, version_opt);
        let json_result = serde_json::to_string(&json!({
            "success": result.success,
            "message": result.message,
            "data": result.data
        })).unwrap_or_else(|_| "{}".to_string());

        CString::new(json_result).unwrap().into_raw()
    }

    /// Lista pacotes
    #[no_mangle]
    pub extern "C" fn oak_list_packages() -> *mut c_char {
        let manager = OakManager::new();
        let result = manager.list_packages();
        let json_result = serde_json::to_string(&json!({
            "success": result.success,
            "message": result.message,
            "data": result.data
        })).unwrap_or_else(|_| "{}".to_string());

        CString::new(json_result).unwrap().into_raw()
    }

    /// Libera memória alocada por strings C
    #[no_mangle]
    pub extern "C" fn oak_free_string(s: *mut c_char) {
        if !s.is_null() {
            unsafe {
                CString::from_raw(s);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_oak_manager_init() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("oaklibs.json");
        
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let manager = OakManager::with_config_path(config_path.to_string_lossy().to_string());
        let result = manager.init_project(Some("test-project".to_string()), None);
        
        assert!(result.success);
        assert!(manager.project_exists());
    }

    #[test]
    fn test_oak_manager_add_package() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let manager = OakManager::new();
        
        // Inicializar projeto primeiro
        manager.init_project(None, None);
        
        // Adicionar pacote
        let result = manager.add_package("test-package".to_string(), Some("1.0.0".to_string()));
        assert!(result.success);
        
        // Verificar se foi adicionado
        let list_result = manager.list_packages();
        assert!(list_result.success);
    }
}
