# `stylex-structures`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Core data structures and configuration types for the StyleX compiler pipeline.
This crate defines the foundational structs — plugin state, style
representations, CSS ordering primitives, and compiler options — that every
higher-level crate depends on. It was isolated so that data definitions stay
decoupled from transform logic and CSS generation, enabling six downstream
crates to share a single source of truth for configuration and state.

- **Plugin state & context** — `PluginPass`, `TopLevelExpression`,
  `UidGenerator` for tracking compilation state
- **Style representations** — `DynamicStyle`, `InlineStyle`, `StyleVarsToKeep`,
  `OrderPair` for modelling CSS artefacts
- **Configuration** — `StyleXOptions`, `StyleXOptionsParams`,
  `StyleXStateOptions`, `ModuleResolution` for compiler behaviour
- **Ordering traits** — `Order` trait and `PropertySpecificity`,
  `ShorthandsOfShorthands` implementations for CSS property expansion
- **Import management** — `NamedImportSource`, `ImportSources`,
  `RuntimeInjection` for tracking StyleX import sources
- **Environment** — `EnvEntry`, `JSFunction` for compile-time env configuration

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
