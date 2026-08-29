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

- **0 -- Primitives** (no internal dependencies): `postcss-value-parser`,
  `stylex-constants`, `stylex-regex`, `stylex-styleq`, `stylex-utils`
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

`postcss-value-parser` is third-party code rather than this project's own, and
that is why it is a crate rather than a module. It has no dependencies, not
even workspace ones, and the boundary is what keeps it that way -- a module
inside a crate can quietly reach for a sibling; a crate with an empty
`[dependencies]` cannot. The workspace `members` glob only matches
`crates/stylex-*`, so it is listed explicitly in the root `Cargo.toml`.
Anything else vendored belongs beside it on the same terms.

Workspace dependencies are defined in the root `Cargo.toml`.

## TS/JS Packages (`packages/`)

How they stack (each package's `dependencies` is the authority):

- `plugin-shared` -- the shared core, on `rs-compiler`
- `webpack-plugin`, `rspack-plugin` -- on `plugin-shared` alone; they reach
  `rs-compiler` through it
- `turbopack-plugin` -- on `rs-compiler`, plus `plugin-shared` for
  `source-map-options` only, not the loader core
- `nextjs-plugin` -- composes the webpack, rspack and turbopack plugins rather
  than the core directly, plus `plugin-shared/constants` for the transformable
  extension list, which the Turbopack rules are built from
- `rollup-plugin`, `jest` -- on `rs-compiler` alone; their hosts have no loader
  chain to share
- `unplugin`, `postcss-plugin` -- on `rs-compiler`, plus
  `plugin-shared/constants` for the transformable extension list

`plugin-shared/constants` and `plugin-shared/cjs-interop` are the entry points
that a config file may read. `constants` holds the extension list and the path
matcher; `cjs-interop` publishes a plugin under CommonJS, which every plugin a
host loads with `require` needs. Neither loads the compiler. Import the package
root only from code that already needs the loader core.

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
- `NOTICE.md` -- third-party work that is _in_ the repository: StyleX, `styleq`
  and `postcss-value-parser`, each with its copyright holder and where its
  licence text lives. Registry dependencies are not listed there; the lockfiles
  are the record for those. Add a row when something else is carried, ported or
  reimplemented here.
- `.oxlintrc.jsonc` -- root Oxlint config (one root config, path overrides).
- `scripts/packages/` -- shared `scripty` build/check scripts used by most TS
  packages.
- `crates/*/scripts/` -- generators for checked-in fixtures a crate's tests read
  (e.g. `stylex-utils/scripts/generate-parse-float-cases.mjs`). A generator is
  exposed as a `generate:<name>` script on the crate's `package.json`, paired
  with a `generate:<name>:check` that diffs a fresh run against what is
  committed, so the fixture cannot drift unnoticed. Generated files carry an
  `@generated` header and are never edited by hand. Pipe the generator through
  `rustfmt --edition 2024` in both scripts when its rows are long enough for
  `cargo fmt` to rewrap them -- otherwise the next `pnpm format` reformats the
  committed fixture and the `:check` fails against a generator that changed
  nothing. The `:check` runs as part of its own package's `test` script, so a
  stale fixture fails locally rather than only in review.

  One chain crosses crates and is easy to trip over: `postcss-value-parser`'s
  `src/tests/cases.rs` is generated from the parity corpus in
  `stylex-rs-compiler`, which is itself harvested from the Rust test sources of
  `stylex-css` and `stylex-transform`. Adding a test that carries a CSS value
  therefore invalidates a fixture in a crate you did not touch:

  ```text
  Rust test sources
    -> rs-compiler: parity:harvest
         -> stylex-rs-compiler/parity/corpus/harvested.json
              -> postcss-value-parser: generate:value-parser-cases
                   -> postcss-value-parser/src/tests/cases.rs
  ```

  `cases.rs` row order is the corpus order, so anything reordering the corpus
  rewrites the whole file. `parity:harvest:check` is the harvester's `:check`.
  It is deliberately _not_ in a `test` script: unlike the per-crate generators,
  which read one fixture's own inputs, the harvester walks every Rust test
  source in two crates, so running it per package would rescan the same tree
  and fail in whichever package ran first. Run it after adding tests that carry
  CSS values.

- `docs/agents/` -- machine-read configuration for the agent skills (issue
  tracker, triage labels, domain docs).
