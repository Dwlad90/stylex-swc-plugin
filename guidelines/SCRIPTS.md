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
`lockfile --baseline <file> [--current <file>]`, which asserts that every catalog
entry a baseline `pnpm-lock.yaml` resolved is still resolved by the current one.
Nothing local runs it: its caller is the `Sync Dependencies` workflow, which
reads both lockfiles out of git -- the head commit's as dependabot wrote it
against the base commit's from before the update -- and runs this before the sync
reinstalls anything. It exists because a dependabot update can drop a catalog
entry from the lockfile, and because the reinstall that would most likely repair
that is not a guard; run after the reinstall it would only confirm the repair.

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
Not a test and not wired into CI. Full docs: `crates/stylex-rs-compiler/parity/README.md`.

- `parity`: runs a corpus of CSS declarations through this compiler and through
  a pinned `@stylexjs/babel-plugin`, and reports which ones disagree on class
  name or rule text. Flags: `--only-mismatches`, repeatable `--set`
  (`reported|edge|harvested`), `--filter <substring>`, `--json <path>`,
  `--font-size-px-to-rem`.
- `parity:harvest`: regenerates `parity/corpus/harvested.json` from the Rust
  test suites. `--check` fails instead of writing when it is out of date.

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
