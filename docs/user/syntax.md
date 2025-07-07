# 📚 Sintaxe da Linguagem Dryad

Esta é a referência completa da sintaxe da linguagem Dryad, incluindo todos os construtos disponíveis.

## 📋 Índice

- [Tipos de Dados](#-tipos-de-dados)
- [Variáveis](#-variáveis)
- [Operadores](#-operadores)
- [Estruturas de Controle](#-estruturas-de-controle)
- [Funções](#-funções)
- [Comentários](#-comentários)
- [Operações de I/O](#-operações-de-io)

## 🏷️ Tipos de Dados

### **Number (Números)**
Todos os números são tratados como ponto flutuante (f64).

```javascript
let inteiro = 42;
let decimal = 3.14;
let negativo = -17;
let zero = 0;
```

### **String (Texto)**
Strings são delimitadas por aspas duplas.

```javascript
let nome = "João";
let frase = "Olá, mundo!";
let vazia = "";
let comEspacos = "  texto com espaços  ";
```

### **Boolean (Lógico)**
Valores verdadeiro ou falso.

```javascript
let verdadeiro = true;
let falso = false;
```

### **Null**
Representa ausência de valor.

```javascript
let vazio = null;
```

## 📦 Variáveis

### Declaração
```javascript
// Sintaxe básica
let nome_variavel = valor;

// Exemplos
let idade = 25;
let nome = "Maria";
let ativo = true;
```

### Regras para Nomes
```javascript
// ✅ Válidos
let variavel;
let minha_variavel;
let minhaVariavel;
let _privada;
let contador1;

// ❌ Inválidos
let 1contador;    // Não pode começar com número
let minha-var;    // Hífen não é permitido
let let;          // Palavra-chave reservada
```

### Reatribuição
⚠️ **Nota:** Atualmente, a reatribuição não está implementada. Use novas declarações.

```javascript
// Funciona (nova declaração)
let x = 10;
let x = 20;  // Nova variável x

// Não implementado ainda
// x = 30;  // Reatribuição direta
```

## ⚙️ Operadores

### **Operadores Aritméticos**
```javascript
let a = 10;
let b = 3;

let soma = a + b;           // 13
let subtracao = a - b;      // 7
let multiplicacao = a * b;  // 30
let divisao = a / b;        // 3.333...
```

### **Operadores de Comparação**
```javascript
let x = 5;
let y = 10;

x == y;   // false (igual)
x != y;   // true  (diferente)
x < y;    // true  (menor que)
x > y;    // false (maior que)
x <= y;   // true  (menor ou igual)
x >= y;   // false (maior ou igual)
```

### **Precedência de Operadores**
Da maior para menor precedência:

1. **Parênteses** `( )`
2. **Multiplicação e Divisão** `*` `/`
3. **Adição e Subtração** `+` `-`
4. **Comparação** `<` `>` `<=` `>=`
5. **Igualdade** `==` `!=`

```javascript
// Exemplos de precedência
let resultado1 = 2 + 3 * 4;     // 14 (não 20)
let resultado2 = (2 + 3) * 4;   // 20
let resultado3 = 10 > 5 + 2;    // true (10 > 7)
```

## 🔄 Estruturas de Controle

### **Condicionais - if/else**
```javascript
// If simples
if (condicao) {
    // código executado se verdadeiro
}

// If/else
if (idade >= 18) {
    print("Maior de idade");
} else {
    print("Menor de idade");
}

// If/else aninhado
if (nota >= 90) {
    print("Excelente");
} else if (nota >= 70) {
    print("Bom");
} else if (nota >= 50) {
    print("Regular");
} else {
    print("Insuficiente");
}
```

### **Loops - while**
```javascript
// Loop while básico
let contador = 0;
while (contador < 5) {
    print("Contador: " + contador);
    contador = contador + 1;
}

// Condição complexa
let x = 10;
while (x > 0 && x != 5) {
    print(x);
    x = x - 1;
}
```

### **Loops - for**
```javascript
// For loop básico
for (let i = 0; i < 5; i = i + 1) {
    print("Iteração: " + i);
}

// For com diferentes incrementos
for (let i = 10; i > 0; i = i - 2) {
    print("Valor: " + i);
}

// For com múltiplas variáveis (futuro)
// Atualmente não implementado
```

### **Blocos de Código**
```javascript
// Bloco simples
{
    let x = 10;
    let y = 20;
    print(x + y);
}

// Blocos aninhados
if (true) {
    let a = 1;
    {
        let b = 2;
        print(a + b);
    }
}
```

## 🔧 Funções

### **Funções Built-in**
Atualmente disponíveis:

```javascript
// Saída no console
print("Mensagem");
print(variavel);
print("Valor: " + numero);

// Operações de arquivo
let conteudo = read_file("arquivo.txt");
write_file("saida.txt", "conteúdo");
append_file("log.txt", "nova linha");
```

### **Funções Definidas pelo Usuário**
⚠️ **Não implementado ainda**

```javascript
// Sintaxe planejada (futuro)
fun somar(a, b) {
    return a + b;
}

let resultado = somar(5, 3);
```

## 💬 Comentários

### **Comentários de Linha**
```javascript
// Este é um comentário de linha
let x = 10; // Comentário no final da linha
```

### **Comentários de Bloco**
⚠️ **Não implementado ainda**

```javascript
/* 
   Este será um comentário
   de múltiplas linhas
*/
```

## 📁 Operações de I/O

### **Leitura de Arquivos**
```javascript
// Ler arquivo completo
let conteudo = read_file("dados.txt");
print(conteudo);

// Verificar se leu com sucesso
if (conteudo != null) {
    print("Arquivo lido com sucesso");
}
```

### **Escrita de Arquivos**
```javascript
// Escrever arquivo (substitui conteúdo)
let dados = "Informações importantes";
write_file("output.txt", dados);

// Adicionar ao arquivo
append_file("log.txt", "Nova entrada");
append_file("log.txt", "\n"); // Nova linha
```

### **Exemplos Práticos de I/O**
```javascript
// Processar arquivo
let entrada = read_file("input.txt");
let processado = "Processado: " + entrada;
write_file("output.txt", processado);

// Log simples
let timestamp = "2025-01-01 12:00:00";
let logEntry = timestamp + " - Operação realizada\n";
append_file("aplicacao.log", logEntry);
```

## 🔤 Literais e Constantes

### **Números**
```javascript
// Inteiros
let zero = 0;
let positivo = 42;
let negativo = -17;

// Decimais
let pi = 3.14159;
let percentual = 0.85;
let pequeno = 0.001;
```

### **Strings**
```javascript
// Strings simples
let nome = "João";
let sobrenome = "Silva";

// Strings com espaços
let frase = "Olá, mundo!";
let caminho = "C:\\Users\\Nome\\arquivo.txt";

// String vazia
let vazia = "";
```

### **Concatenação de Strings**
```javascript
let nome = "João";
let sobrenome = "Silva";
let completo = nome + " " + sobrenome;

let idade = 25;
let mensagem = "Tenho " + idade + " anos";
```

## 🎯 Expressões e Avaliação

### **Expressões Simples**
```javascript
// Literais
42
"texto"
true

// Variáveis
x
minha_variavel
```

### **Expressões Compostas**
```javascript
// Aritméticas
x + y
a * b + c
(x + y) / z

// Lógicas
x > 5
y == 10
x > 0 && y < 100
```

### **Precedência em Expressões**
```javascript
// Matemática
let r1 = 2 + 3 * 4;        // 14
let r2 = (2 + 3) * 4;      // 20
let r3 = 2 * 3 + 4;        // 10

// Comparação
let r4 = 5 + 3 > 6;        // true (8 > 6)
let r5 = 5 > 3 + 6;        // false (5 > 9)
```

## 📝 Exemplo Completo

```javascript
// programa_completo.dryad

// Variáveis
let nome = "Calculadora Dryad";
let versao = 1.0;

print("=== " + nome + " v" + versao + " ===");

// Entrada de dados
let a = 15;
let b = 4;

print("Operações com " + a + " e " + b + ":");

// Cálculos
let soma = a + b;
let subtracao = a - b;
let multiplicacao = a * b;
let divisao = a / b;

// Saída formatada
print("Soma:           " + soma);
print("Subtração:      " + subtracao);
print("Multiplicação:  " + multiplicacao);
print("Divisão:        " + divisao);

// Lógica condicional
if (a > b) {
    print(a + " é maior que " + b);
} else if (a < b) {
    print(a + " é menor que " + b);
} else {
    print(a + " é igual a " + b);
}

// Loop
print("Contagem regressiva:");
let contador = 5;
while (contador > 0) {
    print("T-" + contador);
    contador = contador - 1;
}
print("Fim!");

// I/O de arquivo
let relatorio = "Relatório:\n";
relatorio = relatorio + "Soma: " + soma + "\n";
relatorio = relatorio + "Produto: " + multiplicacao + "\n";

write_file("relatorio.txt", relatorio);
print("Relatório salvo em relatorio.txt");
```

## ⚠️ Limitações Atuais

### **Não Implementado:**
- Funções definidas pelo usuário
- Arrays/listas
- Reatribuição de variáveis (`x = novo_valor`)
- Operadores lógicos (`&&`, `||`, `!`)
- Comentários de bloco (`/* */`)
- Módulos e imports
- Tratamento de exceções
- Operadores unários (`-x`, `!x`)

### **Planejado para Versões Futuras:**
- Sistema de módulos
- Funções personalizadas
- Estruturas de dados complexas
- Pattern matching
- Tratamento de erros robusto

---

Esta referência cobre toda a sintaxe atualmente implementada no Dryad. Para mais exemplos práticos, consulte [Exemplos](./examples.md).
