# 🚀 Guia de Início Rápido

Este guia o ajudará a começar a usar o Dryad em poucos minutos.

## 📋 Pré-requisitos

- **Rust 1.88+** instalado no sistema
- **Git** para clonar o repositório
- **Terminal/PowerShell** para executar comandos

### Verificando Pré-requisitos
```powershell
# Verificar se o Rust está instalado
rustc --version
cargo --version

# Se não estiver instalado, visite: https://rustup.rs/
```

## 🔧 Instalação

### 1. Clone o Repositório
```powershell
git clone <repository-url>
cd testV23
```

### 2. Compile o Projeto
```powershell
# Compilação de desenvolvimento (mais rápida)
cargo build

# Compilação otimizada (mais lenta, mas melhor performance)
cargo build --release
```

### 3. Verifique a Instalação
```powershell
# Mostrar ajuda
cargo run -- --help

# Mostrar versão
cargo run -- --version

# Resultado esperado:
# Dryad Language v0.1.0
```

## 📝 Primeiro Programa

### 1. Crie um Arquivo
Crie um arquivo chamado `primeiro.dryad`:

```javascript
// primeiro.dryad
let mensagem = "Olá, Dryad!";
print(mensagem);

let numero = 42;
let resultado = numero * 2;
print("O dobro de " + numero + " é " + resultado);
```

### 2. Execute o Arquivo
```powershell
cargo run -- primeiro.dryad
```

**Saída esperada:**
```
Olá, Dryad!
O dobro de 42 é 84
```

## 🖥️ Modo Interativo (REPL)

O REPL (Read-Eval-Print Loop) permite experimentar código interativamente.

### Iniciando o REPL
```powershell
cargo run -- --repl
```

### Comandos Básicos do REPL
```
dryad> let x = 10
Number(10.0)

dryad> let y = 5
Number(5.0)

dryad> x + y
Number(15.0)

dryad> x > y
Bool(true)

dryad> help
[mostra comandos disponíveis]

dryad> exit
Goodbye!
```

### Comandos Especiais do REPL
- `help` - Mostra comandos disponíveis
- `exit` ou `quit` - Sair do REPL
- `clear` - Limpar variáveis do ambiente
- `history` - Mostrar histórico de comandos
- `env` - Mostrar variáveis atuais
- `type <variável>` - Mostrar tipo de uma variável

## 📚 Conceitos Básicos

### Variáveis
```javascript
let nome = "João";
let idade = 25;
let ativo = true;
```

### Operações Aritméticas
```javascript
let a = 10;
let b = 3;

let soma = a + b;        // 13
let subtracao = a - b;   // 7
let multiplicacao = a * b; // 30
let divisao = a / b;     // 3.333...
```

### Operações de Comparação
```javascript
let x = 10;
let y = 5;

x == y;  // false
x != y;  // true
x > y;   // true
x < y;   // false
x >= y;  // true
x <= y;  // false
```

### Estruturas Condicionais
```javascript
let idade = 18;

if (idade >= 18) {
    print("Maior de idade");
} else {
    print("Menor de idade");
}
```

### Loops
```javascript
// Loop while
let contador = 0;
while (contador < 5) {
    print("Contador: " + contador);
    contador = contador + 1;
}

// Loop for
for (let i = 0; i < 3; i = i + 1) {
    print("Iteração: " + i);
}
```

## 🔧 Opções de Linha de Comando

### Flags Disponíveis
```powershell
# Executar arquivo
cargo run -- script.dryad

# Modo interativo
cargo run -- --repl

# Modo strict (verificação rigorosa de tipos)
cargo run -- --strict script.dryad

# Modo verbose (saída detalhada)
cargo run -- --verbose script.dryad

# Combinar flags
cargo run -- --strict --verbose script.dryad
```

### Exemplos de Uso
```powershell
# Desenvolvimento normal
cargo run -- meu_programa.dryad

# Com verificação de tipos
cargo run -- --strict meu_programa.dryad

# Para debugging
cargo run -- --verbose --strict meu_programa.dryad

# Modo interativo com strict types
cargo run -- --repl --strict
```

## 📁 Operações de Arquivo

### Lendo Arquivos
```javascript
// Ler conteúdo de um arquivo
let conteudo = read_file("dados.txt");
print(conteudo);
```

### Escrevendo Arquivos
```javascript
// Escrever em um arquivo
let dados = "Dados importantes";
write_file("saida.txt", dados);

// Adicionar ao arquivo
append_file("log.txt", "Nova entrada\n");
```

## 🐛 Debugging e Resolução de Problemas

### Verificar Sintaxe
```powershell
# Se houver erro de sintaxe, o Dryad mostrará uma mensagem
cargo run -- arquivo_com_erro.dryad
```

### Verificar Tipos
```powershell
# Use --strict para verificação rigorosa de tipos
cargo run -- --strict meu_script.dryad
```

### Modo Verbose
```powershell
# Para mais informações sobre a execução
cargo run -- --verbose meu_script.dryad
```

### Problemas Comuns

#### **Erro: "comando não encontrado"**
```powershell
# Certifique-se de estar no diretório correto
cd testV23

# Verifique se o projeto foi compilado
cargo build
```

#### **Erro de sintaxe**
```javascript
// ❌ Incorreto - falta ponto e vírgula
let x = 10

// ✅ Correto
let x = 10;
```

#### **Erro de tipo**
```javascript
// ❌ Pode causar erro em modo strict
let x = 10;
let y = "hello";
let resultado = x + y;  // Operação entre Number e String

// ✅ Correto
let x = 10;
let y = 5;
let resultado = x + y;  // Ambos são Numbers
```

## 📋 Exemplos Práticos

### Calculadora Simples
```javascript
// calculadora.dryad
let a = 15;
let b = 4;

print("Números: " + a + " e " + b);
print("Soma: " + (a + b));
print("Subtração: " + (a - b));
print("Multiplicação: " + (a * b));
print("Divisão: " + (a / b));
```

### Verificador de Números
```javascript
// verificador.dryad
let numero = 17;

if (numero > 10) {
    print(numero + " é maior que 10");
} else {
    print(numero + " é menor ou igual a 10");
}

if (numero == 17) {
    print("O número é exatamente 17!");
}
```

### Contador com Loop
```javascript
// contador.dryad
print("Contagem regressiva:");

let i = 5;
while (i > 0) {
    print("T-" + i);
    i = i - 1;
}

print("Lançamento!");
```

## ➡️ Próximos Passos

1. **Explore a sintaxe completa:** [Sintaxe da Linguagem](./syntax.md)
2. **Aprenda sobre o CLI:** [CLI Reference](./cli-reference.md)
3. **Domine o REPL:** [REPL Guide](./repl-guide.md)
4. **Veja mais exemplos:** [Exemplos](./examples.md)
5. **Resolva problemas:** [Troubleshooting](./troubleshooting.md)

---

**Parabéns!** 🎉 Você agora sabe o básico do Dryad. Continue explorando para descobrir mais recursos!
