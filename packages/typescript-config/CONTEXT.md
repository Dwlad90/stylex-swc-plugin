# @stylexswc/typescript-config

The shared `tsconfig` bases every package and app extends. Config only — no
source, no build.

## Language

**Base**:
`base.json`, the settings everything inherits. `nextjs.json` and
`react-library.json` extend it; nothing extends those two in turn, so the
hierarchy is exactly two levels deep on purpose.
_Avoid_: preset, profile, default config
