# @stylexswc/plugin-shared

Shared core for [`@stylexswc/webpack-plugin`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/packages/webpack-plugin)
and [`@stylexswc/rspack-plugin`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/packages/rspack-plugin).

> [!NOTE]
> This is an internal package. Use one of the bundler plugins above instead of
> depending on it directly; its API may change between minor releases.

## What lives here

- `StyleXPluginCore` — bundler-agnostic plugin logic: option normalization,
  StyleX rule bookkeeping, the Next.js App Router cross-compiler registry, and
  final CSS generation (`processStylexRules` + `transformCss` + asset
  replacement).
- `stylex-loader` — runs the `@stylexswc/rs-compiler` Rust transform on JS/TS
  modules, stores extracted rules on `module.buildInfo` (webpack transport),
  and appends a context-relative dummy CSS import carrying the rules in its
  query (rspack transport + HMR invalidation).
- `stylex-virtual-css-loader` — stamps a content-hashed dummy rule in
  development so style edits invalidate HMR; passes through in production.
- `stylex.css` — the carrier stylesheet consumers import once at their app
  entrypoint; the plugin replaces the emitted asset content with the extracted
  StyleX CSS.
- `stylex-virtual.css` — the physical target of the per-module dummy imports.
- Shared constants, option types, and utilities used by both wrapper plugins.

## Rule transports

The two bundlers move extracted rules from the loader to the plugin
differently:

- **webpack**: the loader writes rules to `module.buildInfo`; the plugin
  collects them in `compilation.hooks.finishModules`. `buildInfo` is persisted
  in webpack's filesystem cache, so cached rebuilds (where loaders don't
  re-run) keep their CSS.
- **rspack**: loaders can't persist `module.buildInfo` across the native
  boundary, so the plugin re-parses rules from the dummy-import queries
  embedded in chunk module identifiers on every compilation. Identifiers
  survive caching and rebuilding the map drops rules of deleted files.

All paths embedded in module identifiers (the `from` query parameter and the
contextified dummy request) are relative to `compiler.context`, keeping chunk
hashes machine-independent across CI runners.

## Acknowledgments

This package was inspired by
[`stylex-webpack`](https://github.com/SukkaW/stylex-webpack).
