# `StyleX Transform`

## Overview

Main SWC transform orchestration crate for the StyleX compiler. Contains the
`StyleXTransform` struct, `StateManager`, SWC `Fold` implementation, and all
code that depends on `StateManager`.

Replaces the former `stylex-shared` crate.

## Contents

- `StyleXTransform<C>` -- Main plugin struct implementing SWC `Fold`
- `StateManager` -- Per-file transformation state (imports, style maps,
  declarations, metadata)
- `transform/fold/` -- 18 `Fold` trait method implementations
- `transform/stylex/` -- 12 StyleX API call transform handlers
- `transform/styleq/` -- StyleQ compatibility
- `shared/transformers/` -- StyleX API implementations (create, defineVars,
  keyframes, createTheme, etc.)
- `shared/structures/` -- StateManager-coupled types (FunctionConfig, PreRule,
  EvaluateResult, etc.)
- `shared/enums/data_structures/` -- Coupled enums (EvaluateResultValue,
  FlatCompiledStylesValue, InjectableStyleKind)
- `shared/utils/` -- Evaluation engine, CSS generation, AST manipulation,
  validators

## Re-exports

Re-exports modules from atomic crates so internal `use crate::shared::*` paths
continue to work:

- Constants from `stylex-constants`
- Types from `stylex-types`
- Regex from `stylex-regex`
- SWC utils from `stylex-misc`
- CSS utils from `stylex-css`
