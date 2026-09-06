# @stylexswc/turbopack-plugin

A Turbopack loader. Turbopack runs no plugin lifecycle, so nothing here emits
CSS: the loader transforms the module and returns it, and the stylesheet is
produced separately by
[@stylexswc/postcss-plugin](../postcss-plugin/CONTEXT.md).

## Language

**Loader**:
The function Turbopack calls per module, exported from its own entry point. It
is the whole integration — there is nowhere to emit a stylesheet from, so
extracted metadata is logged and dropped. The package's main export is a plugin
that throws, since Turbopack runs no webpack plugin.
_Avoid_: plugin, transformer, hook

**Import bail-out**:
The substring check against the configured `stylexImports` that returns the
module untouched before the compiler is called. It runs after the empty-input
check, tests both fields of a `{ from, as }` object, and is a plain `includes`
rather than a parse, because it is paid on every file.
_Avoid_: filter, include check, guard

**Skip-warn resource**:
A resource whose empty input is expected rather than suspicious — anything
matching `empty|client-only`. Everything else empty gets a warning.
_Avoid_: ignored file, excluded module
