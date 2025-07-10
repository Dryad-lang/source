# Catálogo de Erros Dryad

## Visão Geral

Este documento cataloga todos os códigos de erro da linguagem Dryad, organizados por categoria para facilitar o debug e resolução de problemas.

## Sistema de Numeração

- **1000-1999**: Erros do Lexer (Análise Léxica)
- **2000-2999**: Erros do Parser (Análise Sintática)
- **3000-3999**: Erros de Runtime/Interpretador
- **4000-4999**: Erros do Sistema de Tipos
- **5000-5999**: Erros de I/O (Entrada/Saída)
- **6000-6999**: Erros do Sistema de Módulos
- **7000-7999**: Erros de Sintaxe
- **8000-8999**: Avisos (Warnings)
- **9000-9999**: Erros de Sistema

---

## 📝 ERROS DO LEXER (1000-1999)

### E1001 - Unexpected Character
**Descrição**: Caracter não reconhecido pelo lexer
**Exemplo**: `let x = @;` (@ não é um caracter válido)
**Solução**: Verificar se todos os caracteres são válidos na linguagem

### E1002 - Unterminated String Literal
**Descrição**: String não fechada corretamente
**Exemplo**: `let msg = "Hello World;` (falta a aspas fechando)
**Solução**: Adicionar aspas de fechamento da string

### E1003 - Unterminated Comment
**Descrição**: Comentário de bloco não fechado
**Exemplo**: `/* Este comentário nunca fecha`
**Solução**: Adicionar `*/` para fechar o comentário

### E1004 - Invalid Number Format
**Descrição**: Formato de número inválido
**Exemplo**: `let x = 123.45.67;` (múltiplos pontos decimais)
**Solução**: Corrigir o formato do número

### E1005 - Invalid Escape Sequence
**Descrição**: Sequência de escape inválida em string
**Exemplo**: `let s = "Hello\k";` (\k não é válido)
**Solução**: Usar sequências de escape válidas: \n, \t, \r, \\, \"

### E1006 - Character Not Recognized
**Descrição**: Caracter completamente desconhecido
**Exemplo**: Caracteres Unicode não suportados
**Solução**: Usar apenas caracteres ASCII ou Unicode suportados

### E1007 - String Exceeds Maximum Length
**Descrição**: String excede o tamanho máximo permitido
**Solução**: Dividir string em partes menores

### E1008 - Invalid Unicode Sequence
**Descrição**: Sequência Unicode inválida
**Exemplo**: `let s = "\u{GGGG}";` (G não é hexadecimal)
**Solução**: Usar apenas dígitos hexadecimais válidos

---

## 🔍 ERROS DO PARSER (2000-2999)

### E2001 - Unexpected Token
**Descrição**: Token inesperado na análise sintática
**Exemplo**: `let x = 5 +;` (operador sem operando)
**Solução**: Verificar a sintaxe da expressão

### E2002 - Expected Token Not Found
**Descrição**: Token esperado não foi encontrado
**Exemplo**: `if (x > 5 { ... }` (falta fechar parênteses)
**Solução**: Adicionar o token esperado

### E2003 - Missing Semicolon
**Descrição**: Ponto e vírgula ausente quando necessário
**Exemplo**: `let x = 5 let y = 10;`
**Solução**: Adicionar `;` ao final da declaração

### E2004 - Missing Closing Brace
**Descrição**: Chave de fechamento `}` ausente
**Exemplo**: `function test() { return 5;`
**Solução**: Adicionar `}` para fechar o bloco

### E2005 - Missing Closing Parenthesis
**Descrição**: Parênteses de fechamento `)` ausente
**Exemplo**: `if (x > 5 { ... }`
**Solução**: Adicionar `)` para fechar a expressão

### E2006 - Missing Closing Bracket
**Descrição**: Colchete de fechamento `]` ausente
**Exemplo**: `let arr = [1, 2, 3;`
**Solução**: Adicionar `]` para fechar o array

### E2007 - Invalid Expression
**Descrição**: Expressão malformada
**Exemplo**: `let x = + * 5;`
**Solução**: Corrigir a sintaxe da expressão

### E2008 - Invalid Statement
**Descrição**: Declaração inválida
**Exemplo**: `x + y;` (expressão sem atribuição ou contexto)
**Solução**: Criar uma declaração válida

### E2009 - Invalid Function Declaration
**Descrição**: Declaração de função malformada
**Exemplo**: `function { return 5; }`
**Solução**: Adicionar nome e parâmetros à função

### E2010 - Invalid Class Declaration
**Descrição**: Declaração de classe malformada
**Exemplo**: `class { }`
**Solução**: Adicionar nome à classe

### E2011 - Invalid Variable Declaration
**Descrição**: Declaração de variável inválida
**Exemplo**: `let = 5;`
**Solução**: Adicionar nome à variável

### E2012 - Missing Function Name
**Descrição**: Nome da função ausente
**Exemplo**: `function () { }`
**Solução**: Adicionar nome à função

### E2013 - Missing Parameter List
**Descrição**: Lista de parâmetros ausente
**Exemplo**: `function test { }`
**Solução**: Adicionar parênteses com parâmetros

### E2014 - Invalid Parameter
**Descrição**: Parâmetro de função inválido
**Exemplo**: `function test(123) { }`
**Solução**: Usar identificadores válidos para parâmetros

### E2015 - Duplicate Parameter Name
**Descrição**: Nome de parâmetro duplicado
**Exemplo**: `function test(x, x) { }`
**Solução**: Usar nomes únicos para parâmetros

### E2016 - Invalid Return Statement
**Descrição**: Declaração return inválida
**Exemplo**: `return;` fora de função
**Solução**: Usar return apenas dentro de funções

### E2017 - Invalid If Statement
**Descrição**: Declaração if malformada
**Exemplo**: `if { ... }`
**Solução**: Adicionar condição entre parênteses

### E2018 - Invalid While Loop
**Descrição**: Loop while malformado
**Exemplo**: `while { ... }`
**Solução**: Adicionar condição entre parênteses

### E2019 - Invalid For Loop
**Descrição**: Loop for malformado
**Exemplo**: `for { ... }`
**Solução**: Adicionar declaração, condição e incremento

### E2020 - Invalid Namespace Declaration
**Descrição**: Declaração de namespace inválida
**Solução**: Corrigir sintaxe do namespace

### E2021 - Invalid Import Statement
**Descrição**: Declaração import malformada
**Exemplo**: `import;`
**Solução**: Especificar módulo a importar

### E2022 - Invalid Export Statement
**Descrição**: Declaração export malformada
**Solução**: Especificar o que exportar

### E2023 - Missing Closing Quote
**Descrição**: Aspas de fechamento ausentes
**Exemplo**: `print("Test push);`
**Solução**: Adicionar aspas de fechamento

### E2024 - Invalid Array Literal
**Descrição**: Array literal malformado
**Exemplo**: `let arr = [1, 2,];` (vírgula no final)
**Solução**: Remover vírgula extra ou adicionar elemento

### E2025 - Invalid Object Literal
**Descrição**: Objeto literal malformado
**Solução**: Corrigir sintaxe do objeto

---

## ⚡ ERROS DE RUNTIME (3000-3999)

### E3001 - Undefined Variable
**Descrição**: Variável não definida
**Exemplo**: `print(nonexistent);`
**Solução**: Declarar a variável antes de usar

### E3002 - Variable Already Defined
**Descrição**: Variável já foi definida
**Exemplo**: `let x = 5; let x = 10;`
**Solução**: Usar nomes únicos ou reatribuição sem `let`

### E3003 - Function Not Found
**Descrição**: Função não encontrada
**Exemplo**: `nonexistentFunction();`
**Solução**: Verificar se a função foi declarada

### E3004 - Invalid Function Call
**Descrição**: Chamada de função inválida
**Exemplo**: Chamar variável que não é função
**Solução**: Verificar se o identificador é uma função

### E3005 - Wrong Number of Arguments
**Descrição**: Número incorreto de argumentos
**Exemplo**: `function test(a, b) { } test(1);`
**Solução**: Passar o número correto de argumentos

### E3006 - Type Mismatch
**Descrição**: Tipos incompatíveis
**Exemplo**: `5 + "hello"`
**Solução**: Converter tipos ou usar operações compatíveis

### E3007 - Division by Zero
**Descrição**: Divisão por zero
**Exemplo**: `let x = 5 / 0;`
**Solução**: Verificar divisor antes da operação

### E3008 - Index Out of Bounds
**Descrição**: Índice fora dos limites do array
**Exemplo**: `arr[10]` em array de 3 elementos
**Solução**: Verificar tamanho do array antes do acesso

### E3009 - Null Pointer Dereference
**Descrição**: Acesso a valor nulo
**Solução**: Verificar se valor não é nulo antes do acesso

### E3010 - Invalid Assignment
**Descrição**: Atribuição inválida
**Solução**: Verificar se o alvo da atribuição é válido

### E3011 - Cannot Modify Constant
**Descrição**: Tentativa de modificar constante
**Solução**: Usar variável mutável

### E3012 - Class Not Found
**Descrição**: Classe não encontrada
**Solução**: Verificar se a classe foi declarada

### E3013 - Method Not Found
**Descrição**: Método não encontrado
**Exemplo**: `obj.nonexistentMethod()`
**Solução**: Verificar se o método existe na classe

### E3014 - Property Not Found
**Descrição**: Propriedade não encontrada
**Solução**: Verificar se a propriedade existe

### E3015 - Invalid This Context
**Descrição**: Uso inválido de `this`
**Solução**: Usar `this` apenas em contexto de classe

### E3016 - Constructor Error
**Descrição**: Erro no construtor da classe
**Solução**: Verificar implementação do construtor

### E3017 - Inheritance Error
**Descrição**: Erro na herança de classe
**Solução**: Verificar hierarquia de classes

### E3018 - Static Method Access Error
**Descrição**: Erro no acesso a método estático
**Solução**: Usar sintaxe correta para métodos estáticos

### E3019 - Instance Method Access Error
**Descrição**: Erro no acesso a método de instância
**Solução**: Criar instância antes de chamar método

### E3020 - Invalid Array Operation
**Descrição**: Operação inválida em array
**Solução**: Usar operações suportadas por arrays

### E3021 - Array Method Not Found
**Descrição**: Método de array não encontrado
**Exemplo**: `arr.nonexistentMethod()`
**Solução**: Usar métodos válidos: push, pop, slice, etc.

### E3022 - Invalid Array Index
**Descrição**: Índice de array inválido
**Exemplo**: `arr[-1]` ou `arr[3.5]`
**Solução**: Usar índices inteiros não negativos

### E3023 - Array Callback Error
**Descrição**: Erro na função callback de array
**Solução**: Verificar implementação da função callback

### E3024 - Stack Overflow
**Descrição**: Estouro de pilha (recursão infinita)
**Solução**: Adicionar condição de parada na recursão

### E3025 - Memory Limit Exceeded
**Descrição**: Limite de memória excedido
**Solução**: Otimizar uso de memória

### E3026 - Execution Timeout
**Descrição**: Tempo limite de execução excedido
**Solução**: Otimizar algoritmo ou aumentar timeout

### E3027 - Invalid Cast
**Descrição**: Conversão de tipo inválida
**Solução**: Usar conversões válidas

### E3028 - Circular Reference
**Descrição**: Referência circular detectada
**Solução**: Quebrar ciclo de referências

### E3029 - Resource Not Available
**Descrição**: Recurso não disponível
**Solução**: Verificar disponibilidade do recurso

### E3030 - Permission Denied
**Descrição**: Permissão negada
**Solução**: Verificar permissões necessárias

---

## 📐 ERROS DO SISTEMA DE TIPOS (4000-4999)

### E4001 - Invalid Type Annotation
**Descrição**: Anotação de tipo inválida
**Solução**: Usar sintaxe correta para tipos

### E4002 - Type Inference Failed
**Descrição**: Falha na inferência de tipo
**Solução**: Adicionar anotação de tipo explícita

### E4003 - Incompatible Types
**Descrição**: Tipos incompatíveis
**Solução**: Converter tipos ou usar tipos compatíveis

### E4004 - Generic Type Error
**Descrição**: Erro em tipo genérico
**Solução**: Verificar parâmetros de tipo genérico

### E4005 - Interface Not Implemented
**Descrição**: Interface não implementada
**Solução**: Implementar todos os métodos da interface

### E4006 - Abstract Method Not Implemented
**Descrição**: Método abstrato não implementado
**Solução**: Implementar método abstrato na classe filha

### E4007 - Type Constraint Violation
**Descrição**: Violação de restrição de tipo
**Solução**: Atender às restrições do tipo

### E4008 - Invalid Type Parameter
**Descrição**: Parâmetro de tipo inválido
**Solução**: Usar parâmetros válidos

### E4009 - Recursive Type Definition
**Descrição**: Definição de tipo recursiva
**Solução**: Quebrar recursão na definição

### E4010 - Union Type Error
**Descrição**: Erro em tipo união
**Solução**: Verificar compatibilidade dos tipos

---

## 📁 ERROS DE I/O (5000-5999)

### E5001 - File Not Found
**Descrição**: Arquivo não encontrado
**Exemplo**: `import "nonexistent.dryad"`
**Solução**: Verificar se o arquivo existe no caminho correto

### E5002 - Permission Denied
**Descrição**: Permissão negada para acesso a arquivo
**Solução**: Verificar permissões do arquivo

### E5003 - I/O Error
**Descrição**: Erro genérico de entrada/saída
**Solução**: Verificar estado do sistema de arquivos

### E5004 - Invalid File Format
**Descrição**: Formato de arquivo inválido
**Solução**: Usar arquivo no formato correto

### E5005 - File Size Limit Exceeded
**Descrição**: Limite de tamanho de arquivo excedido
**Solução**: Usar arquivo menor

### E5006 - Network Error
**Descrição**: Erro de rede
**Solução**: Verificar conectividade

### E5007 - Timeout Error
**Descrição**: Timeout em operação I/O
**Solução**: Aumentar timeout ou verificar recurso

### E5008 - Invalid Path
**Descrição**: Caminho inválido
**Solução**: Usar caminho válido

### E5009 - Directory Not Found
**Descrição**: Diretório não encontrado
**Solução**: Criar diretório ou usar caminho correto

### E5010 - Cannot Create File
**Descrição**: Não é possível criar arquivo
**Solução**: Verificar permissões e espaço

### E5011 - Cannot Write to File
**Descrição**: Não é possível escrever no arquivo
**Solução**: Verificar permissões

### E5012 - Cannot Read from File
**Descrição**: Não é possível ler do arquivo
**Solução**: Verificar permissões e integridade

---

## 📦 ERROS DO SISTEMA DE MÓDULOS (6000-6999)

### E6001 - Module Not Found
**Descrição**: Módulo não encontrado
**Exemplo**: `import unknown_module`
**Solução**: Verificar se o módulo existe

### E6002 - Circular Dependency
**Descrição**: Dependência circular entre módulos
**Solução**: Quebrar dependência circular

### E6003 - Invalid Module Path
**Descrição**: Caminho de módulo inválido
**Solução**: Usar caminho válido

### E6004 - Module Loading Error
**Descrição**: Erro ao carregar módulo
**Solução**: Verificar integridade do módulo

### E6005 - Export Not Found
**Descrição**: Export não encontrado
**Solução**: Verificar se o item foi exportado

### E6006 - Import Error
**Descrição**: Erro na importação
**Solução**: Verificar sintaxe de import

### E6007 - Namespace Collision
**Descrição**: Colisão de namespace
**Solução**: Usar nomes únicos ou aliases

### E6008 - Invalid Namespace Access
**Descrição**: Acesso inválido a namespace
**Solução**: Verificar escopo do namespace

### E6009 - Module Version Conflict
**Descrição**: Conflito de versão de módulo
**Solução**: Resolver conflito de versões

### E6010 - Missing Module Dependency
**Descrição**: Dependência de módulo ausente
**Solução**: Instalar dependência necessária

---

## 🔤 ERROS DE SINTAXE (7000-7999)

### E7001 - Missing Closing Quote in String
**Descrição**: Aspas de fechamento ausentes em string
**Exemplo**: `print("Test push);`
**Solução**: Adicionar aspas de fechamento

### E7002 - Invalid Character in Identifier
**Descrição**: Caracter inválido em identificador
**Solução**: Usar apenas letras, números e underscore

### E7003 - Invalid Operator Usage
**Descrição**: Uso inválido de operador
**Solução**: Verificar sintaxe do operador

### E7004 - Missing Operator
**Descrição**: Operador ausente
**Solução**: Adicionar operador necessário

### E7005 - Invalid Bracket Nesting
**Descrição**: Aninhamento inválido de colchetes/parênteses
**Solução**: Verificar balanceamento

### E7006 - Unexpected End of File
**Descrição**: Fim de arquivo inesperado
**Solução**: Completar estrutura pendente

### E7007 - Invalid Comment Syntax
**Descrição**: Sintaxe de comentário inválida
**Solução**: Usar // ou /* */

### E7008 - Invalid Keyword Usage
**Descrição**: Uso inválido de palavra-chave
**Solução**: Verificar contexto da palavra-chave

### E7009 - Reserved Word Used as Identifier
**Descrição**: Palavra reservada usada como identificador
**Solução**: Usar identificador diferente

### E7010 - Invalid Statement Termination
**Descrição**: Terminação de declaração inválida
**Solução**: Adicionar ; ou corrigir sintaxe

---

## ⚠️ AVISOS (8000-8999)

### W8001 - Unused Variable
**Descrição**: Variável declarada mas não usada
**Solução**: Remover variável ou usar prefixo _

### W8002 - Unused Function
**Descrição**: Função declarada mas não usada
**Solução**: Remover função ou usar

### W8003 - Unreachable Code
**Descrição**: Código nunca executado
**Solução**: Remover ou corrigir lógica

### W8004 - Deprecated Feature
**Descrição**: Recurso depreciado
**Solução**: Usar alternativa moderna

### W8005 - Performance Warning
**Descrição**: Possível problema de performance
**Solução**: Otimizar código

### W8006 - Style Warning
**Descrição**: Violação de estilo de código
**Solução**: Seguir convenções de estilo

### W8007 - Missing Documentation
**Descrição**: Documentação ausente
**Solução**: Adicionar comentários/documentação

### W8008 - Potential Null Pointer
**Descrição**: Possível ponteiro nulo
**Solução**: Adicionar verificação

### W8009 - Implicit Type Conversion
**Descrição**: Conversão de tipo implícita
**Solução**: Fazer conversão explícita

### W8010 - Large File Warning
**Descrição**: Arquivo muito grande
**Solução**: Dividir em arquivos menores

---

## 💻 ERROS DE SISTEMA (9000-9999)

### E9001 - Internal Compiler Error
**Descrição**: Erro interno do compilador
**Solução**: Reportar bug aos desenvolvedores

### E9002 - Memory Allocation Failed
**Descrição**: Falha na alocação de memória
**Solução**: Liberar memória ou aumentar limite

### E9003 - System Resource Exhausted
**Descrição**: Recursos do sistema esgotados
**Solução**: Liberar recursos

### E9004 - Platform Not Supported
**Descrição**: Plataforma não suportada
**Solução**: Usar plataforma suportada

### E9005 - Configuration Error
**Descrição**: Erro de configuração
**Solução**: Corrigir configuração

### E9006 - License Error
**Descrição**: Erro de licença
**Solução**: Verificar licença

### E9007 - Version Incompatibility
**Descrição**: Incompatibilidade de versão
**Solução**: Atualizar versão

### E9008 - Corrupted Data
**Descrição**: Dados corrompidos
**Solução**: Restaurar backup

### E9009 - Security Violation
**Descrição**: Violação de segurança
**Solução**: Verificar permissões

### E9010 - Fatal System Error
**Descrição**: Erro fatal do sistema
**Solução**: Reiniciar sistema

---

## 🔧 Como Usar Este Catálogo

1. **Identifique o código**: Quando um erro ocorrer, note o código (ex: E3021)
2. **Localize a categoria**: Use o primeiro dígito para encontrar a seção
3. **Leia a descrição**: Entenda o que causou o erro
4. **Aplique a solução**: Siga as instruções para corrigir

## 📞 Suporte

Para erros não documentados ou questões específicas:
- Consulte a documentação oficial
- Visite o repositório no GitHub
- Entre em contato com a comunidade

---

*Última atualização: Julho 2025*
*Versão do catálogo: 1.0*
