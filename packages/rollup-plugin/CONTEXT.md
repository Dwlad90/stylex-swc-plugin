# @stylexswc/rollup-plugin

A standalone Rollup plugin — it does not build on
[@stylexswc/plugin-shared](../plugin-shared/CONTEXT.md), because Rollup has no
loader chain and can emit its CSS as an asset at `generateBundle`.

## Language

**Emitted stylesheet**:
The single CSS asset emitted once every module has been transformed, named by
`fileName` (default `stylex.css`). There is no carrier file and no placeholder,
and the asset is created only when at least one rule was collected.
_Avoid_: carrier, virtual css, injected css

**`[hash]` placeholder**:
The literal `[hash]` in the configured `fileName`, replaced with the first eight
hex characters of a sha256 of the finished CSS.
_Avoid_: fingerprint, content hash, template

**Cached module hook**:
`shouldTransformCachedModule`, used to re-register a cached module's rules and
then decline the re-transform, in that order. Without it, a warm build emits CSS
missing every module Rollup served from cache. In watch mode the transform hook
follows its own output's imports and harvests their rules, which is a second and
separate recovery path.
_Avoid_: cache check, incremental hook

**Rule post-processing**:
Handing the collected rules to `processStylexRules` and then to `lightningcss`
for browser targeting. Minification is not enabled. Only this plugin runs the
lightningcss step.
_Avoid_: minification, optimization, compile
