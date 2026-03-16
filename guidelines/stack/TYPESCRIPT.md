# TypeScript / JS Guidelines

## Linting

- ESLint: `@stylexswc/eslint-config` (extends turbo + typescript-eslint
  - prettier). Root config in `eslint.config.mjs`.
- Some packages (e.g., `rs-compiler`) use `oxlint` instead of ESLint.

## Build Tooling

- `unplugin` is built with `tsup`.
- Other TS packages use `scripty` build scripts (configured in each package's
  `package.json` under `config.scripty.path`). Shared scripts live in
  `scripts/packages/`.

## Commands

Run from within a package directory:

- `pnpm typecheck` -- type check a package
- `pnpm format:check` -- check formatting for a package (prettier)
- `pnpm lint:check` -- check linting for a package
- `pnpm test` -- run tests for a package