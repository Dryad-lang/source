# 📋 Changelog - Histórico de Mudanças

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Versionamento Semântico](https://semver.org/lang/pt-BR/).

## [Não Lançado]

### Planejado
- Funções definidas pelo usuário
- Reatribuição de variáveis
- Arrays e listas
- Sistema de módulos

## [0.1.1] - 2025-01-07

### ✅ Adicionado
- **Sistema de tipos robusto** com verificação opcional (`--strict`)
- **Sistema de erros idiomático** compatível com `std::error::Error`
- **REPL completo** com comandos especiais
- **I/O de arquivos** (`read_file`, `write_file`, `append_file`)
- **Loops `for`** básicos
- **Blocos de código** com `{ ... }`
- **Documentação completa** reorganizada em 3 categorias
- **40 testes** cobrindo todos os módulos

### 🔧 Melhorado
- **Tratamento de EOF no REPL** - agora sai graciosamente
- **Precedência de operadores** corrigida no parser
- **Mensagens de erro** mais claras e informativas
- **Performance geral** do interpretador
- **Cobertura de testes** aumentada para 85%

### 🐛 Corrigido
- **Loop infinito no REPL** com entrada por pipe
- **Problemas de parsing** com expressões complexas
- **Warnings** desnecessários removidos
- **Tipo de erros** inconsistentes padronizados
- **Memory leaks** menores no interpretador

### 🔄 Alterado
- **API de erros** refatorada para ser idiomática
- **Estrutura de documentação** reorganizada
- **Sistema de testes** melhorado e expandido

## [0.1.0] - 2025-01-01

### ✅ Adicionado - MVP Inicial
- **Lexer completo** com tokenização de todos os elementos básicos
- **Parser funcional** com precedência correta de operadores
- **Interpretador básico** para execução de código
- **CLI funcional** com flags essenciais
- **REPL básico** para modo interativo
- **Sistema de variáveis** com `let`
- **Operadores aritméticos** (`+`, `-`, `*`, `/`)
- **Operadores de comparação** (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- **Estruturas condicionais** (`if/else`)
- **Loops `while`**
- **Função `print()`** para saída
- **Tipos básicos** (Number, String, Bool, Null)

### 🏗️ Arquitetura Implementada
- **Pipeline completa** Lexer → Parser → Interpreter
- **AST (Árvore Sintática Abstrata)**
- **Ambiente de execução** para variáveis
- **Sistema básico de erros**
- **Testes unitários** para componentes principais

---

## 📊 Estatísticas por Versão

### v0.1.1
- **Linhas de código:** ~3.000 (Rust)
- **Testes:** 40 (100% passando)
- **Módulos:** 8 principais
- **Funcionalidades:** 15+ funcionalidades
- **Documentação:** 15+ páginas

### v0.1.0
- **Linhas de código:** ~2.000 (Rust)
- **Testes:** 15 (100% passando)
- **Módulos:** 6 principais
- **Funcionalidades:** 10+ funcionalidades
- **Documentação:** 5 páginas básicas

## 🔍 Detalhes Técnicos por Versão

### v0.1.1 - Mudanças Técnicas

#### **Sistema de Erros**
```rust
// Antes (v0.1.0)
pub struct DryadError {
    message: String,
    // Campos básicos
}

// Depois (v0.1.1)
impl std::error::Error for DryadError {}
impl Display for DryadError {}
// Totalmente idiomático
```

#### **REPL**
```rust
// Antes - problema com EOF
match io::stdin().read_line(&mut input) {
    Ok(_) => { /* processava sempre */ }
}

// Depois - trata EOF corretamente
match io::stdin().read_line(&mut input) {
    Ok(0) => break, // EOF
    Ok(_) => { /* processa normalmente */ }
}
```

#### **Sistema de Tipos**
```rust
// Adicionado em v0.1.1
pub struct TypeChecker {
    strict_mode: bool,
}

impl TypeChecker {
    pub fn check_expression(&self, expr: &Expr, env: &Env) -> Option<TypeError> {
        // Verificação rigorosa de tipos
    }
}
```

### v0.1.0 - Implementação Inicial

#### **Lexer**
```rust
pub enum Token {
    // Implementados em v0.1.0
    Let, Fun, If, Else, For, While,
    Identifier(String), Number(f64), String(String),
    Plus, Minus, Star, Slash,
    // ... outros tokens básicos
}
```

#### **Parser**
```rust
pub enum Expr {
    Number(f64),
    String(String),
    Identifier(String),
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
}

pub enum Stmt {
    Let { name: String, value: Expr },
    Expr(Expr),
    // ... outras declarações
}
```

## 🐛 Bugs Conhecidos

### Resolvidos em v0.1.1
- ✅ REPL loop infinito com pipe input
- ✅ Parser precedência incorreta
- ✅ Memory leaks menores
- ✅ Error handling inconsistente

### Ainda Abertos
- 🔄 Reatribuição de variáveis não implementada
- 🔄 Escopo local não implementado
- 🔄 Funções definidas pelo usuário pendentes
- 🔄 Arrays não implementados

## 🎯 Próximas Mudanças (v0.2.0)

### Planejado
- **Funções definidas pelo usuário**
  ```javascript
  fun somar(a, b) {
      return a + b;
  }
  ```

- **Reatribuição de variáveis**
  ```javascript
  let x = 10;
  x = 20;  // Será possível
  ```

- **Escopo local**
  ```javascript
  fun teste() {
      let local = 42;  // Variável local
  }
  ```

- **Return statements**
  ```javascript
  fun exemplo() {
      return "valor";
  }
  ```

## 📈 Métricas de Qualidade

### Testes
| Versão | Testes | Cobertura | Status |
|--------|--------|-----------|--------|
| v0.1.0 | 15 | ~60% | ✅ Todos passando |
| v0.1.1 | 40 | ~85% | ✅ Todos passando |

### Performance
| Versão | Compile Time | Startup | Memory |
|--------|-------------|---------|--------|
| v0.1.0 | ~1.5s | ~80ms | ~3MB |
| v0.1.1 | ~2s | ~50ms | ~5MB |

### Código
| Versão | LOC | Módulos | Warnings |
|--------|-----|---------|----------|
| v0.1.0 | ~2k | 6 | 5+ |
| v0.1.1 | ~3k | 8 | 0 |

## 🔗 Links Relacionados

- [Roadmap](./roadmap.md) - Planos futuros
- [Status Atual](./current-status.md) - Estado detalhado
- [Contribuindo](./contributing.md) - Como ajudar

---

**Formato do versionamento:**
- **Maior (Major):** Mudanças incompatíveis na API
- **Menor (Minor):** Funcionalidades adicionadas (compatível)
- **Patch:** Correções de bugs (compatível)

**Última atualização:** 7 de Janeiro de 2025
