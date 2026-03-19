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

## Coding Standards

- Always use TypeScript for application and library source code in packages.
  Plain JavaScript files are only allowed for tooling and configuration (for
  example, root-level config files).
- Use strict mode (`"strict": true` in tsconfig).
- Prefer `interface` over `type` for object shapes unless a union/intersection
  is needed.
- Use explicit return types on exported functions and public methods.
- Never use `any` — use `unknown` and narrow with type guards when the type is
  truly unknown.
- Use `as const` for literal enums and constant objects.
- Use union types instead of enumerations where possible for better type safety
  and readability.
- Use `readonly` for properties that should not be modified after
  initialization.
- Do not use double-casting (e.g., `as unknown as T`) or broad object assertions
  (e.g., `as { [key: string]: unknown }`) to bypass the type system. Instead,
  utilize type guards, type predicates, or schemas (like Zod) to safely narrow
  types based on runtime logic.

## Commands

Run from within a package directory:

- `pnpm typecheck` -- type check a package
- `pnpm format:check` -- check formatting for a package (prettier)
- `pnpm lint:check` -- check linting for a package
- `pnpm test` -- run tests for a package
