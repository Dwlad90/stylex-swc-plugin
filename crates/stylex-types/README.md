# `StyleX Types`

## Overview

Core type definitions for the StyleX SWC plugin. Contains pure enums, structs,
and configuration types that have no dependency on `StateManager`.

## Contents

### Enums

- `TransformationCycle` -- Plugin transformation phases
- `ArrayJS`, `ObjectJS`, `MathJS`, `StringJS` -- JS built-in operation types
- `VarDeclAction`, `BinaryExprType` -- Evaluation helpers
- `ThemeRef` enum -- Theme reference variants
- `CSSSyntax` -- CSS syntax type definitions
- `ImportPathResolution` -- Import resolution result types
- `StyleVarsToKeep` -- Style variable tracking
- `TopLevelExpression` -- Module-level expression types
- `ValueWithDefault` -- Default value handling

### Structures

- `PluginPass` -- Plugin pass context (cwd, filename)
- `NamedImportSource`, `ImportSources`, `RuntimeInjection` -- Import config
- `EnvEntry`, `JSFunction` -- Compile-time environment types
- `StyleXOptionsParams`, `StyleXOptions`, `StyleXStateOptions` -- Plugin config
- `Pair`, `OrderPair`, `Order` trait -- CSS property ordering
- `DynamicStyle`, `InlineStyle` -- Style representation types
- `PropertySpecificity`, `ShorthandsOfShorthands` -- CSS specificity
