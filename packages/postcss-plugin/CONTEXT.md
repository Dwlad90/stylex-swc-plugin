# @stylexswc/postcss-plugin

Extraction driven from the CSS side: rather than being handed modules by a
bundler, this plugin finds the source files itself. `@stylex;` is where the
extracted CSS is written.

## Language

**Auto-discovery**:
Walking the project for source files that import a StyleX package. The exclude
list — `node_modules`, `.next`, `.turbo` and the other build directories —
applies only when the user gave no `include`, which drops it entirely.
`**/*.d.ts` and `**/*.flow` are always excluded. The default glob comes from
[plugin-shared](../plugin-shared/CONTEXT.md), so discovery and the bundler
plugins see one extension set.
_Avoid_: scanning, crawling, globbing, watching

**Import source**:
A module specifier that marks a file as using StyleX: `@stylexjs/stylex` and
`stylex` by default. The PostCSS option replaces those defaults; the compiler
options' own list merges with them. Discovery also derives the owning _package_
name, so a subpath import still identifies the dependency.
_Avoid_: entry, dependency, package

**Bundler**:
`createBundler`, the accumulator that holds discovered rules across PostCSS
passes. Local to this package, and unrelated to webpack or Vite. When the output
CSS needs rebuilding is `createBuilder`'s mtime map instead, which also drops
the rules of a file that has gone.
_Avoid_: collector, cache, compiler
