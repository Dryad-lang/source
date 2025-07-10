# Documentação Técnica - Implementação e Resolução de Problemas

Esta documentação técnica detalha como implementar funções nativas, diagnosticar erros e resolver problemas comuns no desenvolvimento das bibliotecas Dryad.

## 📋 Índice

1. [Implementação de Funções Nativas](#implementação-de-funções-nativas)
2. [Sistema de Erros](#sistema-de-erros)
3. [Diagnóstico de Problemas](#diagnóstico-de-problemas)
4. [Depuração Avançada](#depuração-avançada)
5. [Padrões de Implementação](#padrões-de-implementação)
6. [Validação e Testes](#validação-e-testes)

---

## 🔧 Implementação de Funções Nativas

### Anatomia de uma Função Nativa

```rust
fn native_exemplo_funcao(args: &[Value]) -> Result<Value, DryadError> {
    // 1. Validação de argumentos
    if args.len() != 2 {
        return Err(DryadError::new(
            "exemplo_funcao: requer exatamente 2 argumentos".to_string(),
            None,
            ErrorSeverity::Error,
        ));
    }

    // 2. Extração e validação de tipos
    let (primeiro, segundo) = match (args.get(0), args.get(1)) {
        (Some(Value::String(s)), Some(Value::Number(n))) => (s, n),
        _ => return Err(DryadError::new(
            "exemplo_funcao: argumentos devem ser (string, number)".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };

    // 3. Lógica da função
    let resultado = format!("{}: {}", primeiro, n);

    // 4. Retorno
    Ok(Value::String(resultado))
}
```

### Registro de Funções

No arquivo `src/interpreter/native.rs`, adicione ao método `register_io_functions`:

```rust
fn register_io_functions(&mut self) {
    // ...existing code...
    
    // Nova função
    self.register("native_exemplo_funcao".to_string(), native_exemplo_funcao);
}
```

### Tipos de Retorno Suportados

```rust
// Valores básicos
Ok(Value::Null)                    // null
Ok(Value::Bool(true))               // boolean
Ok(Value::Number(42.0))             // number
Ok(Value::String("texto".to_string())) // string

// Coleções
Ok(Value::Array(vec![Value::String("item1".to_string())]))
Ok(Value::Object(hashmap))

// Erros
Err(DryadError::new(
    "Mensagem de erro".to_string(),
    None,
    ErrorSeverity::Error,
))
```

---

## ⚠️ Sistema de Erros

### Tipos de Severidade

```rust
pub enum ErrorSeverity {
    Warning,    // Avisos - não interrompem execução
    Error,      // Erros - interrompem execução
    Fatal,      // Erros fatais - terminam programa
}
```

### Criação de Erros

```rust
// Erro simples
DryadError::new(
    "Mensagem do erro".to_string(),
    None,
    ErrorSeverity::Error,
)

// Erro com posição (futuro)
DryadError::new(
    "Mensagem do erro".to_string(),
    Some(Position { line: 10, column: 5 }),
    ErrorSeverity::Error,
)
```

### Padrões de Mensagens de Erro

```rust
// ✅ Bom: Específico e informativo
"console_print: argumento deve ser uma string"
"fs_read_file: arquivo 'dados.txt' não encontrado"
"buffer_create: tamanho deve ser um número positivo"

// ❌ Ruim: Vago e não informativo
"erro"
"argumento inválido"
"falha na operação"
```

### Tratamento de Erros por Categoria

#### Erros de Validação de Argumentos
```rust
// Número incorreto de argumentos
if args.len() != expected_count {
    return Err(DryadError::new(
        format!("{}: requer {} argumentos, recebeu {}", 
                function_name, expected_count, args.len()),
        None,
        ErrorSeverity::Error,
    ));
}

// Tipo incorreto
if let Some(Value::String(s)) = args.get(0) {
    // OK
} else {
    return Err(DryadError::new(
        format!("{}: primeiro argumento deve ser uma string", function_name),
        None,
        ErrorSeverity::Error,
    ));
}
```

#### Erros de Sistema
```rust
// Operações de arquivo
match fs::read_to_string(filename) {
    Ok(content) => Ok(Value::String(content)),
    Err(e) => Err(DryadError::new(
        format!("Erro ao ler arquivo '{}': {}", filename, e),
        None,
        ErrorSeverity::Error,
    )),
}

// Operações de rede (futuro)
match reqwest::get(url).await {
    Ok(response) => { /* processar */ },
    Err(e) => Err(DryadError::new(
        format!("Erro de rede: {}", e),
        None,
        ErrorSeverity::Error,
    )),
}
```

---

## 🔍 Diagnóstico de Problemas

### Checklist de Diagnóstico

Quando uma função não funciona, verifique na ordem:

#### 1. Função Nativa Registrada?
```bash
# Buscar no código se a função está registrada
grep -n "native_minha_funcao" src/interpreter/native.rs
```

#### 2. Sintaxe da Biblioteca Correta?
```dryad
// ✅ Correto
public static fn minhaFuncao(param) {
    return native_minha_funcao(param);
}

// ❌ Incorreto
public static fun minhaFuncao(param) {  // 'fun' em vez de 'fn'
    return native_minha_funcao(param);
}
```

#### 3. Namespace e Export Corretos?
```dryad
// ✅ Correto
namespace IO {
    export class MinhaClasse {
        public static fn minhaFuncao() { ... }
    }
}

// ❌ Incorreto - faltando export
namespace IO {
    class MinhaClasse {  // sem 'export'
        public static fn minhaFuncao() { ... }
    }
}
```

#### 4. Import Correto?
```dryad
// ✅ Correto
using IO.minha_classe;  // nome do arquivo

// ❌ Incorreto
using IO.MinhaClasse;   // nome da classe
```

### Testes de Diagnóstico

#### Teste 1: Função Nativa Direta
```dryad
// Testar se a função nativa existe
let resultado = native_minha_funcao("teste");
```

#### Teste 2: Import da Biblioteca
```dryad
// Testar se o import funciona
using IO.minha_classe;
// Se não der erro, o import está OK
```

#### Teste 3: Acesso à Classe
```dryad
using IO.minha_classe;

// Testar se a classe está disponível
MinhaClasse.minhaFuncao("teste");
```

---

## 🐛 Depuração Avançada

### Compilação com Debug

```bash
# Compilar em modo debug
cargo build

# Executar com informações de debug
RUST_BACKTRACE=1 ./target/debug/dryad.exe arquivo.dryad
```

### Logs de Depuração

Adicione logs temporários nas funções nativas:

```rust
fn native_minha_funcao(args: &[Value]) -> Result<Value, DryadError> {
    eprintln!("DEBUG: native_minha_funcao chamada com {} argumentos", args.len());
    
    for (i, arg) in args.iter().enumerate() {
        eprintln!("DEBUG: arg[{}] = {:?}", i, arg);
    }
    
    // ... resto da função
}
```

### Verificação de Módulos Carregados

```dryad
// Verificar se módulo foi carregado (futuro)
using Core.Debug;

let modulos = Debug.getLoadedModules();
Console.println("Módulos carregados: " + modulos);
```

### Testes Incrementais

```dryad
// test_incremental.dryad
using IO.Console;

Console.println("Passo 1: Import OK");

// Adicionar cada passo gradualmente
using IO.minha_classe;
Console.println("Passo 2: Import classe OK");

MinhaClasse.minhaFuncao("teste");
Console.println("Passo 3: Função OK");
```

---

## 📐 Padrões de Implementação

### Estrutura Padrão de Biblioteca

```dryad
// lib/IO/exemplo.dryad
// Descrição da biblioteca

namespace IO {
    export class Exemplo {
        // Função principal
        public static fn operacaoPrincipal(param1, param2) {
            return native_exemplo_operacao_principal(param1, param2);
        }
        
        // Função auxiliar
        public static fn operacaoAuxiliar(param) {
            return native_exemplo_operacao_auxiliar(param);
        }
        
        // Aliases comuns
        public static fn alias(param) {
            return Exemplo.operacaoPrincipal(param, "default");
        }
    }
}
```

### Padrão de Implementação Nativa

```rust
// Função principal
fn native_exemplo_operacao_principal(args: &[Value]) -> Result<Value, DryadError> {
    // Validação de argumentos
    if args.len() < 1 {
        return Err(DryadError::new(
            "exemplo_operacao_principal: requer pelo menos 1 argumento".to_string(),
            None,
            ErrorSeverity::Error,
        ));
    }
    
    // Extração de argumentos
    let param1 = match args.get(0) {
        Some(Value::String(s)) => s,
        _ => return Err(DryadError::new(
            "exemplo_operacao_principal: primeiro argumento deve ser string".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    let param2 = args.get(1)
        .and_then(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("default");
    
    // Lógica da função
    let resultado = format!("{}-{}", param1, param2);
    
    // Retorno
    Ok(Value::String(resultado))
}

// Função auxiliar
fn native_exemplo_operacao_auxiliar(args: &[Value]) -> Result<Value, DryadError> {
    // Implementação mais simples
    if let Some(Value::String(param)) = args.get(0) {
        Ok(Value::String(format!("aux-{}", param)))
    } else {
        Err(DryadError::new(
            "exemplo_operacao_auxiliar: argumento deve ser string".to_string(),
            None,
            ErrorSeverity::Error,
        ))
    }
}
```

### Padrão de Testes

```dryad
// test_exemplo.dryad
using IO.Console;
using IO.exemplo;

Console.println("=== Teste Biblioteca Exemplo ===");

// Teste 1: Função principal
let resultado1 = Exemplo.operacaoPrincipal("teste", "valor");
Console.println("Teste 1: " + resultado1);

// Teste 2: Função auxiliar
let resultado2 = Exemplo.operacaoAuxiliar("teste");
Console.println("Teste 2: " + resultado2);

// Teste 3: Alias
let resultado3 = Exemplo.alias("teste");
Console.println("Teste 3: " + resultado3);

// Teste 4: Tratamento de erro
try {
    Exemplo.operacaoPrincipal();  // Sem argumentos
} catch (e) {
    Console.println("Teste 4: Erro capturado - " + e);
}

Console.println("=== Todos os testes concluídos ===");
```

---

## ✅ Validação e Testes

### Lista de Verificação Pré-Deploy

- [ ] Função nativa implementada e registrada
- [ ] Validação de argumentos implementada
- [ ] Mensagens de erro informativas
- [ ] Biblioteca Dryad criada com namespace correto
- [ ] Classe exportada corretamente
- [ ] Sintaxe usando `fn` (não `fun`)
- [ ] Teste básico criado e funcionando
- [ ] Teste de erro criado e funcionando
- [ ] Documentação atualizada

### Testes Automatizados (Futuro)

```dryad
// tests/test_exemplo.dryad
using Test.Framework;
using IO.exemplo;

Test.describe("Biblioteca Exemplo", fn() {
    Test.it("deve executar operação principal", fn() {
        let resultado = Exemplo.operacaoPrincipal("a", "b");
        Test.expect(resultado).toBe("a-b");
    });
    
    Test.it("deve tratar erro de argumento", fn() {
        Test.expectError(fn() {
            Exemplo.operacaoPrincipal();
        });
    });
});
```

### Métricas de Qualidade

```rust
// Implementar métricas nas funções nativas
fn native_exemplo_com_metricas(args: &[Value]) -> Result<Value, DryadError> {
    let start_time = std::time::Instant::now();
    
    // ... implementação
    
    let duration = start_time.elapsed();
    if duration.as_millis() > 100 {
        eprintln!("WARNING: função exemplo_com_metricas levou {:?}", duration);
    }
    
    Ok(resultado)
}
```

---

## 🚨 Problemas Comuns e Soluções

### Problema: "Função não encontrada"

**Causa:** Função nativa não registrada  
**Solução:**
```rust
// Adicionar em register_io_functions
self.register("native_minha_funcao".to_string(), native_minha_funcao);
```

### Problema: "Classe não encontrada"

**Causa:** Export ou namespace incorreto  
**Solução:**
```dryad
namespace IO {
    export class MinhaClasse {  // 'export' é obrigatório
        // ...
    }
}
```

### Problema: "Import falha"

**Causa:** Nome do arquivo incorreto  
**Solução:**
```dryad
using IO.nome_do_arquivo;  // deve ser o nome exato do arquivo .dryad
```

### Problema: "Variável não definida"

**Causa:** Sintaxe incorreta ou problema de parsing  
**Solução:**
- Verificar uso de `fn` em vez de `fun`
- Verificar fechamento de chaves
- Verificar declaração de variáveis

### Problema: Função executa mas retorna erro

**Causa:** Validação de argumentos ou lógica incorreta  
**Solução:**
- Adicionar logs de debug
- Testar função nativa diretamente
- Verificar tipos de argumentos

---

## 📚 Recursos Adicionais

### Comandos Úteis

```bash
# Buscar função nativa
grep -rn "native_exemplo" src/

# Verificar registros de função
grep -A 5 -B 5 "register.*exemplo" src/interpreter/native.rs

# Compilar com warnings detalhados
cargo build --verbose

# Verificar sintaxe Dryad
./target/debug/dryad.exe --check arquivo.dryad
```

### Estrutura de Arquivos

```
src/interpreter/
├── native.rs          # Implementações de funções nativas
├── env.rs             # Definições de tipos Value
├── errors.rs          # Sistema de erros
└── evaluator.rs       # Avaliador principal

lib/IO/
├── console.dryad      # Biblioteca de console
├── fs.dryad           # Biblioteca de sistema de arquivos
├── buffer.dryad       # Biblioteca de buffer
└── novo_modulo.dryad  # Novo módulo seguindo padrões
```

---

**📝 Notas:**
- Esta documentação deve ser atualizada sempre que novos padrões forem estabelecidos
- Exemplos devem ser testados antes de serem incluídos
- Mantenha as mensagens de erro em português para consistência

**🔄 Última atualização:** 9 de julho de 2025  
**👥 Contribuidores:** Equipe Dryad  
**📋 Status:** Documentação base estabelecida
