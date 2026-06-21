# Scripts

Use `pnpm` (>=10) exclusively -- never npm, yarn, or bun. Requires Node >=20.

## Root-Level Commands (Turbo)

These run across the entire monorepo via turbo:

```sh
pnpm build           # build all packages
pnpm test            # run all tests
pnpm lint            # lint all packages
pnpm lint:check      # lint with JSON report output
pnpm format          # format TS/MD files (prettier)
pnpm format:check    # check formatting without changes
pnpm test:visual     # Playwright visual regression (all apps)
pnpm typecheck       # TypeScript type checking
```

## Per-Package Commands

Use `--filter` to target a specific package:

```sh
pnpm --filter=@stylexswc/<package-name> build
pnpm --filter=@stylexswc/<package-name> test
pnpm --filter=@stylexswc/<package-name> lint
pnpm --filter=@stylexswc/<package-name> typecheck
pnpm --filter=@stylexswc/<package-name> format
pnpm --filter=@stylexswc/<package-name> format:check
```

Run a subset of tests matching a pattern:

```sh
pnpm --filter=@stylexswc/<package-name> test -- <pattern>
```

## Dependencies

```sh
pnpm install                                              # install all
pnpm add --filter=@stylexswc/<package-name> <dep>         # add
pnpm remove --filter=@stylexswc/<package-name> <dep>      # remove
```

## Per-Crate Rust Commands

Run from within a crate directory (e.g., `crates/stylex-css`):

```sh
cargo nextest run --all-features                          # tests (nextest)
cargo test --doc --all-features                           # doc tests only
cargo fmt --all                                           # format
cargo fmt -- --check                                      # check format
cargo clippy --all-targets --all-features -- -D warnings  # lint
cargo build --release                                     # release build
```

Or run from the root directory with the `-p <crate-name>` flag.

## Coverage Commands

```sh
# Workspace coverage (enforces 100% line coverage)
pnpm run test:coverage:workspace

# Per-crate coverage (from crate directory)
pnpm run test:coverage
```

### Finding uncovered lines

`test:coverage:workspace` only reports pass/fail percentages. When it fails, use
`scripts/coverage-missing.sh` to print the exact `file: line` ranges that no
test exercises, so you know precisely what needs a test.

```sh
# Whole workspace (same crate excludes as test:coverage:workspace)
scripts/coverage-missing.sh

# Single crate — much faster while iterating on one module
scripts/coverage-missing.sh stylex_css
scripts/coverage-missing.sh -p stylex_css   # equivalent, explicit flag

# Also emit an HTML report (and open it in a browser with --open)
scripts/coverage-missing.sh stylex_css --html
scripts/coverage-missing.sh stylex_css --open

scripts/coverage-missing.sh --help
```

The uncovered `file: line` list is printed beneath the per-file table. The
script exits `0` when every measured line is covered and `1` otherwise, so it
doubles as a local gate. It requires the nightly toolchain plus `cargo-llvm-cov`
and `cargo-nextest`:

```sh
rustup toolchain install nightly
cargo install cargo-llvm-cov cargo-nextest --locked
```
