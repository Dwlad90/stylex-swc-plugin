# Project Structure

Monorepo with `pnpm` workspaces (`pnpm-workspace.yaml`): `apps/*`, `crates/*`,
`packages/*`. Turbo (`turbo.json`) orchestrates all tasks.

## Rust Crates (`crates/`)

17 atomic crates arranged in a strict dependency DAG (higher layers depend only
on lower layers):

**Layer 0 -- Primitives:** `stylex-constants`, `stylex-regex`, `stylex-utils`

**Layer 1 -- Macros:** `stylex-macros`

**Layer 2 -- Domain Leaves:** `stylex-enums`, `stylex-js`, `stylex-logs`,
`stylex-css-parser`, `stylex-path-resolver`

**Layer 3 -- Core Data Structures:** `stylex-structures`

**Layer 4 -- Type System:** `stylex-types`

**Layer 5 -- AST Foundations:** `stylex-ast`

**Layer 6 -- Evaluation:** `stylex-evaluator`

**Layer 7 -- CSS Processing:** `stylex-css`

**Layer 8 -- StyleX Transform:** `stylex-transform`

**Layer 9 -- Compilers:** `stylex-rs-compiler` (NAPI-RS bindings to Node.js)

Workspace dependencies are defined in the root `Cargo.toml`.

## TS/JS Packages (`packages/`)

Integration plugins:

- `nextjs-plugin`, `postcss-plugin`, `unplugin`, `webpack-plugin`,
  `rollup-plugin`, `turbopack-plugin`, `jest`

Shared configs:

- `eslint-config`, `typescript-config`, `playwright`, `design-system`

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
- **Coverage exclusion:** Use `#[cfg_attr(coverage_nightly, coverage(off))]`
  on functions/impls that cannot be meaningfully tested (e.g., panic branches,
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
- `.prettierrc.js` -- TS/JS/MD formatting rules.
- `eslint.config.mjs` -- root ESLint config.
- `scripts/packages/` -- shared `scripty` build/check scripts used by most TS
  packages.
