# @stylexswc/webpack-plugin

A thin webpack subclass of `StyleXPluginCore` from
[@stylexswc/plugin-shared](../plugin-shared/CONTEXT.md), which owns the loaders
and the extraction. What lives here is the webpack-specific wiring the core
cannot do generically.

## Language

**Chunk name**:
`_stylex-webpack-generated` — the chunk the extracted CSS is split into, and the
name a consumer sees in build output. Overridable, because a project may already
have a chunk by that name.
_Avoid_: bundle, group, asset name

**Cache group**:
The `splitChunks` entry that routes the carrier stylesheet and the dummy imports
into the StyleX chunk. Installed with an assertion rather than silently, so a
conflicting user configuration fails loudly.
_Avoid_: split rule, chunk config

**Side-effects pin**:
Forcing `sideEffects: true` on the carrier stylesheet as its module is created.
The carrier imports nothing, so without the pin webpack tree-shakes it out of
the graph and there is no asset left to replace.
_Avoid_: keep-alive, preserve flag
