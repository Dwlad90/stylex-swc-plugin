# `stylex-css-order`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Deterministic CSS property ordering and specificity ranking for StyleX. This
crate encapsulates three ordering strategies — `ApplicationOrder`,
`LegacyExpandShorthandsOrder`, and `PropertySpecificityOrder` — together
with their associated constant tables of shorthand expansions and alias
mappings. It was extracted so ordering logic can evolve independently of CSS
generation and transform passes.

- **Three strategies, one trait** — each strategy is a zero-sized struct
  implementing the `Order` trait from `stylex-structures`, providing a
  single `get_expansion_fn(property)` entry point
- **Shorthand expansion** — `Shorthands` structs map CSS shorthand
  properties (e.g. `border`, `flex`, `grid`) to their longhand
  equivalents, returning `Vec<OrderPair>`
- **Alias resolution** — `Aliases` structs map deprecated or alternate
  property names to their canonical forms before expansion
- **Graduated strictness** — `ApplicationOrder` supports 120+ properties,
  `LegacyExpandShorthandsOrder` is moderate (~40), and
  `PropertySpecificityOrder` is the most restrictive (~30)

## Architecture

- **Layer**: 5 — CSS Foundations & AST
- **Depends on**:
  [`stylex-constants`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-constants),
  [`stylex-css-values`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css-values),
  [`stylex-structures`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-structures),
  [`stylex-types`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-types)
- **Depended on by**:
  [`stylex-css`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css),
  [`stylex-transform`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-transform)

### Key Exports

| Export | Kind | Purpose |
|--------|------|---------|
| `ApplicationOrder` | struct | Broadest expansion strategy (~120+ CSS properties) |
| `LegacyExpandShorthandsOrder` | struct | Moderate expansion with backward-compatible rules (~40 properties) |
| `PropertySpecificityOrder` | struct | Most restrictive expansion for specificity calculations (~30 properties) |

All three implement the `Order` trait:

```rust
pub trait Order {
  fn get_expansion_fn(property: &str)
    -> Option<fn(Option<String>) -> Result<Vec<OrderPair>, String>>;
}
```

Each struct's `get_expansion_fn` first checks `Aliases::get()`, then
falls back to `Shorthands::get()` from its corresponding constants
module.

### Modules

| Module | Description |
|--------|-------------|
| `constants::application_order` | `Shorthands` (~148 fns) and `Aliases` for full property expansion |
| `constants::legacy_expand_shorthands_order` | `Shorthands` (~83 fns) and `Aliases` for legacy-compatible expansion |
| `constants::property_specificity_order` | `Shorthands` (~62 fns) and `Aliases` for specificity-based expansion |
| `structures::application_order` | `ApplicationOrder` struct implementing `Order` |
| `structures::legacy_expand_shorthands_order` | `LegacyExpandShorthandsOrder` struct implementing `Order` |
| `structures::property_specificity_order` | `PropertySpecificityOrder` struct implementing `Order` |

### Strategy Comparison

| Strategy | Scope | Behaviour |
|----------|-------|-----------|
| **ApplicationOrder** | General-purpose | Expands most shorthands into all longhand components |
| **LegacyExpandShorthandsOrder** | Backward-compat | Limited expansion; many properties return errors |
| **PropertySpecificityOrder** | Specificity | Minimal expansion focused on logical / writing-mode properties |

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
make crate-css-order-build    # Build the crate
make crate-css-order-lint     # Lint with Clippy
make crate-css-order-docs     # Generate rustdoc
```

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
