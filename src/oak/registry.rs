// src/oak/registry.rs
//! Sistema de registries para pacotes Oak

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde_json::{json, Value};

use super::config::{OakPackage, OakDependency};

/// Trait para diferentes tipos de registries
pub trait PackageRegistry: Send + Sync {
    /// Inicializa o registry
    fn initialize(&self) -> Result<(), String>;
    
    /// Busca um pacote por nome e versão
    fn find_package(&self, name: &str, version: &str) -> Result<Option<OakPackage>, String>;
    
    /// Lista pacotes disponíveis
    fn list_packages(&self) -> Result<Vec<OakPackage>, String>;
    
    /// Instala um pacote
    fn install_package(&self, package: &OakPackage) -> Result<(), String>;
    
    /// Remove um pacote
    fn remove_package(&self, name: &str, version: &str) -> Result<(), String>;
    
    /// Publica um pacote
    fn publish_package(&self, package: &OakPackage, source_path: &Path) -> Result<(), String>;
    
    /// Verifica se um pacote está instalado
    fn is_installed(&self, name: &str, version: &str) -> bool;
    
    /// Obtém metadados de um pacote
    fn get_metadata(&self, name: &str, version: &str) -> Result<Value, String>;
}

/// Registry local (diretório local)
pub struct LocalRegistry {
    cache_path: PathBuf,
    packages: HashMap<String, Vec<OakPackage>>,
}

/// Registry remoto (URL/API)
pub struct RemoteRegistry {
    base_url: String,
    cache_path: PathBuf,
    auth_token: Option<String>,
}

/// Registry Git (repositório Git)
pub struct GitRegistry {
    repository_url: String,
    cache_path: PathBuf,
    branch: String,
}

/// Registry composto (múltiplos registries)
pub struct CompositeRegistry {
    registries: Vec<Box<dyn PackageRegistry>>,
    priority_order: Vec<usize>,
}

impl LocalRegistry {
    /// Cria um novo registry local
    pub fn new(cache_path: PathBuf) -> Self {
        Self {
            cache_path,
            packages: HashMap::new(),
        }
    }

    /// Carrega pacotes do cache local
    fn load_cache(&mut self) -> Result<(), String> {
        if !self.cache_path.exists() {
            return Ok(());
        }

        // Escanear diretórios de pacotes
        let entries = fs::read_dir(&self.cache_path)
            .map_err(|e| format!("Failed to read cache directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(package_name) = path.file_name().and_then(|n| n.to_str()) {
                    let versions = self.scan_package_versions(&path)?;
                    self.packages.insert(package_name.to_string(), versions);
                }
            }
        }

        Ok(())
    }

    /// Escaneia versões de um pacote
    fn scan_package_versions(&self, package_path: &Path) -> Result<Vec<OakPackage>, String> {
        let mut versions = Vec::new();
        
        let entries = fs::read_dir(package_path)
            .map_err(|e| format!("Failed to read package directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read version entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Ok(package) = OakPackage::load_from_dir(&path) {
                    versions.push(package);
                }
            }
        }

        versions.sort_by(|a, b| a.version.cmp(&b.version));
        Ok(versions)
    }

    /// Obtém caminho para um pacote específico
    fn get_package_path(&self, name: &str, version: &str) -> PathBuf {
        self.cache_path.join(name).join(version)
    }
}

impl PackageRegistry for LocalRegistry {
    fn initialize(&self) -> Result<(), String> {
        if !self.cache_path.exists() {
            fs::create_dir_all(&self.cache_path)
                .map_err(|e| format!("Failed to create cache directory: {}", e))?;
        }
        Ok(())
    }

    fn find_package(&self, name: &str, version: &str) -> Result<Option<OakPackage>, String> {
        if let Some(versions) = self.packages.get(name) {
            if version == "latest" {
                return Ok(versions.last().cloned());
            }
            
            for package in versions {
                if package.version == version {
                    return Ok(Some(package.clone()));
                }
            }
        }
        
        // Se não encontrado na cache, tentar carregar do disco
        let package_path = self.get_package_path(name, version);
        if package_path.exists() {
            match OakPackage::load_from_dir(&package_path) {
                Ok(package) => Ok(Some(package)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn list_packages(&self) -> Result<Vec<OakPackage>, String> {
        let mut all_packages = Vec::new();
        
        for versions in self.packages.values() {
            all_packages.extend(versions.clone());
        }
        
        Ok(all_packages)
    }

    fn install_package(&self, package: &OakPackage) -> Result<(), String> {
        let package_path = self.get_package_path(&package.name, &package.version);
        
        // Criar diretório do pacote
        fs::create_dir_all(&package_path)
            .map_err(|e| format!("Failed to create package directory: {}", e))?;
        
        // Salvar metadados do pacote
        let metadata_path = package_path.join("oaklibs.json");
        let metadata = serde_json::to_string_pretty(&package.to_json())
            .map_err(|e| format!("Failed to serialize package metadata: {}", e))?;
        
        fs::write(metadata_path, metadata)
            .map_err(|e| format!("Failed to write package metadata: {}", e))?;
        
        Ok(())
    }

    fn remove_package(&self, name: &str, version: &str) -> Result<(), String> {
        let package_path = self.get_package_path(name, version);
        
        if package_path.exists() {
            fs::remove_dir_all(&package_path)
                .map_err(|e| format!("Failed to remove package: {}", e))?;
        }
        
        Ok(())
    }

    fn publish_package(&self, package: &OakPackage, source_path: &Path) -> Result<(), String> {
        let package_path = self.get_package_path(&package.name, &package.version);
        
        // Criar diretório do pacote
        fs::create_dir_all(&package_path)
            .map_err(|e| format!("Failed to create package directory: {}", e))?;
        
        // Copiar arquivos do source_path para package_path
        copy_dir_all(source_path, &package_path)
            .map_err(|e| format!("Failed to copy package files: {}", e))?;
        
        Ok(())
    }

    fn is_installed(&self, name: &str, version: &str) -> bool {
        self.get_package_path(name, version).exists()
    }

    fn get_metadata(&self, name: &str, version: &str) -> Result<Value, String> {
        let package_path = self.get_package_path(name, version);
        let metadata_path = package_path.join("oaklibs.json");
        
        if metadata_path.exists() {
            let content = fs::read_to_string(metadata_path)
                .map_err(|e| format!("Failed to read metadata: {}", e))?;
            
            let metadata: Value = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse metadata: {}", e))?;
            
            Ok(metadata)
        } else {
            Err("Package metadata not found".to_string())
        }
    }
}

impl RemoteRegistry {
    /// Cria um novo registry remoto
    pub fn new(base_url: String, cache_path: PathBuf) -> Self {
        Self {
            base_url,
            cache_path,
            auth_token: None,
        }
    }

    /// Define token de autenticação
    pub fn with_auth(mut self, token: String) -> Self {
        self.auth_token = Some(token);
        self
    }
}

impl PackageRegistry for RemoteRegistry {
    fn initialize(&self) -> Result<(), String> {
        if !self.cache_path.exists() {
            fs::create_dir_all(&self.cache_path)
                .map_err(|e| format!("Failed to create cache directory: {}", e))?;
        }
        
        // TODO: Verificar conectividade com registry remoto
        Ok(())
    }

    fn find_package(&self, name: &str, version: &str) -> Result<Option<OakPackage>, String> {
        // TODO: Implementar busca remota
        // Por enquanto, buscar apenas no cache local
        let local_registry = LocalRegistry::new(self.cache_path.clone());
        local_registry.find_package(name, version)
    }

    fn list_packages(&self) -> Result<Vec<OakPackage>, String> {
        // TODO: Implementar listagem remota
        Err("Remote listing not implemented yet".to_string())
    }

    fn install_package(&self, package: &OakPackage) -> Result<(), String> {
        // TODO: Implementar download e instalação
        Err("Remote installation not implemented yet".to_string())
    }

    fn remove_package(&self, name: &str, version: &str) -> Result<(), String> {
        let local_registry = LocalRegistry::new(self.cache_path.clone());
        local_registry.remove_package(name, version)
    }

    fn publish_package(&self, package: &OakPackage, source_path: &Path) -> Result<(), String> {
        // TODO: Implementar upload para registry remoto
        Err("Remote publishing not implemented yet".to_string())
    }

    fn is_installed(&self, name: &str, version: &str) -> bool {
        let local_registry = LocalRegistry::new(self.cache_path.clone());
        local_registry.is_installed(name, version)
    }

    fn get_metadata(&self, name: &str, version: &str) -> Result<Value, String> {
        let local_registry = LocalRegistry::new(self.cache_path.clone());
        local_registry.get_metadata(name, version)
    }
}

impl GitRegistry {
    /// Cria um novo registry Git
    pub fn new(repository_url: String, cache_path: PathBuf) -> Self {
        Self {
            repository_url,
            cache_path,
            branch: "main".to_string(),
        }
    }

    /// Define branch específica
    pub fn with_branch(mut self, branch: String) -> Self {
        self.branch = branch;
        self
    }
}

impl PackageRegistry for GitRegistry {
    fn initialize(&self) -> Result<(), String> {
        if !self.cache_path.exists() {
            fs::create_dir_all(&self.cache_path)
                .map_err(|e| format!("Failed to create cache directory: {}", e))?;
        }
        
        // TODO: Implementar clone do repositório Git
        Ok(())
    }

    fn find_package(&self, name: &str, version: &str) -> Result<Option<OakPackage>, String> {
        // TODO: Implementar busca no repositório Git
        Err("Git registry search not implemented yet".to_string())
    }

    fn list_packages(&self) -> Result<Vec<OakPackage>, String> {
        // TODO: Implementar listagem de pacotes Git
        Err("Git registry listing not implemented yet".to_string())
    }

    fn install_package(&self, package: &OakPackage) -> Result<(), String> {
        // TODO: Implementar instalação de pacotes Git
        Err("Git package installation not implemented yet".to_string())
    }

    fn remove_package(&self, name: &str, version: &str) -> Result<(), String> {
        // TODO: Implementar remoção de pacotes Git
        Err("Git package removal not implemented yet".to_string())
    }

    fn publish_package(&self, package: &OakPackage, source_path: &Path) -> Result<(), String> {
        // TODO: Implementar commit e push para Git
        Err("Git publishing not implemented yet".to_string())
    }

    fn is_installed(&self, name: &str, version: &str) -> bool {
        // TODO: Verificar se pacote está no cache Git
        false
    }

    fn get_metadata(&self, name: &str, version: &str) -> Result<Value, String> {
        // TODO: Obter metadados do repositório Git
        Err("Git metadata not implemented yet".to_string())
    }
}

impl CompositeRegistry {
    /// Cria um registry composto
    pub fn new(registries: Vec<Box<dyn PackageRegistry>>) -> Self {
        let priority_order = (0..registries.len()).collect();
        Self {
            registries,
            priority_order,
        }
    }

    /// Define ordem de prioridade dos registries
    pub fn with_priority(mut self, priority: Vec<usize>) -> Self {
        self.priority_order = priority;
        self
    }
}

impl PackageRegistry for CompositeRegistry {
    fn initialize(&self) -> Result<(), String> {
        for registry in &self.registries {
            registry.initialize()?;
        }
        Ok(())
    }

    fn find_package(&self, name: &str, version: &str) -> Result<Option<OakPackage>, String> {
        for &index in &self.priority_order {
            if let Some(registry) = self.registries.get(index) {
                match registry.find_package(name, version) {
                    Ok(Some(package)) => return Ok(Some(package)),
                    Ok(None) => continue,
                    Err(e) => eprintln!("Registry {} error: {}", index, e),
                }
            }
        }
        Ok(None)
    }

    fn list_packages(&self) -> Result<Vec<OakPackage>, String> {
        let mut all_packages = Vec::new();
        
        for registry in &self.registries {
            match registry.list_packages() {
                Ok(mut packages) => all_packages.append(&mut packages),
                Err(e) => eprintln!("Registry error: {}", e),
            }
        }
        
        // Remover duplicatas
        all_packages.sort_by(|a, b| {
            a.name.cmp(&b.name).then(a.version.cmp(&b.version))
        });
        all_packages.dedup_by(|a, b| a.name == b.name && a.version == b.version);
        
        Ok(all_packages)
    }

    fn install_package(&self, package: &OakPackage) -> Result<(), String> {
        // Tentar instalar no primeiro registry disponível
        for registry in &self.registries {
            if let Ok(()) = registry.install_package(package) {
                return Ok(());
            }
        }
        Err("Failed to install package in any registry".to_string())
    }

    fn remove_package(&self, name: &str, version: &str) -> Result<(), String> {
        let mut success = false;
        
        for registry in &self.registries {
            if registry.remove_package(name, version).is_ok() {
                success = true;
            }
        }
        
        if success {
            Ok(())
        } else {
            Err("Failed to remove package from any registry".to_string())
        }
    }

    fn publish_package(&self, package: &OakPackage, source_path: &Path) -> Result<(), String> {
        // Publicar no primeiro registry que suporte publicação
        for registry in &self.registries {
            if let Ok(()) = registry.publish_package(package, source_path) {
                return Ok(());
            }
        }
        Err("Failed to publish package to any registry".to_string())
    }

    fn is_installed(&self, name: &str, version: &str) -> bool {
        self.registries.iter().any(|r| r.is_installed(name, version))
    }

    fn get_metadata(&self, name: &str, version: &str) -> Result<Value, String> {
        for registry in &self.registries {
            if let Ok(metadata) = registry.get_metadata(name, version) {
                return Ok(metadata);
            }
        }
        Err("Package metadata not found in any registry".to_string())
    }
}

/// Função auxiliar para copiar diretórios recursivamente
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_local_registry() {
        let temp_dir = TempDir::new().unwrap();
        let registry = LocalRegistry::new(temp_dir.path().to_path_buf());
        
        assert!(registry.initialize().is_ok());
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn test_package_installation() {
        let temp_dir = TempDir::new().unwrap();
        let registry = LocalRegistry::new(temp_dir.path().to_path_buf());
        
        registry.initialize().unwrap();
        
        let package = OakPackage::new("test-package".to_string(), "1.0.0".to_string());
        assert!(registry.install_package(&package).is_ok());
        assert!(registry.is_installed("test-package", "1.0.0"));
    }

    #[test]
    fn test_composite_registry() {
        let temp_dir1 = TempDir::new().unwrap();
        let temp_dir2 = TempDir::new().unwrap();
        
        let registry1 = Box::new(LocalRegistry::new(temp_dir1.path().to_path_buf()));
        let registry2 = Box::new(LocalRegistry::new(temp_dir2.path().to_path_buf()));
        
        let composite = CompositeRegistry::new(vec![registry1, registry2]);
        assert!(composite.initialize().is_ok());
    }
}
