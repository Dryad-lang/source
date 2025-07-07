# 📚 Documentação do Projeto Dryad

Bem-vindo à documentação oficial do **Dryad**, uma linguagem de programação interpretada moderna, escrita em Rust. Esta documentação está organizada em três categorias principais para atender diferentes necessidades:

## 📁 Estrutura da Documentação

### 🔧 [**Develop**](./develop/) - Para Desenvolvedores do Sistema
Documentação técnica para desenvolvedores que querem contribuir ou entender a implementação:
- [**Pipeline e Arquitetura**](./develop/pipeline.md) - Como funciona o fluxo completo
- [**Sistema de Lexer**](./develop/lexer.md) - Análise léxica e tokenização
- [**Parser e AST**](./develop/parser.md) - Análise sintática e árvore abstrata
- [**Interpretador**](./develop/interpreter.md) - Avaliação e execução
- [**Sistema de Tipos**](./develop/type-system.md) - Verificação e inferência de tipos
- [**Sistema de Erros**](./develop/error-system.md) - Tratamento e reportagem de erros
- [**CLI e REPL**](./develop/cli-repl.md) - Interface de linha de comando
- [**Testes**](./develop/testing.md) - Estratégias e implementação de testes

### 👥 [**User**](./user/) - Para Usuários da Linguagem
Guias e referências para quem quer usar a linguagem Dryad:
- [**Guia de Início Rápido**](./user/quick-start.md) - Primeiros passos
- [**Sintaxe da Linguagem**](./user/syntax.md) - Referência completa da sintaxe
- [**CLI Reference**](./user/cli-reference.md) - Comandos e opções da linha de comando
- [**REPL Guide**](./user/repl-guide.md) - Usando o ambiente interativo
- [**Criando Scripts**](./user/writing-scripts.md) - Como escrever programas em Dryad
- [**Exemplos**](./user/examples.md) - Exemplos práticos e casos de uso
- [**Troubleshooting**](./user/troubleshooting.md) - Solução de problemas comuns

### 📊 [**Project**](./project/) - Status e Roadmap
Informações sobre o estado atual e futuro do projeto:
- [**Status Atual**](./project/current-status.md) - O que já está implementado
- [**Roadmap**](./project/roadmap.md) - Planos futuros e próximos passos
- [**Changelog**](./project/changelog.md) - Histórico de mudanças
- [**Contribuindo**](./project/contributing.md) - Como contribuir para o projeto

## 🚀 Início Rápido

### Instalação e Build
```powershell
# Clonar o repositório
git clone <repo-url>
cd testV23

# Compilar o projeto
cargo build --release

# Executar testes
cargo test

# Verificar se está funcionando
cargo run -- --help
```

### Uso Básico
```powershell
# Modo interativo (REPL)
cargo run -- --repl

# Executar um script
cargo run -- meu_script.dryad

# Com verificação strict de tipos
cargo run -- --strict meu_script.dryad
```

## 🏗️ Arquitetura do Projeto

```
Código Fonte (.dryad)
      ↓
   Lexer (Tokenização)
      ↓
   Parser (AST)
      ↓
Type Checker (Opcional)
      ↓
  Interpretador
      ↓
   Resultado
```

## 📈 Status do Projeto

- ✅ **Lexer** - Tokenização completa
- ✅ **Parser** - Análise sintática funcional
- ✅ **Interpretador** - Avaliação básica
- ✅ **Sistema de Tipos** - Verificação e inferência
- ✅ **Sistema de Erros** - Tratamento robusto
- ✅ **CLI** - Interface de linha de comando
- ✅ **REPL** - Ambiente interativo
- ✅ **Testes** - 40 testes passando
- 🔄 **I/O Avançado** - Em desenvolvimento
- 🔄 **Módulos** - Planejado
- 🔄 **Funções Definidas pelo Usuário** - Planejado

---

**Versão:** 0.1.0 | **Linguagem:** Rust | **Licença:** MIT
