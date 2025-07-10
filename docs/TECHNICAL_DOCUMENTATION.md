# 🔧 Documentação Técnica - Arquitetura Dryad

**Target:** Desenvolvedores e Contribuidores  
**Versão:** 0.1.0  
**Data:** 8 de janeiro de 2025  
**Status:** Sistema Oak Modular Integrado + Refatoração Completa

---

## 📋 Índice

1. [Arquitetura Geral](#arquitetura-geral)
2. [Sistema de Compilação](#sistema-de-compilação)
3. [Pipeline de Execução](#pipeline-de-execução)
4. [Módulos do Interpretador](#módulos-do-interpretador)
5. [Sistema Oak Modular](#sistema-oak-modular)
6. [Common Libraries](#common-libraries)
7. [APIs e FFI](#apis-e-ffi)
8. [Testes e Qualidade](#testes-e-qualidade)
9. [Contribuição](#contribuição)

---

## 🏗️ Arquitetura Geral

### Estrutura do Projeto

```
dryad/
├── src/
│   ├── main.rs                  # Ponto de entrada
│   ├── lib.rs                   # Biblioteca principal
│   ├── lexer/                   # Análise léxica
│   │   ├── mod.rs
│   │   ├── token.rs             # Definições de tokens
│   │   ├── tokenizer.rs         # Tokenizador principal
│   │   └── lexer.rs             # Interface do lexer
│   ├── parser/                  # Análise sintática
│   │   ├── mod.rs
│   │   ├── ast.rs               # Árvore de sintaxe abstrata
│   │   ├── parser.rs            # Parser principal
│   │   └── parser_simple.rs     # Parser simplificado
│   ├── interpreter/             # Interpretação e execução
│   │   ├── mod.rs
│   │   ├── env.rs               # Ambiente de execução
│   │   ├── evaluator.rs         # Avaliador de expressões
│   │   ├── types.rs             # Sistema de tipos
│   │   ├── errors.rs            # Sistema de erros
│   │   ├── io.rs                # Operações I/O
│   │   └── module_loader.rs     # Carregador de módulos
│   ├── oak/                     # Package Manager Oak
│   │   ├── mod.rs               # Ponto de entrada
│   │   ├── config.rs            # Configuração
│   │   ├── manager.rs           # Gerenciador principal
│   │   ├── registry.rs          # Sistema de registry
│   │   ├── resolver.rs          # Resolução de dependências
│   │   ├── api.rs               # APIs externas
│   │   └── cli_integration.rs   # Integração CLI
│   └── cli/                     # Interface de linha de comando
│       ├── mod.rs
│       ├── cli.rs               # CLI principal
│       └── repl.rs              # REPL interativo
├── lib/                         # Common Libraries
├── examples/                    # Exemplos
├── tests/                       # Testes
└── docs/                        # Documentação
```

### Fluxo de Dados

```
┌─────────────┐    ┌──────────┐    ┌─────────┐    ┌──────────────┐
│ Código      │ -> │ Lexer    │ -> │ Parser  │ -> │ Interpreter  │
│ Fonte       │    │ (Tokens) │    │ (AST)   │    │ (Execução)   │
└─────────────┘    └──────────┘    └─────────┘    └──────────────┘
       │                                                   │
       v                                                   v
┌─────────────┐                                   ┌──────────────┐
│ Oak Config  │                                   │ Resultado /  │
│ (oaklibs.   │                                   │ Saída        │
│  json)      │                                   │              │
└─────────────┘                                   └──────────────┘
```

---

## ⚙️ Sistema de Compilação

### Dependências (Cargo.toml)

```toml
[dependencies]
serde_json = "1.0"          # Serialização JSON
serde = { version = "1.0", features = ["derive"] }  # Serialização
semver = "1.0"              # Versionamento semântico

[dev-dependencies]
# Testes e desenvolvimento
```

### Targets de Compilação

```bash
# Debug (desenvolvimento)
cargo build

# Release (produção)
cargo build --release

# Testes
cargo test

# Documentação
cargo doc --open

# Exemplo específico
cargo run --example oak_api_usage
```

---

## 🔄 Pipeline de Execução

### 1. Análise Léxica (Lexer)

```rust
// src/lexer/tokenizer.rs
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

// Tokens suportados
pub enum Token {
    // Literais
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    
    // Identificadores
    Identifier(String),
    
    // Operadores
    Plus, Minus, Multiply, Divide,
    Equal, NotEqual, LessThan, GreaterThan,
    And, Or, Not,
    
    // Palavras-chave
    Let, Function, Class, If, Else,
    Using, Use, Export, Static, Public,
    
    // Delimitadores
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Semicolon, Comma,
    
    // Especiais
    Assign, Eof,
}
```

### 2. Análise Sintática (Parser)

```rust
// src/parser/ast.rs
pub enum Stmt {
    Let { name: String, value: Expr },
    Expression(Expr),
    FunDecl(FunDecl),
    ClassDecl(ClassDecl),
    If { condition: Expr, then_stmt: Box<Stmt>, else_stmt: Option<Box<Stmt>> },
    Using { path: String, alias: Option<String> },
    Use { path: String },
    Export(Box<Stmt>),
}

pub enum Expr {
    Literal(Value),
    Identifier(String),
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
    Unary { op: UnaryOp, operand: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Get { object: Box<Expr>, name: String },
}
```

### 3. Sistema de Tipos

```rust
// src/interpreter/types.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Function { 
        name: String, 
        params: Vec<String>, 
        body: Vec<Stmt>,
        is_static: bool,
    },
    Class(Class),
    Instance(Instance),
    NativeFunction(String),
}

pub struct Class {
    name: String,
    methods: HashMap<String, Value>,
    static_methods: HashMap<String, Value>,
    superclass: Option<Box<Class>>,
}
```

### 4. Avaliação (Interpreter)

```rust
// src/interpreter/evaluator.rs
pub struct Evaluator {
    native_registry: NativeRegistry,
    type_checker: TypeChecker,
}

impl Evaluator {
    pub fn eval_stmt(&mut self, stmt: &Stmt, env: &mut Env) -> EvaluationResult {
        match stmt {
            Stmt::Let { name, value } => self.eval_let(name, value, env),
            Stmt::Expression(expr) => self.eval_expr(expr, env),
            Stmt::FunDecl(fun) => self.eval_fun_decl(fun, env),
            Stmt::ClassDecl(class) => self.eval_class_decl(class, env),
            // ... outros statements
        }
    }
}
```

---

## 🌳 Sistema Oak Modular

### Arquitetura Oak

```
oak/
├── mod.rs              # Entry point, re-exports
├── config.rs           # OakConfig, OakDependency, OakPackage
├── manager.rs          # OakManager (core logic)
├── registry.rs         # PackageRegistry trait, implementations
├── resolver.rs         # Dependency resolution
├── api.rs              # External APIs (C/FFI, Node.js)
└── cli_integration.rs  # CLI integration layer
```

### Componentes Principais

#### 1. Configuração (config.rs)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OakConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: HashMap<String, OakDependency>,
    pub dev_dependencies: HashMap<String, OakDependency>,
    pub lib_paths: Vec<String>,
    pub scripts: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OakDependency {
    pub version: String,
    pub source: DependencySource,
    pub optional: bool,
    pub dev: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencySource {
    Registry(String),
    Git(String),
    Local(PathBuf),
    Url(String),
}
```

#### 2. Gerenciador (manager.rs)

```rust
pub struct OakManager {
    registry: Box<dyn PackageRegistry>,
    resolver: DependencyResolver,
    options: OakOptions,
}

impl OakManager {
    pub fn init_project(&self, name: Option<String>, description: Option<String>) -> OakResult;
    pub fn add_dependency(&self, name: &str, version: Option<&str>, dev: bool) -> OakResult;
    pub fn remove_dependency(&self, name: &str) -> OakResult;
    pub fn list_dependencies(&self) -> OakResult;
    pub fn install_dependencies(&self) -> OakResult;
    pub fn validate_config(&self) -> OakResult;
}
```

#### 3. Registry (registry.rs)

```rust
pub trait PackageRegistry {
    fn find_package(&self, name: &str, version: &str) -> Result<Option<OakPackage>, String>;
    fn install_package(&self, package: &OakPackage) -> Result<(), String>;
    fn remove_package(&self, name: &str, version: &str) -> Result<(), String>;
    fn list_installed(&self) -> Result<Vec<OakPackage>, String>;
}

pub struct LocalRegistry {
    packages_path: PathBuf,
    cache: HashMap<String, Vec<OakPackage>>,
}

pub struct RemoteRegistry {
    base_url: String,
    client: HttpClient,  // Futuro
}
```

#### 4. APIs Externas (api.rs)

```rust
// API C/FFI
#[no_mangle]
pub extern "C" fn oak_api_new() -> *mut OakApiHandle;
#[no_mangle]
pub extern "C" fn oak_init_project(api: *mut OakApiHandle, name: *const c_char, desc: *const c_char) -> *mut c_char;
#[no_mangle]
pub extern "C" fn oak_api_free(api: *mut OakApiHandle);

// API Node.js (preparada)
#[cfg(feature = "napi")]
pub mod nodejs {
    use napi::bindgen_prelude::*;
    
    #[napi]
    pub struct OakNodeApi {
        inner: OakApi,
    }
    
    #[napi]
    impl OakNodeApi {
        #[napi(constructor)]
        pub fn new() -> Self;
        
        #[napi]
        pub async fn init_project(&self, options: InitOptions) -> Result<String>;
    }
}
```

---

## � Module Loader Integrado

### Arquitetura do Module Loader

O Module Loader é o componente responsável por carregar, resolver e gerenciar módulos Dryad, integrando-se perfeitamente com o sistema Oak.

```rust
// src/interpreter/module_loader.rs
pub struct ModuleLoader {
    modules: HashMap<String, Module>,
    search_paths: Vec<String>,
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub path: String,
    pub statements: Vec<Stmt>,
    pub exports: HashMap<String, EnvValue>,
    pub loaded: bool,
}
```

### Funcionalidades Principais

#### 1. Resolução de Caminhos

```rust
impl ModuleLoader {
    pub fn resolve_module_path(&self, module_name: &str) -> Option<String> {
        // Busca em múltiplos paths:
        // 1. Diretório atual
        // 2. ./modules/
        // 3. ./lib/ (para using)
        // 4. Paths do oaklibs.json
        for search_path in &self.search_paths {
            let full_path = format!("{}/{}.dryad", search_path, module_name);
            if Path::new(&full_path).exists() {
                return Some(full_path);
            }
        }
        None
    }
}
```

#### 2. Cache de Módulos

```rust
pub fn load_module(&mut self, module_name: &str) -> Result<Vec<Stmt>, DryadError> {
    // Verifica cache primeiro
    if let Some(module) = self.modules.get(module_name) {
        if module.loaded {
            return Ok(module.statements.clone());
        }
    }
    
    // Carrega e processa módulo
    let path = self.resolve_module_path(module_name)?;
    let content = fs::read_to_string(&path)?;
    
    // Parse do módulo
    let mut lexer = Lexer::new(&content);
    let mut parser = Parser::new(&mut lexer);
    let statements = parser.parse_program()?;
    
    // Adiciona ao cache
    let module = Module {
        name: module_name.to_string(),
        path,
        statements: statements.clone(),
        exports: HashMap::new(),
        loaded: true,
    };
    
    self.modules.insert(module_name.to_string(), module);
    Ok(statements)
}
```

#### 3. Integração com Oak Config

```rust
impl ModuleLoader {
    fn load_oak_config(&mut self) {
        if let Ok(content) = fs::read_to_string("oaklibs.json") {
            if let Ok(config) = serde_json::from_str::<Value>(&content) {
                // Adiciona lib_paths do oaklibs.json
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
}
```

### Tipos de Import Suportados

#### Using (Namespaces)

```dryad
using IO.Console;     // Busca em ./lib/IO/Console.dryad
using Core.Math;      // Busca em ./lib/Core/Math.dryad
```

Resolução: `lib/IO/Console.dryad`, `lib/Core/Math.dryad`

#### Use (Arquivos Locais)

```dryad
use './utils/helper.dryad';     // Caminho relativo direto
use '../shared/common.dryad';   // Caminho relativo pai
```

Resolução: Caminho relativo ao arquivo atual

### Exemplo de Uso Completo

```rust
// No evaluator, durante processamento de `using` ou `use`
let mut module_loader = ModuleLoader::new();

// Carrega configuração Oak automaticamente
module_loader.load_oak_config();

// Resolve e carrega módulo
match module_loader.load_module("IO.Console") {
    Ok(statements) => {
        // Executa statements do módulo
        for stmt in statements {
            self.eval_stmt(&stmt, env)?;
        }
        
        // Importa exports específicos
        if let Some(exported_fn) = module_loader.import_from_module("IO.Console", "println") {
            env.define("println".to_string(), exported_fn);
        }
    }
    Err(error) => {
        return Err(DryadError::new(
            format!("Failed to load module: {}", error),
            None,
            ErrorSeverity::Error,
        ));
    }
}
```

---

## �📚 Common Libraries

### Sistema de Funções Nativas

```rust
// src/interpreter/evaluator.rs
pub struct NativeRegistry {
    functions: HashMap<String, fn(&[Value]) -> Value>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        
        // Registrar funções nativas
        registry.register_core_types();
        registry.register_io_console();
        registry.register_io_filesystem();
        registry.register_system_env();
        
        registry
    }
    
    fn register_core_types(&mut self) {
        self.functions.insert("native_core_typeof".to_string(), native_core_typeof);
        self.functions.insert("native_core_is_number".to_string(), native_core_is_number);
        // ... outras funções
    }
}

// Implementação de função nativa
fn native_core_typeof(args: &[Value]) -> Value {
    if args.is_empty() {
        return Value::String("undefined".to_string());
    }
    
    let type_name = match &args[0] {
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Bool(_) => "boolean",
        Value::Null => "null",
        Value::Function { .. } => "function",
        Value::Class(_) => "class",
        Value::Instance(_) => "instance",
        Value::NativeFunction(_) => "function",
    };
    
    Value::String(type_name.to_string())
}
```

### Módulos Disponíveis

| Módulo | Arquivo | Funcionalidades |
|--------|---------|-----------------|
| `Core.Types` | `lib/core/types.dryad` | Verificação/conversão de tipos |
| `Core.Meta` | `lib/core/meta.dryad` | Reflection e metadados |
| `IO.Console` | `lib/IO/console.dryad` | Entrada/saída console |
| `IO.FileSystem` | `lib/IO/fs.dryad` | Operações arquivo/diretório |
| `IO.Buffer` | `lib/IO/buffer.dryad` | Manipulação de buffers |
| `System.Environment` | `lib/system/env.dryad` | Variáveis de ambiente |
| `System.Process` | `lib/system/process.dryad` | Execução de processos |
| `System.Time` | `lib/system/time.dryad` | Tempo e cronometragem |
| `System.Platform` | `lib/system/platform.dryad` | Informações do sistema |

---

## 🔌 APIs e FFI

### API C/FFI

```c
// Exemplo de cabeçalho C
// oak_api.h
#ifndef OAK_API_H
#define OAK_API_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OakApiHandle OakApiHandle;

// Lifecycle
OakApiHandle* oak_api_new(void);
void oak_api_free(OakApiHandle* api);

// Project management
char* oak_init_project(OakApiHandle* api, const char* name, const char* description);
char* oak_add_package(OakApiHandle* api, const char* name, const char* version, int is_dev);
char* oak_list_packages(OakApiHandle* api);
char* oak_validate_config(OakApiHandle* api);

// Utilities
void oak_free_string(char* str);

#ifdef __cplusplus
}
#endif

#endif // OAK_API_H
```

### Bindings Python (Futuro)

```python
# Exemplo de binding Python usando ctypes
import ctypes
from ctypes import c_char_p, c_void_p, c_int

class OakAPI:
    def __init__(self):
        # Carregar biblioteca
        self.lib = ctypes.CDLL('./target/release/libdryad.so')
        
        # Definir assinaturas
        self.lib.oak_api_new.restype = c_void_p
        self.lib.oak_init_project.argtypes = [c_void_p, c_char_p, c_char_p]
        self.lib.oak_init_project.restype = c_char_p
        
        # Criar handle
        self.handle = self.lib.oak_api_new()
    
    def init_project(self, name: str, description: str = None) -> str:
        name_c = name.encode('utf-8')
        desc_c = description.encode('utf-8') if description else None
        
        result = self.lib.oak_init_project(self.handle, name_c, desc_c)
        return result.decode('utf-8')
    
    def __del__(self):
        if hasattr(self, 'handle'):
            self.lib.oak_api_free(self.handle)
```

---

## 🧪 Testes e Qualidade

### Estrutura de Testes

```
tests/
├── unit/                    # Testes unitários
│   ├── lexer_tests.rs
│   ├── parser_tests.rs
│   ├── interpreter_tests.rs
│   └── oak_tests.rs
├── integration/             # Testes de integração
│   ├── cli_tests.rs
│   ├── oak_integration.rs
│   └── common_lib_tests.rs
├── user/                    # Scripts de teste Dryad
│   └── scripts/
│       ├── basic_syntax.dryad
│       ├── static_methods.dryad
│       └── oak_examples.dryad
└── benchmarks/              # Benchmarks de performance
    └── performance_tests.rs
```

### Comandos de Teste

```bash
# Todos os testes
cargo test

# Testes específicos
cargo test lexer
cargo test oak::manager

# Testes com output
cargo test -- --nocapture

# Benchmarks
cargo bench

# Coverage (com tarpaulin)
cargo tarpaulin --html
```

### Métricas de Qualidade

- **Cobertura de código:** > 80%
- **Tempo de compilação:** < 30s (release)
- **Performance:** > 1000 ops/sec (parsing básico)
- **Warnings:** Zero warnings em build release

---

## 🤝 Contribuição

### Setup do Ambiente de Desenvolvimento

```bash
# 1. Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clonar repositório
git clone <repo-url>
cd dryad

# 3. Instalar ferramentas
cargo install cargo-tarpaulin    # Coverage
cargo install cargo-watch       # Auto-rebuild
cargo install cargo-expand      # Macro expansion

# 4. Executar testes
cargo test

# 5. Build release
cargo build --release
```

### Workflow de Desenvolvimento

1. **Feature Branch**: Criar branch para nova funcionalidade
2. **Desenvolvimento**: Implementar com testes
3. **Testes**: Garantir que todos os testes passam
4. **Documentation**: Atualizar documentação
5. **Pull Request**: Criar PR com descrição detalhada
6. **Review**: Code review e aprovação
7. **Merge**: Merge para main

### Convenções de Código

```rust
// Naming conventions
struct MyStruct { }          // PascalCase para structs/enums
fn my_function() { }         // snake_case para funções
const MY_CONSTANT: u32 = 42; // SCREAMING_SNAKE_CASE para constantes

// Documentação
/// Calcula o quadrado de um número
/// 
/// # Argumentos
/// 
/// * `x` - O número a ser elevado ao quadrado
/// 
/// # Exemplos
/// 
/// ```
/// let result = square(4);
/// assert_eq!(result, 16);
/// ```
pub fn square(x: i32) -> i32 {
    x * x
}
```

### Áreas para Contribuição

1. **Core Language**
   - Loops (for, while)
   - Arrays e Objects
   - Pattern matching
   - Error handling (try/catch)

2. **Oak System**
   - Remote registries
   - Dependency resolution avançada
   - Lock files
   - Workspaces

3. **Common Libraries**
   - Módulos HTTP/Web
   - Criptografia
   - JSON/XML parsing
   - Database connectors

4. **Tooling**
   - Language Server Protocol
   - Debugger
   - Formatter
   - Package registry server

5. **Performance**
   - JIT compilation
   - Optimizações
   - Profiling tools
   - Memory management

---

## 📊 Roadmap Técnico

### Versão 0.2.0 (Q3 2025)
- [ ] Sistema de arrays
- [ ] Loops (for/while)
- [ ] Error handling (try/catch)
- [ ] Remote Oak registries

### Versão 0.3.0 (Q4 2025)
- [ ] JIT compilation básica
- [ ] Language Server Protocol
- [ ] Debugger integrado
- [ ] Package registry público

### Versão 1.0.0 (Q1 2026)
- [ ] Performance otimizada
- [ ] Ecosystem completo
- [ ] Documentação final
- [ ] Estabilidade da API

---

**Esta documentação é mantida pela equipe de desenvolvimento Dryad**  
**Para contribuir, veja as [guidelines de contribuição](CONTRIBUTING.md)**
