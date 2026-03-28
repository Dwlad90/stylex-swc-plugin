# `stylex-evaluator`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Pure utility functions for JS expression evaluation — expression traversal,
value extraction, and type coercion helpers used by the transform layer. This
crate was extracted so that evaluation helpers with no `StateManager` dependency
can be reused by `stylex-css` and tested in isolation from the full transform
pipeline. Every function is stateless and side-effect-free, operating only on
SWC AST nodes and primitive values.

- **Binary expression evaluation** — `evaluate_bin_expr` handles arithmetic
  (`+`, `-`, `*`, `/`, `%`, `**`), bitwise (`|`, `^`, `&`), and shift (`<<`,
  `>>`, `>>>`) operators on `f64` values
- **Hashing utilities** — `create_hash` (Murmur2 → base-36), `create_short_hash`
  (base-62, 5-char max), and `stable_hash` (generic `DefaultHasher`) for
  deterministic class name generation
- **AST helpers** — `get_expr_from_var_decl`, `normalize_expr`,
  `wrap_key_in_quotes` for SWC node manipulation
- **Numeric utilities** — `round_f64`, `hash_f64`, `sort_numbers_factory` for
  float-safe operations
- **Collection helpers** — `find_and_swap_remove` for O(1) vector removal,
  `char_code_at` for Unicode code-point access
- **Node.js integration** — `resolve_node_package_path` resolves package paths
  with CommonJS / ESM support

## Architecture

- **Layer**: 6 — Evaluation
- **Depends on**:
  [`stylex-ast`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-ast),
  [`stylex-constants`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-constants),
  [`stylex-js`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-js),
  [`stylex-macros`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-macros),
  [`stylex-path-resolver`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-path-resolver),
  [`stylex-types`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-types)
- **Depended on by**:
  [`stylex-css`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css)

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
  end

  subgraph L5["AST Foundations"]
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
  stylex_css           --> stylex_css_parser
  stylex_css           --> stylex_enums
  stylex_css           --> stylex_evaluator
  stylex_css           --> stylex_macros
  stylex_css           --> stylex_regex
  stylex_css           --> stylex_structures
  stylex_css           --> stylex_types

  stylex_transform     --> stylex_ast
  stylex_transform     --> stylex_constants
  stylex_transform     --> stylex_css
  stylex_transform     --> stylex_css_parser
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
  class stylex_enums,stylex_js,stylex_logs,stylex_css_parser,stylex_path_resolver l2
  class stylex_structures l3
  class stylex_types l4
  class stylex_ast l5
  class stylex_evaluator l6
  class stylex_css l7
  class stylex_transform l8
  class stylex_compiler_rs l9
```

</details>

---

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
