# 🔄 Pipeline e Arquitetura do Dryad

Esta documentação explica como funciona o fluxo completo de execução do Dryad, desde o código fonte até o resultado final.

## 🏗️ Visão Geral da Pipeline

O Dryad segue uma arquitetura de pipeline clássica para interpretadores, onde cada etapa transforma a representação do código:

```
┌─────────────────┐
│   Código Fonte  │  "let x = 42 + 8;"
│     (.dryad)    │
└─────────┬───────┘
          │
          ▼
┌─────────────────┐
│      LEXER      │  [Let, Identifier("x"), Equal, Number(42), Plus, Number(8), Semicolon]
│   (Tokenização) │
└─────────┬───────┘
          │
          ▼
┌─────────────────┐
│     PARSER      │  Stmt::Let { name: "x", value: Expr::Binary { ... } }
│   (AST Build)   │
└─────────┬───────┘
          │
          ▼
┌─────────────────┐
│  TYPE CHECKER   │  Verificação opcional de tipos
│   (Opcional)    │
└─────────┬───────┘
          │
          ▼
┌─────────────────┐
│  INTERPRETADOR  │  Execução e avaliação
│   (Execution)   │
└─────────┬───────┘
          │
          ▼
┌─────────────────┐
│   RESULTADO     │  Value::Number(50.0)
│     Final       │
└─────────────────┘
```

## 📊 Fluxo Detalhado

### 1. **Entrada** - Código Fonte
```rust
// Exemplo de código Dryad
let x = 42;
let y = x + 8;
y
```

### 2. **Lexer** - Tokenização
O lexer converte o texto em tokens:

```rust
// src/lexer/tokenizer.rs
pub fn next_token(&mut self) -> Token {
    // Analisa caractere por caractere
    // Produz tokens individuais
}
```

**Tokens Gerados:**
```rust
[
    Token::Let,
    Token::Identifier("x".to_string()),
    Token::Equal,
    Token::Number(42.0),
    Token::Semicolon,
    Token::Let,
    Token::Identifier("y".to_string()),
    Token::Equal,
    Token::Identifier("x".to_string()),
    Token::Plus,
    Token::Number(8.0),
    Token::Semicolon,
    Token::Identifier("y".to_string()),
    Token::Eof,
]
```

### 3. **Parser** - Construção da AST
O parser constrói uma árvore sintática abstrata:

```rust
// src/parser/parser.rs
pub fn parse_statement(&mut self) -> Option<Stmt> {
    // Analisa tokens sequencialmente
    // Constrói estruturas da AST
}
```

**AST Resultante:**
```rust
[
    Stmt::Let {
        name: "x".to_string(),
        value: Expr::Number(42.0)
    },
    Stmt::Let {
        name: "y".to_string(),
        value: Expr::Binary {
            left: Box::new(Expr::Identifier("x".to_string())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(8.0))
        }
    },
    Stmt::Expr(Expr::Identifier("y".to_string()))
]
```

### 4. **Type Checker** (Opcional)
Verifica tipos se `--strict` estiver habilitado:

```rust
// src/interpreter/types.rs
pub fn check_expression(&self, expr: &Expr, env: &Env) -> Option<TypeError> {
    // Verifica compatibilidade de tipos
    // Reporta erros se encontrar inconsistências
}
```

### 5. **Interpretador** - Execução
O interpretador executa a AST:

```rust
// src/interpreter/evaluator.rs
pub fn eval_stmt(&mut self, stmt: &Stmt, env: &mut Env) -> EvaluationResult {
    match stmt {
        Stmt::Let { name, value } => {
            let val = self.eval_expr(value, env)?;
            env.set(name.clone(), val.clone());
            // ...
        }
        // ...
    }
}
```

**Execução Passo a Passo:**
1. `let x = 42;` → Environment: `{x: Number(42.0)}`
2. `let y = x + 8;` → Environment: `{x: Number(42.0), y: Number(50.0)}`
3. `y` → Resultado: `Number(50.0)`

## 🔧 Componentes Principais

### **Lexer** (`src/lexer/`)
- **Responsabilidade:** Converter texto em tokens
- **Input:** String com código fonte
- **Output:** Sequência de tokens
- **Arquivo principal:** `tokenizer.rs`

### **Parser** (`src/parser/`)
- **Responsabilidade:** Construir AST a partir de tokens
- **Input:** Sequência de tokens
- **Output:** Árvore sintática abstrata
- **Arquivo principal:** `parser.rs`

### **Type Checker** (`src/interpreter/types.rs`)
- **Responsabilidade:** Verificar compatibilidade de tipos
- **Input:** AST + Environment
- **Output:** Erros de tipo ou validação
- **Modo:** Opcional (ativado com `--strict`)

### **Interpretador** (`src/interpreter/evaluator.rs`)
- **Responsabilidade:** Executar código e calcular resultados
- **Input:** AST + Environment
- **Output:** Valores computados
- **Estado:** Mantém ambiente de execução

### **Environment** (`src/interpreter/env.rs`)
- **Responsabilidade:** Armazenar variáveis e valores
- **Estrutura:** HashMap de identificadores para valores
- **Escopo:** Atualmente global (escopo local planejado)

## 🔄 Interfaces Entre Componentes

### **Lexer → Parser**
```rust
// Interface via Token enum
pub enum Token {
    Let, Fun, If, Else,
    Identifier(String),
    Number(f64),
    // ...
}
```

### **Parser → Type Checker**
```rust
// Interface via AST structs
pub enum Expr {
    Number(f64),
    Identifier(String),
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    // ...
}
```

### **Type Checker → Interpretador**
```rust
// Interface via resultado de verificação
pub enum TypeError {
    TypeMismatch(Type, Type),
    UndefinedVariable(String),
    // ...
}
```

### **Interpretador → Environment**
```rust
// Interface via métodos do ambiente
impl Env {
    pub fn get(&self, name: &str) -> Option<Value>;
    pub fn set(&mut self, name: String, value: Value);
}
```

## 🎯 Pontos de Extensão

### **1. Novos Tokens**
- Adicionar em `src/lexer/token.rs`
- Implementar reconhecimento em `tokenizer.rs`

### **2. Nova Sintaxe**
- Definir na AST (`src/parser/ast.rs`)
- Implementar parsing (`src/parser/parser.rs`)

### **3. Novos Tipos**
- Adicionar em `src/interpreter/types.rs`
- Implementar regras de verificação

### **4. Novas Operações**
- Implementar em `src/interpreter/evaluator.rs`
- Adicionar casos para novos construtos

## 🚀 Exemplo Completo de Fluxo

### Entrada:
```javascript
let factorial = 5;
let result = factorial * 4 * 3 * 2 * 1;
result
```

### Saída do Lexer:
```
[Let, Identifier("factorial"), Equal, Number(5), Semicolon, ...]
```

### Saída do Parser:
```rust
[
    Stmt::Let { name: "factorial", value: Expr::Number(5.0) },
    Stmt::Let { name: "result", value: Expr::Binary { ... } },
    Stmt::Expr(Expr::Identifier("result"))
]
```

### Execução:
```
Environment após stmt 1: {factorial: Number(5.0)}
Environment após stmt 2: {factorial: Number(5.0), result: Number(120.0)}
Resultado final: Number(120.0)
```

Esta pipeline modular permite fácil manutenção e extensão de cada componente independentemente.
