# AGENTS.md

Rust reimplementation of Facebook's `StyleX` CSS-in-JS compiler, built on
`NAPI-RS` and `SWC`.

## Quick Reference

- Package manager: `pnpm` >= 11 -- never use npm, yarn, or bun.
- Node: >= 24.11
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
- `pnpm lint:check` -- lint every Node file (one root Oxlint process; there are
  no per-package lint scripts)
- `pnpm lint:type-aware` -- type-aware lint rules; builds first, then runs
  `lint:node:type-aware` once at the root. Its findings are errors and are not
  reported by `lint:check`, so run it before calling TypeScript work done.
- `pnpm lint:all` -- `lint:check` plus `lint:type-aware`
- `pnpm format:check` -- check formatting (one root Oxfmt process, plus rustfmt
  and Taplo for Rust and TOML)
- `pnpm run --filter=@stylexswc/<package-name> typecheck` -- type check a
  package
- `pnpm run --filter=@stylexswc/<package-name> test:visual` -- playwright visual
  regression for a package

Or run from the package directory without the `--filter` flag.

## Detailed Guidelines

- [Scripts & Commands](./guidelines/SCRIPTS.md)
- [Performance Policy](./guidelines/PERFORMANCE.md)
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

## Agent skills

### Issue tracker

Local markdown under `.scratch/<feature>/`, shared across every worktree and
never committed. See
[docs/agents/issue-tracker.md](./docs/agents/issue-tracker.md).

### Triage labels

The five default triage roles, recorded in each issue file's `Status:` line.
See [docs/agents/triage-labels.md](./docs/agents/triage-labels.md).

### Domain docs

Multi-context: one `CONTEXT.md` per crate, package, and app, indexed by a root
`CONTEXT-MAP.md`. See [docs/agents/domain.md](./docs/agents/domain.md).

## Post actions scripts

When writing code, use necessary scripts to run code after the main action has
been performed.

Run npm scripts after the main action has been performed:

- Type checking: `pnpm typecheck`
- Formatting: `pnpm format:check`
- Linting: `pnpm lint:check`, and `pnpm lint:type-aware` when TypeScript changed
- Testing: `pnpm test`

Tests that exercise the native binding (anything importing
`@stylexswc/rs-compiler`) run against `dist/*.node`, not the Rust sources. Edit
a crate and you must rebuild before the JS suite means anything -- see
[Testing](./guidelines/coding/TESTING.md).
