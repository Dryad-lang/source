# 🖥️ CLI Reference - Referência da Linha de Comando

Este documento fornece uma referência completa da interface de linha de comando do Dryad.

## 📋 Sintaxe Básica

```powershell
cargo run -- [OPTIONS] [FILE]

# Ou após build de release:
dryad [OPTIONS] [FILE]
```

## 🔧 Opções Disponíveis

### **Ajuda e Informações**

#### `--help`, `-h`
Exibe informações de ajuda sobre todas as opções disponíveis.

```powershell
cargo run -- --help
cargo run -- -h
```

**Saída:**
```
Dryad Language v0.1.0
Usage: dryad [OPTIONS] [FILE]
OPTIONS:
  -h, --help       Show this help message
  -v, --version    Show version information
  -r, --repl       Start interactive REPL
      --verbose    Enable verbose output
      --strict     Enable strict type checking
EXAMPLES:
  dryad script.dryad    Run a Dryad script
  dryad --repl          Start interactive mode
  dryad --strict app.dryad  Run with strict types
```

#### `--version`, `-v`
Exibe a versão atual do Dryad.

```powershell
cargo run -- --version
cargo run -- -v
```

**Saída:**
```
Dryad Language v0.1.0
```

### **Modos de Execução**

#### `--repl`, `-r`
Inicia o modo interativo (REPL - Read-Eval-Print Loop).

```powershell
cargo run -- --repl
cargo run -- -r
```

**Funcionalidades do REPL:**
- Execução interativa de código
- Histórico de comandos
- Comandos especiais (`help`, `exit`, `clear`, etc.)
- Ambiente persistente durante a sessão

### **Flags de Controle**

#### `--verbose`
Ativa saída detalhada durante a execução.

```powershell
cargo run -- --verbose script.dryad
```

**O que mostra:**
- Informações sobre parsing
- Detalhes de execução
- Mensagens de debug
- Warnings e informações adicionais

#### `--strict`
Ativa verificação rigorosa de tipos.

```powershell
cargo run -- --strict script.dryad
```

**Funcionalidades:**
- Verifica compatibilidade de tipos
- Detecta operações inválidas
- Reporta warnings de tipo
- Previne erros em tempo de execução

## 📁 Execução de Arquivos

### **Executar Script**
```powershell
# Executar arquivo Dryad
cargo run -- meu_programa.dryad
cargo run -- caminho/para/script.dryad

# Com caminho absoluto
cargo run -- "C:\projetos\meu_script.dryad"
```

### **Combinando Flags**
```powershell
# Strict mode + verbose
cargo run -- --strict --verbose script.dryad

# Todas as flags juntas
cargo run -- --verbose --strict meu_programa.dryad
```

## 💡 Exemplos Práticos

### **Desenvolvimento Básico**
```powershell
# Executar script simples
cargo run -- hello.dryad

# Verificar sintaxe com strict mode
cargo run -- --strict meu_script.dryad

# Debug com verbose
cargo run -- --verbose problematico.dryad
```

### **Modo Interativo**
```powershell
# REPL normal
cargo run -- --repl

# REPL com strict types
cargo run -- --repl --strict

# REPL com verbose (para desenvolvimento)
cargo run -- --repl --verbose
```

### **Debugging e Desenvolvimento**
```powershell
# Para encontrar erros de tipo
cargo run -- --strict calculadora.dryad

# Para debugging detalhado
cargo run -- --verbose --strict app.dryad

# Para experimentação rápida
cargo run -- --repl
```

## 🔍 Códigos de Saída

| Código | Significado | Quando Ocorre |
|--------|------------|---------------|
| 0 | Sucesso | Execução normal |
| 1 | Erro geral | Erro de sintaxe, runtime, etc. |

### **Exemplos:**
```powershell
# Sucesso
cargo run -- script_valido.dryad
echo $LASTEXITCODE  # 0

# Erro
cargo run -- script_com_erro.dryad
echo $LASTEXITCODE  # 1
```

## 🛠️ Casos de Uso Comuns

### **1. Desenvolvimento de Script**
```powershell
# Ciclo típico de desenvolvimento
cargo run -- --strict meu_script.dryad     # Verificar tipos
cargo run -- --verbose meu_script.dryad    # Debug se necessário
cargo run -- meu_script.dryad              # Execução final
```

### **2. Experimentação**
```powershell
# Testar ideias rapidamente
cargo run -- --repl

# Testar com tipos rigorosos
cargo run -- --repl --strict
```

### **3. Automação**
```powershell
# Scripts de automação
cargo run -- processar_dados.dryad
cargo run -- backup_script.dryad
cargo run -- relatorio_diario.dryad
```

### **4. Ensino/Aprendizado**
```powershell
# Para iniciantes
cargo run -- --repl                    # Experimentar interativamente
cargo run -- exercicio1.dryad          # Executar exercícios
cargo run -- --strict exercicio1.dryad # Verificar correção
```

## 🔧 Configurações Avançadas

### **Variáveis de Ambiente**
Atualmente não há suporte a variáveis de ambiente específicas, mas você pode usar:

```powershell
# Windows PowerShell
$env:RUST_LOG = "debug"
cargo run -- --verbose script.dryad

# Para mais detalhes de debug do Rust
$env:RUST_BACKTRACE = "1"
cargo run -- script.dryad
```

### **Redirecionamento de Saída**
```powershell
# Salvar saída em arquivo
cargo run -- script.dryad > resultado.txt

# Capturar erros também
cargo run -- script.dryad 2>&1 > log_completo.txt

# Só os erros
cargo run -- script.dryad 2> erros.txt
```

### **Execução em Lote**
```powershell
# Executar múltiplos scripts
Get-ChildItem *.dryad | ForEach-Object { 
    Write-Host "Executando $($_.Name)"
    cargo run -- $_.FullName 
}

# Com verificação de erros
foreach ($script in Get-ChildItem *.dryad) {
    Write-Host "Testando $($script.Name)"
    cargo run -- --strict $script.FullName
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Erro em $($script.Name)"
    }
}
```

## ⚡ Dicas de Performance

### **Builds Otimizados**
```powershell
# Para uso frequente, compile em modo release
cargo build --release

# Depois use diretamente (mais rápido)
.\target\release\dryad.exe script.dryad
```

### **REPL Persistente**
```powershell
# Para desenvolvimento, mantenha um REPL aberto
cargo run -- --repl

# Em vez de executar arquivos repetidamente
# Use comandos do REPL para carregar código
```

## 🐛 Troubleshooting

### **Problemas Comuns**

#### **"Comando não encontrado"**
```powershell
# Certifique-se de estar no diretório correto
cd testV23
cargo run -- --help
```

#### **"Arquivo não encontrado"**
```powershell
# Verifique o caminho do arquivo
ls *.dryad                           # Listar arquivos Dryad
cargo run -- .\meu_script.dryad     # Use .\ para caminho relativo
```

#### **Erros de Compilação**
```powershell
# Recompile se necessário
cargo clean
cargo build
```

### **Debugging**
```powershell
# Para problemas de execução
cargo run -- --verbose --strict problema.dryad

# Para problemas do próprio Dryad
$env:RUST_BACKTRACE = "full"
cargo run -- problema.dryad
```

## 📚 Referência Rápida

### **Comandos Essenciais**
```powershell
# Executar arquivo
cargo run -- arquivo.dryad

# Modo interativo
cargo run -- --repl

# Com verificação de tipos
cargo run -- --strict arquivo.dryad

# Com debug
cargo run -- --verbose arquivo.dryad

# Ajuda
cargo run -- --help
```

### **Combinações Úteis**
```powershell
# Desenvolvimento rigoroso
cargo run -- --strict --verbose

# Experimentação
cargo run -- --repl

# Produção
cargo build --release
.\target\release\dryad.exe script.dryad
```

---

Para mais informações sobre o uso da linguagem em si, consulte [Sintaxe da Linguagem](./syntax.md) e [Guia do REPL](./repl-guide.md).
