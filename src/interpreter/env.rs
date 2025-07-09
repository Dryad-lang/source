// src/interpreter/env.rs

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::parser::ast::{Stmt, FieldDecl, Visibility};

#[derive(Debug, Clone)]
pub struct Env {
    variables: HashMap<String, Value>,
    this_binding: Option<Value>,
    aliases: HashMap<String, String>, // alias -> full_module_path
    exported_items: HashMap<String, Value>, // exported items from modules
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Class(Rc<Class>),
    Instance(Rc<RefCell<Instance>>),
    Function {
        name: String,
        params: Vec<String>,
        body: Stmt,
        visibility: Visibility,
        is_static: bool,
    },
    Exception {
        message: String,
        value: Option<Box<Value>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Value>,
    pub static_methods: HashMap<String, Value>,
    pub fields: Vec<FieldDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub class: Rc<Class>,
    pub fields: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            this_binding: None,
            aliases: HashMap::new(),
            exported_items: HashMap::new(),
        }
    }

    pub fn with_this(this: Value) -> Self {
        Self {
            variables: HashMap::new(),
            this_binding: Some(this),
            aliases: HashMap::new(),
            exported_items: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if name == "this" {
            self.this_binding.clone()
        } else {
            // First try regular variables
            if let Some(value) = self.variables.get(name) {
                return Some(value.clone());
            }
            
            // Then try alias resolution
            if let Some(value) = self.resolve_with_alias(name) {
                return Some(value);
            }
            
            // Finally try exported items
            self.exported_items.get(name).cloned()
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    pub fn remove(&mut self, name: &str) {
        self.variables.remove(name);
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (key, value) in &self.variables {
            result.push_str(&format!("{}: {:?}\n", key, value));
        }
        result
    }

    pub fn define_in_namespace(&mut self, namespace: &str, name: &str, value: Value) {
        let full_name = if namespace.is_empty() {
            name.to_string()
        } else {
            format!("{}.{}", namespace, name)
        };
        self.variables.insert(full_name, value);
    }
    
    pub fn get_from_namespace(&self, namespace: &str, name: &str) -> Option<Value> {
        let full_name = if namespace.is_empty() {
            name.to_string()
        } else {
            format!("{}.{}", namespace, name)
        };
        self.variables.get(&full_name).cloned()
    }
    
    pub fn resolve_namespace_path(&self, path: &str) -> Option<Value> {
        // Tenta encontrar o valor com o caminho completo
        if let Some(value) = self.variables.get(path) {
            return Some(value.clone());
        }
        
        // Se não encontrar, tenta com caminhos parciais
        let parts: Vec<&str> = path.split('.').collect();
        for i in 1..=parts.len() {
            let partial_path = parts[..i].join(".");
            if let Some(value) = self.variables.get(&partial_path) {
                return Some(value.clone());
            }
        }
        
        None
    }

    pub fn add_alias(&mut self, alias: String, module_path: String) {
        self.aliases.insert(alias, module_path);
    }
    
    pub fn export_item(&mut self, name: String, value: Value) {
        self.exported_items.insert(name, value);
    }
    
    pub fn get_exported(&self, name: &str) -> Option<Value> {
        self.exported_items.get(name).cloned()
    }
    
    pub fn get_all_exported(&self) -> HashMap<String, Value> {
        self.exported_items.clone()
    }
    
    pub fn get_all_variables(&self) -> HashMap<String, Value> {
        self.variables.clone()
    }
    
    pub fn resolve_with_alias(&self, path: &str) -> Option<Value> {
        // Check if path starts with an alias
        let parts: Vec<&str> = path.split('.').collect();
        if let Some(first_part) = parts.first() {
            if let Some(full_module_path) = self.aliases.get(*first_part) {
                // Replace alias with full path
                let remaining_path = if parts.len() > 1 {
                    parts[1..].join(".")
                } else {
                    String::new()
                };
                
                let resolved_path = if remaining_path.is_empty() {
                    full_module_path.clone()
                } else {
                    format!("{}.{}", full_module_path, remaining_path)
                };
                
                return self.resolve_namespace_path(&resolved_path);
            }
        }
        
        None
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Class(_) => true,
            Value::Instance(_) => true,
            Value::Function { .. } => true,
            Value::Exception { .. } => false,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Class(a), Value::Class(b)) => a.name == b.name,
            (Value::Instance(a), Value::Instance(b)) => {
                let a_ref = a.borrow();
                let b_ref = b.borrow();
                a_ref.class.name == b_ref.class.name && a_ref.fields == b_ref.fields
            }
            (Value::Function { name: a, .. }, Value::Function { name: b, .. }) => a == b,
            (Value::Exception { message: a, .. }, Value::Exception { message: b, .. }) => a == b,
            _ => false,
        }
    }
}

impl Class {
    pub fn new(name: String, fields: Vec<FieldDecl>) -> Self {
        Self {
            name,
            methods: HashMap::new(),
            static_methods: HashMap::new(),
            fields,
        }
    }

    pub fn add_method(&mut self, name: String, method: Value) {
        self.methods.insert(name, method);
    }
    
    pub fn add_static_method(&mut self, name: String, method: Value) {
        self.static_methods.insert(name, method);
    }

    pub fn get_method(&self, name: &str) -> Option<Value> {
        self.methods.get(name).cloned()
    }
    
    pub fn get_static_method(&self, name: &str) -> Option<Value> {
        self.static_methods.get(name).cloned()
    }
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Self {
        let mut fields = HashMap::new();
        
        // Inicializar campos com valores padrão
        for field in &class.fields {
            fields.insert(field.name.clone(), Value::Null);
        }
        
        Self { class, fields }
    }

    pub fn get_field(&self, name: &str) -> Option<Value> {
        self.fields.get(name).cloned()
    }

    pub fn set_field(&mut self, name: String, value: Value) {
        self.fields.insert(name, value);
    }

    pub fn get_method(&self, name: &str) -> Option<Value> {
        self.class.get_method(name)
    }
}
