# `stylex-transform`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Main SWC transform orchestration crate for the StyleX compiler, and still the
largest in the workspace (185 files, ~47,000 lines including tests). It owns the
`StyleXTransform` entry point struct, the SWC `Fold` visitor implementation, the
StyleX API transformers and the style-semantics layer.

The per-file compiler state it threads through all of that is **not** here: the
`StateManager` and the value types it composes live one layer down, in
[`stylex-state`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-state).

- **`StyleXTransform` entry point** — the single public struct that implements
  SWC's `Fold` trait, serving as the bridge between the NAPI-RS compiler layer
  and the internal transform pipeline.
- **21 `fold_*` visitors** — fine-grained SWC `Fold` implementations for every
  relevant AST node type (`fold_module`, `fold_call_expr`,
  `fold_var_declarator`, etc.), each in its own module for readability.
- **StyleX API transformers** — dedicated modules for every StyleX API surface:
  `stylex.create`, `stylex.defineVars`, `stylex.keyframes`,
  `stylex.createTheme`, `stylex.positionTry`, `stylex.viewTransitionClass`, and
  more.
- **`styleq` compatibility layer** — runtime-compatible `styleq()` transform
  that merges class name arrays at compile time.
- **High-level transformer pipeline** — 10+ transformer modules that compose
  lower-level utilities into end-to-end API call transformations.
- **Comprehensive utility suites** — AST helpers, CSS processing utilities, JS
  evaluation helpers, and core transform utilities (flatten, merge, class name
  generation).

## Architecture

#### `transform::stylex` — StyleX API call transformers

Dedicated transform modules for every public StyleX API:

- `create` — `stylex.create()` style object compilation
- `props` — `stylex.props()` property object compilation
- `define_vars` — `stylex.defineVars()` CSS custom property generation
- `define_consts` — `stylex.defineConsts()` CSS custom property generation
- `default_marker` — `stylex.defaultMarker()` default marker handling
- `define_marker` — `stylex.defineMarker()` define marker handling
- `env` — `stylex.env()` environment variable handling
- `keyframes` — `stylex.keyframes()` `@keyframes` rule generation
- `create_theme` — `stylex.createTheme()` theme override handling
- `position_try` — `stylex.positionTry()` anchor-positioning support
- `view_transition_class` — `stylex.viewTransitionClass()` view-transition name
  generation
- `when` — `stylex.when()` conditional style generation
- and additional API surface modules

#### `transform::styleq` — styleq compatibility layer

Compiles `styleq()` calls at build time, merging class name arrays so the
runtime `styleq` library is not required in production bundles.

#### `shared::structures`

The pre-rule chain the style-semantics layer builds, and the evaluator's own
result types. The state manager, the function configs and the compiled-style
value types moved to `stylex-state`.

#### `shared::transformers`

Ten high-level transformer modules that compose lower-level CSS, AST, and
evaluation utilities into complete API call transformations. Each transformer
corresponds to one StyleX API and is invoked by the `Fold` visitor when the
matching call expression is encountered.

#### `shared::utils::ast`

AST helper functions that read or write the compilation state. These differ from
the stateless helpers in
[`stylex-ast`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-ast)
because they read or mutate compiler state while manipulating the AST.

#### `shared::utils::css`

CSS processing utilities and normalizers used during the transform phase.
Builds on top of
[`stylex-css`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-css)
with additional state-aware logic.

#### `shared::utils::js`

JavaScript evaluation utilities — `evaluate`, `check_declaration`,
`native_functions` — that interpret JS expressions at compile time to resolve
constant values.

#### `shared::utils::core`

Core transform utilities for flattening nested style objects, merging
declarations, and generating deterministic class names.

#### `shared::enums::data_structures`

The two intermediate enums used only inside the transform pipeline. The
evaluated-value and compiled-style enums moved to `stylex-state`.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
