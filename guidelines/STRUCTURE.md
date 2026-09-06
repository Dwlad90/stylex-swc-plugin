# Project Structure

Monorepo with `pnpm` workspaces (`pnpm-workspace.yaml`): `apps/*`, `crates/*`,
`packages/*`. Turbo (`turbo.json`) orchestrates all tasks.

## Rust Crates (`crates/`)

Atomic crates arranged in a strict dependency DAG (higher layers depend only on
lower layers). Each crate owns exactly one concern -- no mixed-domain files, and
no re-export facades. For what a `pub use` may and may not do, see
[Rust / SWC](./stack/RUST.md#re-exports).

The layer decides what a crate may depend on. What each one is _responsible
for_, and the vocabulary it defines, is in
[CONTEXT-MAP.md](../CONTEXT-MAP.md).

A crate's layer is the **longest** path down to a crate with no workspace
dependency: a crate must sit above the deepest crate it reaches. Layer 0 means
no internal dependencies. The `[dependencies]` tables decide these numbers, and
`the_documented_ladder_matches_the_manifests`, in the addon's test module, fails
when the list below stops matching them. Edit the list by hand and let the
workspace suite check it. `cargo tree -p <crate> -e normal` prints the path a
number is measured along. Dev dependencies are not counted.

- **0 -- Primitives** (no internal dependencies): `postcss-value-parser`,
  `stylex-constants`, `stylex-regex`, `stylex-utils`
- **1 -- Macros and style merging**: `stylex-macros`, `stylex-styleq`
- **2 -- Domain leaves**: `stylex-ast`, `stylex-css-parser`, `stylex-enums`,
  `stylex-js`, `stylex-logs`, `stylex-path-resolver`
- **3 -- Core data structures and lookup**: `stylex-atoms`,
  `stylex-state-index`, `stylex-structures`
- **4 -- Types, diagnostics and nested config**: `stylex-diagnostics`,
  `stylex-nested-config`, `stylex-types`
- **5 -- CSS processing and compilation state**: `stylex-css`, `stylex-state`
- **6 -- Evaluation**: `stylex-evaluator`
- **7 -- StyleX transform**: `stylex-transform`
- **8 -- Compilers** (top-level consumers): `stylex-rs-compiler`

Two crates on one layer never depend on each other. A crate can depend on any
lower layer, not only the layer directly below. `stylex-css` sits _below_
`stylex-evaluator`, so CSS generation cannot call the evaluator.

`stylex-test-parser` sits outside the DAG: nothing depends on it, and it is a
developer binary rather than part of the compiler. The ladder check reads only
what the addon links, so this crate stays out without being named.

`postcss-value-parser` is third-party code rather than this project's own, and
that is why it is a crate rather than a module. It has no dependencies, not
even workspace ones, and the boundary is what keeps it that way -- a module
inside a crate can quietly reach for a sibling; a crate with an empty
`[dependencies]` cannot. The workspace `members` glob only matches
`crates/stylex-*`, so it is listed explicitly in the root `Cargo.toml`.
Third-party code _ported_ into this project belongs beside it on the same
terms.

An upstream tree carried **as upstream wrote it** would belong under `vendor/`
rather than in `crates/`, on the reasoning that a carried tree keeps its own
workspace and cannot be a member of this one. Nothing is carried that way today:
the JavaScript engine the evaluator folds method calls through was, for a
version-bound conflict its release has since resolved, and it is now an ordinary
registry dependency like every other.

Workspace dependencies are defined in the root `Cargo.toml`.

### Every crate is `rlib` only, and the addon is `cdylib` only

`crates/stylex-rs-compiler` declares `crate-type = ["cdylib"]`, because Node
loads its `cdylib` as the `.node` addon. Every other crate that builds a library
declares `crate-type = ["rlib"]`. `stylex-test-parser` builds only a binary, so
it declares no `crate-type`.

**Never give the addon an `rlib`.** LTO reaches only a final artifact. A crate
that also emits an `rlib` is not final, so it gets no LTO, and cargo prints no
warning: an unused `rlib` held the fat LTO of `profile.release` off the shipped
`.node`. Measured with `pnpm bench` in `crates/stylex-rs-compiler`, two runs per
configuration: without the `rlib`, every fixture is faster, by a median of
16% and up to 39%, with no overlap between runs. The release build then takes
about four times as long.

For every other crate the rule is build cost, not throughput. No target linked
the `cdylib` those crates used to emit. Dropping it from nineteen crates cut the
workspace-only rebuild by about a quarter, in the `dev` and `release` profiles,
and took the dynamic libraries the workspace emits from 20 to 1. Seven bench
builds found no throughput effect. Two of twelve groups read 1 to 2 points
_slower_ without it, but that sign changes from build to build. Do not add a
`cdylib` back.

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

- **Workspace coverage:** `pnpm test:coverage:workspace`. It runs
  `cargo llvm-cov nextest --workspace --all-features` with
  `--fail-uncovered-lines 0`, `--fail-uncovered-regions 0`,
  `--fail-under-functions 0`, an `--ignore-filename-regex` for test, bench and
  example paths, and one `--exclude` for each crate below. Run the script, not
  a copy of the command.
- **100% line coverage is enforced** via `--fail-uncovered-lines 0`.
- **Coverage exclusion:** Use `#[cfg_attr(coverage_nightly, coverage(off))]` on
  functions/impls that cannot be meaningfully tested (e.g., panic branches,
  mutex poisoning). On stable Rust this is a no-op; file-level exclusions via
  `--ignore-filename-regex` handle the rest. Do NOT add new exclusions without
  justification.

### Excluded from Coverage

Three lists hold the crate names and must agree: `test:coverage:workspace` in
the root `package.json`, `EXCLUDED_CRATES` in `scripts/coverage-missing.sh`, and
the `case` in `scripts/packages/test/coverage.sh`. This section says why a crate
is off the gate. Each row is permanent, with the reason stated, or temporary,
with the ticket that removes it named. Do not add a row without one of the two.
A new crate joins the gate at full coverage when it is created. A temporary row
must say why the coverage could not travel with the code.

Permanent:

- `stylex_logs` -- logging utilities
- `stylex_compiler_rs` -- NAPI-RS bindings
- `stylex_test_parser` -- test fixture parser
- `stylex_transform` -- SWC transform, tested through snapshot tests

Both temporary crates came out of the transform, which is itself off the gate.
The transform's tests had covered them, and the new crate boundary stopped that
coverage counting for them. Both tickets sit in the `split-transform-crate`
tracker (see [issue-tracker.md](../docs/agents/issue-tracker.md)).

Temporary:

- `stylex_state` -- covered through the transform until direct tests exist.
  Ticket `11-cover-the-state-crate` removes this row. The row also excludes
  the crate's `resolution` module, which was a crate on the gate and is still
  at 100%: the ticket that removes the row must keep it there.
- `stylex_evaluator` -- the same, for the evaluator moved out of the transform.
  Ticket `15-cover-the-evaluator-crate` removes this row.

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
  nothing. The `:check` runs as its own package's `pretest`, ahead of that
  package's `test` script, so a stale fixture fails locally rather than only in
  review. `scripts/git/generated-fixtures.test.mjs` asserts that wiring: each
  generator has a `:check`, something runs it, and a generator that reads
  another package declares that package as a Turbo input. It finds a generator
  by what the script does: a `generate:*` name is one signal, and a `:check`
  twin that runs the same script file is the other.

  One chain crosses crates and is easy to trip over: `postcss-value-parser`'s
  `src/tests/cases.rs` is generated from the parity corpus in
  `stylex-rs-compiler`, which is itself harvested from every Rust source in the
  workspace. Adding a test that carries a CSS value therefore invalidates a
  fixture in a crate you did not touch:

  ```text
  Rust test sources
    -> rs-compiler: parity:harvest
         -> stylex-rs-compiler/parity/corpus/harvested.json
              -> postcss-value-parser: generate:value-parser-cases
                   -> postcss-value-parser/src/tests/cases.rs
  ```

  `cases.rs` row order is the corpus order, so anything reordering the corpus
  rewrites the whole file. The corpus is not one of `postcss-value-parser`'s own
  files, so the root `turbo.json` names it in the `inputs` of that package's
  `test` task. Without it Turbo replays a cached pass and the `pretest` never
  sees the drift. Keep the entry at the root: 23 of the 24 crates reach one
  `turbo.rs.json` through a symlink, so a per-crate edit would reach them all.

  `parity:harvest` is that harvester. `parity:harvest:check` is its `:check`,
  and the `pretest` of `stylex-rs-compiler` alone. It walks the Rust sources of
  the whole workspace, so putting it on each crate would rescan the same tree
  and fail in whichever package ran first. It reads the crate names off the
  tree, not from a list: a stale list dropped the crates a split had created,
  and nothing failed. Sources marked `@generated` in their header are skipped,
  which keeps `cases.rs` out of the corpus it is generated from. Run the
  harvester after adding tests that carry CSS values. The root `turbo.json`
  also gives the `stylex-rs-compiler` `test` task an input for the workspace
  Rust sources, so a test edited in another crate moves that task's hash.

- `docs/agents/` -- machine-read configuration for the agent skills (issue
  tracker, triage labels, domain docs).
