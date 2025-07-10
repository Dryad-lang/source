# 📚 Índice da Documentação - Bibliotecas Dryad

Esta pasta contém toda a documentação necessária para desenvolver e manter as bibliotecas padrão da linguagem Dryad.

## 📁 Estrutura da Documentação

### 📖 Documentos Principais

1. **[README.md](README.md)** - 📋 Documentação principal
   - Estrutura das bibliotecas IO
   - Como importar e usar
   - Exemplos práticos
   - Funções disponíveis

2. **[TECHNICAL_GUIDE.md](TECHNICAL_GUIDE.md)** - 🔧 Guia técnico detalhado
   - Implementação de funções nativas
   - Sistema de erros
   - Padrões de implementação
   - Validação e testes

3. **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)** - 🚨 Resolução de problemas
   - Problemas comuns e soluções
   - Diagnóstico de erros
   - Ferramentas de depuração
   - Checklist de resolução

4. **[TEMPLATES.md](TEMPLATES.md)** - 🏭 Templates e exemplos
   - Templates prontos para usar
   - Exemplos por categoria
   - Utilitários de desenvolvimento

5. **[TESTING_VALIDATION.md](TESTING_VALIDATION.md)** - 🧪 Testes e validação
   - Testes unitários e integração
   - Testes de performance
   - Validação de erros
   - Métricas de qualidade

6. **[PROJECT_STATUS.md](PROJECT_STATUS.md)** - 📊 Status do projeto
   - Progresso atual e métricas
   - Objetivos alcançados
   - Próximos passos
   - Lições aprendidas
   - Referência rápida

## 🎯 Como Usar Esta Documentação

### Para Iniciantes
1. Comece com **README.md** para entender o básico
2. Use **TEMPLATES.md** para implementações rápidas
3. Consulte **TROUBLESHOOTING.md** quando tiver problemas

### Para Desenvolvedores Experientes
1. Use **TECHNICAL_GUIDE.md** para implementações avançadas
2. Consulte **TEMPLATES.md** para acelerar desenvolvimento
3. Use **TROUBLESHOOTING.md** para diagnósticos complexos

### Para Manutenção
1. **TECHNICAL_GUIDE.md** - padrões e arquitetura
2. **TROUBLESHOOTING.md** - problemas conhecidos
3. **README.md** - documentação para usuários

## 🚀 Quick Start

### Implementar Nova Biblioteca

1. **Copiar template** (TEMPLATES.md):
   ```bash
   cp templates/biblioteca_template.dryad lib/IO/minha_lib.dryad
   ```

2. **Implementar funções nativas** (TECHNICAL_GUIDE.md):
   - Adicionar ao `src/interpreter/native.rs`
   - Registrar funções
   - Implementar validações

3. **Testar** (README.md):
   ```dryad
   using IO.minha_lib;
   MinhaClasse.minhaFuncao("teste");
   ```

4. **Resolver problemas** (TROUBLESHOOTING.md):
   - Verificar checklist
   - Usar ferramentas de debug
   - Aplicar soluções conhecidas

### Resolver Problema

1. **Identificar categoria** (TROUBLESHOOTING.md):
   - Função não encontrada
   - Import falha
   - Erro de execução

2. **Aplicar diagnóstico** (TROUBLESHOOTING.md):
   - Testes mínimos
   - Verificações sistemáticas
   - Logs de debug

3. **Implementar solução** (TECHNICAL_GUIDE.md):
   - Padrões corretos
   - Validações adequadas
   - Tratamento de erros

## 📊 Status Atual

### ✅ Implementado e Funcionando
- **IO.Console** - println, print, input, clear
- **IO.fs.FileSystem** - read, write, append, exists, delete
- **IO.buffer.Buffer** - create, length, toString
- **Sistema de namespaces** - importação com `using IO.module`
- **Funções nativas** - registradas e funcionais
- **Documentação** - completa e estruturada

### 🚧 Em Desenvolvimento
- Sistema de try/catch para tratamento de erros
- Validação automática de schemas
- Profiling e métricas de performance
- Templates automatizados

### 📋 Planejado
- **IO.network** - HTTP, WebSocket
- **IO.crypto** - Hash, encrypt, decrypt
- **IO.compress** - ZIP, GZIP
- **IO.image** - manipulação de imagens
- **IO.audio** - manipulação de áudio

## 🔗 Links Rápidos

### Por Tipo de Tarefa

**Implementar nova função:**
- [Padrões de implementação](TECHNICAL_GUIDE.md#padrões-de-implementação)
- [Template de função nativa](TEMPLATES.md#2-template-de-função-nativa)

**Resolver erro:**
- [Problemas críticos](TROUBLESHOOTING.md#problemas-críticos)
- [Checklist de resolução](TROUBLESHOOTING.md#checklist-de-resolução-de-problemas)

**Criar nova biblioteca:**
- [Template de biblioteca](TEMPLATES.md#1-template-de-biblioteca-dryad)
- [Como criar novos módulos](README.md#como-criar-novos-módulos)

**Testar implementação:**
- [Template de teste](TEMPLATES.md#3-template-de-teste)
- [Validação de implementação](README.md#validação-de-implementação)

### Por Categoria de Problema

**"Função não encontrada":**
- [Diagnóstico](TROUBLESHOOTING.md#2-função-não-encontrada-com-import-correto)
- [Validação](TECHNICAL_GUIDE.md#checklist-de-diagnóstico)

**"Import falha":**
- [Como importar](README.md#como-importar)
- [Problemas de import](TROUBLESHOOTING.md#problema-import-falha)

**Performance lenta:**
- [Problemas de performance](TROUBLESHOOTING.md#problemas-de-performance)
- [Benchmark](TEMPLATES.md#3-benchmark-simples)

## 📝 Convenções

### Nomenclatura
- **Arquivos:** `snake_case.dryad`
- **Classes:** `PascalCase`
- **Funções:** `camelCase`
- **Funções nativas:** `native_module_function`

### Estrutura de Arquivos
```
lib/IO/
├── module.dryad      # Biblioteca Dryad
├── README.md         # Documentação principal
├── TECHNICAL_GUIDE.md # Guia técnico
├── TROUBLESHOOTING.md # Resolução de problemas
└── TEMPLATES.md      # Templates e exemplos

src/interpreter/
├── native.rs         # Implementações nativas
├── env.rs           # Tipos de dados
└── errors.rs        # Sistema de erros

tests/
└── test_module.dryad # Testes da biblioteca
```

### Padrões de Código
- Use `public static fn` para funções de biblioteca
- Sempre valide argumentos nas funções nativas
- Implemente tratamento de erros adequado
- Documente funções com comentários claros

## 🤝 Contribuição

### Para Adicionar Nova Funcionalidade
1. Leia **TECHNICAL_GUIDE.md** - padrões
2. Use **TEMPLATES.md** - implementação
3. Teste conforme **README.md** - validação
4. Documente alterações

### Para Corrigir Bug
1. Use **TROUBLESHOOTING.md** - diagnóstico
2. Aplique **TECHNICAL_GUIDE.md** - correção
3. Teste conforme **README.md** - validação
4. Atualize documentação se necessário

### Para Melhorar Documentação
1. Identifique lacuna ou problema
2. Atualize documento relevante
3. Mantenha consistência com outros documentos
4. Teste exemplos antes de publicar

---

**🎯 Objetivo:** Facilitar desenvolvimento e manutenção das bibliotecas Dryad  
**📅 Atualizado:** 9 de julho de 2025  
**👥 Público:** Desenvolvedores da linguagem Dryad  
**📊 Status:** Base sólida estabelecida, pronta para expansão
