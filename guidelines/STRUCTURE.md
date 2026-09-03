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

A crate's layer is the **longest** path from it down to a crate with no
workspace dependency. Longest rather than shortest, because the rule is that a
crate depends only on lower layers: it has to sit above the deepest thing it
reaches. So layer 0 means "no internal dependencies", and the top layer is the
addon every shipped artifact is built from.

The `[dependencies]` tables decide these numbers, and
`the_documented_ladder_matches_the_manifests`, in the addon's own test module,
fails when the list below stops matching them. So edit the list by hand and let
the workspace suite check it; `cargo tree -p <crate> -e normal` prints the path
a number is measured along. Dev dependencies are not counted: a test that
reaches sideways says nothing about what the compiler links.

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

A layer is a floor and not a ceiling: two crates on one rung never depend on
each other, and a crate two rungs up may reach any rung below it. `stylex-css`
and `stylex-evaluator` are the pair to keep straight -- CSS generation sits
_below_ evaluation, so it cannot call the evaluator.

`stylex-test-parser` sits outside the DAG: nothing depends on it, and it is a
developer binary rather than part of the compiler. It has no internal
dependencies either, so a rung would put it at 0 and say nothing true about it.
The check reads the ladder as what the addon links, which is what keeps it out
without anyone naming it.

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

### Every crate is `rlib` only, except the addon

`crates/stylex-rs-compiler` declares `crate-type = ["cdylib", "rlib"]`, because
Node loads its `cdylib` as the `.node` addon. Every other crate that builds a
library declares `crate-type = ["rlib"]`. `stylex-test-parser` builds only a
binary, so it has no library target to type and declares no `crate-type`.

This is a throughput rule, not tidiness. A `cdylib` exports its public symbols
as preemptible, so a caller cannot optimize into them, and Cargo cannot hand the
crate's bitcode to the fat LTO the `bench` and `release` profiles ask for.
Nineteen crates declared a `cdylib` that nothing ever linked. Moving 11k lines
of the evaluator into one of them cost between 4% and 18% on the memo-key
benches, and dropping the `cdylib` recovered it. Adding one back to a crate on
the compiler's path costs throughput for an artifact no one loads.

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
  example paths, and one `--exclude` for each crate below. Run the script rather
  than a copy of the command: an out-of-date copy gates something other than
  what CI gates.
- **100% line coverage is enforced** via `--fail-uncovered-lines 0`.
- **Coverage exclusion:** Use `#[cfg_attr(coverage_nightly, coverage(off))]` on
  functions/impls that cannot be meaningfully tested (e.g., panic branches,
  mutex poisoning). On stable Rust this is a no-op; file-level exclusions via
  `--ignore-filename-regex` handle the rest. Do NOT add new exclusions without
  justification.

### Excluded from Coverage

The crate names are held in three lists that must agree:
`test:coverage:workspace` in the root `package.json`, `EXCLUDED_CRATES` in
`scripts/coverage-missing.sh`, and the `case` in
`scripts/packages/test/coverage.sh`. All three point here, and this section is
the one place that says why a crate is off the gate.

Every row is either permanent, with the reason stated, or temporary, with the
ticket that removes it named. Do not add a row without one or the other. A
temporary row is an exception, not a deferral: a new crate still joins the gate
at full coverage when it is created, and a row that waits on a ticket must say
why the coverage could not travel with the code.

Permanent:

- `stylex_logs` -- logging utilities
- `stylex_compiler_rs` -- NAPI-RS bindings
- `stylex_test_parser` -- test fixture parser
- `stylex_transform` -- SWC transform, tested through snapshot tests

The next two rows are a holding position, not a judgement that the code needs no
tests. Both crates came out of the transform, which is itself off the gate. The
transform's own tests had covered them, and the new crate boundary stopped that
coverage counting for them. No line that was covered became uncovered. Both
tickets sit in the `split-transform-crate` tracker (see
[issue-tracker.md](../docs/agents/issue-tracker.md)), which holds their state.

Temporary:

- `stylex_state` -- covered through the transform until direct tests exist.
  Ticket `11-cover-the-state-crate` removes this row. The row also shelters the
  crate's `resolution` module, which was a crate on the gate before it was
  folded in and is still at 100%: the ticket that removes the row must keep it
  there.
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
  review. `scripts/git/generated-fixtures.test.mjs` asserts that wiring: that
  each generator has a `:check`, that something runs it, and that a generator
  reading another package has that package declared as a Turbo input. It finds
  a generator by what the script does, not by what it is called: the
  `generate:*` name is one signal, and a `:check` twin that runs the same
  script file is the other. The harvester below is the second kind.

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
  rewrites the whole file. Because the corpus is not one of
  `postcss-value-parser`'s own files, the root `turbo.json` names it in the
  `inputs` of that package's `test` task; without it Turbo replays a cached
  pass and the `pretest` never sees the drift. The entry is at the root because
  every crate shares one `turbo.json` through a symlink, so a per-crate change
  would reach all of them.

  `parity:harvest` is that harvester. `parity:harvest:check` is its `:check`,
  and the `pretest` of `stylex-rs-compiler` alone. Unlike the per-crate
  generators, which read one fixture's own inputs, the harvester walks the Rust
  sources of the whole workspace, so putting it on each crate would rescan the
  same tree and fail in whichever package ran first. It reads the crate names
  off the tree rather than from a list, because a list stopped naming the crates
  a split had just created and the values under them left the corpus with
  nothing failing.
  Sources marked `@generated` in their header are skipped, which is what keeps
  `cases.rs` from harvesting back into the corpus it is generated from. Run it
  after adding tests that carry CSS values. Because it reads every crate, the
  root `turbo.json` gives the `stylex-rs-compiler` `test` task an input for the
  Rust sources of the whole workspace, next to the package's own files: a Rust
  test edited in any other crate must move that task's hash, or Turbo replays a
  cached pass and the `pretest` never runs.

- `docs/agents/` -- machine-read configuration for the agent skills (issue
  tracker, triage labels, domain docs).
