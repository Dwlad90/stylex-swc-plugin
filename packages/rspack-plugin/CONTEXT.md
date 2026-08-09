# @stylexswc/rspack-plugin

The Rspack counterpart of
[@stylexswc/webpack-plugin](../webpack-plugin/CONTEXT.md), over the same
`StyleXPluginCore`. The two are near-identical by intent; where they differ,
Rspack's API is the reason.

## Language

**Static module rule**:
A rule pushed onto `module.rules` up front, as against webpack's per-module
loader injection. Rspack computes loader lists natively, so the loader cannot be
added per module; `loaderOrder` is expressed through `enforce` instead —
`'first'` becomes `'pre'`, `'last'` becomes `'post'`.
_Avoid_: loader config, rule injection

**Carrier copy**:
This package's own `stylex.css`. npm `exports` cannot point into another
package, so each wrapper ships a copy rather than sharing one — which is why the
shared code matches the carrier by filename pattern instead of resolved path.
_Avoid_: duplicate, vendored file, symlink
