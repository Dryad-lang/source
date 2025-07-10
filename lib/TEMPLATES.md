# Templates e Exemplos - Desenvolvimento Rápido

Este documento contém templates prontos para usar no desenvolvimento de novas bibliotecas e funções nativas.

## 📋 Templates

### 1. Template de Biblioteca Dryad

```dryad
// lib/IO/template.dryad
// Descrição da biblioteca Template

namespace IO {
    export class Template {
        // Função principal
        public static fn operacaoPrincipal(parametro1, parametro2) {
            return native_template_operacao_principal(parametro1, parametro2);
        }
        
        // Função auxiliar
        public static fn operacaoAuxiliar(parametro) {
            return native_template_operacao_auxiliar(parametro);
        }
        
        // Verificação/validação
        public static fn isValid(valor) {
            return native_template_is_valid(valor);
        }
        
        // Aliases comuns
        public static fn op(param1, param2) {
            return Template.operacaoPrincipal(param1, param2);
        }
        
        public static fn aux(param) {
            return Template.operacaoAuxiliar(param);
        }
        
        public static fn check(valor) {
            return Template.isValid(valor);
        }
    }
}
```

### 2. Template de Função Nativa

```rust
// Adicionar ao src/interpreter/native.rs

// No método register_io_functions:
self.register("native_template_operacao_principal".to_string(), native_template_operacao_principal);
self.register("native_template_operacao_auxiliar".to_string(), native_template_operacao_auxiliar);
self.register("native_template_is_valid".to_string(), native_template_is_valid);

// Função principal com validação completa
fn native_template_operacao_principal(args: &[Value]) -> Result<Value, DryadError> {
    // 1. Validação do número de argumentos
    if args.len() < 2 {
        return Err(DryadError::new(
            "template_operacao_principal: requer pelo menos 2 argumentos".to_string(),
            None,
            ErrorSeverity::Error,
        ));
    }
    
    // 2. Extração e validação de tipos
    let parametro1 = match args.get(0) {
        Some(Value::String(s)) => s,
        Some(Value::Number(n)) => &n.to_string(),
        _ => return Err(DryadError::new(
            "template_operacao_principal: primeiro parâmetro deve ser string ou number".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    let parametro2 = match args.get(1) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Null) => "null".to_string(),
        _ => return Err(DryadError::new(
            "template_operacao_principal: segundo parâmetro deve ser string, number, boolean ou null".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    // 3. Lógica da função
    let resultado = format!("{}-{}", parametro1, parametro2);
    
    // 4. Retorno
    Ok(Value::String(resultado))
}

// Função auxiliar mais simples
fn native_template_operacao_auxiliar(args: &[Value]) -> Result<Value, DryadError> {
    let parametro = match args.get(0) {
        Some(Value::String(s)) => s,
        _ => return Err(DryadError::new(
            "template_operacao_auxiliar: parâmetro deve ser string".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    let resultado = format!("aux-{}", parametro);
    Ok(Value::String(resultado))
}

// Função de validação
fn native_template_is_valid(args: &[Value]) -> Result<Value, DryadError> {
    let valor = args.get(0);
    
    let valido = match valor {
        Some(Value::String(s)) => !s.is_empty(),
        Some(Value::Number(n)) => *n > 0.0,
        Some(Value::Bool(_)) => true,
        Some(Value::Null) => false,
        None => false,
        _ => false,
    };
    
    Ok(Value::Bool(valido))
}
```

### 3. Template de Teste

```dryad
// test_template.dryad
using IO.Console;
using IO.template;

Console.println("=== Teste Biblioteca Template ===");

// Teste 1: Operação principal - caso normal
Console.println("Teste 1: Operação principal");
let resultado1 = Template.operacaoPrincipal("teste", "valor");
Console.println("  Resultado: " + resultado1);
Console.println("  Esperado: teste-valor");

// Teste 2: Operação auxiliar
Console.println("Teste 2: Operação auxiliar");
let resultado2 = Template.operacaoAuxiliar("aux");
Console.println("  Resultado: " + resultado2);
Console.println("  Esperado: aux-aux");

// Teste 3: Validação - valores válidos
Console.println("Teste 3: Validação (válidos)");
let valido1 = Template.isValid("texto");
let valido2 = Template.isValid(42);
let valido3 = Template.isValid(true);
Console.println("  String válida: " + valido1);
Console.println("  Number válido: " + valido2);
Console.println("  Bool válido: " + valido3);

// Teste 4: Validação - valores inválidos
Console.println("Teste 4: Validação (inválidos)");
let invalido1 = Template.isValid("");
let invalido2 = Template.isValid(0);
let invalido3 = Template.isValid(null);
Console.println("  String vazia: " + invalido1);
Console.println("  Zero: " + invalido2);
Console.println("  Null: " + invalido3);

// Teste 5: Aliases
Console.println("Teste 5: Aliases");
let alias1 = Template.op("a", "b");
let alias2 = Template.aux("c");
let alias3 = Template.check("texto");
Console.println("  Alias op: " + alias1);
Console.println("  Alias aux: " + alias2);
Console.println("  Alias check: " + alias3);

Console.println("=== Todos os testes concluídos ===");
```

---

## 🏭 Exemplos por Categoria

### 1. Manipulação de Strings

```rust
// String utilities
fn native_string_util_example(args: &[Value]) -> Result<Value, DryadError> {
    let texto = match args.get(0) {
        Some(Value::String(s)) => s,
        _ => return Err(DryadError::new(
            "string_util: primeiro argumento deve ser string".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    let operacao = args.get(1)
        .and_then(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("upper");
    
    let resultado = match operacao {
        "upper" => texto.to_uppercase(),
        "lower" => texto.to_lowercase(),
        "reverse" => texto.chars().rev().collect(),
        "length" => return Ok(Value::Number(texto.len() as f64)),
        _ => return Err(DryadError::new(
            "string_util: operação deve ser 'upper', 'lower', 'reverse' ou 'length'".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    Ok(Value::String(resultado))
}
```

### 2. Operações Matemáticas

```rust
// Math utilities
fn native_math_util_example(args: &[Value]) -> Result<Value, DryadError> {
    let num1 = match args.get(0) {
        Some(Value::Number(n)) => *n,
        _ => return Err(DryadError::new(
            "math_util: primeiro argumento deve ser number".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    let num2 = match args.get(1) {
        Some(Value::Number(n)) => *n,
        _ => return Err(DryadError::new(
            "math_util: segundo argumento deve ser number".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    let operacao = args.get(2)
        .and_then(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("add");
    
    let resultado = match operacao {
        "add" => num1 + num2,
        "sub" => num1 - num2,
        "mul" => num1 * num2,
        "div" => {
            if num2 == 0.0 {
                return Err(DryadError::new(
                    "math_util: divisão por zero".to_string(),
                    None,
                    ErrorSeverity::Error,
                ));
            }
            num1 / num2
        },
        "pow" => num1.powf(num2),
        "mod" => num1 % num2,
        _ => return Err(DryadError::new(
            "math_util: operação deve ser 'add', 'sub', 'mul', 'div', 'pow' ou 'mod'".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    Ok(Value::Number(resultado))
}
```

### 3. Manipulação de Arrays

```rust
// Array utilities
fn native_array_util_example(args: &[Value]) -> Result<Value, DryadError> {
    let array = match args.get(0) {
        Some(Value::Array(arr)) => arr,
        _ => return Err(DryadError::new(
            "array_util: primeiro argumento deve ser array".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    let operacao = args.get(1)
        .and_then(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("length");
    
    match operacao {
        "length" => Ok(Value::Number(array.len() as f64)),
        "first" => Ok(array.first().cloned().unwrap_or(Value::Null)),
        "last" => Ok(array.last().cloned().unwrap_or(Value::Null)),
        "reverse" => {
            let mut reversed = array.clone();
            reversed.reverse();
            Ok(Value::Array(reversed))
        },
        "sort" => {
            let mut sorted = array.clone();
            sorted.sort_by(|a, b| {
                match (a, b) {
                    (Value::Number(n1), Value::Number(n2)) => n1.partial_cmp(n2).unwrap(),
                    (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                    _ => std::cmp::Ordering::Equal,
                }
            });
            Ok(Value::Array(sorted))
        },
        _ => Err(DryadError::new(
            "array_util: operação deve ser 'length', 'first', 'last', 'reverse' ou 'sort'".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    }
}
```

### 4. Operações de Rede (Futuro)

```rust
// Network utilities (exemplo para quando implementar)
fn native_http_get_example(args: &[Value]) -> Result<Value, DryadError> {
    let url = match args.get(0) {
        Some(Value::String(s)) => s,
        _ => return Err(DryadError::new(
            "http_get: URL deve ser string".to_string(),
            None,
            ErrorSeverity::Error,
        )),
    };
    
    // Validar URL
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(DryadError::new(
            "http_get: URL deve começar com http:// ou https://".to_string(),
            None,
            ErrorSeverity::Error,
        ));
    }
    
    // TODO: Implementar requisição HTTP real
    // Por enquanto, retornar simulação
    let response = format!("Resposta simulada para {}", url);
    Ok(Value::String(response))
}
```

---

## 🛠️ Utilitários de Desenvolvimento

### 1. Gerador de Template

```bash
#!/bin/bash
# generate_template.sh

MODULE_NAME=$1
CLASS_NAME=$2

if [ -z "$MODULE_NAME" ] || [ -z "$CLASS_NAME" ]; then
    echo "Uso: ./generate_template.sh module_name ClassName"
    exit 1
fi

# Criar arquivo da biblioteca
cat > "lib/IO/${MODULE_NAME}.dryad" << EOF
// lib/IO/${MODULE_NAME}.dryad
// Descrição da biblioteca ${CLASS_NAME}

namespace IO {
    export class ${CLASS_NAME} {
        public static fn operacaoPrincipal(parametro) {
            return native_${MODULE_NAME}_operacao_principal(parametro);
        }
        
        public static fn isValid(valor) {
            return native_${MODULE_NAME}_is_valid(valor);
        }
    }
}
EOF

# Criar arquivo de teste
cat > "test_${MODULE_NAME}.dryad" << EOF
// test_${MODULE_NAME}.dryad
using IO.Console;
using IO.${MODULE_NAME};

Console.println("=== Teste ${CLASS_NAME} ===");

let resultado = ${CLASS_NAME}.operacaoPrincipal("teste");
Console.println("Resultado: " + resultado);

let valido = ${CLASS_NAME}.isValid("teste");
Console.println("Válido: " + valido);

Console.println("=== Teste concluído ===");
EOF

echo "Template criado para ${MODULE_NAME}:"
echo "  - lib/IO/${MODULE_NAME}.dryad"
echo "  - test_${MODULE_NAME}.dryad"
echo ""
echo "Próximos passos:"
echo "1. Implementar funções nativas em src/interpreter/native.rs"
echo "2. Registrar funções em register_io_functions"
echo "3. Compilar: cargo build"
echo "4. Testar: ./target/debug/dryad.exe test_${MODULE_NAME}.dryad"
```

### 2. Verificador de Implementação

```dryad
// check_implementation.dryad
using IO.Console;

// Lista de módulos para verificar
let modulos = ["console", "fs", "buffer"];

Console.println("=== Verificação de Implementação ===");

// TODO: Implementar quando tivermos arrays funcionais
// for (let modulo in modulos) {
//     Console.println("Verificando " + modulo + "...");
//     // Tentar importar
//     // Tentar usar função básica
//     // Reportar status
// }

Console.println("Verificação manual necessária por enquanto");
```

### 3. Benchmark Simples

```rust
// Adicionar ao native.rs para benchmarking
fn native_benchmark_function(args: &[Value]) -> Result<Value, DryadError> {
    let iterations = match args.get(0) {
        Some(Value::Number(n)) => *n as usize,
        _ => 1000,
    };
    
    let start = std::time::Instant::now();
    
    // Operação a ser medida
    for _ in 0..iterations {
        // Simular trabalho
        let _ = format!("iteração {}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos());
    }
    
    let duration = start.elapsed();
    let per_iteration = duration.as_nanos() / iterations as u128;
    
    let resultado = format!(
        "Benchmark: {} iterações em {:?} ({} ns/iter)",
        iterations, duration, per_iteration
    );
    
    Ok(Value::String(resultado))
}
```

---

## 📚 Referência Rápida

### Tipos de Value Suportados

```rust
Value::Null                           // null
Value::Bool(true)                     // boolean
Value::Number(42.0)                   // number (sempre f64)
Value::String("texto".to_string())    // string
Value::Array(vec![Value::Number(1.0)]) // array
Value::Object(hashmap)                // object/map
```

### Padrões de Erro Comuns

```rust
// Argumento faltando
Err(DryadError::new(
    "função: requer X argumentos".to_string(),
    None, ErrorSeverity::Error,
))

// Tipo incorreto
Err(DryadError::new(
    "função: argumento deve ser string/number/boolean/array/object".to_string(),
    None, ErrorSeverity::Error,
))

// Valor inválido
Err(DryadError::new(
    "função: valor deve ser positivo/não-vazio/válido".to_string(),
    None, ErrorSeverity::Error,
))

// Erro de sistema
Err(DryadError::new(
    format!("função: erro do sistema - {}", system_error),
    None, ErrorSeverity::Error,
))
```

### Comandos de Build e Teste

```bash
# Compilar
cargo build

# Compilar com optimizações
cargo build --release

# Executar teste
./target/debug/dryad.exe test_arquivo.dryad

# Executar com debug
RUST_BACKTRACE=1 ./target/debug/dryad.exe arquivo.dryad

# Verificar sintaxe
./target/debug/dryad.exe --check arquivo.dryad
```

---

**📝 Nota:** Estes templates são pontos de partida. Adapte conforme necessário para cada caso específico.

**🔄 Última atualização:** 9 de julho de 2025  
**🎯 Foco:** Aceleração do desenvolvimento com templates prontos  
**📋 Status:** Templates ativos e testados
