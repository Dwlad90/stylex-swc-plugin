# @stylexswc/webpack-plugin

A thin webpack subclass of `StyleXPluginCore` from
[@stylexswc/plugin-shared](../plugin-shared/CONTEXT.md), which owns the loaders
and the extraction.

## Language

**Chunk name**:
`_stylex-webpack-generated`, the chunk the extracted CSS is split into. The only
way to change it is a static-string `cacheGroup.name`, because the asset is
found again by a named-chunk lookup.
_Avoid_: bundle, group, asset name

**Cache group**:
The `splitChunks` entry that routes the carrier stylesheet and the dummy imports
into the StyleX chunk. Installing it asserts that `splitChunks` is enabled, and
a user `cacheGroup` replaces the default outright rather than merging with it.
_Avoid_: split rule, chunk config

**Side-effects pin**:
Forcing `sideEffects: true` on the carrier stylesheet as its module is created,
since the carrier imports nothing and webpack would tree-shake it out of the
graph.
_Avoid_: keep-alive, preserve flag
