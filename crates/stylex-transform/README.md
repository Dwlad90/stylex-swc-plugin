# `stylex-transform`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Main SWC transform orchestration crate for the StyleX compiler. This is the
**largest** crate in the workspace (108 files, ~27,700 lines) and replaces the
former `stylex-shared` monolith. It owns the `StyleXTransform` entry point
struct, the central `StateManager`, the SWC `Fold` visitor implementation, and
every piece of logic that depends on per-file compiler state. All other crates
in the pipeline are stateless utilities; `stylex-transform` is where stateful
orchestration happens.

- **`StyleXTransform` entry point** — the single public struct that
  implements SWC's `Fold` trait, serving as the bridge between the NAPI-RS
  compiler layer and the internal transform pipeline.
- **`StateManager`** — central state holder for each file compilation,
  tracking declarations, injected styles, metadata, theme variables, and
  generated class names.
- **21 `fold_*` visitors** — fine-grained SWC `Fold` implementations for
  every relevant AST node type (`fold_module`, `fold_call_expr`,
  `fold_var_declarator`, etc.), each in its own module for readability.
- **StyleX API transformers** — dedicated modules for every StyleX API
  surface: `stylex.create`, `stylex.defineVars`, `stylex.keyframes`,
  `stylex.createTheme`, `stylex.positionTry`, `stylex.viewTransitionClass`,
  and more.
- **`styleq` compatibility layer** — runtime-compatible `styleq()`
  transform that merges class name arrays at compile time.
- **High-level transformer pipeline** — 10+ transformer modules that
  compose lower-level utilities into end-to-end API call transformations.
- **Comprehensive utility suites** — AST helpers, CSS processing
  utilities, JS evaluation helpers, and core transform utilities
  (flatten, merge, class name generation).

## Architecture

- **Layer**: 8 — StyleX Transform
- **Depends on** (all 15 internal crates):
  [`stylex-ast`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-ast),
  [`stylex-constants`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-constants),
  [`stylex-css`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css),
  [`stylex-css-order`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-order),
  [`stylex-css-parser`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-parser),
  [`stylex-css-utils`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-utils),
  [`stylex-css-values`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-values),
  [`stylex-enums`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-enums),
  [`stylex-logs`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-logs),
  [`stylex-macros`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-macros),
  [`stylex-path-resolver`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-path-resolver),
  [`stylex-regex`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-regex),
  [`stylex-structures`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-structures),
  [`stylex-types`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-types),
  [`stylex-utils`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-utils)
- **Depended on by**:
  [`stylex-rs-compiler`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

### Key Exports / Public API

| Export | Description |
| --- | --- |
| `StyleXTransform` | SWC `Fold` implementation — the plugin entry point |
| `shared::structures::state_manager::StateManager` | Per-file compiler state |

### Modules

#### `transform::fold` — SWC Fold visitor

Contains 21 `fold_*` modules, one per AST node type. Each module implements
the corresponding arm of SWC's `Fold` trait, keeping the visitor logic
granular and testable.

#### `transform::stylex` — StyleX API call transformers

Dedicated transform modules for every public StyleX API:

- `create` — `stylex.create()` style object compilation
- `define_vars` — `stylex.defineVars()` CSS custom property generation
- `keyframes` — `stylex.keyframes()` `@keyframes` rule generation
- `create_theme` — `stylex.createTheme()` theme override handling
- `position_try` — `stylex.positionTry()` anchor-positioning support
- `view_transition_class` — `stylex.viewTransitionClass()` view-transition
  name generation
- and additional API surface modules

#### `transform::styleq` — styleq compatibility layer

Compiles `styleq()` calls at build time, merging class name arrays so the
runtime `styleq` library is not required in production bundles.

#### `shared::structures::state_manager`

Central `StateManager` struct holding all per-file compiler state:
declarations, injected styles, metadata, theme variables, generated class
names, and configuration.

#### `shared::structures::functions`

Function type definitions and closure representations used during
transformation to model StyleX function arguments and return values.

#### `shared::transformers`

Ten high-level transformer modules that compose lower-level CSS, AST, and
evaluation utilities into complete API call transformations. Each
transformer corresponds to one StyleX API and is invoked by the `Fold`
visitor when the matching call expression is encountered.

#### `shared::utils::ast`

AST helper functions that depend on `StateManager`. These differ from the
stateless helpers in
[`stylex-ast`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-ast)
because they read or mutate compiler state while manipulating the AST.

#### `shared::utils::css`

CSS processing utilities, validators, and normalizers used during the
transform phase. Builds on top of
[`stylex-css`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css)
with additional state-aware logic.

#### `shared::utils::js`

JavaScript evaluation utilities — `evaluate`, `check_declaration`,
`native_functions` — that interpret JS expressions at compile time to
resolve constant values.

#### `shared::utils::core`

Core transform utilities for flattening nested style objects, merging
declarations, and generating deterministic class names.

#### `shared::enums::data_structures`

Transform-specific enum types that model intermediate data structures
used exclusively within the transform pipeline.

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

## Development

```bash
# Build
make crate-transform-build

# Lint
make crate-transform-lint

# Generate docs
make crate-transform-docs
```

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
