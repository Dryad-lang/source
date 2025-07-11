# 🌳 Dryad Programming Language

![Dryad Logo](https://img.shields.io/badge/Dryad-v0.1.0-green)
![Oak Package Manager](https://img.shields.io/badge/Oak-Modular-blue)
![License](https://img.shields.io/badge/License-MIT-blue)

**Dryad** é uma linguagem de programação moderna e expressiva com foco em simplicidade, modularidade e produtividade. Inclui o sistema de package manager **Oak** integrado e um ecossistema completo de desenvolvimento.


## ✨ Principais Características

- 🚀 **Sintaxe Simples**: Fácil de aprender e usar
- 📦 **Sistema Oak**: Package manager modular integrado
- 🔗 **Imports/Exports**: Sistema modular avançado (`using` vs `use`)
- 📚 **Common Libraries**: Bibliotecas nativas organizadas
- ⚡ **Performance**: Interpretador otimizado em Rust
- 🛠️ **CLI Moderna**: Interface de linha de comando completa
- 🔧 **APIs Externas**: Suporte C/FFI e Node.js preparado

## 🚀 Início Rápido

### 1. Instalação

```bash
# Clone o repositório
git clone https://github.com/dryad-lang/dryad.git
cd dryad

# Compile o projeto
cargo build --release

# Execute um script
./target/release/dryad script.dryad
```

### 2. Primeiro Programa

```dryad
// hello.dryad
print("Hello, Dryad!");

let nome = "Mundo";
print("Olá, " + nome + "!");

// Classes com métodos estáticos
class Math {
    static function square(x) {
        return x * x;
    }
}

let resultado = Math.square(5);
print("5² = " + resultado);
```

### 3. Sistema Oak (Package Manager)

```bash
# Inicializar projeto Oak
dryad oak init

# Listar dependências
dryad oak list

# Adicionar dependência (futuro)
dryad oak add math-utils
```

### 4. Sistema de Imports

```dryad
// Bibliotecas do sistema (using)
using IO.Console;
using Core.Types;

// Arquivos locais (use)
use './utils/helper.dryad';
use '../shared/common.dryad';

// Uso das funcionalidades
Console.println("Hello World!");
let type = Types.typeof(42);
helper.processData();
```

## 📁 Estrutura do Projeto

```
dryad/
├── src/                    # Código fonte Rust
│   ├── lexer/             # Tokenização
│   ├── parser/            # Parsing para AST
│   ├── interpreter/       # Execução e ambiente
│   ├── oak/               # Package manager Oak
│   └── cli/               # Interface linha de comando
├── lib/                   # Common Libraries Dryad
│   ├── core/              # Tipos e utilidades básicas
│   ├── IO/                # Entrada/saída
│   └── system/            # Sistema operacional
├── examples/              # Exemplos e demos
├── tests/                 # Testes automatizados
├── docs/                  # Documentação
└── oaklibs.json           # Configuração Oak
```

## 📚 Documentação

- **[📖 Documentação Completa](DRYAD_LANGUAGE_DOCUMENTATION.md)** - Guia completo da linguagem
- **[🔧 Documentação Técnica](TECHNICAL_DOCUMENTATION.md)** - Arquitetura e desenvolvimento
- **[🚀 Guia de Início Rápido](QUICK_START_GUIDE.md)** - Comece rapidamente

## 🌟 Exemplos

### Aplicação CLI

```dryad
using IO.Console;
using System.Environment as Env;

function main() {
    let args = Env.getArgs();
    
    if (args.length == 0) {
        Console.println("Usage: app <command>");
        return;
    }
    
    let command = args[0];
    if (command == "hello") {
        Console.println("Hello from Dryad!");
    }
}

main();
```

### Módulo com Exports

```dryad
// math.dryad
export function add(a, b) {
    return a + b;
}

export class Calculator {
    static function multiply(a, b) {
        return a * b;
    }
}
```

```dryad
// main.dryad
use './math.dryad';

let sum = add(5, 3);
let product = Calculator.multiply(4, 6);
print("Sum: " + sum + ", Product: " + product);
```

## 🛠️ Comandos CLI

### Execução Básica

```bash
# Executar arquivo
dryad script.dryad

# REPL interativo
dryad --repl

# Modo verbose
dryad --verbose script.dryad

# Ajuda
dryad --help
```

### Sistema Oak

```bash
# Inicializar projeto
dryad oak init

# Gerenciar dependências
dryad oak add package-name
dryad oak list
dryad oak remove package-name  # (futuro)

# Informações do projeto
dryad oak info                 # (futuro)
```

## 🔄 Sistema Oak Modular

O Oak é o sistema de package manager integrado, proporcionando:

- ✅ **Inicialização de projetos** (`oak init`)
- ✅ **Gerenciamento de dependências** (`oak add`, `oak list`)
- ✅ **Configuração centralizada** (oaklibs.json)
- ✅ **Resolução automática de módulos**
- ✅ **Cache inteligente de imports**
- 🔮 **Registry remoto** (em desenvolvimento)

### Configuração Oak (oaklibs.json)

```json
{
  "name": "my-dryad-project",
  "version": "1.0.0",
  "description": "Projeto usando Oak",
  "dependencies": {
    "math-utils": "1.0.0"
  },
  "lib_paths": [
    "./lib",
    "./node_modules"
  ],
  "scripts": {
    "test": "dryad tests/runner.dryad",
    "build": "dryad build.dryad"
  }
}
```

## 📦 Common Libraries

O Dryad inclui um conjunto abrangente de bibliotecas prontas:

### IO - Entrada e Saída
- **IO.Console**: Print, input, logs
- **IO.FileSystem**: Operações de arquivo
- **IO.Buffer**: Manipulação de buffers

### Core - Funcionalidades Essenciais
- **Core.Types**: Verificação e conversão de tipos
- **Core.Math**: Operações matemáticas
- **Core.Collections**: Arrays e mapas

### System - Sistema Operacional
- **System.Environment**: Variáveis de ambiente
- **System.Process**: Execução de comandos

## 🧪 Status de Desenvolvimento

### ✅ Implementado
- [x] Lexer e Parser completos
- [x] Interpreter funcional
- [x] Sistema de classes e métodos estáticos
- [x] Operadores lógicos (&&, ||, !)
- [x] Concatenação de strings automática
- [x] Sistema Oak modular
- [x] Imports/Exports (`using`/`use`)
- [x] Common libraries extensas
- [x] CLI moderna
- [x] Module loader com cache
- [x] APIs externas preparadas

### 🔄 Em Desenvolvimento
- [ ] Registry remoto Oak
- [ ] Arrays e loops (for/while)
- [ ] Error handling (try/catch)
- [ ] Módulos HTTP/Web

### 🔮 Planejado
- [ ] JIT compilation
- [ ] Language Server Protocol
- [ ] Package registry online
- [ ] WebAssembly target
- [ ] Native compilation

## 🤝 Contribuição

Contribuições são bem-vindas! Veja como ajudar:

1. **Fork** o repositório
2. **Clone** localmente: `git clone <your-fork>`
3. **Create branch**: `git checkout -b feature/nova-funcionalidade`
4. **Implemente** sua funcionalidade
5. **Teste**: `cargo test`
6. **Commit**: `git commit -m "Add: nova funcionalidade"`
7. **Push**: `git push origin feature/nova-funcionalidade`
8. **Pull Request**: Abra um PR descrevendo as mudanças

### Áreas para Contribuição

- 🐛 **Bug fixes** e melhorias
- 📚 **Documentação** e exemplos
- ⚡ **Performance** e otimizações
- 🌟 **Novas funcionalidades** da linguagem
- 🧪 **Testes** e quality assurance

## 📊 Roadmap

### Q1 2025
- [x] Sistema Oak modular integrado
- [x] Refatoração completa da arquitetura
- [x] Common libraries organizadas
- [x] Documentação completa

### Q2 2025
- [ ] Registry remoto Oak
- [ ] Arrays e estruturas de controle
- [ ] Error handling avançado
- [ ] Módulos HTTP

### Q3 2025
- [ ] JIT compilation
- [ ] Language Server Protocol
- [ ] Package registry online
- [ ] Build system integrado

## 📄 Licença

Este projeto está licenciado sob a [MIT License](LICENSE).

## 🌐 Links

- **Documentação**: [docs.dryad-lang.org](https://docs.dryad-lang.org) (futuro)
- **Exemplos**: [github.com/dryad-lang/examples](https://github.com/dryad-lang/examples) (futuro)
- **Discord**: [discord.gg/dryad-lang](https://discord.gg/dryad-lang) (futuro)

---

**Desenvolvido com ❤️ pela equipe Dryad**  
*Uma linguagem moderna para desenvolvimento produtivo*
