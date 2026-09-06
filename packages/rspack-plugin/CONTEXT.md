# @stylexswc/rspack-plugin

The Rspack counterpart of
[@stylexswc/webpack-plugin](../webpack-plugin/CONTEXT.md), over the same
`StyleXPluginCore`. Where the two differ, Rspack's API is the reason.

## Language

**Static module rule**:
A rule pushed onto `module.rules` up front, as against webpack's per-module
loader injection: Rspack computes loader lists natively. `loaderOrder` is
expressed through `enforce` instead, where `'last'` becomes `'post'` and
everything else, the default `'first'` included, becomes `'pre'`.
`sideEffects: true` is pinned by rule too.
_Avoid_: loader config, rule injection

**Chunk name**:
`_stylex-rspack-generated`, which is not webpack's name. The rules that reach
the emitted asset are re-read off module identifiers in `processAssets`, a
second pass webpack has no counterpart for.
_Avoid_: bundle, asset name
