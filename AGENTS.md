# AGENTS.md

Rust reimplementation of Facebook's `StyleX` CSS-in-JS compiler, built on
`NAPI-RS` and `SWC`.

## Quick Reference

- Package manager: `pnpm` >= 10 -- never use npm, yarn, or bun.
- Node: >= 20
- Rust: edition 2024 or later, toolchain 1.90.0 or later, SWC core v56 or later
- Default branch: `develop`
- Indent: 2 spaces everywhere (Rust, TS, JSON, TOML, YAML)
- Line width: 100 chars (80 for markdown)
- Rust hashing: use `FxHashMap`/`FxHashSet` from `rustc-hash`, not std
  `HashMap`/`HashSet`
- Rust errors: use `anyhow`, handle all cases with `match` -- never `.unwrap()`
  or `.expect()`
- Commits: conventional commits via `commitizen` --
  `<type>(<scope>): <description>`

## Common Commands

- `pnpm run --filter=@stylexswc/<package-name> test` -- run tests for a package
- `pnpm run --filter=@stylexswc/<package-name> lint:check` -- check linting for
  a package
- `pnpm run --filter=@stylexswc/<package-name> format:check` -- check formatting
  for a package (prettier)
- `pnpm run --filter=@stylexswc/<package-name> typecheck` -- type check a
  package
- `pnpm run --filter=@stylexswc/<package-name> test:visual` -- playwright visual
  regression for a package

Or run from the package directory without the `--filter` flag.

## Detailed Guidelines

- [Scripts & Commands](./guidelines/SCRIPTS.md)
- [Project Structure](./guidelines/STRUCTURE.md)
- [Rust / SWC](./guidelines/stack/RUST.md)
- [TypeScript / JS](./guidelines/stack/TYPESCRIPT.md)
- [Testing](./guidelines/coding/TESTING.md)
- [Plan Code](./guidelines/coding/PLAN.md)
- [Coding Workflow](./guidelines/coding/WORKFLOW.md)
- [Implement Code](./guidelines/coding/IMPLEMENT.md)
- [Git Branching](./guidelines/git/BRANCHING.md)
- [Git Conventions](./guidelines/git/CONVENTIONS.md)
- [Official StyleX Links](./guidelines/LINKS.md)

## Post actions scripts

When writing code, use necessary scripts to run code after the main action has
been performed.

Run npm scripts after the main action has been performed:

- Type checking: `pnpm typecheck`
- Formatting: `pnpm format:check`
- Linting: `pnpm lint:check`
- Testing: `pnpm test`
