// src/interpreter/module_loader_new.rs
// Carregador de módulos refatorado para suporte completo ao Oak

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::env;
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
    aliases: HashMap<String, String>, // alias -> real_path
    oak_config: Option<crate::oak::config::OakConfig>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        let mut search_paths = vec![
            "./lib".to_string(),
            "./oak_modules".to_string(),
        ];
        
        let mut aliases = HashMap::new();
        
        // 1. Detectar diretório do executável e procurar lib ao lado do exe
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let lib_path = exe_dir.join("lib");
                if lib_path.exists() {
                    search_paths.push(lib_path.to_string_lossy().to_string());
                }
            }
        }
        
        // 2. Procurar oak_modules local (projeto com oak init)
        if std::path::Path::new("./oak_modules").exists() {
            search_paths.push("./oak_modules".to_string());
        }
        
        // 3. Aliases padrão para common libs
        aliases.insert("io".to_string(), "lib/io".to_string());
        aliases.insert("math".to_string(), "lib/math".to_string());
        aliases.insert("core".to_string(), "lib/core".to_string());
        aliases.insert("system".to_string(), "lib/system".to_string());
        
        let mut loader = Self {
            modules: HashMap::new(),
            search_paths,
            aliases,
            oak_config: None,
        };
        
        // 4. Carregar configuração Oak se existir
        loader.load_oak_config();
        
        loader
    }
    
    pub fn add_search_path(&mut self, path: String) {
        if !self.search_paths.contains(&path) {
            self.search_paths.push(path);
        }
    }
    
    pub fn add_alias(&mut self, alias: String, real_path: String) {
        self.aliases.insert(alias, real_path);
    }
    
    pub fn resolve_alias(&self, module_name: &str) -> String {
        // Primeiro verifica se é um alias
        if let Some(real_path) = self.aliases.get(module_name) {
            return real_path.clone();
        }
        
        // Se não é alias, verifica se contém "." (namespace)
        if let Some(dot_pos) = module_name.find('.') {
            let (prefix, suffix) = module_name.split_at(dot_pos);
            if let Some(real_prefix) = self.aliases.get(prefix) {
                return format!("{}{}", real_prefix, suffix);
            }
        }
        
        // Retorna o nome original se não encontrou alias
        module_name.to_string()
    }
    
    fn load_oak_config(&mut self) {
        if let Ok(config) = crate::oak::config::OakConfig::load() {
            // Adicionar lib_paths do Oak
            for path in &config.lib_paths {
                self.add_search_path(path.clone());
            }
            
            // Adicionar aliases do Oak
            for (alias, path) in &config.aliases {
                self.add_alias(alias.clone(), path.clone());
            }
            
            self.oak_config = Some(config);
        }
    }
    
    pub fn resolve_module_path(&self, module_name: &str) -> Option<String> {
        // 1. Resolver alias primeiro
        let resolved_name = self.resolve_alias(module_name);
        
        // 2. Determinar extensão
        let file_name = if resolved_name.ends_with(".dryad") {
            resolved_name
        } else {
            // Converte io.console -> io/console.dryad
            let path_with_extension = format!("{}.dryad", resolved_name.replace('.', "/"));
            path_with_extension
        };
        
        // 3. Procurar nos search paths
        for search_path in &self.search_paths {
            let full_path = Path::new(search_path).join(&file_name);
            if full_path.exists() {
                return Some(full_path.to_string_lossy().to_string());
            }
        }
        
        // 4. Tentar como caminho absoluto ou relativo
        if Path::new(&file_name).exists() {
            return Some(file_name);
        }
        
        None
    }
    
    pub fn load_module(&mut self, module_name: &str) -> Result<Vec<Stmt>, DryadError> {
        // Verificar se já foi carregado
        if let Some(module) = self.modules.get(module_name) {
            if module.loaded {
                return Ok(module.statements.clone());
            }
        }
        
        // Resolver caminho
        let file_path = self.resolve_module_path(module_name)
            .ok_or_else(|| DryadError::new(
                format!("Module '{}' not found in search paths: {:?}", module_name, self.search_paths),
                None,
                ErrorSeverity::Error
            ))?;
        
        // Carregar arquivo
        self.load_file(&file_path)
    }

    pub fn load_file(&mut self, file_path: &str) -> Result<Vec<Stmt>, DryadError> {
        // Ler arquivo
        let content = fs::read_to_string(file_path)
            .map_err(|e| DryadError::new(
                format!("Failed to read file '{}': {}", file_path, e),
                None,
                ErrorSeverity::Error
            ))?;
        
        // Fazer parse
        let lexer = Lexer::new(&content);
        let mut parser = Parser::new(lexer);
        
        let mut statements = Vec::new();
        while let Some(stmt) = parser.parse_statement() {
            statements.push(stmt);
        }
        
        // Verificar erros do parser
        let parser_errors = parser.get_errors();
        if !parser_errors.is_empty() {
            let error_messages: Vec<String> = parser_errors.iter()
                .map(|e| e.to_string())
                .collect();
            return Err(DryadError::new(
                format!("Parse errors in '{}': {}", file_path, error_messages.join(", ")),
                None,
                ErrorSeverity::Error
            ));
        }
        
        // Salvar módulo
        let mut module = Module::new(file_path.to_string(), file_path.to_string());
        module.statements = statements.clone();
        module.loaded = true;
        
        self.modules.insert(file_path.to_string(), module);
        
        Ok(statements)
    }
    
    pub fn get_module(&self, module_name: &str) -> Option<&Module> {
        self.modules.get(module_name)
    }
    
    pub fn import_from_module(&self, module_name: &str, item_name: &str) -> Option<EnvValue> {
        if let Some(module) = self.modules.get(module_name) {
            module.exports.get(item_name).cloned()
        } else {
            None
        }
    }
    
    pub fn get_search_paths(&self) -> &Vec<String> {
        &self.search_paths
    }
    
    pub fn get_aliases(&self) -> &HashMap<String, String> {
        &self.aliases
    }
}
