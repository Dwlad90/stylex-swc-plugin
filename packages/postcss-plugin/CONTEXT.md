# @stylexswc/postcss-plugin

Extraction driven from the CSS side: rather than being handed modules by a
bundler, this plugin goes and finds the source files itself. That inversion is
what most of its vocabulary is about.

## Language

**Auto-discovery**:
Walking the project for source files that import a StyleX package, instead of
receiving them from a compiler. Deliberately narrow — `node_modules`, `.next`,
`.turbo` and the other build directories are excluded — while an explicit
`include` from the user is always honoured as given.
_Avoid_: scanning, crawling, globbing, watching

**Import source**:
A module specifier that marks a file as using StyleX: `@stylexjs/stylex` and
`stylex` by default, either a bare string or `{ from, as }`. Discovery also
derives the owning _package_ name from it, so a subpath import still identifies
the dependency to look for.
_Avoid_: entry, dependency, package

**Bundler**:
The accumulator that holds discovered rules across PostCSS passes and decides
when the output CSS needs rebuilding. Local to this package — unrelated to
webpack, Vite or any other tool called a bundler.
_Avoid_: collector, cache, compiler
