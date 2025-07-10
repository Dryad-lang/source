# Documentação das Bibliotecas IO - Dryad

Esta documentação cobre todas as bibliotecas disponíveis na pasta `lib/IO` e como utilizá-las corretamente.

## 📁 Estrutura da lib/IO

```
lib/IO/
├── console.dryad    # Funções de console (entrada/saída)
├── fs.dryad         # Sistema de arquivos
└── buffer.dryad     # Manipulação de buffers
```

## 🔧 Como Importar

As bibliotecas IO usam namespaces. Para importar, use:

```dryad
// Import específico por arquivo
using IO.Console;    // Importa de lib/IO/console.dryad
using IO.fs;         // Importa de lib/IO/fs.dryad  
using IO.buffer;     // Importa de lib/IO/buffer.dryad
```

## 📚 Bibliotecas Disponíveis

### 1. IO.Console

**Arquivo:** `lib/IO/console.dryad`  
**Namespace:** `IO`  
**Classe:** `Console`

#### Funções Disponíveis:

- `Console.println(message)` - Imprime mensagem com quebra de linha
- `Console.print(message)` - Imprime mensagem sem quebra de linha
- `Console.input(prompt)` - Lê entrada do usuário com prompt opcional
- `Console.readLine()` - Lê uma linha da entrada
- `Console.log(message)` - Alias para println
- `Console.write(message)` - Alias para print
- `Console.clear()` - Limpa o console
- `Console.prompt(message)` - Alias para input

#### Exemplo de Uso:

```dryad
using IO.Console;

Console.println("Olá, mundo!");
Console.print("Digite seu nome: ");
let nome = Console.readLine();
Console.println("Olá, " + nome + "!");
```

### 2. IO.fs (FileSystem)

**Arquivo:** `lib/IO/fs.dryad`  
**Namespace:** `IO`  
**Classe:** `FileSystem`

#### Funções Disponíveis:

- `FileSystem.readFile(filename)` - Lê conteúdo de um arquivo
- `FileSystem.writeFile(filename, content)` - Escreve conteúdo em arquivo
- `FileSystem.appendFile(filename, content)` - Anexa conteúdo a arquivo
- `FileSystem.fileExists(filename)` - Verifica se arquivo existe
- `FileSystem.deleteFile(filename)` - Deleta um arquivo
- `FileSystem.read(filename)` - Alias para readFile
- `FileSystem.write(filename, content)` - Alias para writeFile
- `FileSystem.append(filename, content)` - Alias para appendFile
- `FileSystem.exists(filename)` - Alias para fileExists
- `FileSystem.delete(filename)` - Alias para deleteFile

#### Exemplo de Uso:

```dryad
using IO.Console;
using IO.fs;

// Escrever arquivo
FileSystem.writeFile("dados.txt", "Meus dados importantes");

// Verificar se existe
if (FileSystem.fileExists("dados.txt")) {
    // Ler arquivo
    let conteudo = FileSystem.readFile("dados.txt");
    Console.println("Conteúdo: " + conteudo);
    
    // Anexar mais dados
    FileSystem.appendFile("dados.txt", "\nMais uma linha");
    
    // Ler novamente
    let novoConteudo = FileSystem.readFile("dados.txt");
    Console.println("Novo conteúdo: " + novoConteudo);
}
```

### 3. IO.buffer (Buffer)

**Arquivo:** `lib/IO/buffer.dryad`  
**Namespace:** `IO`  
**Classe:** `Buffer`

#### Funções Disponíveis:

- `Buffer.create(size)` - Cria um buffer com tamanho especificado
- `Buffer.length(buffer)` - Retorna tamanho do buffer
- `Buffer.toString(buffer)` - Converte buffer para string

#### Exemplo de Uso:

```dryad
using IO.Console;
using IO.buffer;

// Criar buffer
let meuBuffer = Buffer.create(100);
Console.println("Buffer criado");

// Verificar tamanho
let tamanho = Buffer.length(meuBuffer);
Console.println("Tamanho do buffer: " + tamanho);

// Converter para string
let bufferStr = Buffer.toString(meuBuffer);
Console.println("Buffer como string: " + bufferStr);
```

## 🔍 Funções Nativas Suportadas

As seguintes funções nativas estão implementadas e disponíveis:

### Console:
- `native_console_print`
- `native_console_println`
- `native_console_input`
- `native_console_clear`

### FileSystem:
- `native_fs_read_file`
- `native_fs_write_file`
- `native_fs_append_file`
- `native_fs_file_exists`
- `native_fs_delete_file`

### Buffer:
- `native_buffer_create`
- `native_buffer_length`

### Utilitários:
- `native_types_to_string`

## 🧪 Como Criar Novos Módulos

Para criar um novo módulo na lib/IO:

### 1. Estrutura do Arquivo

```dryad
// lib/IO/novo_modulo.dryad
// Descrição do módulo

namespace IO {
    export class MinhaClasse {
        // Funções estáticas
        public static fn minhaFuncao(parametro) {
            return native_minha_funcao_nativa(parametro);
        }
    }
}
```

### 2. Implementar Funções Nativas

No arquivo `src/interpreter/native.rs`, adicione:

```rust
// Registrar a função
self.register("native_minha_funcao_nativa".to_string(), native_minha_funcao_nativa);

// Implementar a função
fn native_minha_funcao_nativa(args: &[Value]) -> Result<Value, DryadError> {
    // Implementação aqui
    Ok(Value::Null)
}
```

### 3. Testar o Módulo

```dryad
using IO.Console;
using IO.novo_modulo;

Console.println("Testando novo módulo...");
let resultado = MinhaClasse.minhaFuncao("teste");
Console.println("Resultado: " + resultado);
```

## ✅ Validação de Implementação

Para validar se sua implementação está correta:

1. **Sintaxe:** Use `public static fn` para funções
2. **Namespace:** Use `namespace IO { export class ... }`
3. **Import:** Use `using IO.nome_arquivo;`
4. **Teste:** Crie um arquivo de teste para validar

## 🚨 Problemas Comuns

- **Função não encontrada:** Verifique se a função nativa está registrada
- **Import falha:** Verifique se o nome do arquivo está correto
- **Namespace não funciona:** Certifique-se de usar a estrutura correta
- **Sintaxe:** Use `fn` em vez de `fun` ou `function`

## 📝 Exemplo Completo

```dryad
// exemplo_completo.dryad
using IO.Console;
using IO.fs;
using IO.buffer;

// Teste Console
Console.println("=== Teste Console ===");
Console.print("Digite algo: ");
let entrada = Console.readLine();
Console.println("Você digitou: " + entrada);

// Teste FileSystem
Console.println("=== Teste FileSystem ===");
FileSystem.writeFile("teste.txt", "Dados de teste");
let dados = FileSystem.readFile("teste.txt");
Console.println("Dados do arquivo: " + dados);
FileSystem.deleteFile("teste.txt");

// Teste Buffer
Console.println("=== Teste Buffer ===");
let buffer = Buffer.create(50);
Console.println("Buffer criado com tamanho: " + Buffer.length(buffer));

Console.println("=== Todos os testes concluídos! ===");
```

---

**🎯 Status:** Bibliotecas IO totalmente funcionais com namespaces  
**📅 Atualizado:** 9 de julho de 2025
