# `stylex-styleq`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Rust port of the [`styleq`](https://github.com/necolas/styleq) class-name merger
used by the StyleX runtime. `styleq` walks an array of (possibly nested)
compiled style objects, merges their atomic class names with last-write-wins
semantics on the underlying CSS property, and returns the final `class` string
plus any inline-style fallback. This crate provides the same algorithm in Rust
so the SWC transform can perform the merge **at compile time** and emit a
literal class string instead of leaving a `styleq()` call in the bundle.

- **JS-parity behavior** — line-for-line port of `styleq/src/styleq.js`,
  including handling of nested arrays, `false`/`null` inputs, the `$$css`
  (`COMPILED_KEY`) marker, and inline-style fallback when a non-compiled object
  is encountered (`disable_mix`, `dedupe_class_name_chunks` options match the JS
  API).
- **Generic over the value type** — the `StyleqValue` trait abstracts away what
  a "style value" is, with a built-in `StyleValue` enum that covers the common
  case (string class names, `null`, booleans, numbers, etc.). Consumers can plug
  in their own AST-aware value type without forking the algorithm.
- **Thread-safe cache** — class-name chunks are memoised in an `RwLock` over an
  `FxHashMap` keyed either by stable identity (`Identity(usize)`) or by a
  structural `FxHasher` digest (`Hash(u64)`). The cache transparently recovers
  from a poisoned lock so a single panicking writer can never permanently
  disable caching for the rest of the process.
- **Allocation-conscious** — cache entries are stored behind `Arc`, so a hit is
  a refcount bump rather than a deep clone of three owned strings plus a
  property `Vec`. Property-membership lookups use an `FxHashSet<Arc<str>>` for
  O(1) "have I seen this prop?" checks (vs. the previous O(n) `Vec::contains`),
  and the `Arc<str>` for each property name is allocated once and shared between
  the membership set and the cache chunk.
- **`Send + Sync` guarantees** — `CacheEntry` and `CacheKey` are
  compile-time-asserted `Send + Sync` so the cache layer is safe to share across
  threads when SWC is driven by Rayon/Tokio for parallel file processing.

## Public API

- `styleq(&[StyleqInput<V>]) -> StyleqResult<V>` — convenience entry point using
  the default options and a freshly-constructed cache.
- `create_styleq(options) -> Styleq<V>` — build a long-lived `Styleq` with a
  persistent cache and custom
  `StyleqOptions { disable_cache, disable_mix, dedupe_class_name_chunks, transform }`.
- `Styleq::styleq(&self, &[A]) -> StyleqResult<V>` — run the merge against a
  caller-supplied argument type implementing `StyleqArgument<V>`.
- Traits: `StyleqValue` (what is a style value?), `StyleqArgument` (what is a
  `styleq` argument? — supports nested arrays, identity-based cache keys, and
  skip flags).
- Built-in implementations: `StyleValue` enum + `StyleqInput<V>` enum cover the
  runtime-style use case so consumers don't have to define their own types for
  tests, benchmarks, or simple transforms.
- Result: `StyleqResult { class_name, inline_style, data_style_src }`.

## Architecture

- **Layer**: 0 — Primitives (no internal deps)
- **Depends on**: None (leaf crate; only `indexmap`, `log`, `rustc-hash` from
  the workspace dependency set)
- **Depended on by**:
  [`stylex-transform`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-transform)
  — used by the compile-time `styleq` transformer to fold `styleq()` calls into
  static class strings.

## Testing & Benchmarks

- Integration tests in `tests/styleq_test.rs` mirror the JS `styleq.test.js`
  suite case-by-case and additionally cover Rust-specific concerns
  (poisoned-lock recovery, `Send + Sync` of cache types, `Rc`/`Arc` value
  blanket impls).
- Criterion benchmarks in `benches/performance_bench.rs` exercise both the cold
  and hot cache paths against representative compiled-style fixtures.

```sh
pnpm run --filter=@stylexswc/stylex-styleq test
pnpm run --filter=@stylexswc/stylex-styleq bench
```

## Dependency Graph

<details>
<summary><h3>Dependency Graph</h3></summary>

```mermaid
graph TD
  subgraph L0["Primitives"]
    stylex_constants["constants"]
    stylex_regex["regex"]
    stylex_styleq["styleq"]
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

  stylex_utils         --> stylex_regex

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
  stylex_css           --> stylex_utils

  stylex_transform     --> stylex_ast
  stylex_transform     --> stylex_constants
  stylex_transform     --> stylex_css
  stylex_transform     --> stylex_css_parser
  stylex_transform     --> stylex_enums
  stylex_transform     --> stylex_evaluator
  stylex_transform     --> stylex_logs
  stylex_transform     --> stylex_macros
  stylex_transform     --> stylex_path_resolver
  stylex_transform     --> stylex_regex
  stylex_transform     --> stylex_structures
  stylex_transform     --> stylex_styleq
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

  class stylex_constants,stylex_regex,stylex_styleq,stylex_utils l0
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

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
