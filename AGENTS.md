# AGENTS.md

Rust reimplementation of Facebook's `StyleX` CSS-in-JS compiler, built on
`NAPI-RS` and `SWC`.

## Quick Reference

- Package manager: `pnpm` >= 11 -- never npm, yarn, or bun. Node >= 24.11
- Rust: edition 2024+, toolchain 1.90.0+, SWC core v56+
- Default branch: `develop`
- Indent 2 spaces everywhere (Rust, TS, JSON, TOML, YAML); line width 100 chars,
  80 for markdown
- Rust hashing: `FxHashMap`/`FxHashSet` from `rustc-hash`, not std
- Rust errors: `anyhow`, handle every case with `match` -- never `.unwrap()` or
  `.expect()`
- Commits: conventional commits via `commitizen` --
  `<type>(<scope>): <description>`
- Git hooks: `lefthook` (`lefthook.yml`), installed by `pnpm install`

## Commands

Per package: `pnpm run --filter=@stylexswc/<pkg> <script>` -- `build`, `test`,
`typecheck`, `format:check`, `test:visual` (playwright visual regression); drop
`--filter` inside the package. Lint and format are root-only, one process each.
A crate's own `test` prints a skip line: the Rust suites run once for the whole
workspace, from `pnpm test:crates:workspace`.

After writing code run `pnpm typecheck`, `pnpm format:check` (Oxfmt; rustfmt and
Taplo for Rust and TOML), `pnpm lint:check` (Oxlint over every Node file,
shellcheck via `pnpm lint:shell`), `pnpm test` and, for the Rust half,
`pnpm test:crates:workspace`. TypeScript changes also need
`pnpm lint:type-aware`: it builds first and reports errors `lint:check` never
does, so it gates calling that work done. `pnpm lint:all` runs both linters.

Tests that import `@stylexswc/rs-compiler` exercise `dist/*.node`, not the Rust
sources: edit a crate and you must rebuild before the JS suite means anything --
see [Testing](./guidelines/coding/TESTING.md).

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
- [Git Hooks](./guidelines/git/HOOKS.md)
- [Official StyleX Links](./guidelines/LINKS.md)

## Agent skills

- Issue tracker: local markdown under `.scratch/<feature>/`, shared across every
  worktree and never committed --
  [docs/agents/issue-tracker.md](./docs/agents/issue-tracker.md)
- Triage labels: the five default triage roles, recorded in each issue file's
  `Status:` line --
  [docs/agents/triage-labels.md](./docs/agents/triage-labels.md)
- Domain docs: this repo is multi-context -- a `CONTEXT.md` glossary per crate
  and package, indexed by the root `CONTEXT-MAP.md`. Read before exploring an
  unfamiliar area or naming a domain concept --
  [docs/agents/domain.md](./docs/agents/domain.md)
