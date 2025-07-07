# 👥 Documentação para Usuários

Esta seção contém guias e referências para usuários que querem aprender e usar a linguagem Dryad.

## 📑 Índice

### Primeiros Passos
- [**Guia de Início Rápido**](./quick-start.md) - Como começar a usar o Dryad
- [**Sintaxe da Linguagem**](./syntax.md) - Referência completa da sintaxe

### Ferramentas
- [**CLI Reference**](./cli-reference.md) - Comandos e opções da linha de comando
- [**REPL Guide**](./repl-guide.md) - Usando o ambiente interativo

### Desenvolvimento
- [**Criando Scripts**](./writing-scripts.md) - Como escrever programas em Dryad
- [**Exemplos**](./examples.md) - Exemplos práticos e casos de uso

### Suporte
- [**Troubleshooting**](./troubleshooting.md) - Solução de problemas comuns

## 🚀 Início Rápido

### Instalação
```powershell
# Clone o repositório
git clone <repo-url>
cd testV23

# Compile o projeto
cargo build --release

# Verifique se está funcionando
cargo run -- --help
```

### Primeiro Programa
```javascript
// Arquivo: hello.dryad
let message = "Hello, Dryad!";
print(message);
```

```powershell
# Execute o arquivo
cargo run -- hello.dryad
```

### Modo Interativo
```powershell
# Inicie o REPL
cargo run -- --repl

# Experimente alguns comandos
dryad> let x = 42
dryad> let y = 8
dryad> x + y
Number(50.0)
dryad> exit
```

## 📋 Recursos da Linguagem

### **✅ Implementado**
- **Variáveis** - `let x = 42;`
- **Operações aritméticas** - `+`, `-`, `*`, `/`
- **Operações de comparação** - `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Estruturas condicionais** - `if (condition) { ... } else { ... }`
- **Loops** - `while (condition) { ... }`, `for (init; condition; post) { ... }`
- **Funções built-in** - `print()`, `read_file()`, `write_file()`
- **Tipos básicos** - Numbers, Strings, Booleans
- **Verificação de tipos** - Modo strict opcional

### **🔄 Em Desenvolvimento**
- **Funções definidas pelo usuário**
- **Arrays e listas**
- **Módulos e imports**
- **Tratamento de exceções**

## 🎯 Casos de Uso

### **Scripts de Automação**
```javascript
// Processamento de arquivos
let content = read_file("input.txt");
let processed = content + " - Processado";
write_file("output.txt", processed);
```

### **Cálculos Matemáticos**
```javascript
// Calculadora simples
let a = 10;
let b = 5;
let resultado = a * b + (a / b);
print("Resultado: " + resultado);
```

### **Lógica de Controle**
```javascript
// Estruturas condicionais
let idade = 18;
if (idade >= 18) {
    print("Maior de idade");
} else {
    print("Menor de idade");
}
```

## 🔗 Links Úteis

- [Sintaxe Completa](./syntax.md) - Referência detalhada
- [Exemplos Práticos](./examples.md) - Código de exemplo
- [Comandos CLI](./cli-reference.md) - Referência da linha de comando
- [Guia do REPL](./repl-guide.md) - Ambiente interativo

## 💡 Dicas Rápidas

1. **Use o REPL** para experimentar código rapidamente
2. **Modo strict** (`--strict`) ajuda a encontrar erros de tipo
3. **Modo verbose** (`--verbose`) mostra informações detalhadas
4. **Semicolons são opcionais** na última expressão de um bloco
5. **Variáveis são case-sensitive** (`x` e `X` são diferentes)

---

**Próximo passo:** [Guia de Início Rápido](./quick-start.md)
