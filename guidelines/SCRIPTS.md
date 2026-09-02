# Scripts

## Invoking a script from `package.json`

Name the interpreter -- `sh ./scripts/x.sh`, `bash ./scripts/y.sh`,
`node scripts/z.mjs` -- never `./scripts/x.sh` on its own. npm and pnpm hand a
script line to `cmd.exe` on Windows, which cannot execute a `*.sh` by path: it
fails with `'.' is not recognized as an internal or external command`. The
release workflow's Windows build legs run `pnpm install`, so this reaches
lifecycle scripts too; `prepare` in particular is Node for that reason, since it
runs on every install on every platform before any guard inside it can.

Match the prefix to the shebang. `sh` over a `#!/usr/bin/env bash` script drops
it into dash on Linux, where its bashisms fail.

## Root (Turbo)

`pnpm build`, `test`, `lint`, `lint:check` (JSON report), `format`,
`format:check` (oxfmt plus Rust/TOML), `test:visual`, `typecheck`.

- `pnpm test` is the local gate. It runs three legs in order, and a failure in
  one stops the legs after it: `test:scripts`, then `test:crates:workspace`,
  then `turbo run test`. Each leg goes through Turbo, so a leg whose inputs did
  not change replays from the cache instead of running. A crate package prints
  a skip line for its own `test`, because the Rust suites run once for the whole
  workspace in the second leg rather than once per crate.
- `pnpm test:crates:workspace` -- the Rust suites, through Turbo so that a
  tree with no Rust change hits the cache: `test:crates:workspace:regular`
  (`cargo nextest run --workspace --all-features`) and
  `test:crates:workspace:doc` (`cargo test --doc --workspace --all-features`).
  The regular leg runs nextest's `default` profile, which reports a failure at
  once. CI sets `NEXTEST_PROFILE=ci` on its own leg to get the two retries that
  hide an infrastructure flake; a local gate wants the opposite, so the profile
  is on the CI command rather than in the shared script. The variable is part
  of the task's cache key, so a run under one profile never replays under the
  other.
- `pnpm test:scripts` -- `node --test` over `.github/scripts` and `scripts/git`.
  `pnpm test` runs it first, CI runs it as a `basic-checks` leg, and `pre-push`
  runs it when the push touches those directories.
- `pnpm lint:shell` -- shellchecks every tracked `*.sh`; the CI counterpart of
  the pre-commit `shell` job. Folded into `lint` and `lint:check` but
  deliberately not `lint:node` -- CI runs it as its own build-free
  `basic-checks` leg, so folding it in would only run it twice.
- `pnpm hooks:validate` schema-checks `lefthook.yml`, `pnpm hooks:dump`
  re-baselines the resolved-config golden (refused while a `lefthook-local.yml`
  would pollute it), `pnpm hooks:test` runs the `scripts/git` half of
  `test:scripts`.
- `pnpm lint:dead-exports` -- knip's export scan; a `basic-checks` leg and a
  `pre-push` job.
- `pnpm audit:rust` -- `cargo deny` plus `cargo audit`; both optional installs,
  and the script says how to get them.

The manifest gate is `scripts/git/version-mismatch-check.sh`, invoked with no
arguments by the pre-commit `version-mismatch` job, the `pr-validation` matrix
and the docs-validation format job. It is `syncpack lint` plus
`node scripts/git/catalog-integrity.mjs manifests`, which asserts that every
dependency version is declared once, by name, in `pnpm-workspace.yaml` -- and
names the file, the dependency and a suggested catalog when one is not. Both
halves run on every invocation, so a failing commit reports everything it got
wrong at once.

`catalog-integrity.mjs` has a second mode,
`lockfile --baseline <file> [--current <file>]`, which asserts that every
catalog entry a baseline `pnpm-lock.yaml` resolved is still resolved by the
current one.
Nothing local runs it: its caller is the `Sync Dependencies` workflow, which
reads both lockfiles out of git -- the head commit's as dependabot wrote it
against the base commit's from before the update -- and runs this before the
sync reinstalls anything. It exists because a dependabot update can drop a
catalog entry from the lockfile, and because the reinstall that would most
likely repair that is not a guard; run after the reinstall it would only
confirm the repair.

See [Git Hooks](./git/HOOKS.md).

## Per-Package

`pnpm --filter=@stylexswc/<pkg> <script>`, where `<script>` is `build`, `test`,
`typecheck`, `format` or `format:check`; `test -- <pattern>` runs matching
tests. Linting runs once from the workspace root.

## Dependencies

```sh
pnpm install                                # install all
pnpm add --filter=@stylexswc/<pkg> <dep>    # add
pnpm remove --filter=@stylexswc/<pkg> <dep> # remove
```

## Per-Crate Rust

From a crate directory (or the root with `-p <crate-name>`):

```sh
cargo nextest run --all-features                          # tests (nextest)
cargo test --doc --all-features                           # doc tests only
cargo fmt --all                                           # format
cargo fmt -- --check                                      # check format
cargo clippy --all-targets --all-features -- -D warnings  # lint
cargo build --release                                     # release build
```

Two crates commit a Rust test file that a Node script writes:
`postcss-value-parser` has `generate:value-parser-cases` and `stylex-utils`
has `generate:parse-float-cases`. Each generator has a `:check` twin that runs
as that crate's `pretest`, so a stale file fails the gate. The convention
behind them, and the chain that crosses crates, are in
[Structure](./STRUCTURE.md).

## Benchmarks

In `crates/stylex-rs-compiler`; run `build` first (they use `dist/*.node`). All
accept `--help`. Policy: [Performance](./PERFORMANCE.md).

- `bench`: single-subject run over 22 fixtures; writes output and raw stats.
- `bench:compare`: compares Rust against Babel; writes `compare-output.txt`.
- `bench:revisions`: paired measurement; writes revision raw stats.
- `bench:verdict`: bootstrap verdict and retry; writes JSON and Markdown.
- `bench:budget`: p95 versus `budget.json`; writes JSON and Markdown.

JSON and Markdown artifacts land under `benchmark/results/`; each command's
`--help` documents the exact filenames.

Subject dirs need `package.json`, `dist/index.js` exporting `transform`, and one
`*.node`. Passing the same dir as base and candidate (with differing
`--base-label`/`--candidate-label`) is a same-vs-same calibration run.

Flags -- `bench:revisions`: `--rounds` (10), `--seed` (1), `--time` (300 ms),
repeatable `--category` (`transform|perf|rollup`) and `--fixture` substring.
`bench:verdict`: `--warn` (1.10), `--fail` (1.20), `--improvement-warn` (0.50),
`--seed` (1), `--resamples` (10000), `--confidence` (0.95), `--retry <path>`.
`bench:budget`: `--report-only`.

## Parity harness

Also in `crates/stylex-rs-compiler`; run `build` first (it reads `dist/`).
Not a test, but wired into CI. Full docs:
`crates/stylex-rs-compiler/parity/README.md`.

Which harness runs where, and why:

- `parity`, `parity:positions` and `fuzz:pseudo-order` run per pull request, in
  the `checks` matrix's `parity` leg, after a `build`. They are the oracle every
  expectation in the CSS-value corpus was derived from, and each is seconds
  long. `fuzz:pseudo-order` is there rather than nightly because it is the one
  that guards a class name.
- `fuzz:shorthand` runs on the nightly schedule only, in the `parity-sweep`
  job. It crosses an alphabet with itself -- around forty times the cost of
  `parity` -- and a value-splitter defect shows up when a value pass or the
  alphabet changes, which a nightly sweep catches as surely as a per-commit one.
- `parity:harvest:check` needs neither `dist/` nor either compiler, since it
  only scans Rust sources, so it runs ahead of this package's `vitest` suite as
  its `pretest` -- a corpus that has fallen behind the Rust tests fails rather
  than waiting to be noticed. It is a `pretest` rather than the first half of
  `test` so that `test` means "run this package's tests": the check harvests
  from Rust suites across the whole workspace, so it can fail for a declaration
  added in another crate, and that reads better as a gate in front of the suite
  than as part of it. The gate is unchanged -- a stale corpus still exits
  non-zero and `vitest` still does not run.

- `parity`: runs a corpus of CSS declarations through this compiler and through
  a pinned `@stylexjs/babel-plugin`, and reports which ones disagree on class
  name or rule text. Flags: `--only-mismatches`, repeatable `--set`
  (`reported|modules|edge|harvested`), `--filter <substring>`, `--json <path>`,
  `--font-size-px-to-rem`, `--style-resolution <name>`
  (`application-order|property-specificity|legacy-expand-shorthands`, default
  `property-specificity`).
- `parity:harvest`: regenerates `parity/corpus/harvested.json` from the Rust
  test suites. `--check` fails instead of writing when it is out of date, and
  runs as this package's `pretest`. Regenerating also
  invalidates `crates/postcss-value-parser/src/tests/cases.rs`, whose row order
  is the corpus order -- run that package's `generate:value-parser-cases` next.
- `parity:positions`: the same comparison for the position corpus -- where in a
  file a declaration sits, rather than what it holds.
- `fuzz:pseudo-order`: crosses an alphabet of pseudo-class keys and checks the
  order this compiler sorts them in against the reference compiler's, both by
  class name and by reading the order off the emitted selector. Flags:
  `--pairs <n>` (1000), `--seed <hex>` (`0x2545f4914f6cdd1d`), `--show <n>`
  (20). The run prints its seed and the Node ICU version, so a disagreement one
  machine reports can be re-run on another.
- `fuzz:shorthand`: generates shorthand values from an alphabet of token classes
  and joiners and compares how each splits. Every divergence it reports must
  belong to a refusal family in `parity/lib/refusal-families.ts`; a divergence
  no family accounts for fails the run. Flags: `--show <n>`, `--json <path>`,
  repeatable `--property <name>`.

## Coverage

`pnpm run test:coverage:workspace` (enforces 100% line coverage) and
`pnpm run test:coverage` (per crate) report only percentages;
`scripts/coverage-missing.sh` prints uncovered `file: line` ranges and exits `1`
when any measured line is uncovered.

```sh
scripts/coverage-missing.sh                     # whole workspace
scripts/coverage-missing.sh stylex_css          # one crate (or -p stylex_css)
scripts/coverage-missing.sh stylex_css --html   # add an HTML report
scripts/coverage-missing.sh stylex_css --open   # ...and open it
```

Requires nightly plus `cargo install cargo-llvm-cov cargo-nextest --locked`.
