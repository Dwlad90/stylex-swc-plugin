# Scripts

Use `pnpm` (>=11) exclusively -- never npm, yarn, or bun. Requires Node >=22.

## Root (Turbo)

`pnpm build`, `test`, `lint`, `lint:check` (JSON report), `format`,
`format:check` (oxfmt plus Rust/TOML), `test:visual`, `typecheck`.

## Per-Package

`pnpm --filter=@stylexswc/<pkg> <build|test|lint|typecheck|format|format:check>`;
add `test -- <pattern>` to run matching tests.

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

| Script                                           | Does                                                               | Writes to `benchmark/results/`                            |
| ------------------------------------------------ | ------------------------------------------------------------------ | --------------------------------------------------------- |
| `bench`                                          | Single-subject run, 22 fixtures                                    | `output.json`, `output-extended.txt`, `raw-stats.v1.json` |
| `bench:compare`                                  | Rust vs Babel                                                      | `compare-output.txt`                                      |
| `bench:revisions --base <dir> --candidate <dir>` | Paired measurement                                                 | `revisions-raw-stats.v1.json`                             |
| `bench:verdict --primary <raw-stats>`            | Ratios, bootstrap bound, one retry; exits 1 on a reproduced breach | `compare-revisions.verdict.v1.json`, `.summary.md`        |
| `bench:budget`                                   | p95 vs `budget.json`                                               | `budget-report.v1.json`, `budget-report.md`               |

Subject dirs need `package.json`, `dist/index.js` exporting `transform`, and one
`*.node`. Passing the same dir as base and candidate (with differing
`--base-label`/`--candidate-label`) is a same-vs-same calibration run.

Flags -- `bench:revisions`: `--rounds` (10), `--seed` (1), `--time` (1000 ms),
repeatable `--category` (`transform|perf|rollup`) and `--fixture` substring.
`bench:verdict`: `--warn` (1.10), `--fail` (1.20), `--improvement-warn` (0.50),
`--seed` (1), `--resamples` (10000), `--confidence` (0.95), `--retry <path>`.
`bench:budget`: `--report-only`.

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
