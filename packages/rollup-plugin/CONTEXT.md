# @stylexswc/rollup-plugin

A standalone Rollup plugin — it does not build on
[@stylexswc/plugin-shared](../plugin-shared/CONTEXT.md), because Rollup has no
loader chain and can simply emit its CSS as an asset at `generateBundle`.

## Language

**Emitted stylesheet**:
The single CSS asset emitted once every module has been transformed. There is no
carrier file and no placeholder here — the asset is created outright, and only
when at least one rule was collected.
_Avoid_: carrier, virtual css, injected css

**`[hash]` placeholder**:
The literal `[hash]` in the configured `fileName`, replaced with a sha256 prefix
of the finished CSS. Purely opt-in: a `fileName` without it is emitted verbatim.
_Avoid_: fingerprint, content hash, template

**Cached module hook**:
`shouldTransformCachedModule`, used to re-register a cached module's rules and
then decline the re-transform. Without it, a warm build emits CSS missing every
module Rollup served from cache.
_Avoid_: cache check, incremental hook

**Rule post-processing**:
Handing the collected rules to `processStylexRules` and then to `lightningcss`
for targeting and minification. Only this plugin does the lightningcss step; the
webpack-family plugins leave that to the host's CSS pipeline.
_Avoid_: minification, optimization, compile
