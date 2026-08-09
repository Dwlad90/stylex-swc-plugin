# @stylexswc/turbopack-plugin

A Turbopack loader, not a plugin. Turbopack runs no plugin lifecycle, so nothing
here emits CSS: the loader transforms the module and returns it, and the
stylesheet is produced separately by
[@stylexswc/postcss-plugin](../postcss-plugin/CONTEXT.md). It borrows only
`source-map-options` from
[@stylexswc/plugin-shared](../plugin-shared/CONTEXT.md) — the loader core there
has nothing a Turbopack loader can use.

## Language

**Loader**:
The single exported function Turbopack calls per module. It is the whole
integration — no compilation hook, no asset stage, nowhere to emit a stylesheet
from. Extracted metadata is logged and dropped, not registered anywhere.
_Avoid_: plugin, transformer, hook

**Import bail-out**:
The substring check against the configured `stylexImports` that returns the
module untouched before the compiler is called. A per-module cost paid on every
file, so it is a plain `includes` rather than a parse.
_Avoid_: filter, include check, guard

**Skip-warn resource**:
A resource whose empty input is expected rather than suspicious — anything
matching `empty|client-only`. Everything else empty gets a warning, because it
usually means a loader ordering mistake.
_Avoid_: ignored file, excluded module
