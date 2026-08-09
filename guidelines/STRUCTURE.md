# Project Structure

Monorepo with `pnpm` workspaces (`pnpm-workspace.yaml`): `apps/*`, `crates/*`,
`packages/*`. Turbo (`turbo.json`) orchestrates all tasks.

## Rust Crates (`crates/`)

Atomic crates arranged in a strict dependency DAG (higher layers depend only on
lower layers). Each crate owns exactly one concern -- no re-export facades, no
mixed-domain files.

The layer decides what a crate may depend on. What each one is _responsible
for_, and the vocabulary it defines, is in
[CONTEXT-MAP.md](../CONTEXT-MAP.md).

- **0 -- Primitives** (no internal dependencies): `stylex-constants`,
  `stylex-regex`, `stylex-styleq`, `stylex-utils`
- **1 -- Macros**: `stylex-macros`
- **2 -- Domain leaves**: `stylex-enums`, `stylex-js`, `stylex-logs`,
  `stylex-css-parser`, `stylex-path-resolver`
- **3 -- Core data structures**: `stylex-structures`
- **4 -- Type system**: `stylex-types`
- **5 -- AST foundations**: `stylex-ast`
- **6 -- Evaluation and inline syntax**: `stylex-evaluator`, `stylex-atoms`
- **7 -- CSS processing**: `stylex-css`
- **8 -- StyleX transform**: `stylex-transform`
- **9 -- Compilers** (top-level consumers): `stylex-rs-compiler`

`stylex-test-parser` sits outside the DAG: nothing depends on it, and it is a
developer binary rather than part of the compiler.

Workspace dependencies are defined in the root `Cargo.toml`.

## TS/JS Packages (`packages/`)

How they stack (each package's `dependencies` is the authority):

- `plugin-shared` -- the shared core, on `rs-compiler`
- `webpack-plugin`, `rspack-plugin` -- on `plugin-shared` alone; they reach
  `rs-compiler` through it
- `turbopack-plugin` -- on `rs-compiler`, plus `plugin-shared` for
  `source-map-options` only, not the loader core
- `nextjs-plugin` -- composes the webpack, rspack and turbopack plugins rather
  than the core directly
- `rollup-plugin`, `unplugin`, `postcss-plugin`, `jest` -- on `rs-compiler`
  alone; their hosts have no loader chain to share

Shared configs: `typescript-config`, `playwright` (neither depends on the
compiler), `design-system` (does).

What each is responsible for is in [CONTEXT-MAP.md](../CONTEXT-MAP.md).

## Example Apps (`apps/`)

20+ apps covering Next.js, Vite, Webpack, Rollup, Rspack, Rsbuild, Farm,
esbuild, Vue, Solid, and Storybook integrations. Each has a
`playwright.config.ts` for visual testing.

## Testing & Coverage Infrastructure

### Test Runner: `cargo-nextest`

All Rust tests use [`cargo-nextest`](https://nexte.st/) as the primary test
runner. Configuration lives in `.config/nextest.toml`.

- **Workspace tests:** `cargo nextest run --workspace --all-features`
- **Doc tests:** `cargo test --doc --workspace --all-features` (nextest does not
  support doc tests; `cargo test --doc` is used separately)
- **Per-crate tests:** `cargo nextest run --all-features` (from crate directory)
- **CI profile:** `cargo nextest run --profile ci` (retries flaky tests)

### Coverage: `cargo-llvm-cov`

Code coverage uses [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov)
with LLVM source-based instrumentation. All flags are passed via CLI (no config
file).

- **Workspace coverage:**
  ```sh
  cargo llvm-cov nextest --workspace --all-features \
    --exclude stylex_logs --exclude stylex_compiler_rs \
    --exclude stylex_test_parser --exclude stylex_css_parser \
    --exclude stylex_transform \
    --fail-uncovered-lines 0 \
    --fail-uncovered-regions 0 \
    --fail-under-functions 0 \
    --ignore-filename-regex '<pattern>'
  ```
- **100% line coverage is enforced** via `--fail-uncovered-lines 0`.
- **Coverage exclusion:** Use `#[cfg_attr(coverage_nightly, coverage(off))]` on
  functions/impls that cannot be meaningfully tested (e.g., panic branches,
  mutex poisoning). On stable Rust this is a no-op; file-level exclusions via
  `--ignore-filename-regex` handle the rest. Do NOT add new exclusions without
  justification.

### Excluded from Coverage

These crates are excluded because they are either integration-level (tested via
other means) or thin wrappers:

- `stylex_logs` -- logging utilities
- `stylex_compiler_rs` -- NAPI-RS bindings
- `stylex_test_parser` -- test fixture parser
- `stylex_css_parser` -- CSS parser (tested independently)
- `stylex_transform` -- SWC transform (tested via snapshot tests)

## Key Config Files

- `Cargo.toml` -- Rust workspace definition and shared dependencies.
- `.config/nextest.toml` -- nextest test runner configuration.
- `pnpm-workspace.yaml` -- pnpm workspace globs.
- `turbo.json` -- task dependency graph and caching.
- `rust-toolchain.toml` -- Rust version and compilation targets.
- `rustfmt.toml` -- Rust formatting rules.
- `clippy.toml` -- Rust linting thresholds.
- `.oxfmtrc.json` -- formatting rules for all Node files (one root config).
- `.oxlintrc.jsonc` -- root Oxlint config (one root config, path overrides).
- `scripts/packages/` -- shared `scripty` build/check scripts used by most TS
  packages.
- `docs/agents/` -- machine-read configuration for the agent skills (issue
  tracker, triage labels, domain docs).
