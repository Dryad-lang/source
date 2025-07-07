## 🗺️ Roadmap para reescrever a linguagem Dryad em Rust

### 🔰 Fase 1 — Planejamento e Definições

* [x] Definir objetivos principais da linguagem (interpretada, modular, sintaxe estilo JS, suporte a I/O).
* [x] Escolher linguagem base para o core: Rust ✅
* [x] Especificar a gramática da linguagem (tokens, sintaxe, estruturas).
* [x] Planejar arquitetura: Lexer → Parser → AST → Interpretador → Ambiente → I/O.

---

### ⚙️ Fase 2 — Implementação do Núcleo da Linguagem

#### 🔡 Etapa 2.1: Lexer

* [x] Definir todos os *tokens* (palavras-chave, operadores, identificadores, números, strings, etc.).
* [x] Implementar o analisador léxico (tokenizador) em Rust.

#### 🌳 Etapa 2.2: Parser

* [x] Definir a AST (árvore de sintaxe abstrata).
* [x] Implementar o parser com suporte para:
  * [x] Expressões aritméticas e lógicas
  * [x] Declarações de variáveis (`let`)
  * [ ] Funções (`fun`)
  * [ ] Blocos, condicionais (`if`, `else`), ciclos (`for`, `while`)
  * [x] Chamadas de função

#### 🧠 Etapa 2.3: Interpretador

* [x] Implementar a avaliação da AST.
* [x] Criar o sistema de escopos (variáveis locais e globais).
* [ ] Adicionar suporte a `return`, `break`, `continue`.

---

### 📦 Fase 3 — Recursos Básicos

* [ ] Sistema de importação de módulos `.dryad`
* [ ] Funções nativas (`print`, `input`, `read_file`, `write_file`, etc.)
* [ ] Sistema de erro com mensagens compreensíveis
* [ ] Tipagem dinâmica ou simples (por exemplo: `Number`, `String`, `Bool`, `Null`, `Function`, `Object`)

---

### 🔌 Fase 4 — Extensões e Integrações

* [ ] Módulos nativos em Rust (ex: biblioteca padrão)
* [ ] Suporte para arquivos e I/O
* [ ] Interoperabilidade com arquivos JSON ou TOML
* [ ] Modo REPL (interpretação interativa)
* [ ] Suporte a plugins?

---

### 🎯 Fase 5 — Empacotamento

* [ ] CLI (`dryad file.dryad`)
* [ ] Versão REPL (`dryad` interativo)
* [ ] Documentação da linguagem
* [ ] Empacotamento como binário multiplataforma (Windows, Linux, macOS)

---

## 📘 Estrutura Gramatical Básica da Dryad (BNF simplificada)

### **Tokens**

```bnf
<keyword>     ::= "let" | "fun" | "if" | "else" | "for" | "while" | "return" | "import"
<operator>    ::= "+" | "-" | "*" | "/" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "="
<separator>   ::= "(" | ")" | "{" | "}" | "," | ";"
<literal>     ::= <number> | <string>
<identifier>  ::= [a-zA-Z_][a-zA-Z0-9_]*
<number>      ::= [0-9]+("."[0-9]+)?
<string>      ::= "\"" .*? "\""
```

---

### **Gramática**

```bnf
<program>       ::= { <statement> }

<statement>     ::= <var_decl> ";"
                 | <fun_decl>
                 | <expr_stmt> ";"
                 | <if_stmt>
                 | <while_stmt>
                 | <for_stmt>
                 | <return_stmt> ";"
                 | <import_stmt> ";"
                 | <block>

<var_decl>      ::= "let" <identifier> "=" <expression>

<fun_decl>      ::= "fun" <identifier> "(" [ <identifier> { "," <identifier> } ] ")" <block>

<if_stmt>       ::= "if" "(" <expression> ")" <block> [ "else" <block> ]

<while_stmt>    ::= "while" "(" <expression> ")" <block>

<for_stmt>      ::= "for" "(" <var_decl> ";" <expression> ";" <expression> ")" <block>

<return_stmt>   ::= "return" [ <expression> ]

<import_stmt>   ::= "import" <identifier>

<block>         ::= "{" { <statement> } "}"

<expr_stmt>     ::= <expression>

<expression>    ::= <assignment>

<assignment>    ::= <identifier> "=" <assignment>
                 | <logic_or>

<logic_or>      ::= <logic_and> { "||" <logic_and> }

<logic_and>     ::= <equality> { "&&" <equality> }

<equality>      ::= <comparison> { ( "==" | "!=" ) <comparison> }

<comparison>    ::= <term> { ( ">" | "<" | ">=" | "<=" ) <term> }

<term>          ::= <factor> { ( "+" | "-" ) <factor> }

<factor>        ::= <unary> { ( "*" | "/" ) <unary> }

<unary>         ::= ( "!" | "-" ) <unary> | <call>

<call>          ::= <primary> { "(" [ <expression> { "," <expression> } ] ")" }

<primary>       ::= <literal>
                 | <identifier>
                 | "(" <expression> ")"
```

---

### 🧪 Exemplos válidos em Dryad

```dryad
let name = "Dryad"

fun greet(who) {
    print("Hello, " + who)
}

if (name == "Dryad") {
    greet(name)
} else {
    print("Unknown.")
}

for (let i = 0; i < 5; i = i + 1) {
    print(i)
}
```

---

## 🎯 Status Atual

✅ **Implementado e Funcionando:**
- Lexer completo com suporte a todos os tokens básicos
- Parser com precedência de operadores
- Interpretador básico com avaliação de expressões
- Sistema de variáveis (`let x = valor`)
- Operações aritméticas (`+`, `-`, `*`, `/`)
- Operações de comparação (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- Chamadas de função básicas (`print()`)
- Suite completa de testes (11 testes passando)

🚧 **Em Desenvolvimento:**
- Declaração de funções (`fun nome() {}`)
- Estruturas de controle (`if`, `else`, `while`, `for`)
- Sistema de escopo avançado
- Tratamento de erros robusto

💡 **Próximos Passos:**
1. Implementar declaração de funções
2. Adicionar estruturas de controle
3. Melhorar sistema de tipos
4. Adicionar módulos e importação
