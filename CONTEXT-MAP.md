# Context Map

One row per context that has a `CONTEXT.md`, saying what that context is
responsible for -- enough to pick the two or three worth reading. How to use
these files is in [Domain Docs](./docs/agents/domain.md); the dependency
layering between crates is in [Project Structure](./guidelines/STRUCTURE.md).

## Root

- **Root** ([CONTEXT.md](./CONTEXT.md)) -- cross-context terms only: the
  catalogs, the manifests they govern, and the release bumper.

## Crates

- **postcss-value-parser**
  ([CONTEXT.md](./crates/postcss-value-parser/CONTEXT.md)) -- third-party: a
  loose CSS value scanner that answers what a value _says_, not what it means.
- **stylex-constants**
  ([CONTEXT.md](./crates/stylex-constants/CONTEXT.md)) -- static tables and the
  string constants shared with the StyleX runtime.
- **stylex-regex** ([CONTEXT.md](./crates/stylex-regex/CONTEXT.md)) -- every
  regex the compiler uses, compiled once.
- **stylex-styleq** ([CONTEXT.md](./crates/stylex-styleq/CONTEXT.md)) -- the
  runtime class-name merger, ported to Rust.
- **stylex-utils** ([CONTEXT.md](./crates/stylex-utils/CONTEXT.md)) -- AST,
  string and collection helpers, and the hashing that generates class names.
- **stylex-macros** ([CONTEXT.md](./crates/stylex-macros/CONTEXT.md)) -- the
  error and panic vocabulary every crate raises failures through.
- **stylex-enums** ([CONTEXT.md](./crates/stylex-enums/CONTEXT.md)) -- the
  compiler's closed sets, most of them user-facing options.
- **stylex-js** ([CONTEXT.md](./crates/stylex-js/CONTEXT.md)) -- predicates,
  coercions and numeric operators over JavaScript semantics, asked before
  trusting an expression and while folding one.
- **stylex-logs** ([CONTEXT.md](./crates/stylex-logs/CONTEXT.md)) -- the logging
  backend and message format the Node host sees.
- **stylex-path-resolver**
  ([CONTEXT.md](./crates/stylex-path-resolver/CONTEXT.md)) -- import specifier
  to filesystem path, including the cases bundlers introduce.
- **stylex-css-parser**
  ([CONTEXT.md](./crates/stylex-css-parser/CONTEXT.md)) -- parser combinators
  for CSS _values_; never stylesheets or selectors.
- **stylex-structures**
  ([CONTEXT.md](./crates/stylex-structures/CONTEXT.md)) -- the data carried
  between phases: options, the per-file pass, and small value types.
- **stylex-types** ([CONTEXT.md](./crates/stylex-types/CONTEXT.md)) -- the
  output types that get serialized into JavaScript.
- **stylex-ast** ([CONTEXT.md](./crates/stylex-ast/CONTEXT.md)) -- factories
  that build SWC nodes, convertors that read them back, and the readers that
  answer what a key is written as.
- **stylex-state-index**
  ([CONTEXT.md](./crates/stylex-state-index/CONTEXT.md)) -- the lookup indices
  the state manager answers "which declarator, which call, which span" from.
- **stylex-state** ([CONTEXT.md](./crates/stylex-state/CONTEXT.md)) -- the
  per-file compilation state, and the value vocabulary it composes.
- **stylex-diagnostics**
  ([CONTEXT.md](./crates/stylex-diagnostics/CONTEXT.md)) -- code frames and the
  declaration position an error should point at.
- **stylex-nested-config**
  ([CONTEXT.md](./crates/stylex-nested-config/CONTEXT.md)) -- flattening the
  nested config objects `defineVars` and friends accept.
- **stylex-declarations**
  ([CONTEXT.md](./crates/stylex-declarations/CONTEXT.md)) -- which declaration
  binds a name, and what that declaration spells when read literally.
- **stylex-evaluator**
  ([CONTEXT.md](./crates/stylex-evaluator/CONTEXT.md)) -- what an authored
  expression folds to, or why it cannot; and the stack a fold descends on.
- **stylex-css** ([CONTEXT.md](./crates/stylex-css/CONTEXT.md)) -- a resolved
  property/value pair into injectable CSS: expansion, normalization, direction,
  priority.
- **stylex-atoms** ([CONTEXT.md](./crates/stylex-atoms/CONTEXT.md)) -- the
  `@stylexjs/atoms` inline syntax, compiled through an injected trait to avoid
  depending on the transform.
- **stylex-transform** ([CONTEXT.md](./crates/stylex-transform/CONTEXT.md)) --
  the SWC visitor and the JavaScript evaluator.
- **stylex-rs-compiler**
  ([CONTEXT.md](./crates/stylex-rs-compiler/CONTEXT.md)) -- the NAPI-RS
  boundary and the TypeScript wrapper shipped with it.
- **stylex-test-parser**
  ([CONTEXT.md](./crates/stylex-test-parser/CONTEXT.md)) -- a developer binary
  that harvests the upstream JS test suites; not part of the compiler.

## Packages

- **plugin-shared** ([CONTEXT.md](./packages/plugin-shared/CONTEXT.md)) -- the
  loader, rule registry and CSS-extraction core the webpack-family plugins are
  built from.
- **webpack-plugin** ([CONTEXT.md](./packages/webpack-plugin/CONTEXT.md)) --
  webpack-specific wiring over that core.
- **rspack-plugin** ([CONTEXT.md](./packages/rspack-plugin/CONTEXT.md)) -- the
  same, where Rspack's API forces a different shape.
- **nextjs-plugin** ([CONTEXT.md](./packages/nextjs-plugin/CONTEXT.md)) -- the
  Next.js config wrapper, and the App Router's three-compiler problem.
- **turbopack-plugin**
  ([CONTEXT.md](./packages/turbopack-plugin/CONTEXT.md)) -- a bare Turbopack
  loader; extraction happens elsewhere.
- **postcss-plugin** ([CONTEXT.md](./packages/postcss-plugin/CONTEXT.md)) --
  extraction driven from the CSS side, finding its own source files.
- **rollup-plugin** ([CONTEXT.md](./packages/rollup-plugin/CONTEXT.md)) --
  standalone Rollup plugin that emits its stylesheet as an asset.
- **unplugin** ([CONTEXT.md](./packages/unplugin/CONTEXT.md)) -- one factory
  served to nine hosts, reconciling how each gets CSS into its output.
- **jest** ([CONTEXT.md](./packages/jest/CONTEXT.md)) -- a Jest transformer,
  and the cache-key work that keeps a rebuilt binary from being replayed past.
- **playwright** ([CONTEXT.md](./packages/playwright/CONTEXT.md)) -- the shared
  visual-regression config every example app extends.
- **design-system** ([CONTEXT.md](./packages/design-system/CONTEXT.md)) --
  tokens and consts the examples import across a package boundary.
- **typescript-config**
  ([CONTEXT.md](./packages/typescript-config/CONTEXT.md)) -- the shared
  `tsconfig` bases.

`apps/` has no rows: each app consumes the packages above and coins no
vocabulary of its own.
