# 🚀 Guia de Início Rápido - Dryad

**Bem-vindo ao Dryad!** Este guia irá te ajudar a começar a usar a linguagem Dryad em minutos.

**Versão:** 0.1.0  
**Atualizado:** 8 de janeiro de 2025  
**Status:** Sistema Oak Modular Integrado

---

## 📋 O que você vai aprender

- ✅ Como instalar e executar Dryad
- ✅ Sintaxe básica da linguagem  
- ✅ Sistema Oak (package manager)
- ✅ Imports e exports modulares
- ✅ Common libraries disponíveis
- ✅ Exemplos práticos prontos para usar

---

## ⚡ Instalação Rápida

### 1. Compilar Dryad

```bash
# Clonar o repositório
git clone <repo-url>
cd dryad

# Compilar (requer Rust)
cargo build --release

# Verificar instalação
./target/release/dryad --version
```

### 2. Primeiro Programa

Crie um arquivo `hello.dryad`:

```dryad
print("Olá, Dryad!");
```

Execute:

```bash
./target/release/dryad hello.dryad
```

---

## 🎯 Conceitos Básicos em 5 Minutos

### Variáveis e Tipos

```dryad
// Declaração de variáveis
let nome = "João";
let idade = 25;
let ativo = true;
let vazio = null;

// Concatenação automática
let mensagem = "Olá, " + nome + "! Você tem " + idade + " anos.";
print(mensagem);  // "Olá, João! Você tem 25 anos."
```

### Funções

```dryad
// Função simples
function saudacao(nome) {
    return "Olá, " + nome + "!";
}

// Chamada
let msg = saudacao("Maria");
print(msg);
```

### Classes

```dryad
class Pessoa {
    function constructor(nome, idade) {
        this.nome = nome;
        this.idade = idade;
    }
    
    function apresentar() {
        return "Eu sou " + this.nome + " e tenho " + this.idade + " anos";
    }
    
    // Método estático
    static function especie() {
        return "Homo sapiens";
    }
}

// Uso
let pessoa = new Pessoa("Ana", 30);
print(pessoa.apresentar());
print(Pessoa.especie());
```

### Condicionais

```dryad
let idade = 18;

if (idade >= 18) {
    print("Maior de idade");
} else {
    print("Menor de idade");
}
```

---

## 📦 Sistema Oak (Package Manager)

### Inicializar Projeto

```bash
# Criar projeto Oak
dryad oak init

# Verificar configuração criada
cat oaklibs.json
```

### Usar Bibliotecas

```dryad
// Usar biblioteca do sistema
using IO.Console;
using Core.Types;

// Verificar tipo
let numero = 42;
let tipo = Types.typeof(numero);
Console.println("Tipo: " + tipo);  // "Tipo: number"

// Entrada do usuário
let nome = Console.input("Digite seu nome: ");
Console.println("Olá, " + nome + "!");
```

---

## 🔧 Exemplos Práticos

### 1. Calculadora Simples

```dryad
// calculadora.dryad
using IO.Console;

class Calculadora {
    static function somar(a, b) {
        return a + b;
    }
    
    static function multiplicar(a, b) {
        return a * b;
    }
}

// Programa principal
Console.println("=== Calculadora Dryad ===");

let a = 10;
let b = 5;

let soma = Calculadora.somar(a, b);
let mult = Calculadora.multiplicar(a, b);

Console.println(a + " + " + b + " = " + soma);
Console.println(a + " × " + b + " = " + mult);
```

### 2. Sistema de Arquivos

```dryad
// arquivo.dryad
using IO.FileSystem;
using IO.Console;

// Escrever arquivo
let conteudo = "Olá, mundo!\nEste é um arquivo criado pelo Dryad.";
FileSystem.writeFile("exemplo.txt", conteudo);
Console.println("Arquivo criado!");

// Ler arquivo
if (FileSystem.fileExists("exemplo.txt")) {
    let lido = FileSystem.readFile("exemplo.txt");
    Console.println("Conteúdo do arquivo:");
    Console.println(lido);
}
```

### 3. Trabalhando com Tempo

```dryad
// tempo.dryad
using System.Time;
using IO.Console;

Console.println("Iniciando cronômetro...");
let inicio = Time.startTimer();

// Simular trabalho
Time.sleep(2);

let tempo = Time.elapsed(inicio);
Console.println("Tempo decorrido: " + tempo + "ms");

// Timestamp atual
let agora = Time.now();
let formatado = Time.format(agora, "DD/MM/YYYY HH:mm:ss");
Console.println("Agora: " + formatado);
```

---

## 🎨 Dicas e Truques

### 1. Concatenação Inteligente

```dryad
// Todos estes funcionam automaticamente:
let msg1 = "Número: " + 42;           // "Número: 42"
let msg2 = "Ativo: " + true;          // "Ativo: true"
let msg3 = 100 + " produtos";         // "100 produtos"
```

### 2. Operadores Lógicos

```dryad
let a = true;
let b = false;

if (a && !b) {
    print("A é verdadeiro E B é falso");
}

if (a || b) {
    print("A OU B é verdadeiro");
}
```

### 3. Imports Organizados

```dryad
// Para bibliotecas do sistema
using IO.Console;
using Core.Types;
using System.Environment;

// Para seus módulos
use './meus-helpers.dryad';
use '../utils/matematica.dryad';
```

### 4. Estruturas Úteis

```dryad
// Verificação rápida de tipo
function isNumber(value) {
    return Types.typeof(value) == "number";
}

// Logger simples
class Logger {
    static function log(nivel, mensagem) {
        let timestamp = Time.format(Time.now(), "HH:mm:ss");
        Console.println("[" + timestamp + "] " + nivel + ": " + mensagem);
    }
}

Logger.log("INFO", "Aplicação iniciada");
```

---

## 🎯 Próximos Passos

### 1. Explore a Documentação

- [Documentação Completa](DRYAD_LANGUAGE_DOCUMENTATION.md)
- [Documentação Técnica](TECHNICAL_DOCUMENTATION.md)
- [Referência de APIs](API_REFERENCE.md)

### 2. Experimente os Exemplos

```bash
# Executar exemplos inclusos
dryad example_complete.dryad
dryad simpleuse.dryad

# Testar API externa
cargo run --example oak_api_usage
```

### 3. Crie Seu Primeiro Projeto

```bash
# Criar diretório
mkdir meu-projeto-dryad
cd meu-projeto-dryad

# Inicializar Oak
../target/release/dryad oak init

# Criar arquivo principal
echo 'print("Meu primeiro projeto Dryad!");' > main.dryad

# Executar
../target/release/dryad main.dryad
```

### 4. Explore as Common Libraries

```dryad
// Experimente diferentes módulos
using IO.Console;
using IO.FileSystem;
using System.Environment;
using System.Platform;
using Core.Types;

// Obter informações do sistema
let os = Platform.getOS();
let user = Environment.get("USER");
let hostname = Platform.getHostname();

Console.println("Sistema: " + os);
Console.println("Usuário: " + user);
Console.println("Computador: " + hostname);
```

---

## 🆘 Solução de Problemas

### Problemas Comuns

**1. "Command not found"**
```bash
# Use caminho completo ou adicione ao PATH
./target/release/dryad arquivo.dryad
```

**2. "File not found"**
```bash
# Verifique se o arquivo existe
ls arquivo.dryad

# Use caminho absoluto se necessário
dryad /caminho/completo/arquivo.dryad
```

**3. Erro de sintaxe**
```bash
# Use modo verbose para mais detalhes
dryad --verbose arquivo.dryad
```

**4. Problemas com Oak**
```bash
# Validar configuração
dryad oak validate

# Verificar se oaklibs.json está correto
cat oaklibs.json
```

### Modo Debug

```bash
# Executar com informações detalhadas
dryad --verbose --strict arquivo.dryad

# REPL para testes interativos
dryad --repl
```

---

## 📚 Recursos Adicionais

### Comunidade

- **GitHub**: Issues e discussões
- **Documentação**: Guias detalhados
- **Exemplos**: Repositório de exemplos

### Ferramentas de Desenvolvimento

```bash
# Verificar sintaxe sem executar
dryad --check arquivo.dryad

# Modo interativo para experimentação
dryad --repl

# Informações de debug
dryad --verbose arquivo.dryad
```

### Templates de Projeto

```dryad
// template-basico.dryad
using IO.Console;

class MinhaAplicacao {
    function constructor(nome) {
        this.nome = nome;
    }
    
    function executar() {
        Console.println("=== " + this.nome + " ===");
        Console.println("Aplicação funcionando!");
    }
}

let app = new MinhaAplicacao("Minha App");
app.executar();
```

---

## 🎉 Conclusão

Parabéns! Você agora conhece o básico do Dryad:

- ✅ **Sintaxe fundamental**
- ✅ **Sistema Oak**
- ✅ **Common Libraries**
- ✅ **Estruturas básicas**
- ✅ **Exemplos práticos**

**Continue explorando e criando projetos incríveis com Dryad!**

---

**Precisa de ajuda?** Consulte a [documentação completa](DRYAD_LANGUAGE_DOCUMENTATION.md) ou abra uma issue no GitHub.

**Desenvolvido com ❤️ pela equipe Dryad**
