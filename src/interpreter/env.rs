// src/interpreter/env.rs

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    variables: HashMap<String, Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl Env {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (key, value) in &self.variables {
            result.push_str(&format!("{}: {:?}\n", key, value));
        }
        result
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }
}
