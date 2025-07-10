# Guia de Troubleshooting - Problemas Específicos e Soluções

Este documento contém soluções para problemas específicos encontrados durante o desenvolvimento e uso das bibliotecas Dryad.

## 🚨 Problemas Críticos

### 1. "Variável 'message' não definida"

**Sintoma:**
```
ERROR: [0:0] Variável 'message' não definida
```

**Causa Raiz:**
- Problema no parsing ou avaliação de parâmetros de função
- Possível conflito entre nomes de variáveis na função nativa

**Diagnóstico:**
```dryad
// Teste mínimo para identificar o problema
using IO.Console;
Console.println("teste simples");  // Se isto falha, problema é estrutural
```

**Solução:**
1. Verificar se não há conflitos de nomes em funções nativas
2. Testar com strings literais primeiro
3. Verificar se todas as chaves estão fechadas corretamente

### 2. "Função não encontrada" (com import correto)

**Sintoma:**
```
ERROR: [0:0] Função 'MinhaClasse.minhaFuncao' não encontrada
```

**Diagnóstico:**
```dryad
// Teste 1: Verificar se a função nativa existe
let resultado = native_minha_funcao("teste");  // Teste direto

// Teste 2: Verificar se o import carrega
using IO.minha_classe;  // Se isto falha, problema é no arquivo

// Teste 3: Verificar se a classe está visível
// (implementar função de debug para listar classes disponíveis)
```

**Soluções:**
1. Verificar export da classe
2. Confirmar que a função nativa está registrada no native.rs
3. Verificar se o nome da função está correto

### 3. Erro de Sintaxe em Bibliotecas

**Sintoma:**
```
ERROR: [linha:coluna] Unexpected token
```

**Diagnóstico Passo a Passo:**
```dryad
// Teste 1: Arquivo mínimo funcional
namespace IO {
    export class Teste {
        public static fn hello() {
            return "Hello";
        }
    }
}

// Teste 2: Adicionar função por função
namespace IO {
    export class Teste {
        public static fn hello() {
            return "Hello";
        }
        
        public static fn problema() {
            // Isolar esta função
            return "problema";
        }
    }
}
```

**Soluções Comuns:**
1. Verificar se `fn` está sendo usado (não `fun`)
2. Confirmar que todas as strings estão entre aspas
3. Verificar se todas as chaves e parênteses estão fechados
4. Confirmar que `public static` está sendo usado

## 🔧 Problemas de Implementação

### 1. Função Nativa não Registrada

**Sintoma:**
```
ERROR: [0:0] Função 'native_minha_funcao' não encontrada
```

**Diagnóstico:**
```rust
// Verificar no native.rs se a função está registrada
impl NativeFunctions {
    pub fn new() -> Self {
        let mut functions = Self {
            functions: HashMap::new(),
        };
        
        // Deve ter esta linha:
        functions.register("native_minha_funcao".to_string(), native_minha_funcao);
        
        functions
    }
}
```

**Solução:**
1. Adicionar registro da função em `new()`
2. Implementar a função nativa
3. Recompilar o projeto

### 2. Tipos de Argumento Incorretos

**Sintoma:**
```
ERROR: [0:0] Argumento deve ser string, recebido number
```

**Diagnóstico:**
```rust
// Implementar validação detalhada
fn native_exemplo_funcao(args: &[Value]) -> Result<Value, DryadError> {
    println!("Recebidos {} argumentos", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("Arg {}: {:?}", i, arg);
    }
    
    // Validação específica
    match args.get(0) {
        Some(Value::String(s)) => println!("String: {}", s),
        Some(Value::Number(n)) => println!("Number: {}", n),
        Some(Value::Bool(b)) => println!("Bool: {}", b),
        None => println!("Nenhum argumento"),
        _ => println!("Tipo desconhecido"),
    }
    
    Ok(Value::String("debug".to_string()))
}
```

**Soluções:**
1. Implementar validação de tipos robusta
2. Adicionar conversão automática quando possível
3. Fornecer mensagens de erro claras

### 3. Problemas de Namespace

**Sintoma:**
```
ERROR: [0:0] Namespace 'IO' não encontrado
```

**Diagnóstico:**
```dryad
// Teste 1: Verificar se o arquivo existe
// Deve estar em lib/IO/nome_arquivo.dryad

// Teste 2: Verificar estrutura do arquivo
namespace IO {  // Deve ser exatamente "IO"
    export class MinhaClasse {  // Deve ter "export"
        // ...
    }
}

// Teste 3: Verificar import
using IO.MinhaClasse;  // Deve corresponder ao nome da classe
```
