# Rust / SWC Guidelines

## Toolchain

- Edition 2024 or later, toolchain 1.90.0 or later, SWC core v56 or later.
- WASM target `wasm32-wasip1` is supported (see `rust-toolchain.toml` for all
  targets).
- Release profile: `opt-level = "z"`, LTO enabled, symbol stripping.

## Key Modules

- `StyleXTransform<C: Comments>` in `crates/stylex-shared/src/transform/mod.rs`
  -- main SWC Fold visitor. All transform logic lives under
  `crates/stylex-shared/src/transform/`.
- `crates/stylex-shared/src/shared/structures/` -- core data models
  (`StateManager`, `PluginPass`, `StyleXOptions`, etc.).
- `crates/stylex-path-resolver/` -- path resolution and package.json parsing.
- `crates/stylex-css-parser/` -- CSS value parsing using `cssparser` crate.

## Coding Standards

- Use workspace dependencies defined in root `Cargo.toml`. Do not add duplicate
  version specs in crate-level `Cargo.toml`.
- Use `FxHashMap`/`FxHashSet` (from `rustc-hash`), not std `HashMap`/`HashSet`.
- Prefer `anyhow` for error propagation. Do not use `.expect()` or `.unwrap()`
  -- handle all cases with `match` statements.
- Use `serde` / `serde_json` for serialization. The compiler crate also uses
  `serde_plain` for simple string conversions.
- Avoid using `unsafe` blocks unless absolutely necessary.

## Commands

Run from within a crate directory:

- `cargo test --lib --bins --tests -p <package-name>` -- unit/integration tests
- `cargo fmt --all -p <package-name>` / `cargo fmt -- --check -p <package-name>`
- `cargo clippy --all-targets --all-features -- -D warnings -p <package-name>`
- `cargo build --release -p <package-name>` -- release build
