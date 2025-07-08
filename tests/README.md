# 🧪 Sistema de Testes Dryad

Este sistema de testes organizado permite desenvolvimento e validação eficiente da linguagem Dryad.

## 📁 Estrutura de Testes

```
tests/
├── develop/                    # 🔧 Testes Internos do Compilador
│   └── compiler_internal_tests.rs
├── user/                       # 👥 Testes de Funcionalidade
│   ├── test_runner.rs
│   └── scripts/
│       ├── 01_variables.dryad
│       ├── 02_arithmetic.dryad
│       ├── 03_strings.dryad
│       ├── ...
│       └── 15_this_keyword.dryad
├── environment/                # 🖥️ Testes de CLI/REPL
│   └── cli_repl_tests.rs
└── test_all.rs                # 🎯 Sistema Centralizado
```

## 🚀 Como Executar os Testes

### Todos os Testes
```bash
cargo test
```

### Por Categoria

#### 🔧 Testes Internos (Develop)
```bash
cargo test --test compiler_internal_tests
```

#### 👥 Testes de Usuário 
```bash
cargo test --test test_runner
```

#### 🖥️ Testes de Ambiente
```bash
cargo test --test cli_repl_tests
```

#### 🎯 Relatório Completo
```bash
cargo test --test test_all
```

## ✅ Como Adicionar Novos Testes

### 1. Testes Internos (Develop)
Edite `tests/develop/compiler_internal_tests.rs` e adicione:

```rust
#[test]
fn test_new_feature() {
    // Teste do componente interno
}
```

### 2. Testes de Usuário
Crie um novo arquivo `.dryad` em `tests/user/scripts/`:

```dryad
// EXPECTED: Expected Output Here
// Teste: Descrição da funcionalidade
let x = exemplo;
print(x);
```

### 3. Testes de Ambiente
Edite `tests/environment/cli_repl_tests.rs` para CLI/REPL:

```rust
#[test]
fn test_new_cli_feature() {
    // Teste de funcionalidade CLI
}
```

## 📊 Status Atual dos Testes

### ✅ Funcionando
- Variáveis e constantes
- Operações aritméticas
- Operações de string
- Lógica booleana
- Estruturas de controle
- Classes básicas

### 🟡 Parcialmente Implementado
- Funções
- POO (métodos, this)
- Sistema de tipos
- I/O de arquivos

### ❌ Não Implementado
- Herança
- Módulos/Imports
- Sistema de exceções

## 🎯 Checklist de Desenvolvimento

Sempre que implementar uma nova funcionalidade:

1. ✅ **Adicionar teste interno** em `develop/`
2. ✅ **Criar script de teste** em `user/scripts/`
3. ✅ **Testar CLI/REPL** se aplicável
4. ✅ **Executar suite completa**
5. ✅ **Atualizar documentação**

## 🔍 Debug e Troubleshooting

### Teste Individual
```bash
cargo run tests/user/scripts/01_variables.dryad
```

### Modo Verbose
```bash
cargo run -- --verbose script.dryad
```

### REPL para Debug
```bash
cargo run -- --repl
```

### Ver Erros Detalhados
```bash
cargo test -- --nocapture
```

## 📈 Métricas de Cobertura

O sistema automaticamente reporta:
- ✅ Testes passando/falhando
- 🎯 Funcionalidades implementadas
- 📊 Progresso percentual
- 🔧 Prioridades de correção

Execute `cargo test --test test_all` para ver o relatório completo!
