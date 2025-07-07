# 🔤 Sistema de Lexer

O Lexer (analisador léxico) é o primeiro componente da pipeline do Dryad. Ele converte código fonte em uma sequência de tokens que podem ser processados pelo parser.

## 🎯 Objetivo e Responsabilidades

O Lexer é responsável por:
- **Tokenização:** Converter texto em tokens estruturados
- **Reconhecimento de padrões:** Identificar palavras-chave, operadores, literais
- **Filtragem:** Ignorar espaços em branco e comentários (quando implementados)
- **Detecção de erros léxicos:** Caracteres inválidos, strings não fechadas, etc.

## 🏗️ Arquitetura

### Estrutura de Arquivos
```
src/lexer/
├── mod.rs          # Módulo principal e re-exports
├── token.rs        # Definição de todos os tokens
└── tokenizer.rs    # Implementação do lexer
```

### Fluxo de Funcionamento
```
Código Fonte → Análise Caractere por Caractere → Token → Token → ... → EOF
```

## 📋 Tipos de Tokens

### Definição Completa (`src/lexer/token.rs`)
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Palavras-chave
    Let, Fun, If, Else, For, While, Return, Export, Use,
    
    // Literais
    Identifier(String),  // nomes de variáveis/funções
    Number(f64),         // números (42, 3.14)
    String(String),      // strings ("hello")
    
    // Operadores aritméticos
    Plus, Minus, Star, Slash,  // + - * /
    
    // Operadores de comparação
    Equal, EqualEqual,         // = ==
    Bang, BangEqual,           // ! !=
    Less, LessEqual,           // < <=
    Greater, GreaterEqual,     // > >=
    
    // Símbolos estruturais
    LParen, RParen,      // ( )
    LBrace, RBrace,      // { }
    Comma, Semicolon,    // , ;
    
    // Marcador de fim
    Eof,
}
```

### Categorias de Tokens

#### **1. Palavras-chave**
```rust
"let"    → Token::Let
"fun"    → Token::Fun
"if"     → Token::If
"else"   → Token::Else
"for"    → Token::For
"while"  → Token::While
"return" → Token::Return
"export" → Token::Export
"use"    → Token::Use
```

#### **2. Identificadores**
```rust
"variable"   → Token::Identifier("variable".to_string())
"myFunction" → Token::Identifier("myFunction".to_string())
"_private"   → Token::Identifier("_private".to_string())
"snake_case" → Token::Identifier("snake_case".to_string())
```

**Regras:**
- Deve começar com letra ou underscore
- Pode conter letras, números e underscores
- Case-sensitive

#### **3. Números**
```rust
"42"     → Token::Number(42.0)
"3.14"   → Token::Number(3.14)
"0.5"    → Token::Number(0.5)
"999"    → Token::Number(999.0)
```

**Formatos suportados:**
- Inteiros: `42`, `0`, `999`
- Decimais: `3.14`, `0.5`, `123.456`

#### **4. Strings**
```rust
"\"hello\""        → Token::String("hello".to_string())
"\"world test\""   → Token::String("world test".to_string())
"\"\""             → Token::String("".to_string())
```

## 🔧 Implementação do Lexer

### Estrutura Principal (`src/lexer/tokenizer.rs`)
```rust
pub struct Lexer {
    input: Vec<char>,    // Código fonte como array de caracteres
    current: usize,      // Posição atual no array
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            current: 0,
        }
    }
    
    pub fn next_token(&mut self) -> Token {
        // Método principal que retorna o próximo token
    }
}
```

### Métodos Auxiliares

#### **Navegação no Código**
```rust
fn is_at_end(&self) -> bool {
    self.current >= self.input.len()
}

fn advance(&mut self) -> char {
    let ch = self.input[self.current];
    self.current += 1;
    ch
}

fn peek(&self) -> Option<char> {
    self.input.get(self.current).copied()
}

fn match_char(&mut self, expected: char) -> bool {
    if self.peek() == Some(expected) {
        self.advance();
        true
    } else {
        false
    }
}
```

#### **Análise de Conteúdo**
```rust
fn skip_whitespace(&mut self) {
    while let Some(&ch) = self.input.get(self.current) {
        if ch.is_whitespace() {
            self.advance();
        } else {
            break;
        }
    }
}

fn read_number(&mut self, first: char) -> Token {
    let mut number = first.to_string();
    
    // Ler parte inteira
    while let Some(&ch) = self.input.get(self.current) {
        if ch.is_ascii_digit() {
            number.push(self.advance());
        } else {
            break;
        }
    }
    
    // Verificar se há parte decimal
    if self.peek() == Some('.') && 
       self.input.get(self.current + 1).map_or(false, |c| c.is_ascii_digit()) {
        number.push(self.advance()); // '.'
        
        while let Some(&ch) = self.input.get(self.current) {
            if ch.is_ascii_digit() {
                number.push(self.advance());
            } else {
                break;
            }
        }
    }
    
    Token::Number(number.parse().unwrap())
}

fn read_string(&mut self) -> Token {
    let mut string = String::new();
    
    while let Some(&ch) = self.input.get(self.current) {
        if ch == '"' {
            self.advance(); // Fechar aspas
            break;
        }
        string.push(self.advance());
    }
    
    Token::String(string)
}

fn read_identifier(&mut self, first: char) -> Token {
    let mut ident = first.to_string();
    
    while let Some(&ch) = self.input.get(self.current) {
        if ch.is_alphanumeric() || ch == '_' {
            ident.push(self.advance());
        } else {
            break;
        }
    }
    
    // Verificar se é palavra-chave
    match ident.as_str() {
        "let" => Token::Let,
        "fun" => Token::Fun,
        "if" => Token::If,
        "else" => Token::Else,
        "for" => Token::For,
        "while" => Token::While,
        "return" => Token::Return,
        "export" => Token::Export,
        "use" => Token::Use,
        _ => Token::Identifier(ident),
    }
}
```

### Método Principal - `next_token()`
```rust
pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    
    if self.is_at_end() {
        return Token::Eof;
    }
    
    let c = self.advance();
    
    match c {
        '+' => Token::Plus,
        '-' => Token::Minus,
        '*' => Token::Star,
        '/' => Token::Slash,
        '(' => Token::LParen,
        ')' => Token::RParen,
        '{' => Token::LBrace,
        '}' => Token::RBrace,
        ',' => Token::Comma,
        ';' => Token::Semicolon,
        
        '=' => {
            if self.match_char('=') {
                Token::EqualEqual
            } else {
                Token::Equal
            }
        }
        
        '!' => {
            if self.match_char('=') {
                Token::BangEqual
            } else {
                Token::Bang
            }
        }
        
        '<' => {
            if self.match_char('=') {
                Token::LessEqual
            } else {
                Token::Less
            }
        }
        
        '>' => {
            if self.match_char('=') {
                Token::GreaterEqual
            } else {
                Token::Greater
            }
        }
        
        '"' => self.read_string(),
        
        c if c.is_ascii_digit() => self.read_number(c),
        
        c if Self::is_identifier_start(c) => self.read_identifier(c),
        
        _ => {
            // Caractere não reconhecido - poderia gerar erro
            Token::Eof
        }
    }
}
```

## 📊 Exemplo de Tokenização

### Entrada:
```javascript
let x = 42 + 8;
if (x > 10) {
    x = x * 2;
}
```

### Processo passo a passo:

1. **`let`** → `Token::Let`
2. **` `** → (ignorado - whitespace)
3. **`x`** → `Token::Identifier("x".to_string())`
4. **` `** → (ignorado)
5. **`=`** → `Token::Equal`
6. **` `** → (ignorado)
7. **`42`** → `Token::Number(42.0)` (chama `read_number`)
8. **` `** → (ignorado)
9. **`+`** → `Token::Plus`
10. **` `** → (ignorado)
11. **`8`** → `Token::Number(8.0)`
12. **`;`** → `Token::Semicolon`
... e assim por diante

### Saída Final:
```rust
[
    Token::Let,
    Token::Identifier("x".to_string()),
    Token::Equal,
    Token::Number(42.0),
    Token::Plus,
    Token::Number(8.0),
    Token::Semicolon,
    Token::If,
    Token::LParen,
    Token::Identifier("x".to_string()),
    Token::Greater,
    Token::Number(10.0),
    Token::RParen,
    Token::LBrace,
    Token::Identifier("x".to_string()),
    Token::Equal,
    Token::Identifier("x".to_string()),
    Token::Star,
    Token::Number(2.0),
    Token::Semicolon,
    Token::RBrace,
    Token::Eof,
]
```

## 🔍 Tratamento de Casos Especiais

### **Operadores Compostos**
```rust
'=' seguido de '=' → Token::EqualEqual
'!' seguido de '=' → Token::BangEqual
'<' seguido de '=' → Token::LessEqual
'>' seguido de '=' → Token::GreaterEqual
```

### **Números Decimais**
```rust
"3.14" → Lê '3', verifica '.', lê '14' → Token::Number(3.14)
"42"   → Lê '42', não encontra '.', → Token::Number(42.0)
```

### **Identificadores vs Palavras-chave**
```rust
"let"    → Reconhece como palavra-chave → Token::Let
"letter" → Reconhece como identificador → Token::Identifier("letter")
```

## 🚀 Estendendo o Lexer

### **Adicionando Novos Operadores**

1. **Definir o token:**
```rust
// Em src/lexer/token.rs
pub enum Token {
    // ... tokens existentes ...
    Percent,  // %
}
```

2. **Implementar reconhecimento:**
```rust
// Em src/lexer/tokenizer.rs, no método next_token()
match c {
    // ... casos existentes ...
    '%' => Token::Percent,
}
```

### **Adicionando Novas Palavras-chave**

1. **Definir o token:**
```rust
pub enum Token {
    // ... tokens existentes ...
    Match,  // match
}
```

2. **Atualizar reconhecimento:**
```rust
// Em read_identifier()
match ident.as_str() {
    // ... casos existentes ...
    "match" => Token::Match,
    _ => Token::Identifier(ident),
}
```

### **Suporte a Comentários (futuro)**
```rust
'/' seguido de '/' → Ignora até fim da linha
'/' seguido de '*' → Ignora até encontrar '*/'
```

## ✅ Estado Atual vs Futuras Melhorias

### **Implementado:**
- ✅ Tokens básicos (palavras-chave, operadores, literais)
- ✅ Números inteiros e decimais
- ✅ Strings básicas
- ✅ Identificadores
- ✅ Operadores compostos

### **Planejado:**
- 🔄 Comentários (`//` e `/* */`)
- 🔄 Escape sequences em strings (`\n`, `\"`, etc.)
- 🔄 Números em outras bases (hexadecimal, binário)
- 🔄 Melhor tratamento de erros léxicos
- 🔄 Informações de localização (linha, coluna)

O Lexer atual é robusto e suficiente para as funcionalidades implementadas do Dryad, fornecendo uma base sólida para extensões futuras.
