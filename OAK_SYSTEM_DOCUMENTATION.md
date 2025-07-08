# Sistema de Imports e Package Manager Oak - Implementação Completa

## Funcionalidades Implementadas

### 1. Sistema de Imports Duplo

#### `using` - Para namespaces (bibliotecas em lib/)
```dryad
using IO.Console;        // Importa ./lib/IO/Console.dryad
using Core.Types;        // Importa ./lib/Core/Types.dryad
using System.Environment as Env;  // Com alias
```

#### `use` - Para arquivos locais
```dryad
use './mymodule.dryad';  // Importa arquivo relativo
use '../utils/helper.dryad';  // Caminhos relativos suportados
```

### 2. Sistema de Exports
```dryad
export function myFunction() {
    // código
}

export class MyClass {
    // código
}
```

### 3. Package Manager Oak

#### Comandos disponíveis:
```bash
# Inicializar projeto Oak
dryad oak init

# Adicionar dependência
dryad oak add package-name

# Listar dependências
dryad oak list
```

### 4. Configuração oaklibs.json
```json
{
  "name": "my-dryad-project",
  "version": "1.0.0",
  "description": "A Dryad project using Oak package manager",
  "dependencies": {
    "math-utils": "latest"
  },
  "lib_paths": [
    "./lib"
  ]
}
```

## Estrutura do Projeto

```
projeto/
├── oaklibs.json          # Configuração Oak
├── main.dryad           # Arquivo principal
├── lib/                 # Bibliotecas (acessíveis via using)
│   ├── IO/
│   │   ├── console.dryad
│   │   └── filesystem.dryad
│   ├── Core/
│   │   └── types.dryad
│   └── System/
│       └── environment.dryad
└── modules/             # Módulos locais (acessíveis via use)
    └── mymodule.dryad
```

## Exemplos de Uso

### Arquivo principal (main.dryad)
```dryad
// Imports de bibliotecas do sistema
using IO.Console;
using Core.Types;
using System.Environment as Env;

// Import de módulo local
use './modules/helper.dryad';

// Uso das funcionalidades
Console.println("Hello World!");
let type = Types.typeof(42);
let home = Env.get("HOME");
helper.doSomething();
```

### Módulo local (modules/helper.dryad)
```dryad
// helper.dryad
export function doSomething() {
    print("Helper function called!");
}

export class Utility {
    static function calculate(x) {
        return x * 2;
    }
}
```

### Biblioteca do sistema (lib/IO/console.dryad)
```dryad
// lib/IO/console.dryad
export function println(message) {
    print(message);
}

export function log(level, message) {
    print("[" + level + "] " + message);
}
```

## Vantagens do Sistema

1. **Separação clara**: `using` para bibliotecas, `use` para arquivos locais
2. **Configurabilidade**: `oaklibs.json` centraliza configurações
3. **Package management**: Base para futuro sistema de pacotes
4. **Compatibilidade**: Funciona com o sistema existente
5. **Modularidade**: Evita poluição do escopo global

## Roadmap Futuro

1. **Registry remoto**: Suporte para baixar pacotes de repositório central
2. **Versionamento**: Controle de versões mais sofisticado
3. **Resolução de dependências**: Sistema automático de resolução
4. **Build system**: Integração com sistema de build
5. **Documentação automática**: Geração de docs a partir do código

## Status Atual

✅ **Funcional**: Import/export básico
✅ **Funcional**: Oak CLI (init, add, list)
✅ **Funcional**: Configuração via oaklibs.json
✅ **Funcional**: Namespace vs arquivo local
🔄 **Em desenvolvimento**: Registry remoto
🔄 **Planejado**: Sistema de versionamento avançado
