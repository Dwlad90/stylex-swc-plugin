# `StyleX Data Structures`

## Overview

Domain-specific data structures for the StyleX system.

Provides specialized types such as `TopLevelExpression` and `StyleVarsToKeep`
used in the StyleX NAPI-RS compiler's transformation pipeline.

## Contents

- `top_level_expression` --
  `TopLevelExpression(TopLevelExpressionKind, Expr, Option<Atom>)` -- wraps a
  module-level SWC expression with its kind tag and optional binding name; used
  to track `stylex.create`, `stylex.defineVars`, etc. calls during the transform
  pass
- `style_vars_to_keep` -- `StyleVarsToKeep(Atom, NonNullProp, NonNullProps)` --
  records which style variables must be preserved after dead-code elimination,
  keyed by variable name and property path
