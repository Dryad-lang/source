# 🔧 Documentação para Desenvolvedores

Esta seção contém documentação técnica detalhada sobre a implementação interna do Dryad, destinada a desenvolvedores que querem contribuir ou entender como o sistema funciona.

## 📑 Índice

### Arquitetura Central
- [**Pipeline e Arquitetura**](./pipeline.md) - Visão geral do fluxo de execução
- [**Sistema de Lexer**](./lexer.md) - Análise léxica e tokenização
- [**Parser e AST**](./parser.md) - Análise sintática e construção da árvore
- [**Interpretador**](./interpreter.md) - Avaliação e execução do código

### Sistemas Auxiliares
- [**Sistema de Tipos**](./type-system.md) - Verificação e inferência de tipos
- [**Sistema de Erros**](./error-system.md) - Tratamento e reportagem de erros
- [**CLI e REPL**](./cli-repl.md) - Interfaces de usuário

### Desenvolvimento
- [**Testes**](./testing.md) - Estratégias e implementação de testes
- [**Contribuindo**](./contributing.md) - Guia para novos contribuidores

## 🏗️ Visão Geral da Arquitetura

O Dryad segue uma arquitetura de pipeline clássica para interpretadores:

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Código    │───▶│    Lexer    │───▶│   Parser    │───▶│Type Checker │
│   Fonte     │    │(tokenização)│    │    (AST)    │    │ (opcional)  │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                                                                   │
┌─────────────┐    ┌─────────────┐    ┌─────────────┐             │
│  Resultado  │◀───│Interpretador│◀───│ Ambiente de │◀────────────┘
│   Final     │    │ (execução)  │    │  Execução   │
└─────────────┘    └─────────────┘    └─────────────┘
```

## 📦 Estrutura de Módulos

```
src/
├── lexer/           # Análise léxica
│   ├── token.rs     # Definição de tokens
│   └── tokenizer.rs # Implementação do lexer
├── parser/          # Análise sintática
│   ├── ast.rs       # Definição da AST
│   └── parser.rs    # Implementação do parser
├── interpreter/     # Sistema de interpretação
│   ├── env.rs       # Ambiente de execução
│   ├── evaluator.rs # Avaliador principal
│   ├── types.rs     # Sistema de tipos
│   ├── errors.rs    # Sistema de erros
│   └── io.rs        # Operações de I/O
└── cli/             # Interface de usuário
    ├── cli.rs       # Interface de linha de comando
    └── repl.rs      # Ambiente interativo
```

## 🔍 Conceitos Chave

### 1. **Token**
Unidade léxica básica (palavra-chave, operador, literal, etc.)

### 2. **AST (Abstract Syntax Tree)**
Representação estrutural do código como árvore

### 3. **Environment**
Contexto de execução que mantém variáveis e seus valores

### 4. **Evaluator**
Motor de execução que percorre a AST e executa operações

### 5. **Type Checker**
Sistema opcional de verificação de tipos

## 🚀 Como Começar a Contribuir

1. **Leia a documentação técnica** de cada módulo
2. **Execute os testes** para entender o comportamento esperado
3. **Explore o código** começando pelo `main.rs`
4. **Identifique uma área** de interesse (lexer, parser, etc.)
5. **Consulte o roadmap** para ver o que precisa ser implementado

## 📚 Documentos Recomendados para Iniciantes

1. [Pipeline e Arquitetura](./pipeline.md) - Entenda o fluxo geral
2. [Sistema de Lexer](./lexer.md) - Como o código é tokenizado
3. [Parser e AST](./parser.md) - Como a estrutura é construída
4. [Testes](./testing.md) - Como garantir qualidade

## 🛠️ Ferramentas de Desenvolvimento

- **Rust 1.88+** - Linguagem de implementação
- **Cargo** - Build e gerenciamento de dependências
- **cargo test** - Execução de testes
- **cargo doc** - Geração de documentação
- **cargo clippy** - Linting e sugestões
- **cargo fmt** - Formatação de código
