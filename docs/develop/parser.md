# 🌳 Parser e AST

O Parser é responsável por converter a sequência de tokens do Lexer em uma Árvore Sintática Abstrata (AST), que representa a estrutura hierárquica do código.

## 🎯 Objetivo e Responsabilidades

O Parser é responsável por:
- **Análise sintática:** Verificar se os tokens seguem a gramática da linguagem
- **Construção da AST:** Criar estrutura hierárquica representando o código
- **Precedência de operadores:** Garantir ordem correta de avaliação
- **Detecção de erros sintáticos:** Identificar construções inválidas

## 🏗️ Arquitetura

### Estrutura de Arquivos
```
src/parser/
├── mod.rs     # Módulo principal
├── ast.rs     # Definição da AST
└── parser.rs  # Implementação do parser
```

### Fluxo de Funcionamento
```
Tokens → Análise Sintática → Construção da AST → Árvore Estruturada
```

## 🌲 Definição da AST

### Estruturas Principais (`src/parser/ast.rs`)

#### **Expressões (Expr)**
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    // Literais
    Number(f64),
    String(String),
    Identifier(String),
    
    // Operações binárias
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    
    // Chamadas de função
    Call {
        function: String,
        args: Vec<Expr>,
    },
}
```

#### **Operadores Binários (BinaryOp)**
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    // Aritméticos
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    
    // Comparação
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
}
```

#### **Declarações (Stmt)**
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    // Declaração de variável
    Let { 
        name: String, 
        value: Expr 
    },
    
    // Expressão como declaração
    Expr(Expr),
    
    // Bloco de código
    Block(Vec<Stmt>),
    
    // Estruturas de controle
    If { 
        cond: Expr, 
        then_branch: Box<Stmt>, 
        else_branch: Option<Box<Stmt>> 
    },
    
    While { 
        cond: Expr, 
        body: Box<Stmt> 
    },
    
    For { 
        init: Option<Box<Stmt>>, 
        cond: Option<Expr>, 
        post: Option<Expr>, 
        body: Box<Stmt> 
    },
}
```

## ⚙️ Implementação do Parser

### Estrutura Principal (`src/parser/parser.rs`)
```rust
pub struct Parser {
    lexer: Lexer,      // Fonte de tokens
    current: Token,    // Token atual
    peek: Token,       // Próximo token (lookahead)
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let peek = lexer.next_token();
        Self { lexer, current, peek }
    }
}
```

### Métodos de Navegação
```rust
impl Parser {
    fn advance(&mut self) {
        self.current = std::mem::replace(&mut self.peek, self.lexer.next_token());
    }
    
    fn check(&self, token: &Token) -> bool {
        std::mem::discriminant(&self.current) == std::mem::discriminant(token)
    }
    
    fn matches(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }
}
```

## 📊 Gramática e Precedência

### Hierarquia de Precedência (menor para maior)
```
1. Equality      (==, !=)
2. Comparison    (<, >, <=, >=)
3. Term          (+, -)
4. Factor        (*, /)
5. Primary       (números, strings, identificadores, parênteses)
```

### Implementação da Precedência

#### **Método Principal - Expressões**
```rust
pub fn parse_expression(&mut self) -> Option<Expr> {
    self.parse_equality()
}

fn parse_equality(&mut self) -> Option<Expr> {
    let mut expr = self.parse_comparison()?;
    
    while self.matches(&Token::EqualEqual) || self.matches(&Token::BangEqual) {
        let op = match self.current {
            Token::EqualEqual => BinaryOp::Equal,
            Token::BangEqual => BinaryOp::NotEqual,
            _ => unreachable!(),
        };
        self.advance();
        let right = self.parse_comparison()?;
        expr = Expr::Binary {
            left: Box::new(expr),
            op,
            right: Box::new(right),
        };
    }
    
    Some(expr)
}

fn parse_comparison(&mut self) -> Option<Expr> {
    let mut expr = self.parse_term()?;
    
    while matches!(self.current, Token::Greater | Token::GreaterEqual | 
                                 Token::Less | Token::LessEqual) {
        let op = match self.current {
            Token::Greater => BinaryOp::Greater,
            Token::GreaterEqual => BinaryOp::GreaterEqual,
            Token::Less => BinaryOp::Less,
            Token::LessEqual => BinaryOp::LessEqual,
            _ => unreachable!(),
        };
        self.advance();
        let right = self.parse_term()?;
        expr = Expr::Binary {
            left: Box::new(expr),
            op,
            right: Box::new(right),
        };
    }
    
    Some(expr)
}

fn parse_term(&mut self) -> Option<Expr> {
    let mut expr = self.parse_factor()?;
    
    while self.matches(&Token::Plus) || self.matches(&Token::Minus) {
        let op = match self.current {
            Token::Plus => BinaryOp::Add,
            Token::Minus => BinaryOp::Sub,
            _ => unreachable!(),
        };
        self.advance();
        let right = self.parse_factor()?;
        expr = Expr::Binary {
            left: Box::new(expr),
            op,
            right: Box::new(right),
        };
    }
    
    Some(expr)
}

fn parse_factor(&mut self) -> Option<Expr> {
    let mut expr = self.parse_primary()?;
    
    while self.matches(&Token::Star) || self.matches(&Token::Slash) {
        let op = match self.current {
            Token::Star => BinaryOp::Mul,
            Token::Slash => BinaryOp::Div,
            _ => unreachable!(),
        };
        self.advance();
        let right = self.parse_primary()?;
        expr = Expr::Binary {
            left: Box::new(expr),
            op,
            right: Box::new(right),
        };
    }
    
    Some(expr)
}

fn parse_primary(&mut self) -> Option<Expr> {
    match &self.current {
        Token::Number(n) => {
            let value = *n;
            self.advance();
            Some(Expr::Number(value))
        }
        
        Token::String(s) => {
            let value = s.clone();
            self.advance();
            Some(Expr::String(value))
        }
        
        Token::Identifier(name) => {
            let name = name.clone();
            self.advance();
            
            // Verificar se é chamada de função
            if self.matches(&Token::LParen) {
                let mut args = Vec::new();
                
                if !self.check(&Token::RParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.matches(&Token::Comma) {
                            break;
                        }
                    }
                }
                
                if !self.matches(&Token::RParen) {
                    return None; // Erro: ')' esperado
                }
                
                Some(Expr::Call { function: name, args })
            } else {
                Some(Expr::Identifier(name))
            }
        }
        
        Token::LParen => {
            self.advance();
            let expr = self.parse_expression()?;
            if !self.matches(&Token::RParen) {
                return None; // Erro: ')' esperado
            }
            Some(expr)
        }
        
        _ => None,
    }
}
```

## 📋 Parsing de Declarações

### Método Principal - Declarações
```rust
pub fn parse_statement(&mut self) -> Option<Stmt> {
    match &self.current {
        Token::Let => self.parse_let(),
        Token::If => self.parse_if(),
        Token::While => self.parse_while(),
        Token::For => self.parse_for(),
        Token::LBrace => self.parse_block(),
        _ => {
            // Tratar como expressão
            let expr = self.parse_expression()?;
            if self.matches(&Token::Semicolon) {
                // Opcional para última expressão
            }
            Some(Stmt::Expr(expr))
        }
    }
}

fn parse_let(&mut self) -> Option<Stmt> {
    self.advance(); // consumir 'let'
    
    if let Token::Identifier(name) = &self.current {
        let name = name.clone();
        self.advance();
        
        if self.matches(&Token::Equal) {
            let value = self.parse_expression()?;
            
            if self.matches(&Token::Semicolon) {
                // Semicolon opcional
            }
            
            Some(Stmt::Let { name, value })
        } else {
            None // Erro: '=' esperado
        }
    } else {
        None // Erro: identificador esperado
    }
}

fn parse_if(&mut self) -> Option<Stmt> {
    self.advance(); // consumir 'if'
    
    if !self.matches(&Token::LParen) {
        return None; // Erro: '(' esperado
    }
    
    let cond = self.parse_expression()?;
    
    if !self.matches(&Token::RParen) {
        return None; // Erro: ')' esperado
    }
    
    let then_branch = Box::new(self.parse_statement()?);
    
    let else_branch = if self.matches(&Token::Else) {
        Some(Box::new(self.parse_statement()?))
    } else {
        None
    };
    
    Some(Stmt::If { cond, then_branch, else_branch })
}

fn parse_block(&mut self) -> Option<Stmt> {
    self.advance(); // consumir '{'
    
    let mut statements = Vec::new();
    
    while !self.check(&Token::RBrace) && !self.check(&Token::Eof) {
        if let Some(stmt) = self.parse_statement() {
            statements.push(stmt);
        } else {
            return None; // Erro de parsing
        }
    }
    
    if !self.matches(&Token::RBrace) {
        return None; // Erro: '}' esperado
    }
    
    Some(Stmt::Block(statements))
}
```

## 📊 Exemplo de Parsing

### Entrada (Tokens):
```rust
[
    Token::Let, Token::Identifier("x"), Token::Equal, 
    Token::Number(1.0), Token::Plus, Token::Number(2.0), 
    Token::Star, Token::Number(3.0), Token::Semicolon
]
```

### Processo de Parsing:
1. **`parse_statement()`** vê `Token::Let` → chama `parse_let()`
2. **`parse_let()`**:
   - Consome `let`
   - Lê identificador `x`
   - Consome `=`
   - Chama `parse_expression()`
3. **`parse_expression()`** → `parse_equality()` → `parse_comparison()` → `parse_term()`
4. **`parse_term()`**:
   - Chama `parse_factor()` para `1`
   - Encontra `+`, então:
     - `left` = `Expr::Number(1.0)`
     - `op` = `BinaryOp::Add`
     - `right` = resultado de `parse_factor()`
5. **`parse_factor()`** (para parte direita):
   - Chama `parse_primary()` para `2`
   - Encontra `*`, então:
     - `left` = `Expr::Number(2.0)`
     - `op` = `BinaryOp::Mul`
     - `right` = `Expr::Number(3.0)`

### AST Resultante:
```rust
Stmt::Let {
    name: "x".to_string(),
    value: Expr::Binary {
        left: Box::new(Expr::Number(1.0)),
        op: BinaryOp::Add,
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::Number(2.0)),
            op: BinaryOp::Mul,
            right: Box::new(Expr::Number(3.0)),
        }),
    },
}
```

**Nota:** A precedência garante que `2 * 3` seja agrupado antes de `1 + (resultado)`.

## 🔍 Tratamento de Precedência

### Exemplo: `1 + 2 * 3`

**Tokens:** `[Number(1), Plus, Number(2), Star, Number(3)]`

**Fluxo de parsing:**
1. `parse_term()` processa `1 +`
2. Chama `parse_factor()` para lado direito
3. `parse_factor()` encontra `2 *` e processa com maior precedência
4. Resultado: `1 + (2 * 3)` em vez de `(1 + 2) * 3`

### Árvore AST Visual:
```
       Binary(+)
      /         \
  Number(1)   Binary(*)
             /         \
        Number(2)   Number(3)
```

## 🚀 Estendendo o Parser

### **Adicionando Novos Operadores**

1. **Definir na AST:**
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    // ... operadores existentes ...
    Mod,  // %
}
```

2. **Adicionar na precedência apropriada:**
```rust
fn parse_factor(&mut self) -> Option<Expr> {
    // ... código existente ...
    while matches!(self.current, Token::Star | Token::Slash | Token::Percent) {
        let op = match self.current {
            Token::Star => BinaryOp::Mul,
            Token::Slash => BinaryOp::Div,
            Token::Percent => BinaryOp::Mod,  // <-- Novo
            _ => unreachable!(),
        };
        // ... resto do código ...
    }
}
```

### **Adicionando Novas Construções**

1. **Definir na AST:**
```rust
pub enum Stmt {
    // ... declarações existentes ...
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
    },
}
```

2. **Implementar parsing:**
```rust
fn parse_match(&mut self) -> Option<Stmt> {
    self.advance(); // consumir 'match'
    
    let expr = self.parse_expression()?;
    
    if !self.matches(&Token::LBrace) {
        return None;
    }
    
    // ... implementar parsing dos arms ...
    
    Some(Stmt::Match { expr, arms })
}
```

## ✅ Estado Atual vs Futuras Melhorias

### **Implementado:**
- ✅ Expressões aritméticas e de comparação
- ✅ Declarações `let`
- ✅ Estruturas condicionais `if/else`
- ✅ Loops `while` e `for`
- ✅ Blocos de código
- ✅ Chamadas de função básicas
- ✅ Precedência correta de operadores

### **Planejado:**
- 🔄 Declarações de função (`fun`)
- 🔄 Expressões de atribuição (`x = value`)
- 🔄 Operadores unários (`-x`, `!x`)
- 🔄 Arrays e indexação
- 🔄 Expressões condicionais ternárias
- 🔄 Pattern matching
- 🔄 Melhor tratamento de erros sintáticos

O Parser atual fornece uma base sólida para a linguagem Dryad, implementando corretamente a precedência de operadores e suportando as principais construções sintáticas necessárias.
