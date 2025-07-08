// src/oak/config.rs
//! Configuração e metadados de projetos Oak

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    Url(String),            // URL direta
}

/// Metadados de um pacote Oak
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OakPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub license: Option<String>,
    pub main: Option<String>,           // Arquivo principal
    pub exports: Vec<String>,           // Exports disponíveis
    pub dependencies: HashMap<String, OakDependency>,
    pub size: Option<u64>,              // Tamanho em bytes
    pub checksum: Option<String>,       // Checksum para verificação
}

impl Default for OakConfig {
    fn default() -> Self {
        Self {
            name: "my-dryad-project".to_string(),
            version: "1.0.0".to_string(),
            description: "A Dryad project using Oak package manager".to_string(),
            author: None,
            license: Some("MIT".to_string()),
            homepage: None,
            repository: None,
            keywords: Vec::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            lib_paths: vec!["./lib".to_string()],
            scripts: HashMap::new(),
            oak_version: crate::oak::OAK_VERSION.to_string(),
        }
    }
}

impl OakConfig {
    /// Cria uma nova configuração com nome customizado
    pub fn new(name: String) -> Self {
        let mut config = Self::default();
        config.name = name;
        config
    }

    /// Carrega configuração do arquivo oaklibs.json
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, DryadError> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(DryadError::new(
                format!("Configuration file not found: {}", path.display()),
                None,
                ErrorSeverity::Error,
            ));
        }

        let content = fs::read_to_string(path)
            .map_err(|e| DryadError::new(
                format!("Failed to read {}: {}", path.display(), e),
                None,
                ErrorSeverity::Error,
            ))?;

        let config: OakConfig = serde_json::from_str(&content)
            .map_err(|e| DryadError::new(
                format!("Failed to parse {}: {}", path.display(), e),
                None,
                ErrorSeverity::Error,
            ))?;

        Ok(config)
    }

    /// Carrega configuração padrão (oaklibs.json)
    pub fn load_default() -> Result<Self, DryadError> {
        Self::load("oaklibs.json")
    }

    /// Salva configuração em arquivo
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), DryadError> {
        let path = path.as_ref();
        
        let json_string = serde_json::to_string_pretty(self)
            .map_err(|e| DryadError::new(
                format!("Failed to serialize config: {}", e),
                None,
                ErrorSeverity::Error,
            ))?;

        // Criar diretório pai se não existir
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| DryadError::new(
                    format!("Failed to create directory {}: {}", parent.display(), e),
                    None,
                    ErrorSeverity::Error,
                ))?;
        }

        fs::write(path, json_string)
            .map_err(|e| DryadError::new(
                format!("Failed to write {}: {}", path.display(), e),
                None,
                ErrorSeverity::Error,
            ))?;

        Ok(())
    }

    /// Salva configuração padrão (oaklibs.json)
    pub fn save_default(&self) -> Result<(), DryadError> {
        self.save("oaklibs.json")
    }

    /// Adiciona uma dependência
    pub fn add_dependency(&mut self, name: String, dependency: OakDependency) {
        if dependency.dev {
            self.dev_dependencies.insert(name, dependency);
        } else {
            self.dependencies.insert(name, dependency);
        }
    }

    /// Remove uma dependência
    pub fn remove_dependency(&mut self, name: &str) -> bool {
        self.dependencies.remove(name).is_some() || 
        self.dev_dependencies.remove(name).is_some()
    }

    /// Obtém uma dependência (dev ou normal)
    pub fn get_dependency(&self, name: &str) -> Option<&OakDependency> {
        self.dependencies.get(name)
            .or_else(|| self.dev_dependencies.get(name))
    }

    /// Lista todas as dependências
    pub fn all_dependencies(&self) -> HashMap<String, &OakDependency> {
        let mut all = HashMap::new();
        
        for (name, dep) in &self.dependencies {
            all.insert(name.clone(), dep);
        }
        
        for (name, dep) in &self.dev_dependencies {
            all.insert(name.clone(), dep);
        }
        
        all
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
        if let Some(pos) = self.lib_paths.iter().position(|x| x == path) {
            self.lib_paths.remove(pos);
            true
        } else {
            false
        }
    }

    /// Adiciona um script
    pub fn add_script(&mut self, name: String, command: String) {
        self.scripts.insert(name, command);
    }

    /// Remove um script
    pub fn remove_script(&mut self, name: &str) -> bool {
        self.scripts.remove(name).is_some()
    }

    /// Valida a configuração
    pub fn validate(&self) -> Vec<String> {
        let mut issues = Vec::new();

        // Verificar nome
        if self.name.trim().is_empty() {
            issues.push("Project name cannot be empty".to_string());
        }

        // Verificar versão
        if self.version.trim().is_empty() {
            issues.push("Project version cannot be empty".to_string());
        }

        // Verificar caminhos de lib
        for path in &self.lib_paths {
            if !Path::new(path).exists() {
                issues.push(format!("Library path does not exist: {}", path));
            }
        }

        // Verificar dependências circulares básica
        if self.dependencies.contains_key(&self.name) {
            issues.push("Project cannot depend on itself".to_string());
        }

        issues
    }

    /// Converte para JSON Value para compatibilidade
    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or(json!({}))
    }
}

impl Default for OakDependency {
    fn default() -> Self {
        Self {
            version: "latest".to_string(),
            source: DependencySource::Registry("default".to_string()),
            optional: false,
            dev: false,
        }
    }
}

impl OakDependency {
    /// Cria uma nova dependência simples
    pub fn new(version: String) -> Self {
        Self {
            version,
            ..Default::default()
        }
    }

    /// Cria uma dependência de desenvolvimento
    pub fn dev(version: String) -> Self {
        Self {
            version,
            dev: true,
            ..Default::default()
        }
    }

    /// Cria uma dependência opcional
    pub fn optional(version: String) -> Self {
        Self {
            version,
            optional: true,
            ..Default::default()
        }
    }

    /// Cria uma dependência local
    pub fn local<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            version: "local".to_string(),
            source: DependencySource::Local(path.into()),
            ..Default::default()
        }
    }

    /// Cria uma dependência Git
    pub fn git(url: String, version: Option<String>) -> Self {
        Self {
            version: version.unwrap_or_else(|| "main".to_string()),
            source: DependencySource::Git(url),
            ..Default::default()
        }
    }
}

impl Default for OakPackage {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: "1.0.0".to_string(),
            description: String::new(),
            author: None,
            license: None,
            main: None,
            exports: Vec::new(),
            dependencies: HashMap::new(),
            size: None,
            checksum: None,
        }
    }
}

impl OakPackage {
    /// Cria um novo pacote
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            ..Default::default()
        }
    }

    /// Carrega informações do pacote de um diretório
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Self, DryadError> {
        let dir = dir.as_ref();
        let config_path = dir.join("oaklibs.json");
        
        if config_path.exists() {
            let config = OakConfig::load(config_path)?;
            Ok(Self {
                name: config.name,
                version: config.version,
                description: config.description,
                author: config.author,
                license: config.license,
                main: None, // TODO: detectar arquivo principal
                exports: Vec::new(), // TODO: detectar exports
                dependencies: config.dependencies,
                size: None, // TODO: calcular tamanho
                checksum: None, // TODO: calcular checksum
            })
        } else {
            Err(DryadError::new(
                format!("No oaklibs.json found in {}", dir.display()),
                None,
                ErrorSeverity::Error,
            ))
        }
    }

    /// Converte para JSON Value
    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or(json!({}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_oak_config_default() {
        let config = OakConfig::default();
        assert_eq!(config.name, "my-dryad-project");
        assert_eq!(config.version, "1.0.0");
        assert!(config.lib_paths.contains(&"./lib".to_string()));
    }

    #[test]
    fn test_oak_config_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_oak.json");
        
        let mut config = OakConfig::new("test-project".to_string());
        config.add_dependency("test-dep".to_string(), OakDependency::new("1.0.0".to_string()));
        
        config.save(&config_path).unwrap();
        let loaded_config = OakConfig::load(&config_path).unwrap();
        
        assert_eq!(config.name, loaded_config.name);
        assert_eq!(config.dependencies.len(), loaded_config.dependencies.len());
    }

    #[test]
    fn test_dependency_types() {
        let normal_dep = OakDependency::new("1.0.0".to_string());
        assert!(!normal_dep.dev);
        assert!(!normal_dep.optional);

        let dev_dep = OakDependency::dev("2.0.0".to_string());
        assert!(dev_dep.dev);

        let optional_dep = OakDependency::optional("3.0.0".to_string());
        assert!(optional_dep.optional);
    }

    #[test]
    fn test_config_validation() {
        let mut config = OakConfig::default();
        config.name = "".to_string(); // Nome vazio
        
        let issues = config.validate();
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.contains("name cannot be empty")));
    }
}
