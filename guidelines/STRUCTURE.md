# Project Structure

Monorepo with `pnpm` workspaces (`pnpm-workspace.yaml`): `apps/*`, `crates/*`,
`packages/*`. Turbo (`turbo.json`) orchestrates all tasks.

## Rust Crates (`crates/`)

- `stylex-shared` -- Core SWC transform logic. The main entry point.
- `stylex-path-resolver` -- Module/path resolution, package.json parsing.
- `stylex-css-parser` -- CSS value parsing.
- `stylex-rs-compiler` -- NAPI-RS bindings exposing the compiler to Node.js.
- `stylex-test-parser` -- CLI tool for parsing test fixtures.

Dependency chain: `stylex-rs-compiler` -> `stylex-shared` ->
`stylex-path-resolver` + `stylex-css-parser`.

Workspace dependencies are defined in the root `Cargo.toml`.

## TS/JS Packages (`packages/`)

Integration plugins:

- `nextjs-plugin`, `postcss-plugin`, `unplugin`, `webpack-plugin`,
  `rollup-plugin`, `turbopack-plugin`, `jest`

Shared configs:

- `eslint-config`, `typescript-config`, `playwright`, `design-system`

## Example Apps (`apps/`)

20+ apps covering Next.js, Vite, Webpack, Rollup, Rspack, Rsbuild, Farm,
esbuild, Vue, Solid, and Storybook integrations. Each has a
`playwright.config.ts` for visual testing.

## Key Config Files

- `Cargo.toml` -- Rust workspace definition and shared dependencies.
- `pnpm-workspace.yaml` -- pnpm workspace globs.
- `turbo.json` -- task dependency graph and caching.
- `rust-toolchain.toml` -- Rust version and compilation targets.
- `rustfmt.toml` -- Rust formatting rules.
- `clippy.toml` -- Rust linting thresholds.
- `.prettierrc.js` -- TS/JS/MD formatting rules.
- `eslint.config.mjs` -- root ESLint config.
- `scripts/packages/` -- shared `scripty` build/check scripts used by most TS
  packages.
