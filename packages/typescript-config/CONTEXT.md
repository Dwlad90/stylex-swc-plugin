# @stylexswc/typescript-config

The shared `tsconfig` bases. Config only — no source, no build.

## Language

**Base**:
`base.json`, which every package that uses this package extends directly.
`nextjs.json` and `react-library.json` extend it in turn, and nothing in this
repo extends those two.
_Avoid_: preset, profile, default config
