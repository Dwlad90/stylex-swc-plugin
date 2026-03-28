# `stylex-css`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Pure CSS generation and normalization crate for the StyleX compiler pipeline.
This crate was extracted from the former `stylex-shared` monolith to
encapsulate all CSS output logic — LTR and RTL generation plus whitespace
normalization — in a single, stateless package that does **not** depend on
per-file compiler state (`StateManager`). By keeping CSS generation isolated,
the crate can be tested, profiled, and reasoned about independently from the
rest of the transform layer.

- **Stateless CSS generation** — produces CSS strings from StyleX declarations
  without requiring a `StateManager`, making every function a pure
  input → output transform.
- **Bidirectional (LTR / RTL) output** — dedicated modules generate
  left-to-right and right-to-left stylesheets, enabling automatic
  bidirectional support in downstream consumers.
- **Whitespace normalization** — a normalizer pass canonicalises whitespace in
  generated CSS so that output is deterministic and diff-friendly.
- **Orchestrates all CSS sub-crates** — pulls together
  [`stylex-css-order`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-order),
  [`stylex-css-parser`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-parser),
  [`stylex-css-utils`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-utils),
  and
  [`stylex-css-values`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-values)
  into a unified CSS processing layer.
- **Deterministic output** — given the same input declarations and
  configuration, the crate always produces byte-identical CSS, which
  simplifies snapshot testing and caching.

## Architecture

- **Layer**: 7 — CSS Processing
- **Depends on**:
  [`stylex-ast`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-ast),
  [`stylex-constants`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-constants),
  [`stylex-css-order`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-order),
  [`stylex-css-parser`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-parser),
  [`stylex-css-utils`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-utils),
  [`stylex-css-values`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-values),
  [`stylex-enums`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-enums),
  [`stylex-evaluator`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-evaluator),
  [`stylex-macros`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-macros),
  [`stylex-regex`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-regex),
  [`stylex-structures`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-structures),
  [`stylex-types`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-types)
- **Depended on by**:
  [`stylex-transform`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-transform)

### Key Exports / Public API

| Export | Description |
| --- | --- |
| `css::generate_ltr` | Generates LTR CSS rules from StyleX declarations |
| `css::generate_rtl` | Generates RTL CSS rules for bidirectional style support |
| `css::normalizers::whitespace_normalizer` | Canonicalises whitespace in CSS output |

### Modules

- **`css::generate_ltr`** — LTR CSS generation from StyleX declarations.
  Accepts resolved style values and produces left-to-right CSS rule strings.
- **`css::generate_rtl`** — RTL CSS generation for bidirectional style
  support. Mirrors logical properties and values so that a single set of
  declarations can serve both writing directions.
- **`css::normalizers::whitespace_normalizer`** — CSS whitespace
  normalization. Collapses and trims whitespace so that generated CSS is
  compact and deterministic regardless of formatting upstream.

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
make crate-css-build

# Lint
make crate-css-lint

# Generate docs
make crate-css-docs
```

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
