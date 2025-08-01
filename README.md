# StyleX in Rust &middot; [![GitHub license](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE) [![npm version](https://img.shields.io/npm/v/@stylexswc/rs-compiler.svg?style=flat)](https://www.npmjs.com/package/@stylexswc/rs-compiler) ![GitHub tag check runs](https://img.shields.io/github/check-runs/Dwlad90/stylex-swc-plugin/0.10.4?label=Release%20status) ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Dwlad90/stylex-swc-plugin/pr-validation.yml?branch=develop&label=Project%20Health)

This is a monorepo for an unofficial [`napi-rs`](https://napi.rs/) compiler and
an [SWC](https://swc.rs/) plugin for
[StyleX](https://github.com/facebook/stylex). Using SWC allows us to completely
ditch Babel and make StyleX faster.

**Key Benefits:**

- Faster build times by leveraging NAPI-RS/SWC instead of Babel.
- Seamless integration with Next.js SWC Compiler.
- Almost 100% compatibility with official StyleX tests.

This is specifically useful for Next.js projets as it allows us to use
[SWC Next.js Compiler](https://nextjs.org/docs/architecture/nextjs-compiler).

## Project Structure

This project is organized into several packages:

**Core:**

- [`rs-compiler`](./crates/stylex-rs-compiler) - Rust-based
  [`napi-rs`](https://napi.rs/) compiler for transforming StyleX code.

**Integration:**

- [`nextjs-plugin`](./packages/nextjs-plugin) - A wrapper for
  [`Next.JS configuration`](https://nextjs.org/docs/app/api-reference/next-config-js)
  that integrates the StyleX [napi-rs](https://napi.rs/) compiler into the
  Webpack processing pipeline.

- [`webpack-plugin`](./packages/webpack-plugin) - A `Webpack pluign` that
  integrates the StyleX [napi-rs](https://napi.rs/) compiler.

- [`rollup-plugin`](./packages/rollup-plugin) - A `Rollup plugin` that
  integrates the StyleX [napi-rs](https://napi.rs/) compiler.

- [`unplugin`](./packages/unplugin) - Plugin collection for various build tools
  that integrates the StyleX [napi-rs](https://napi.rs/) compiler.

  Supported build tools and libraries:

  - Farm
  - Rollup
  - Rsbuild
  - Rspack
  - Solid
  - Vite
  - Vue
  - Webpack

- [`postcss-plugin`](./packages/postcss-plugin) - A `PostCSS plugin` that
  integrates the StyleX [napi-rs](https://napi.rs/) compiler.

- [`jest`](./packages/jest) - Jest transformer that integrates the StyleX
  [napi-rs](https://napi.rs/) compiler.

**Utilities:**

- [`stylex-shared`](./crates/stylex-shared) - Shared Rust codebase for the
  StyleX RS compiler and SWC plugin.

- [`path-resolver`](./crates/stylex-path-resolver) - Path handling and resolving
  utilities for the StyleX NAPI-RS/SWC plugin.

- [`test-parser`](./crates/stylex-test-parser) - Parser for
  [StyleX](https://github.com/facebook/stylex) repo Jest tests that helps to
  understand last changes and keeps the project up to date

**Internal Configurations:**

- [`eslint-config`](./packages/eslint-config) - Internal
  [ESLint](https://eslint.org/) configuration

- [`typescript-config`](./packages/typescript-config) - Internal
  [Typescript](https://www.typescriptlang.org/docs/handbook/tsconfig-json.htm)
  configuration

**Other packages:**

- [`design-system`](./packages/design-system) - Design system for the StyleX
  project, intended solely for internal use with in-workspace examples to
  support consistent UI experimentation and prototyping.

- [`playwright`](./packages/playwright) - Playwright integration for StyleX
  visual regression testing.

## Development

This project includes a comprehensive Makefile that provides convenient
shortcuts for common development tasks. The Makefile integrates with both the
Node.js ecosystem (using pnpm and Turborepo) and Rust toolchain.

### Quick Start

```bash
# Setup development environment
make setup

# Show all available commands
make help

# Build all packages
make build

# Start development servers
make dev

# Run tests
make test

# Run quality checks
make quick-check
```

### Available Commands

The Makefile organizes commands into several categories:

**Setup Commands:**

- `make install` - Install all dependencies (Node.js and Rust)
- `make setup` - Full development environment setup
- `make prepare` - Prepare hooks and development tools

**Build Commands:**

- `make build` - Build all packages (Node.js and Rust)
- `make build-rust` - Build only Rust packages
- `make build-node` - Build only Node.js packages
- `make clean` - Clean all build artifacts

**Development Commands:**

- `make dev` - Start development servers
- `make watch` - Watch for changes and rebuild

**Quality Commands:**

- `make lint` - Run linting on all packages
- `make format` - Format all code
- `make typecheck` - Run TypeScript type checking
- `make quick-check` - Quick development check (format, lint, typecheck)
- `make full-check` - Full development check including tests

**Test Commands:**

- `make test` - Run all tests
- `make test-visual` - Run visual regression tests
- `make bench` - Run benchmarks

**App Commands:**

- `make apps-build` - Build all example apps
- `make apps-dev` - Start development servers for all apps
- `make apps-clean` - Clean all app build artifacts
- `make app-nextjs-dev` - Start Next.js example app in development mode
- `make app-nextjs-build` - Build Next.js example app
- `make app-nextjs-serve` - Serve Next.js example app (requires build first)
- `make app-vite-dev` - Start Vite example app in development mode
- `make app-vite-build` - Build Vite example app
- `make app-vite-serve` - Serve Vite example app (requires build first)
- `make app-webpack-dev` - Start Webpack example app in development mode
- `make app-webpack-build` - Build Webpack example app
- `make app-rollup-dev` - Start Rollup example app in development mode
- `make app-rollup-build` - Build Rollup example app
- `make apps-serve-common` - Serve commonly used example apps simultaneously

**Documentation & Release:**

- `make docs` - Generate documentation
- `make info` - Show project information

**Package Commands:**

_Bulk Package Operations:_

- `make packages-build` - Build all Node.js packages
- `make packages-lint` - Lint all Node.js packages
- `make packages-test` - Test all Node.js packages
- `make packages-typecheck` - Typecheck all Node.js packages
- `make packages-clean` - Clean all Node.js packages

_Bulk Rust Crate Operations:_

- `make crates-build` - Build all Rust crates
- `make crates-format` - Format all Rust crates
- `make crates-lint` - Lint all Rust crates
- `make crates-clean` - Clean all Rust crates
- `make crates-docs` - Generate docs for all Rust crates

_Individual Package Commands:_

Each package has individual commands available in the format
`pkg-{name}-{action}` and `crate-{name}-{action}`:

- **Node.js packages**: unplugin, nextjs, webpack, rollup, postcss, jest,
  design, playwright, eslint, typescript
- **Rust crates**: compiler, shared, resolver, parser
- **Available actions**: build, lint, test, typecheck, clean (for Node.js) /
  build, format, lint, clean, docs (for Rust)

Examples:

- `make pkg-unplugin-build` - Build unplugin package
- `make pkg-webpack-lint` - Lint webpack plugin package
- `make crate-compiler-format` - Format Rust compiler crate
- `make crate-shared-docs` - Generate docs for shared crate

### Manual Commands (Alternative to Makefile)

If you prefer to use the tools directly:

```bash
# Install dependencies
pnpm install

# Build all packages
pnpm build

# Run tests
pnpm test

# Run visual regression tests
pnpm test:visual

# Lint code
pnpm lint

# Check lint
pnpm lint:check

# Format code
pnpm format

# Check format
pnpm format:check

# Typecheck code
pnpm typecheck
```
