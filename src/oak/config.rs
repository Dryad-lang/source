// src/oak/config.rs
//! Configuração e metadados de projetos Oak

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::interpreter::errors::{DryadError, ErrorSeverity};

/// Configuração principal de um projeto Oak
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OakConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub dependencies: HashMap<String, OakDependency>,
    pub dev_dependencies: HashMap<String, OakDependency>,
    pub lib_paths: Vec<String>,
    pub aliases: HashMap<String, String>, // alias -> caminho real
    pub scripts: HashMap<String, String>,
    pub oak_version: String,
}

/// Representação de uma dependência Oak
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OakDependency {
    pub version: String,
    pub source: DependencySource,
    pub optional: bool,
    pub dev: bool,
}

/// Fonte de uma dependência
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencySource {
    Registry(String),        // Nome do registry
    Git(String),            // URL do Git
    Local(PathBuf),         // Caminho local
}

/// Metadados de um pacote Oak
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OakPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub main: Option<String>,
    pub dependencies: HashMap<String, OakDependency>,
    pub config: OakConfig,
}

impl Default for OakConfig {
    fn default() -> Self {
        let mut aliases = HashMap::new();
        aliases.insert("io".to_string(), "lib/io".to_string());
        aliases.insert("math".to_string(), "lib/math".to_string());
        aliases.insert("core".to_string(), "lib/core".to_string());
        aliases.insert("system".to_string(), "lib/system".to_string());
        
        Self {
            name: "my-dryad-project".to_string(),
            version: "1.0.0".to_string(),
            description: "A Dryad project".to_string(),
            author: None,
            license: Some("MIT".to_string()),
            homepage: None,
            repository: None,
            keywords: vec![],
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            lib_paths: vec!["./lib".to_string(), "./oak_modules".to_string()],
            aliases,
            scripts: HashMap::new(),
            oak_version: crate::oak::OAK_VERSION.to_string(),
        }
    }
}

impl Default for OakDependency {
    fn default() -> Self {
        Self {
            version: "*".to_string(),
            source: DependencySource::Registry("official".to_string()),
            optional: false,
            dev: false,
        }
    }
}

impl OakConfig {
    /// Carrega configuração do arquivo oaklibs.json
    pub fn load() -> Result<Self, DryadError> {
        let config_path = "oaklibs.json";
        
        if !Path::new(config_path).exists() {
            return Err(DryadError::new(
                "oaklibs.json not found".to_string(),
                None,
                ErrorSeverity::Error
            ));
        }
        
        let content = fs::read_to_string(config_path)
            .map_err(|e| DryadError::new(
                format!("Failed to read oaklibs.json: {}", e),
                None,
                ErrorSeverity::Error
            ))?;
        
        let config: OakConfig = serde_json::from_str(&content)
            .map_err(|e| DryadError::new(
                format!("Failed to parse oaklibs.json: {}", e),
                None,
                ErrorSeverity::Error
            ))?;
        
        Ok(config)
    }
    
    /// Salva configuração no arquivo oaklibs.json
    pub fn save(&self) -> Result<(), DryadError> {
        let config_path = "oaklibs.json";
        
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| DryadError::new(
                format!("Failed to serialize config: {}", e),
                None,
                ErrorSeverity::Error
            ))?;
        
        fs::write(config_path, content)
            .map_err(|e| DryadError::new(
                format!("Failed to write oaklibs.json: {}", e),
                None,
                ErrorSeverity::Error
            ))?;
        
        Ok(())
    }
    
    /// Adiciona uma dependência
    pub fn add_dependency(&mut self, name: String, dependency: OakDependency) {
        self.dependencies.insert(name, dependency);
    }
    
    /// Remove uma dependência
    pub fn remove_dependency(&mut self, name: &str) -> bool {
        self.dependencies.remove(name).is_some()
    }
    
    /// Adiciona um caminho de biblioteca
    pub fn add_lib_path(&mut self, path: String) -> bool {
        if !self.lib_paths.contains(&path) {
            self.lib_paths.push(path);
            true
        } else {
            false
        }
    }
    
    /// Remove um caminho de biblioteca
    pub fn remove_lib_path(&mut self, path: &str) -> bool {
        if let Some(pos) = self.lib_paths.iter().position(|p| p == path) {
            self.lib_paths.remove(pos);
            true
        } else {
            false
        }
    }
    
    /// Valida a configuração
    pub fn validate(&self) -> Vec<String> {
        let mut issues = Vec::new();
        
        // Validar dependências
        for (name, dependency) in &self.dependencies {
            if name.is_empty() {
                issues.push("Dependency name cannot be empty".to_string());
            }
            if dependency.version.is_empty() {
                issues.push(format!("Dependency '{}' has empty version", name));
            }
        }
        
        // Validar aliases
        for (alias, path) in &self.aliases {
            if alias.is_empty() {
                issues.push("Alias name cannot be empty".to_string());
            }
            if path.is_empty() {
                issues.push(format!("Alias '{}' has empty path", alias));
            }
        }
        
        issues
    }
}

impl OakDependency {
    /// Cria uma nova dependência
    pub fn new(version: String) -> Self {
        Self {
            version,
            source: DependencySource::Registry("official".to_string()),
            optional: false,
            dev: false,
        }
    }
}

impl OakPackage {
    /// Cria um novo pacote
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: "A Dryad package".to_string(),
            main: None,
            dependencies: HashMap::new(),
            config: OakConfig::default(),
        }
    }
    
    /// Carrega um pacote de um diretório
    pub fn load_from_dir(path: &Path) -> Result<Self, DryadError> {
        let oaklibs_path = path.join("oaklibs.json");
        
        if !oaklibs_path.exists() {
            return Err(DryadError::new(
                format!("oaklibs.json not found in {}", path.display()),
                None,
                ErrorSeverity::Error,
            ));
        }
        
        let content = fs::read_to_string(&oaklibs_path)
            .map_err(|e| DryadError::new(
                format!("Failed to read oaklibs.json: {}", e),
                None,
                ErrorSeverity::Error,
            ))?;
        
        let config: OakConfig = serde_json::from_str(&content)
            .map_err(|e| DryadError::new(
                format!("Failed to parse oaklibs.json: {}", e),
                None,
                ErrorSeverity::Error,
            ))?;
        
        // Convert OakConfig to OakPackage
        Ok(OakPackage {
            name: config.name.clone(),
            version: config.version.clone(),
            description: config.description.clone(),
            main: None,
            dependencies: config.dependencies.clone(),
            config,
        })
    }
    
    /// Converte para JSON
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "version": self.version,
            "description": self.description,
            "main": self.main,
            "dependencies": self.dependencies,
            "author": self.config.author,
            "license": self.config.license,
            "repository": self.config.repository,
            "homepage": self.config.homepage,
            "keywords": self.config.keywords
        })
    }
}
