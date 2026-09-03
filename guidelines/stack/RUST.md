# Rust / SWC Guidelines

## Toolchain

- Edition 2024 or later, toolchain 1.90.0 or later, SWC core v56 or later.
- WASM target `wasm32-wasip1` is supported (see `rust-toolchain.toml` for all
  targets).
- Release profile: `opt-level = 3`, fat LTO, symbol stripping. Never `"z"` or
  `"s"` -- they optimize for size by slashing the inliner budget, and the hot
  path here is SWC's visitor traversal.
- `lto = true` reaches only a _final_ artifact. A crate that also emits a
  reusable `rlib` is not final, so it silently gets no LTO and cargo prints no
  warning. That is why the addon is `cdylib` only. See `Cargo.toml`, whose
  `[profile.release]` comments carry the full reasoning.

## Key Modules

- `StyleXTransform<C: Comments>` in
  `crates/stylex-transform/src/transform/mod.rs` -- main SWC visitor. All
  transform logic lives under `crates/stylex-transform/src/transform/`.
- `crates/stylex-state/` -- the per-file compilation state (`StateManager`) and
  the value types it composes.
- `crates/stylex-structures/` -- core data models (`PluginPass`,
  `StyleXOptions`, etc.).
- `crates/stylex-rs-compiler/` -- the NAPI entry point: parses, drives the
  transform, prints, and owns source-map and comment plumbing.
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

## Items Kept But Not Called

An item that is kept on purpose and has no caller must say so with
`#[allow(dead_code)]`. Do not say it with a leading underscore in the name.

The dead-code lint skips any name that starts with `_`. So the underscore does
two jobs at once: it marks the item as deliberate for a reader, and it hides the
item from the lint for the compiler. The second job is not wanted. An item that
loses its last caller by accident then stays silent, and no one learns of it.

The attribute marks the item for the reader and keeps the lint able to speak.
Write next to it why the item is kept, and whether the attribute does work: on a
public item of a library crate the lint cannot fire at all, so the attribute is
only a note.

Some underscore-named items from before this rule are still in the repo. Rename
one when you touch it. To find them all:

```bash
grep -rn --include='*.rs' -E '\bfn _[a-z]' crates/
```

## Re-exports

A `pub use` must make a boundary. It must not copy a path that exists.

Permitted:

- `lib.rs` publishes what the crate defines. The module tree stays free to
  change.
- A parent publishes an item from a private `mod`. That module has no path of
  its own, so this is the only route to it.
- A crate publishes a dependency type that its own API shows. Callers then do
  not add that dependency again and get a second version of it.
- A test prelude. Test code is not part of the crate graph.

Not permitted:

- A shim that keeps an old path alive after a move. Update the callers.
- A shorter path to a module that is already public.
- A glob, such as `pub use foo::*`. A new item upstream then changes this API.

Import from the crate that defines the item. A crate boundary here shows the
layer, so a republished item hides the true graph.

## SWC Pitfalls

These fail silently rather than at compile time, so they are worth knowing
before touching the parse/print path in `crates/stylex-rs-compiler/src/lib.rs`.

- Comments are not carried by the AST. The lexer, the transform and the printer
  each need the _same_ `Comments` store, or comments are dropped -- taking
  `/* webpackChunkName */` chunk names and `/* #__PURE__ */` annotations with
  them. Pass one `SingleThreadedComments` to all three.
- Do not hand a transform `PluginCommentsProxy` in this codebase. It only
  forwards to a wasm plugin host; outside `wasm32` every method is a no-op, and
  there is no wasm plugin build here. Prefer a real store, and keep helpers
  generic over `C: Comments` rather than naming a concrete one.
- `SourceMap::build_source_map_with_config` returns `orig` verbatim once its
  mappings are adjusted whenever an input map is chained. Anything the printer
  would have inlined -- `sourcesContent` above all -- is discarded, so options
  of that kind must be applied to the input map before printing, not passed to
  the printer and assumed to take effect.
- `adjust_mappings` keeps the input map's own tokens and shifts each by one
  delta per covering range, so the emitted column granularity of a chained map
  is the upstream map's, not this compiler's.

## Commands

Run from within a crate directory:

- `cargo nextest run --all-features -p <package-name>` -- unit/integration tests
- `cargo test --doc --all-features -p <package-name>` -- doc tests only
- `cargo fmt --all -p <package-name>` / `cargo fmt -- --check -p <package-name>`
- `cargo clippy --all-targets --all-features -- -D warnings -p <package-name>`
- `cargo build --release -p <package-name>` -- release build

## Coverage

- Use `#[cfg_attr(coverage_nightly, coverage(off))]` to exclude functions from
  coverage (replaces the old `#[cfg(not(tarpaulin_include))]` pattern). On
  stable Rust, this is a no-op; on nightly with `--cfg coverage_nightly`, it
  activates `#[coverage(off)]`.
- Do NOT add new coverage exclusions without justification.
- 100% line coverage is enforced via
  `--fail-uncovered-lines 0 --fail-uncovered-regions 0 --fail-under-functions 0`.
