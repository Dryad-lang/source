# 🗺️ Roadmap do Projeto Dryad

Este documento detalha os planos de desenvolvimento futuro para a linguagem Dryad.

## 🎯 Visão Geral

### **Objetivo de Longo Prazo**
Criar uma linguagem de programação moderna, expressiva e fácil de usar, adequada para:
- Scripts de automação
- Prototipagem rápida
- Ensino de programação
- Aplicações de linha de comando
- Processamento de dados

### **Princípios Orientadores**
- **Simplicidade:** Sintaxe clara e intuitiva
- **Expressividade:** Código conciso mas legível
- **Robustez:** Detecção precoce de erros
- **Modularidade:** Código reutilizável
- **Performance:** Adequada para casos de uso típicos

## 📅 Cronograma de Desenvolvimento

### **✅ v0.1.0 - MVP Básico** (Concluído)
**Janeiro 2025**

**Objetivos:**
- [x] Lexer funcional
- [x] Parser básico
- [x] Interpretador mínimo
- [x] CLI simples
- [x] Testes básicos

**Funcionalidades:**
- [x] Variáveis (`let x = valor`)
- [x] Operações aritméticas (`+`, `-`, `*`, `/`)
- [x] Operações de comparação (`==`, `!=`, `<`, `>`)
- [x] Estruturas condicionais (`if/else`)
- [x] Loops básicos (`while`)
- [x] Funções built-in (`print`)

### **✅ v0.1.1 - Refinamentos** (Concluído)
**Janeiro 2025**

**Objetivos:**
- [x] Sistema de tipos robusto
- [x] Melhor tratamento de erros
- [x] REPL completo
- [x] Testes abrangentes
- [x] Documentação básica

**Funcionalidades:**
- [x] Verificação de tipos (modo `--strict`)
- [x] Sistema de erros idiomático
- [x] REPL interativo com comandos
- [x] I/O de arquivos (`read_file`, `write_file`)
- [x] Loops `for`
- [x] Blocos de código

---

## 🚀 Próximas Versões

### **🔄 v0.2.0 - Funções e Escopo** (Em Planejamento)
**Meta: Março 2025**

**Objetivos Principais:**
- [ ] Implementar funções definidas pelo usuário
- [ ] Sistema de escopo local
- [ ] Melhorar reutilização de código

**Funcionalidades Planejadas:**

#### **Declaração de Funções**
```javascript
fun somar(a, b) {
    return a + b;
}

fun saudar(nome) {
    print("Olá, " + nome + "!");
}
```

#### **Chamadas de Função**
```javascript
let resultado = somar(5, 3);
saudar("João");
```

#### **Escopo Local**
```javascript
let global = 10;

fun teste() {
    let local = 20;  // Variável local
    return global + local;
}
```

#### **Return Statements**
```javascript
fun ehPar(numero) {
    if (numero % 2 == 0) {
        return true;
    } else {
        return false;
    }
}
```

#### **Reatribuição de Variáveis**
```javascript
let x = 10;
x = 20;  // Reatribuir valor
x = x + 5;
```

**Componentes Técnicos:**
- [ ] Extend AST para declarações de função
- [ ] Implementar stack de ambientes
- [ ] Parser para sintaxe de função
- [ ] Return statement handling
- [ ] Scope resolution

**Testes:**
- [ ] Testes de declaração de função
- [ ] Testes de escopo
- [ ] Testes de recursão básica
- [ ] Testes de return

### **🔄 v0.3.0 - Estruturas de Dados** (Planejado)
**Meta: Maio 2025**

**Objetivos Principais:**
- [ ] Implementar arrays/listas
- [ ] Operações de indexação
- [ ] Iteração sobre coleções

**Funcionalidades Planejadas:**

#### **Arrays**
```javascript
let numeros = [1, 2, 3, 4, 5];
let nomes = ["João", "Maria", "Pedro"];
let misto = [1, "texto", true];
```

#### **Indexação**
```javascript
let primeiro = numeros[0];
numeros[1] = 10;
let tamanho = len(numeros);
```

#### **Métodos de Array**
```javascript
// Adicionar elementos
push(numeros, 6);
insert(numeros, 0, 0);

// Remover elementos
let ultimo = pop(numeros);
remove(numeros, 2);

// Operações
let existe = contains(numeros, 5);
```

#### **Iteração**
```javascript
// For-in loops
for (item in numeros) {
    print(item);
}

// For-range loops
for (i in 0..5) {
    print("Índice: " + i);
}
```

**Componentes Técnicos:**
- [ ] Array type implementation
- [ ] Indexing operations
- [ ] Built-in array functions
- [ ] Memory management for arrays
- [ ] Iteration constructs

### **🔄 v0.4.0 - Sistema de Módulos** (Planejado)
**Meta: Julho 2025**

**Objetivos Principais:**
- [ ] Sistema de imports/exports
- [ ] Biblioteca padrão
- [ ] Organização de código em módulos

**Funcionalidades Planejadas:**

#### **Exports**
```javascript
// math.dryad
export fun somar(a, b) {
    return a + b;
}

export let PI = 3.14159;
```

#### **Imports**
```javascript
// main.dryad
use math;
use math.{somar, PI};

let resultado = somar(5, 3);
print("Pi é " + PI);
```

#### **Biblioteca Padrão**
```javascript
// Matemática
use std.math;
let raiz = sqrt(16);

// Strings
use std.string;
let maiuscula = upper("hello");

// I/O
use std.io;
let entrada = input("Digite algo: ");
```

**Componentes Técnicos:**
- [ ] Module system architecture
- [ ] Import resolution
- [ ] Standard library structure
- [ ] Path handling
- [ ] Circular dependency detection

### **🔄 v0.5.0 - Programação Avançada** (Planejado)
**Meta: Setembro 2025**

**Funcionalidades Planejadas:**

#### **Closures**
```javascript
fun criarContador() {
    let count = 0;
    return fun() {
        count = count + 1;
        return count;
    };
}
```

#### **Higher-order Functions**
```javascript
fun map(lista, funcao) {
    let resultado = [];
    for (item in lista) {
        push(resultado, funcao(item));
    }
    return resultado;
}
```

#### **Pattern Matching**
```javascript
match valor {
    0 -> print("Zero");
    1..10 -> print("Entre 1 e 10");
    _ -> print("Outro valor");
}
```

## 🎯 Objetivos por Categoria

### **🔤 Linguagem Core**

#### **v0.2.0**
- [ ] Funções definidas pelo usuário
- [ ] Return statements
- [ ] Escopo local
- [ ] Reatribuição de variáveis

#### **v0.3.0**
- [ ] Arrays e listas
- [ ] Indexação
- [ ] Loops for-in
- [ ] Métodos de coleção

#### **v0.4.0**
- [ ] Sistema de módulos
- [ ] Imports/exports
- [ ] Namespace management

#### **v0.5.0**
- [ ] Closures
- [ ] Pattern matching
- [ ] Error handling (try/catch)

### **🛠️ Ferramentas**

#### **v0.2.0**
- [ ] Melhor debugging no REPL
- [ ] Formatter de código
- [ ] Lint básico

#### **v0.3.0**
- [ ] Package manager básico
- [ ] Documentação automática
- [ ] Profiler simples

#### **v0.4.0**
- [ ] Language server protocol
- [ ] Syntax highlighting
- [ ] Auto-completion

### **📚 Biblioteca Padrão**

#### **v0.3.0**
- [ ] `std.math` - Funções matemáticas
- [ ] `std.string` - Manipulação de strings
- [ ] `std.array` - Operações de array

#### **v0.4.0**
- [ ] `std.io` - I/O avançado
- [ ] `std.os` - Operações de sistema
- [ ] `std.json` - Parsing JSON

#### **v0.5.0**
- [ ] `std.http` - Cliente HTTP
- [ ] `std.regex` - Expressões regulares
- [ ] `std.datetime` - Data e hora

### **⚡ Performance e Qualidade**

#### **v0.3.0**
- [ ] Otimizações básicas do interpretador
- [ ] Garbage collection melhorado
- [ ] Benchmarks

#### **v0.4.0**
- [ ] Bytecode compilation
- [ ] Otimizações de tail call
- [ ] Memory profiling

#### **v0.5.0**
- [ ] JIT compilation (experimental)
- [ ] Parallel execution
- [ ] Advanced optimizations

## 🔄 Processo de Desenvolvimento

### **Ciclo de Release**
1. **Planejamento** (2 semanas)
   - Definir funcionalidades
   - Criar issues
   - Priorizar tarefas

2. **Desenvolvimento** (6-8 semanas)
   - Implementação incremental
   - Testes contínuos
   - Code review

3. **Estabilização** (2 semanas)
   - Bug fixes
   - Testes de integração
   - Documentação

4. **Release** (1 semana)
   - Final testing
   - Changelog
   - Deploy

### **Critérios de Qualidade**
- [ ] 95%+ cobertura de testes
- [ ] Zero warnings no build
- [ ] Documentação completa
- [ ] Backwards compatibility
- [ ] Performance benchmarks

## 🤝 Contribuições

### **Como Contribuir**
1. **Issues:** Reportar bugs e sugestões
2. **Documentation:** Melhorar documentação
3. **Code:** Implementar funcionalidades
4. **Testing:** Adicionar testes
5. **Examples:** Criar exemplos

### **Áreas que Precisam de Ajuda**
- [ ] Implementação de arrays
- [ ] Sistema de módulos
- [ ] Biblioteca padrão
- [ ] Tooling (LSP, formatter)
- [ ] Documentação e exemplos

## 📋 Backlog de Funcionalidades

### **Alta Prioridade**
- [ ] User-defined functions
- [ ] Variable reassignment
- [ ] Arrays/lists
- [ ] For-in loops
- [ ] Basic standard library

### **Média Prioridade**
- [ ] Module system
- [ ] Error handling (try/catch)
- [ ] String interpolation
- [ ] Regular expressions
- [ ] Better debugging tools

### **Baixa Prioridade**
- [ ] Object-oriented features
- [ ] Async/await
- [ ] Type annotations
- [ ] Macros
- [ ] FFI (Foreign Function Interface)

## 🎯 Metas de Longo Prazo (2026+)

### **Ecossistema**
- Package manager maduro
- Biblioteca padrão rica
- Comunidade ativa
- Integração com IDEs

### **Performance**
- Compilação JIT
- Otimizações avançadas
- Execução paralela
- Memory efficiency

### **Funcionalidades Avançadas**
- Type system opcional
- Macro system
- FFI para C/Rust
- Web assembly target

---

**Nota:** Este roadmap é flexível e pode ser ajustado baseado no feedback da comunidade e necessidades do projeto.

**Última atualização:** Janeiro 2025
