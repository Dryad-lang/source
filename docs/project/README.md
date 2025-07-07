# 📊 Status e Roadmap do Projeto

Esta seção contém informações sobre o estado atual do Dryad e planos para o futuro.

## 📑 Índice

### Status Atual
- [**Status Atual**](./current-status.md) - O que já está implementado
- [**Changelog**](./changelog.md) - Histórico de mudanças

### Planejamento
- [**Roadmap**](./roadmap.md) - Planos futuros e próximos passos
- [**Contribuindo**](./contributing.md) - Como contribuir para o projeto

## 📈 Visão Geral do Projeto

### **Missão**
Criar uma linguagem de programação interpretada moderna, focada em simplicidade, expressividade e facilidade de uso.

### **Objetivos**
- ✅ **Sintaxe familiar** - Similar ao JavaScript
- ✅ **Tipagem dinâmica** - Com verificação opcional strict
- ✅ **Ambiente interativo** - REPL funcional
- ✅ **Facilidade de uso** - CLI intuitivo
- 🔄 **Modularidade** - Sistema de módulos
- 🔄 **Extensibilidade** - Fácil adição de funcionalidades

## 🎯 Status de Desenvolvimento

### **Núcleo da Linguagem**
| Componente | Status | Completude |
|------------|--------|------------|
| Lexer | ✅ Completo | 95% |
| Parser | ✅ Funcional | 85% |
| AST | ✅ Implementado | 90% |
| Interpretador | ✅ Básico | 75% |
| Sistema de Tipos | ✅ Implementado | 80% |
| Sistema de Erros | ✅ Robusto | 90% |

### **Ferramentas**
| Ferramenta | Status | Completude |
|------------|--------|------------|
| CLI | ✅ Funcional | 85% |
| REPL | ✅ Completo | 95% |
| I/O de Arquivos | ✅ Básico | 70% |
| Testes | ✅ Abrangentes | 90% |

### **Funcionalidades da Linguagem**
| Funcionalidade | Status | Notas |
|---------------|--------|-------|
| Variáveis | ✅ | `let x = valor;` |
| Operadores Aritméticos | ✅ | `+`, `-`, `*`, `/` |
| Operadores de Comparação | ✅ | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Condicionais | ✅ | `if/else` |
| Loops | ✅ | `while`, `for` |
| Funções Built-in | ✅ | `print`, `read_file`, `write_file` |
| Blocos | ✅ | `{ ... }` |
| Strings | ✅ | Básico |
| Números | ✅ | Float64 |
| Booleans | ✅ | `true`, `false` |
| Funções Definidas pelo Usuário | 🔄 | Planejado |
| Arrays | 🔄 | Planejado |
| Módulos | 🔄 | Planejado |
| Reatribuição | 🔄 | Planejado |

## 📊 Métricas do Projeto

### **Código**
- **Linhas de Código:** ~3.000 linhas (Rust)
- **Módulos:** 8 principais
- **Testes:** 40 testes (100% passando)
- **Cobertura:** ~85% do código

### **Funcionalidades**
- **Tokens Suportados:** 20+ tipos
- **Operadores:** 12 operadores
- **Palavras-chave:** 9 palavras-chave
- **Tipos de Dados:** 4 tipos básicos
- **Comandos CLI:** 5 flags principais

### **Performance**
- **Compile Time:** ~2s (debug), ~5s (release)
- **Startup Time:** ~50ms
- **Memory Usage:** ~5MB base
- **Test Suite:** ~3s para todos os testes

## 🏆 Marcos Alcançados

### **v0.1.0 - MVP Funcional** ✅
- [x] Lexer completo
- [x] Parser básico
- [x] Interpretador funcional
- [x] CLI e REPL
- [x] Sistema de erros
- [x] Testes básicos

### **v0.1.1 - Refinamentos** ✅
- [x] Sistema de tipos robusto
- [x] Melhor tratamento de erros
- [x] REPL aprimorado
- [x] Testes abrangentes
- [x] Documentação inicial

## 🎯 Próximos Marcos

### **v0.2.0 - Funções** 🔄
- [ ] Declaração de funções
- [ ] Chamadas de função
- [ ] Parâmetros e argumentos
- [ ] Escopo local
- [ ] Return statements

### **v0.3.0 - Estruturas de Dados** 📋
- [ ] Arrays/Listas
- [ ] Indexação
- [ ] Métodos de array
- [ ] Iteração sobre arrays

### **v0.4.0 - Módulos** 📦
- [ ] Sistema de imports
- [ ] Exports
- [ ] Módulos da biblioteca padrão
- [ ] Gerenciamento de dependências

## 📈 Gráfico de Progresso

```
Funcionalidades Implementadas por Versão:

v0.1.0 |████████████████████           | 60%
v0.1.1 |██████████████████████████████ | 85%
v0.2.0 |████████████████████████████   | 80% (planejado)
v0.3.0 |██████████████████████████     | 75% (planejado)
v0.4.0 |████████████████████████       | 70% (planejado)
```

## 🔗 Links Úteis

- [Status Detalhado](./current-status.md) - Informações técnicas completas
- [Roadmap](./roadmap.md) - Planos de desenvolvimento
- [Changelog](./changelog.md) - Histórico de mudanças
- [Como Contribuir](./contributing.md) - Guia para desenvolvedores

## 📧 Contato e Comunidade

- **Issues:** Use GitHub Issues para reportar bugs
- **Discussões:** GitHub Discussions para ideias
- **Documentação:** Contribuições bem-vindas
- **Código:** Pull Requests seguindo as diretrizes

---

**Última atualização:** Janeiro 2025  
**Versão atual:** 0.1.1  
**Status:** Em desenvolvimento ativo
