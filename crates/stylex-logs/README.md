# `stylex-logs`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Branded logging utilities for the StyleX compiler. Provides a
`[StyleX]`-prefixed, ANSI-colored log formatter and one-time logger
initialization. Isolated so that any crate needing diagnostics output
can pull in logging without depending on compiler internals.

> **Note:** This crate was formerly named `stylex-logger`.

- Emits all diagnostics with a recognizable `[StyleX]` prefix using
  ANSI color codes for terminal readability
- Thread-safe one-time initialization ensures the logger is set up
  exactly once per process
- Leaf-level crate — depends only on `stylex-macros`, keeping the
  dependency footprint minimal

## Architecture

- **Layer**: 2 — Domain Leaves
- **Depends on**: `stylex-macros`
- **Depended on by**: `stylex-rs-compiler`, `stylex-transform`

### Key Exports

- `STYLEX_PREFIX` — the branded log prefix constant
- ANSI-colored `[StyleX]` log formatter
- One-time logger initializer

### Modules

| Module | Purpose |
| --- | --- |
| `constants` | `STYLEX_PREFIX` and other logging constants |
| `formatter` | ANSI-colored `[StyleX]` log formatting |
| `initializer` | One-time logger setup and initialization |

## Dependency Graph

<details>
<summary><h3>Dependency Graph</h3></summary>

```mermaid
graph TD
  subgraph L0["Primitives"]
    stylex_constants["constants"]
    stylex_regex["regex"]
    stylex_utils["utils"]
  end
  subgraph L1["Proc Macros"]
    stylex_macros["macros"]
  end
  subgraph L2["Domain Leaves"]
    stylex_enums["enums"]
    stylex_css_values["css-values"]
    stylex_js["js"]
    stylex_logs["logs"]
    stylex_css_parser["css-parser"]
    stylex_path_resolver["path-resolver"]
  end
  subgraph L3["Core Data Structures"]
    stylex_structures["structures"]
  end
  subgraph L4["Type System"]
    stylex_types["types"]
    stylex_css_utils["css-utils"]
  end
  subgraph L5["CSS Foundations & AST"]
    stylex_css_order["css-order"]
    stylex_ast["ast"]
  end
  subgraph L6["Evaluation"]
    stylex_evaluator["evaluator"]
  end
  subgraph L7["CSS Processing"]
    stylex_css["css"]
  end
  subgraph L8["StyleX Transform"]
    stylex_transform["transform"]
  end
  subgraph L9["Compilers"]
    stylex_compiler_rs["rs-compiler"]
  end
  stylex_macros --> stylex_constants
  stylex_enums --> stylex_macros
  stylex_css_values --> stylex_macros
  stylex_js --> stylex_constants
  stylex_js --> stylex_macros
  stylex_logs --> stylex_macros
  stylex_css_parser --> stylex_macros
  stylex_path_resolver --> stylex_macros
  stylex_structures --> stylex_constants
  stylex_structures --> stylex_enums
  stylex_structures --> stylex_macros
  stylex_types --> stylex_constants
  stylex_types --> stylex_enums
  stylex_types --> stylex_macros
  stylex_types --> stylex_structures
  stylex_types --> stylex_utils
  stylex_css_utils --> stylex_structures
  stylex_css_order --> stylex_constants
  stylex_css_order --> stylex_css_values
  stylex_css_order --> stylex_structures
  stylex_css_order --> stylex_types
  stylex_ast --> stylex_constants
  stylex_ast --> stylex_macros
  stylex_ast --> stylex_types
  stylex_ast --> stylex_utils
  stylex_evaluator --> stylex_ast
  stylex_evaluator --> stylex_constants
  stylex_evaluator --> stylex_js
  stylex_evaluator --> stylex_macros
  stylex_evaluator --> stylex_path_resolver
  stylex_evaluator --> stylex_types
  stylex_css --> stylex_ast
  stylex_css --> stylex_constants
  stylex_css --> stylex_css_order
  stylex_css --> stylex_css_parser
  stylex_css --> stylex_css_utils
  stylex_css --> stylex_css_values
  stylex_css --> stylex_enums
  stylex_css --> stylex_evaluator
  stylex_css --> stylex_macros
  stylex_css --> stylex_regex
  stylex_css --> stylex_structures
  stylex_css --> stylex_types
  stylex_transform --> stylex_ast
  stylex_transform --> stylex_constants
  stylex_transform --> stylex_css
  stylex_transform --> stylex_css_order
  stylex_transform --> stylex_css_parser
  stylex_transform --> stylex_css_utils
  stylex_transform --> stylex_css_values
  stylex_transform --> stylex_enums
  stylex_transform --> stylex_logs
  stylex_transform --> stylex_macros
  stylex_transform --> stylex_path_resolver
  stylex_transform --> stylex_regex
  stylex_transform --> stylex_structures
  stylex_transform --> stylex_types
  stylex_transform --> stylex_utils
  stylex_compiler_rs --> stylex_ast
  stylex_compiler_rs --> stylex_enums
  stylex_compiler_rs --> stylex_logs
  stylex_compiler_rs --> stylex_macros
  stylex_compiler_rs --> stylex_regex
  stylex_compiler_rs --> stylex_structures
  stylex_compiler_rs --> stylex_transform
  stylex_compiler_rs --> stylex_types
  stylex_compiler_rs --> stylex_utils
  classDef l0 fill:#e8e8e8,stroke:#999,color:#333
  classDef l1 fill:#dce8ff,stroke:#6699cc,color:#333
  classDef l2 fill:#dcf5dc,stroke:#66aa66,color:#333
  classDef l3 fill:#fff3dc,stroke:#cc9933,color:#333
  classDef l4 fill:#ffe8dc,stroke:#cc6633,color:#333
  classDef l5 fill:#f5dcff,stroke:#9933cc,color:#333
  classDef l6 fill:#dcfff5,stroke:#33aaaa,color:#333
  classDef l7 fill:#ffdcdc,stroke:#cc3333,color:#333
  classDef l8 fill:#fffdc0,stroke:#aaaa33,color:#333
  classDef l9 fill:#ffc0c0,stroke:#cc0000,color:#333
  class stylex_constants,stylex_regex,stylex_utils l0
  class stylex_macros l1
  class stylex_enums,stylex_css_values,stylex_js,stylex_logs,stylex_css_parser,stylex_path_resolver l2
  class stylex_structures l3
  class stylex_types,stylex_css_utils l4
  class stylex_css_order,stylex_ast l5
  class stylex_evaluator l6
  class stylex_css l7
  class stylex_transform l8
  class stylex_compiler_rs l9
```

</details>

## Development

```bash
make crates-build          # Build all Rust crates
make crates-format         # Format all Rust crates
make crates-lint           # Lint all Rust crates
make crates-clean          # Clean all Rust crates
make crates-docs           # Generate docs for all Rust crates
```

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
