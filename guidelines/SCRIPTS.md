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

Run from within a crate directory (e.g., `crates/stylex-shared`):

```sh
cargo test --lib --bins --tests                           # tests
cargo fmt --all                                           # format
cargo fmt -- --check                                      # check format
cargo clippy --all-targets --all-features -- -D warnings  # lint
cargo build --release                                     # release build
```

Or run from the root directory with the `-p <crate-name>` flag.
