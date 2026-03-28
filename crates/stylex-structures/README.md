# `stylex-structures`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Core data structures and configuration types for the StyleX compiler
pipeline. This crate defines the foundational structs — plugin state, style
representations, CSS ordering primitives, and compiler options — that every
higher-level crate depends on. It was isolated so that data definitions stay
decoupled from transform logic and CSS generation, enabling six downstream
crates to share a single source of truth for configuration and state.

- **Plugin state & context** — `PluginPass`, `TopLevelExpression`,
  `UidGenerator` for tracking compilation state
- **Style representations** — `DynamicStyle`, `InlineStyle`,
  `StyleVarsToKeep`, `OrderPair` for modelling CSS artefacts
- **Configuration** — `StyleXOptions`, `StyleXOptionsParams`,
  `StyleXStateOptions`, `ModuleResolution` for compiler behaviour
- **Ordering traits** — `Order` trait and `PropertySpecificity`,
  `ShorthandsOfShorthands` implementations for CSS property expansion
- **Import management** — `NamedImportSource`, `ImportSources`,
  `RuntimeInjection` for tracking StyleX import sources
- **Environment** — `EnvEntry`, `JSFunction` for compile-time env
  configuration

## Architecture

- **Layer**: 3 — Core Data Structures
- **Depends on**:
  [`stylex-constants`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-constants),
  [`stylex-enums`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-enums),
  [`stylex-macros`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-macros)
- **Depended on by**:
  [`stylex-css`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css),
  [`stylex-css-order`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-order),
  [`stylex-css-utils`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-utils),
  [`stylex-rs-compiler`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler),
  [`stylex-transform`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-transform),
  [`stylex-types`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-types)

### Key Exports

| Export | Kind | Purpose |
|--------|------|---------|
| `DynamicStyle` | struct | Dynamic style with expression, key, variable name, and file path |
| `InlineStyle` | struct | Inline style with path, original expression, and transformed expression |
| `NamedImportSource` | struct | Manages import sources for StyleX calls |
| `ImportSources` | enum | Distinguishes regular and named imports |
| `RuntimeInjection` | enum | Configures runtime CSS injection strategy |
| `Order` | trait | CSS property expansion interface implemented by ordering strategies |
| `OrderPair` | struct | Key-value pair `(String, Option<String>)` for property expansion |
| `Pair` | struct | Generic string key-value pair |
| `PluginPass` | struct | Compilation context: CWD and source file name |
| `PropertySpecificity` | struct | `Order` impl for specificity-based resolution |
| `ShorthandsOfShorthands` | struct | `Order` impl for nested shorthand expansion |
| `StyleVarsToKeep` | struct | CSS variable properties to preserve during compilation |
| `EnvEntry` | enum | Compile-time environment entry (static or callable) |
| `JSFunction` | struct | Callable JS function for expression transforms |
| `StyleXOptions` | struct | Resolved compiler options |
| `StyleXOptionsParams` | struct | Raw user-facing compiler options |
| `StyleXStateOptions` | struct | Serialisable runtime state options |
| `ModuleResolution` | struct | Module resolution configuration |
| `TopLevelExpression` | struct | Top-level AST expression with kind and optional atom |
| `UidGenerator` | struct | Thread-safe unique identifier generator |

### Modules

| Module | Description |
|--------|-------------|
| `dynamic_style` | Dynamic style expression representation |
| `inline_style` | Inline style path and expression tracking |
| `named_import_source` | Import source and runtime injection management |
| `order` | `Order` trait for CSS property expansion strategies |
| `order_pair` | `OrderPair` tuple struct for expansion results |
| `pair` | Generic key-value pair |
| `plugin_pass` | Compilation context (CWD, file name) |
| `property_specificity` | Specificity-based `Order` implementation |
| `shorthands_of_shorthands` | Nested-shorthand `Order` implementation |
| `style_vars_to_keep` | CSS variable preservation tracking |
| `stylex_env` | Compile-time environment entries and JS functions |
| `stylex_options` | Compiler options, module resolution, validation modes |
| `stylex_state_options` | Serialisable runtime state options |
| `top_level_expression` | Top-level AST expression wrapper |
| `uid_generator` | Thread-safe UID generation (global, local, thread-local) |

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

  stylex_macros        --> stylex_constants

  stylex_enums         --> stylex_macros
  stylex_css_values    --> stylex_macros
  stylex_js            --> stylex_constants
  stylex_js            --> stylex_macros
  stylex_logs          --> stylex_macros
  stylex_css_parser    --> stylex_macros
  stylex_path_resolver --> stylex_macros

  stylex_structures    --> stylex_constants
  stylex_structures    --> stylex_enums
  stylex_structures    --> stylex_macros

  stylex_types         --> stylex_constants
  stylex_types         --> stylex_enums
  stylex_types         --> stylex_macros
  stylex_types         --> stylex_structures
  stylex_types         --> stylex_utils
  stylex_css_utils     --> stylex_structures

  stylex_css_order     --> stylex_constants
  stylex_css_order     --> stylex_css_values
  stylex_css_order     --> stylex_structures
  stylex_css_order     --> stylex_types
  stylex_ast           --> stylex_constants
  stylex_ast           --> stylex_macros
  stylex_ast           --> stylex_types
  stylex_ast           --> stylex_utils

  stylex_evaluator     --> stylex_ast
  stylex_evaluator     --> stylex_constants
  stylex_evaluator     --> stylex_js
  stylex_evaluator     --> stylex_macros
  stylex_evaluator     --> stylex_path_resolver
  stylex_evaluator     --> stylex_types

  stylex_css           --> stylex_ast
  stylex_css           --> stylex_constants
  stylex_css           --> stylex_css_order
  stylex_css           --> stylex_css_parser
  stylex_css           --> stylex_css_utils
  stylex_css           --> stylex_css_values
  stylex_css           --> stylex_enums
  stylex_css           --> stylex_evaluator
  stylex_css           --> stylex_macros
  stylex_css           --> stylex_regex
  stylex_css           --> stylex_structures
  stylex_css           --> stylex_types

  stylex_transform     --> stylex_ast
  stylex_transform     --> stylex_constants
  stylex_transform     --> stylex_css
  stylex_transform     --> stylex_css_order
  stylex_transform     --> stylex_css_parser
  stylex_transform     --> stylex_css_utils
  stylex_transform     --> stylex_css_values
  stylex_transform     --> stylex_enums
  stylex_transform     --> stylex_logs
  stylex_transform     --> stylex_macros
  stylex_transform     --> stylex_path_resolver
  stylex_transform     --> stylex_regex
  stylex_transform     --> stylex_structures
  stylex_transform     --> stylex_types
  stylex_transform     --> stylex_utils

  stylex_compiler_rs   --> stylex_ast
  stylex_compiler_rs   --> stylex_enums
  stylex_compiler_rs   --> stylex_logs
  stylex_compiler_rs   --> stylex_macros
  stylex_compiler_rs   --> stylex_regex
  stylex_compiler_rs   --> stylex_structures
  stylex_compiler_rs   --> stylex_transform
  stylex_compiler_rs   --> stylex_types
  stylex_compiler_rs   --> stylex_utils

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

---

## Development

```bash
make crate-structures-build    # Build the crate
make crate-structures-lint     # Lint with Clippy
make crate-structures-docs     # Generate rustdoc
```

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
