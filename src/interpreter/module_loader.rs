// src/interpreter/module_loader.rs
// Carregador de módulos para suporte a múltiplos arquivos

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::Value;
use crate::lexer::tokenizer::Lexer;
use crate::parser::parser::Parser;
use crate::parser::ast::Stmt;
use crate::interpreter::env::Value as EnvValue;
use crate::interpreter::errors::{DryadError, ErrorSeverity};

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub path: String,
    pub statements: Vec<Stmt>,
    pub exports: HashMap<String, EnvValue>,
    pub loaded: bool,
}

impl Module {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            statements: Vec::new(),
            exports: HashMap::new(),
            loaded: false,
        }
    }
}

pub struct ModuleLoader {
    modules: HashMap<String, Module>,
    search_paths: Vec<String>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        let mut loader = Self {
            modules: HashMap::new(),
            search_paths: vec![
                ".".to_string(),
                "modules".to_string(),
                "tests/modules".to_string(),
                "lib".to_string(),
            ],
        };
        
        // Load oaklibs.json if it exists
        loader.load_oak_config();
        loader
    }
    
    pub fn add_search_path(&mut self, path: String) {
        self.search_paths.push(path);
    }

    fn load_oak_config(&mut self) {
        if let Ok(content) = fs::read_to_string("oaklibs.json") {
            if let Ok(config) = serde_json::from_str::<Value>(&content) {
                // Add lib_paths from oaklibs.json
                if let Some(lib_paths) = config["lib_paths"].as_array() {
                    for path in lib_paths {
                        if let Some(path_str) = path.as_str() {
                            self.search_paths.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }
    
    pub fn resolve_module_path(&self, module_name: &str) -> Option<String> {
        // Converte namespace path para file path
        let file_name = if module_name.ends_with(".dryad") {
            module_name.to_string()
        } else {
            // Converte Math.Vector -> Math/Vector.dryad
            let path_parts: Vec<&str> = module_name.split('.').collect();
            if path_parts.len() == 1 {
                format!("{}.dryad", module_name)
            } else {
                let dir_path = path_parts[..path_parts.len() - 1].join("/");
                let file_name = path_parts.last().unwrap();
                format!("{}/{}.dryad", dir_path, file_name)
            }
        };
        
        // Procura nos caminhos de busca
        for search_path in &self.search_paths {
            let full_path = if search_path == "." {
                file_name.clone()
            } else {
                format!("{}/{}", search_path, file_name)
            };
            
            if Path::new(&full_path).exists() {
                return Some(full_path);
            }
        }
        
        None
    }
    
    pub fn load_module(&mut self, module_name: &str) -> Result<Vec<Stmt>, DryadError> {
        // Verifica se já está carregado
        if let Some(module) = self.modules.get(module_name) {
            if module.loaded {
                return Ok(module.statements.clone());
            }
        }
        
        // Resolve o caminho do arquivo
        let file_path = self.resolve_module_path(module_name)
            .ok_or_else(|| DryadError::new(
                format!("Módulo não encontrado: {}", module_name),
                None,
                ErrorSeverity::Error,
            ))?;
        
        // Lê o arquivo
        let content = fs::read_to_string(&file_path)
            .map_err(|e| DryadError::new(
                format!("Erro ao ler arquivo {}: {}", file_path, e),
                None,
                ErrorSeverity::Error,
            ))?;
        
        // Faz parse do conteúdo
        let lexer = Lexer::new(&content);
        let mut parser = Parser::new(lexer);
        
        let mut statements = Vec::new();
        while let Some(stmt) = parser.parse_statement() {
            statements.push(stmt);
        }
        
        // Cria o módulo
        let mut module = Module::new(module_name.to_string(), file_path);
        module.statements = statements.clone();
        module.loaded = true;
        
        // Armazena o módulo
        self.modules.insert(module_name.to_string(), module);
        
        Ok(statements)
    }

    pub fn load_file(&mut self, file_path: &str) -> Result<Vec<Stmt>, DryadError> {
        // Remove leading "./" if present
        let clean_path = if file_path.starts_with("./") {
            &file_path[2..]
        } else {
            file_path
        };
        
        // Verifica se já está carregado
        if let Some(module) = self.modules.get(clean_path) {
            if module.loaded {
                return Ok(module.statements.clone());
            }
        }
        
        // Verifica se o arquivo existe
        if !Path::new(clean_path).exists() {
            return Err(DryadError::new(
                format!("Arquivo não encontrado: {}", file_path),
                None,
                ErrorSeverity::Error,
            ));
        }
        
        // Lê o arquivo
        let content = fs::read_to_string(clean_path)
            .map_err(|e| DryadError::new(
                format!("Erro ao ler arquivo {}: {}", clean_path, e),
                None,
                ErrorSeverity::Error,
            ))?;
        
        // Faz parse do conteúdo
        let lexer = Lexer::new(&content);
        let mut parser = Parser::new(lexer);
        
        let mut statements = Vec::new();
        while let Some(stmt) = parser.parse_statement() {
            statements.push(stmt);
        }
        
        // Cria o módulo
        let mut module = Module::new(clean_path.to_string(), clean_path.to_string());
        module.statements = statements.clone();
        module.loaded = true;
        
        // Armazena o módulo
        self.modules.insert(clean_path.to_string(), module);
        
        Ok(statements)
    }
    
    pub fn get_module(&self, module_name: &str) -> Option<&Module> {
        self.modules.get(module_name)
    }
    
    pub fn import_from_module(&self, module_name: &str, item_name: &str) -> Option<EnvValue> {
        if let Some(module) = self.get_module(module_name) {
            module.exports.get(item_name).cloned()
        } else {
            None
        }
    }
}
