# 🌳 Documentação Completa da Linguagem Dryad

**Versão:** 0.1.0  
**Data:** 8 de janeiro de 2025  
**Status:** Sistema Oak Modular Completo + Refatoração Integrada

---

## 📋 Índice

1. [Visão Geral](#visão-geral)
2. [Instalação e Configuração](#instalação-e-configuração)
3. [Sintaxe Básica](#sintaxe-básica)
4. [Sistema de Tipos](#sistema-de-tipos)
5. [Operadores](#operadores)
6. [Estruturas de Controle](#estruturas-de-controle)
7. [Funções e Classes](#funções-e-classes)
8. [Sistema de Imports/Exports](#sistema-de-importsexports)
9. [Package Manager Oak](#package-manager-oak)
10. [Common Libraries](#common-libraries)
11. [APIs Externas](#apis-externas)
12. [Exemplos Práticos](#exemplos-práticos)
13. [Referência CLI](#referência-cli)
14. [Guia de Troubleshooting](#guia-de-troubleshooting)
15. [Roadmap e Contribuição](#roadmap-e-contribuição)

---

## 🎯 Visão Geral

Dryad é uma linguagem de programação moderna e expressiva, projetada para ser:

- **Simples e Intuitiva**: Sintaxe limpa e fácil de aprender
- **Modular**: Sistema robusto de imports/exports e package management
- **Extensível**: APIs para integração com C/FFI e Node.js
- **Performática**: Interpretador otimizado em Rust
- **Completa**: Common libraries abrangentes e organizadas

### 🚀 Principais Características

- ✅ **Sistema de Classes** com herança e métodos estáticos
- ✅ **Concatenação de Strings** automática e inteligente
- ✅ **Operadores Lógicos** completos (&&, ||, !)
- ✅ **Sistema Oak Modular** para package management integrado
- ✅ **Imports/Exports** modulares com distinção clara (`using` vs `use`)
- ✅ **Common Libraries** nativas organizadas em namespaces
- ✅ **APIs Externas** C/FFI e Node.js preparadas
- ✅ **CLI Moderna** com REPL interativo e Oak commands
- ✅ **Module Loader** avançado com cache e resolução automática
- ✅ **Configuração Centralizada** via `oaklibs.json`

### 🌳 Ecossistema Oak

O sistema Oak fornece uma experiência moderna de package management:

- **Oak Init**: Inicialização rápida de projetos
- **Oak Add**: Gerenciamento de dependências
- **Oak List**: Visualização de pacotes instalados
- **Common Libraries**: Bibliotecas prontas para uso
- **Module Resolution**: Resolução automática de dependências
- **Configuração Flexível**: Paths customizáveis via JSON

---

## 🛠️ Instalação e Configuração

### Compilação

```bash
# Clonar repositório
git clone <repo-url> dryad
cd dryad

# Compilar
cargo build --release

# Executar
./target/release/dryad --help
```

### Comandos Básicos

```bash
# Executar arquivo
dryad script.dryad

# Modo interativo (REPL)
dryad --repl

# Modo verbose
dryad --verbose script.dryad

# Verificação de tipos estrita
dryad --strict script.dryad
```

---

## 📝 Sintaxe Básica

### Variáveis

```dryad
// Declaração de variáveis
let name = "João";
let age = 30;
let isActive = true;
let value = null;

// Reatribuição
age = 31;
name = "João Silva";
```

### Comentários

```dryad
// Comentário de linha única

/*
  Comentário
  de múltiplas
  linhas
*/
```

### Literais

```dryad
// Números
let integer = 42;
let float = 3.14;
let negative = -10;

// Strings
let singleQuote = 'Hello World';
let doubleQuote = "Hello World";

// Booleanos
let isTrue = true;
let isFalse = false;

// Nulo
let empty = null;
```

---

## 🔢 Sistema de Tipos

### Tipos Primitivos

| Tipo | Descrição | Exemplo |
|------|-----------|---------|
| `Number` | Números inteiros e decimais | `42`, `3.14` |
| `String` | Texto | `"Hello"`, `'World'` |
| `Bool` | Booleano | `true`, `false` |
| `Null` | Valor nulo | `null` |

### Verificação de Tipos

```dryad
// Usando Common Library
using Core.Types;

let value = 42;
let type = Types.typeof(value);        // "number"
let isNum = Types.isNumber(value);     // true
let isStr = Types.isString(value);     // false
```

### Conversões

```dryad
// Conversões automáticas
let result = "Valor: " + 42;           // "Valor: 42"
let sum = "10" + "20";                 // "1020" (concatenação)

// Conversões explícitas
let number = Types.toNumber("42");     // 42
let text = Types.toString(42);         // "42"
let bool = Types.toBool("true");       // true
```

---

## ⚡ Operadores

### Operadores Aritméticos

```dryad
let a = 10;
let b = 3;

let sum = a + b;        // 13
let diff = a - b;       // 7
let mult = a * b;       // 30
let div = a / b;        // 3.333...
```

### Operadores de Comparação

```dryad
let x = 5;
let y = 10;

let equal = x == y;     // false
let notEqual = x != y;  // true
let less = x < y;       // true
let greater = x > y;    // false
let lessEq = x <= y;    // true
let greaterEq = x >= y; // false
```

### Operadores Lógicos

```dryad
let a = true;
let b = false;

let and = a && b;       // false
let or = a || b;        // true
let not = !a;           // false

// Precedência correta
let result = !a || b && true;  // false
```

### Operadores Unários

```dryad
let x = 10;
let y = true;

let negative = -x;      // -10
let positive = +x;      // 10
let notBool = !y;       // false
```

### Concatenação de Strings

```dryad
// String + String
let full = "Hello" + " World";          // "Hello World"

// String + Number
let msg = "Idade: " + 25;               // "Idade: 25"

// Mixed concatenation
let product = "Laptop";
let price = 2500.99;
let inStock = true;
let info = "Produto: " + product + ", Preço: R$" + price + ", Disponível: " + inStock;
// "Produto: Laptop, Preço: R$2500.99, Disponível: true"
```

---

## 🔀 Estruturas de Controle

### Condicionais

```dryad
let age = 18;

if (age >= 18) {
    print("Maior de idade");
} else {
    print("Menor de idade");
}

// Condicionais aninhadas
let score = 85;

if (score >= 90) {
    print("Excelente");
} else if (score >= 70) {
    print("Bom");
} else {
    print("Precisa melhorar");
}
```

### Expressões Condicionais

```dryad
let status = age >= 18 ? "Adulto" : "Menor";
let max = a > b ? a : b;
```

---

## 🔧 Funções e Classes

### Funções

```dryad
// Função simples
function greet(name) {
    return "Olá, " + name + "!";
}

// Função com múltiplos parâmetros
function calculate(a, b, operation) {
    if (operation == "add") {
        return a + b;
    } else if (operation == "multiply") {
        return a * b;
    }
    return 0;
}

// Chamada de função
let message = greet("João");
let result = calculate(10, 5, "add");
```

### Classes

```dryad
// Definição de classe
class Person {
    // Construtor implícito com parâmetros
    function constructor(name, age) {
        this.name = name;
        this.age = age;
    }
    
    // Método de instância
    function introduce() {
        return "Meu nome é " + this.name + " e tenho " + this.age + " anos";
    }
    
    // Método público
    public function getName() {
        return this.name;
    }
    
    // Método estático
    public static function species() {
        return "Homo sapiens";
    }
}

// Uso da classe
let person = new Person("Ana", 25);
let intro = person.introduce();
let name = person.getName();
let species = Person.species();  // Método estático
```

### Herança

```dryad
class Student extends Person {
    function constructor(name, age, course) {
        super(name, age);  // Chama construtor da classe pai
        this.course = course;
    }
    
    function study() {
        return this.name + " está estudando " + this.course;
    }
    
    // Sobrescrita de método
    function introduce() {
        return super.introduce() + " e estudo " + this.course;
    }
}

let student = new Student("Carlos", 20, "Engenharia");
let studying = student.study();
```

---

## 📦 Sistema de Imports/Exports

### Sistema Duplo de Imports

#### `using` - Para Bibliotecas (Namespaces)

```dryad
// Import simples
using IO.Console;
using Core.Types;

// Import com alias
using System.Environment as Env;

// Import múltiplo
using Core.Types, Core.Meta, IO.FileSystem;

// Uso
Console.println("Hello World!");
let type = Types.typeof(42);
let home = Env.get("HOME");
```

#### `use` - Para Arquivos Locais

```dryad
// Import de arquivo local
use './mymodule.dryad';
use '../utils/helper.dryad';

// Import com alias
use './math-utils.dryad' as MathUtil;

// Uso
helper.doSomething();
let result = MathUtil.calculate(10, 20);
```

### Exports

```dryad
// Export de função
export function utilityFunction(x) {
    return x * 2;
}

// Export de classe
export class Calculator {
    static function add(a, b) {
        return a + b;
    }
    
    static function multiply(a, b) {
        return a * b;
    }
}

// Export com namespace
namespace Math.Utils {
    export function factorial(n) {
        if (n <= 1) return 1;
        return n * factorial(n - 1);
    }
    
    export class Advanced {
        static function power(base, exp) {
            let result = 1;
            // Implementação...
            return result;
        }
    }
}
```

---

## 🌳 Package Manager Oak

### Comandos Básicos

```bash
# Inicializar projeto Oak
dryad oak init

# Adicionar dependência
dryad oak add package-name

# Adicionar dependência de desenvolvimento
dryad oak add package-name --dev

# Listar dependências
dryad oak list

# Instalar dependências
dryad oak install

# Atualizar dependências
dryad oak update

# Remover dependência
dryad oak remove package-name

# Validar configuração
dryad oak validate
```

### Configuração (oaklibs.json)

```json
{
  "name": "meu-projeto-dryad",
  "version": "1.0.0",
  "description": "Descrição do projeto",
  "author": "Seu Nome",
  "license": "MIT",
  "homepage": "https://github.com/user/repo",
  "repository": "https://github.com/user/repo.git",
  "keywords": ["dryad", "package"],
  "dependencies": {
    "math-utils": {
      "version": "1.2.0",
      "source": {
        "Registry": "default"
      },
      "optional": false,
      "dev": false
    }
  },
  "dev_dependencies": {
    "test-framework": {
      "version": "latest",
      "source": {
        "Git": "https://github.com/test/framework.git"
      },
      "optional": true,
      "dev": true
    }
  },
  "lib_paths": [
    "./lib",
    "./modules"
  ],
  "scripts": {
    "test": "dryad test-runner.dryad",
    "build": "dryad build-script.dryad"
  },
  "oak_version": "1.0.0"
}
```

### Estrutura de Projeto

```
meu-projeto/
├── oaklibs.json              # Configuração Oak
├── main.dryad               # Arquivo principal
├── lib/                     # Bibliotecas (using)
│   ├── core/
│   │   ├── types.dryad
│   │   └── meta.dryad
│   ├── IO/
│   │   ├── console.dryad
│   │   ├── filesystem.dryad
│   │   └── buffer.dryad
│   └── system/
│       ├── env.dryad
│       ├── process.dryad
│       └── platform.dryad
├── modules/                 # Módulos locais (use)
│   ├── utils.dryad
│   └── helpers.dryad
└── tests/
    └── test-suite.dryad
```

---

## 📚 Common Libraries

### Core.Types - Manipulação de Tipos

```dryad
using Core.Types;

// Verificação de tipos
let value = 42;
let isNumber = Types.isNumber(value);      // true
let isString = Types.isString(value);      // false
let isBool = Types.isBool(value);          // false
let isNull = Types.isNull(value);          // false

// Conversões
let asString = Types.toString(value);      // "42"
let asNumber = Types.toNumber("100");      // 100
let asBool = Types.toBool("true");         // true

// Comparações
let equals = Types.equals(42, 42);         // true
let deepEquals = Types.deepEquals(obj1, obj2);

// Utilidades
let isEmpty = Types.stringIsEmpty("");     // true
let type = Types.typeof(value);            // "number"
```

### IO.Console - Entrada/Saída

```dryad
using IO.Console;

// Saída
Console.print("Hello ");                   // Sem quebra de linha
Console.println("World!");                // Com quebra de linha

// Entrada
let name = Console.input("Digite seu nome: ");
let age = Console.input("Digite sua idade: ");
```

### IO.FileSystem - Sistema de Arquivos

```dryad
using IO.FileSystem;

// Operações com arquivos
let content = FileSystem.readFile("data.txt");
FileSystem.writeFile("output.txt", "Hello World!");
FileSystem.appendFile("log.txt", "Nova entrada\n");

let exists = FileSystem.fileExists("config.json");  // true/false
FileSystem.deleteFile("temp.txt");

// Operações com diretórios
let files = Directory.listFiles("./src");
let exists = Directory.exists("./backup");
Directory.create("./new-folder");
Directory.delete("./old-folder");

// Utilitários de caminho
let absolute = Path.absolute("./relative/path");
let dirname = Path.dirname("/usr/local/bin/app");
let basename = Path.basename("/usr/local/bin/app");
let extension = Path.extension("document.pdf");
```

### IO.Buffer - Manipulação de Buffers

```dryad
using IO.Buffer;

// Criação e manipulação
let buffer = Buffer.create(1024);
let length = Buffer.length(buffer);
Buffer.append(buffer, "Hello World!");
Buffer.clear(buffer);

let slice = Buffer.slice(buffer, 0, 10);
let index = Buffer.indexOf(buffer, "World");
```

### System.Environment - Variáveis de Ambiente

```dryad
using System.Environment;

// Variáveis de ambiente
let path = Environment.get("PATH");
Environment.set("DRYAD_HOME", "/usr/local/dryad");
let exists = Environment.exists("HOME");
let all = Environment.getAll();
Environment.remove("TEMP_VAR");
```

### System.Process - Execução de Processos

```dryad
using System.Process;

// Execução de comandos
let output = Process.execute("echo 'Hello World'");
let result = Process.executeWithArgs("ls", ["-la", "/home"]);
let asyncId = Process.executeAsync("long-running-command");

// Gerenciamento de processos
let pid = Process.getCurrentPid();
let isRunning = Process.isRunning(pid);
Process.kill(pid);
```

### System.Time - Tempo e Cronometragem

```dryad
using System.Time;

// Timestamps
let now = Time.now();                      // Unix timestamp
let millis = Time.nowMillis();             // Milissegundos

// Pausas
Time.sleep(2);                             // 2 segundos
Time.sleepMillis(500);                     // 500ms

// Formatação
let formatted = Time.format(now, "YYYY-MM-DD HH:mm:ss");

// Performance
let timer = Time.startTimer();
// ... código a ser medido ...
let elapsed = Time.elapsed(timer);
```

### System.Platform - Informações do Sistema

```dryad
using System.Platform;

// Informações do sistema
let os = Platform.getOS();                 // "Windows", "Linux", "macOS"
let arch = Platform.getArch();             // "x86_64", "arm64"
let hostname = Platform.getHostname();     // Nome do computador
let version = Platform.getOSVersion();     // Versão do SO
let user = Platform.getCurrentUser();      // Usuário atual
let uptime = Platform.getUptime();         // Tempo ligado
```

---

## 🔌 APIs Externas

### API C/FFI

```c
// Exemplo de uso da API C
#include "oak_api.h"

int main() {
    // Criar instância Oak
    OakApiHandle* oak = oak_api_new();
    
    // Inicializar projeto
    const char* result = oak_init_project(oak, "my-project", "Meu projeto");
    printf("Resultado: %s\n", result);
    
    // Adicionar pacote
    const char* add_result = oak_add_package(oak, "math-utils", "1.0.0", 0);
    printf("Adição: %s\n", add_result);
    
    // Listar pacotes
    const char* list = oak_list_packages(oak);
    printf("Pacotes: %s\n", list);
    
    // Validar configuração
    const char* validation = oak_validate_config(oak);
    printf("Validação: %s\n", validation);
    
    // Limpar recursos
    oak_api_free(oak);
    return 0;
}
```

### API Rust (Nativa)

```rust
use dryad::oak::{OakManager, OakCliIntegration, OakCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Via CLI Integration
    let cli = OakCliIntegration::new();
    
    let init_cmd = OakCommand::Init { 
        name: Some("rust-project".to_string()), 
        description: Some("Projeto via Rust API".to_string()) 
    };
    
    let result = cli.execute_command(init_cmd);
    if result.success {
        println!("✅ {}", result.output);
    }
    
    // Via Manager direto
    let manager = OakManager::new();
    let result = manager.init_project(
        Some("direct-project".to_string()), 
        Some("Via manager direto".to_string())
    );
    
    if result.success {
        println!("✅ {}", result.message);
    }
    
    Ok(())
}
```

### API Node.js (Preparada)

```javascript
// Exemplo de uso futuro da API Node.js
const oak = require('dryad-oak');

async function main() {
    try {
        // Inicializar projeto
        const initResult = await oak.initProject({
            name: 'node-project',
            description: 'Projeto via Node.js'
        });
        
        console.log('Init:', initResult);
        
        // Adicionar dependência
        const addResult = await oak.addPackage({
            name: 'utilities',
            version: '1.0.0',
            dev: false
        });
        
        console.log('Add:', addResult);
        
        // Listar dependências
        const packages = await oak.listPackages();
        console.log('Packages:', packages);
        
    } catch (error) {
        console.error('Erro:', error);
    }
}

main();
```

---

## 🧪 Exemplos Práticos

### Exemplo Completo: Sistema de Logging

```dryad
// logger.dryad
using System.Time;
using IO.FileSystem;
using IO.Console;

export class Logger {
    private static let logFile = "app.log";
    
    public static function info(message) {
        Logger.writeLog("INFO", message);
    }
    
    public static function warn(message) {
        Logger.writeLog("WARN", message);
    }
    
    public static function error(message) {
        Logger.writeLog("ERROR", message);
    }
    
    private static function writeLog(level, message) {
        let timestamp = Time.format(Time.now(), "YYYY-MM-DD HH:mm:ss");
        let logEntry = "[" + timestamp + "] " + level + ": " + message + "\n";
        
        // Escrever no arquivo
        FileSystem.appendFile(Logger.logFile, logEntry);
        
        // Mostrar no console também
        Console.println(logEntry);
    }
}
```

### Exemplo: Aplicação Completa

```dryad
// main.dryad
using IO.Console;
using System.Environment;
use './logger.dryad';

class Application {
    private let name;
    private let version;
    
    function constructor(name, version) {
        this.name = name;
        this.version = version;
        Logger.info("Aplicação iniciada: " + name + " v" + version);
    }
    
    public function run() {
        Console.println("=== " + this.name + " v" + this.version + " ===");
        
        // Verificar ambiente
        let user = Environment.get("USER");
        if (user != null) {
            Logger.info("Usuário logado: " + user);
            Console.println("Olá, " + user + "!");
        } else {
            Logger.warn("Usuário não identificado");
        }
        
        // Simular operações
        this.processData();
        this.generateReport();
        
        Logger.info("Aplicação finalizada com sucesso");
    }
    
    private function processData() {
        Logger.info("Processando dados...");
        // Simular processamento
        Console.println("Dados processados!");
    }
    
    private function generateReport() {
        Logger.info("Gerando relatório...");
        let reportContent = "Relatório gerado em " + Time.format(Time.now(), "DD/MM/YYYY");
        FileSystem.writeFile("report.txt", reportContent);
        Logger.info("Relatório salvo em report.txt");
    }
}

// Execução principal
let app = new Application("MeuApp", "1.0.0");
app.run();
```

---

## 💻 Referência CLI

### Comandos Principais

```bash
# Execução
dryad script.dryad              # Executar arquivo
dryad --repl                    # Modo interativo
dryad --help                    # Ajuda
dryad --version                 # Versão

# Opções
dryad --verbose script.dryad    # Modo verboso
dryad --strict script.dryad     # Verificação estrita de tipos
```

### Comandos Oak

```bash
# Gestão de Projeto
dryad oak init                  # Inicializar projeto
dryad oak init --name "MyApp"   # Com nome específico
dryad oak validate              # Validar configuração

# Gestão de Dependências
dryad oak add package-name      # Adicionar dependência
dryad oak add pkg --dev         # Dependência de desenvolvimento
dryad oak remove package-name   # Remover dependência
dryad oak list                  # Listar dependências
dryad oak install               # Instalar dependências
dryad oak update                # Atualizar dependências

# Utilitários
dryad oak info                  # Informações do projeto
```

### Códigos de Saída

| Código | Significado |
|--------|-------------|
| 0 | Sucesso |
| 1 | Erro de sintaxe |
| 2 | Erro de execução |
| 3 | Arquivo não encontrado |
| 4 | Erro de configuração Oak |

---

## 🔧 Guia de Troubleshooting

### Problemas Comuns

#### 1. Erro de Import/Module

```
Erro: Module not found: 'Utils.Helper'
```

**Soluções:**
- Verifique se o arquivo existe no caminho especificado
- Para bibliotecas do sistema use `using`, para arquivos locais use `use`
- Confira os `lib_paths` no `oaklibs.json`
- Verifique se a estrutura de diretórios está correta

```bash
# Debug de paths
dryad --verbose script.dryad

# Verificar configuração Oak
cat oaklibs.json
```

#### 2. Dependência Oak não encontrada

```
Erro: Package 'math-utils' not found
```

**Soluções:**
```bash
# Verificar dependências listadas
dryad oak list

# Adicionar dependência específica
dryad oak add math-utils

# Verificar configuração
dryad oak info
```

#### 3. Erro de Sintaxe

```
Erro: Unexpected token at line 15, column 8
```

**Soluções:**
- Verificar balanceamento de chaves `{}` e parênteses `()`
- Conferir sintaxe de funções: `function name() {}`
- Verificar declarações de variáveis: `let name = value;`
- Usar modo verbose para detalhes: `dryad --verbose script.dryad`

#### 4. Problemas de Permissão

```
Erro: Permission denied accessing 'oaklibs.json'
```

**Soluções:**
```bash
# Linux/macOS
chmod 644 oaklibs.json
sudo dryad oak init

# Windows (executar como administrador)
# Botão direito > "Executar como administrador"
```

#### 5. Conflitos de Export/Import

```
Erro: Cannot import 'functionName' from module
```

**Soluções:**
- Verificar se a função foi exportada: `export function functionName()`
- Conferir nome exato (case-sensitive)
- Verificar se o módulo está sendo carregado corretamente

```dryad
// No módulo (helper.dryad)
export function processData() {
    return "processed";
}

// No arquivo principal
use './helper.dryad';
let result = processData(); // Correto
```

### Debug e Desenvolvimento

#### Modo Verbose

```bash
dryad --verbose script.dryad
```

Mostra informações detalhadas sobre:
- Module loading process
- Import/export resolution
- Execution steps
- Error stack traces

#### Modo Strict

```bash
dryad --strict script.dryad
```

Ativa verificações rigorosas para:
- Variáveis não declaradas
- Tipos incompatíveis
- Imports não utilizados
- Exports não referenciados

#### Logs de Debug Estruturados

```dryad
using IO.Console;

// Diferentes níveis de log
Console.log("DEBUG", "Variable x = " + x);
Console.log("INFO", "Processing file: " + filename);
Console.log("WARN", "Deprecated function used");
Console.log("ERROR", "Operation failed: " + error);

// Debug condicional
if (DEBUG_MODE) {
    Console.log("DEBUG", "Detailed execution info");
}
```

### Performance e Otimização

#### Dicas de Performance

1. **Use imports específicos:**
```dryad
// Bom
using IO.Console;
using Core.Math;

// Evitar imports desnecessários
// using Core.*;  // (quando disponível no futuro)
```

2. **Cache resultados computacionalmente pesados:**
```dryad
let cache = {};

function expensiveCalculation(input) {
    let cacheKey = "calc_" + input;
    
    if (cache.hasOwnProperty(cacheKey)) {
        return cache[cacheKey];
    }
    
    let result = /* cálculo pesado */;
    cache[cacheKey] = result;
    return result;
}
```

3. **Use métodos estáticos para operações simples:**
```dryad
class StringUtils {
    static function capitalize(str) {
        return str.charAt(0).toUpperCase() + str.slice(1);
    }
    
    static function reverse(str) {
        return str.split("").reverse().join("");
    }
}

// Melhor que criar instâncias desnecessárias
let title = StringUtils.capitalize("hello world");
```

4. **Otimize loops e operações de array:**
```dryad
// Bom para arrays pequenos
let filtered = data.filter(item => item.active);

// Para arrays grandes, considere loops manuais
let filtered = [];
for (let i = 0; i < data.length; i++) {
    if (data[i].active) {
        filtered.push(data[i]);
    }
}
```

### Debugging Específico do Oak

#### Verificar Configuração Oak

```bash
# Mostrar configuração atual
cat oaklibs.json

# Validar sintaxe JSON
dryad oak info

# Listar todos os paths de busca
dryad --verbose script.dryad | grep "Search path"
```

#### Resolver Problemas de Path

```json
// oaklibs.json
{
  "lib_paths": [
    "./lib",
    "./node_modules",
    "./custom/libs",
    "../shared/modules"
  ]
}
```

#### Module Loading Debug

```dryad
// Adicionar debug no início do arquivo
using IO.Console;

Console.log("DEBUG", "Starting module load");

try {
    using Core.Math;
    Console.log("INFO", "Math module loaded successfully");
} catch (error) {
    Console.log("ERROR", "Failed to load Math: " + error);
}
```

---

## 🚀 Roadmap e Contribuição

### Roadmap de Funcionalidades

#### Versão 0.2.0 (Q2 2025)
- [ ] **Arrays e Loops**: Implementação de `for`, `while`, `foreach`
- [ ] **Error Handling**: Sistema `try/catch/finally`
- [ ] **Registry Remoto**: Oak registry para distribuição de pacotes
- [ ] **Módulos HTTP**: Common library para requisições web
- [ ] **Debugger CLI**: Debug step-by-step integrado

#### Versão 0.3.0 (Q3 2025)
- [ ] **JIT Compilation**: Compilação Just-In-Time para performance
- [ ] **Language Server**: LSP para editores (VS Code, Vim, etc.)
- [ ] **Package Registry**: Servidor central de pacotes Oak
- [ ] **Build System**: Sistema de build e bundling
- [ ] **Workspaces**: Suporte para projetos multi-módulo

#### Versão 0.4.0 (Q4 2025)
- [ ] **WebAssembly**: Target WASM para execução no browser
- [ ] **Native Compilation**: AOT compilation para binários nativos
- [ ] **Profiler**: Ferramentas de profiling de performance
- [ ] **Package Templates**: Templates para diferentes tipos de projeto
- [ ] **Documentation Generator**: Geração automática de docs

### Como Contribuir

#### 1. Setup de Desenvolvimento

```bash
# Clone o repositório
git clone https://github.com/dryad-lang/dryad.git
cd dryad

# Build em modo desenvolvimento
cargo build

# Executar testes
cargo test

# Executar com debug
RUST_LOG=debug cargo run -- script.dryad
```

#### 2. Estrutura do Projeto

```
src/
├── lexer/              # Tokenização
├── parser/             # Parsing para AST
├── interpreter/        # Execução e ambiente
├── oak/                # Package manager
└── cli/                # Interface de linha de comando
```

#### 3. Convenções de Código

- **Rust Style**: Seguir `rustfmt` e `clippy`
- **Documentação**: Todos os métodos públicos devem ter docs
- **Testes**: Cada funcionalidade deve ter testes unitários
- **Error Handling**: Usar `Result<T, DryadError>` consistentemente

#### 4. Áreas para Contribuição

**Iniciante:**
- Correção de bugs menores
- Melhoria de mensagens de erro
- Documentação e exemplos
- Testes adicionais

**Intermediário:**
- Novas funcionalidades da linguagem
- Melhorias no Oak system
- Common libraries adicionais
- Otimizações de performance

**Avançado:**
- Compilação JIT/AOT
- Language Server Protocol
- APIs externas (C/FFI, WASM)
- Registry distribuído

#### 5. Process de Contribuição

1. **Fork** o repositório
2. **Create branch**: `git checkout -b feature/nova-funcionalidade`
3. **Implement**: Código + testes + documentação
4. **Test**: `cargo test` + testes manuais
5. **Commit**: Mensagens descritivas
6. **Pull Request**: Descrição clara das mudanças

### Recursos da Comunidade

- **GitHub**: [dryad-lang/dryad](https://github.com/dryad-lang/dryad)
- **Documentação**: [docs.dryad-lang.org](https://docs.dryad-lang.org)
- **Exemplos**: [github.com/dryad-lang/examples](https://github.com/dryad-lang/examples)
- **Discord**: [discord.gg/dryad-lang](https://discord.gg/dryad-lang)
- **Reddit**: [r/DryadLang](https://reddit.com/r/DryadLang)

### Licença e Créditos

**Licença:** MIT License
**Maintainers:** Dryad Core Team
**Contribuidores:** [Lista completa](https://github.com/dryad-lang/dryad/contributors)

---

## 🎯 Conclusão

A linguagem Dryad oferece um ambiente completo de desenvolvimento com:

- **Sintaxe moderna e intuitiva**
- **Sistema modular robusto (Oak)**
- **Common libraries abrangentes**
- **APIs para integração externa**
- **Tooling completo via CLI**

Para mais informações, consulte os arquivos de documentação específicos ou explore os exemplos no diretório `examples/`.

---

**Desenvolvido com ❤️ pela equipe Dryad**  
**© 2025 - MIT License**
