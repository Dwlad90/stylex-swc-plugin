# `StyleX Enums`

## Overview

Enum definitions for the StyleX system.

Provides shared enum types used throughout the StyleX NAPI-RS compiler crates
for type-safe pattern matching and configuration.

## Contents

- `core` -- `TransformationCycle` -- 9-variant plugin lifecycle state machine
  (Initializing → StateFilling → TransformEnter → TransformExit → PreCleaning →
  Cleaning → Recounting → InjectStyles → Skip)
- `js` -- `ArrayJS` (Map, Filter, Join), `ObjectJS` (Entries, Keys, Values,
  FromEntries), `MathJS` (Pow, Round, Ceil, Floor, Abs, …), `StringJS` -- JS
  built-in method discriminants for the evaluator
- `misc` -- `VarDeclAction` (Increase, Reduce, None), `BinaryExprType` (Number,
  String, Null) -- evaluation helpers
- `aliases` -- `Aliases` -- union of `HashMap<String, String>` and
  `HashMap<String, Vec<String>>` for path alias configuration
- `css_syntax` -- `CSSSyntax` -- CSS property syntax type discriminant
- `import_path_resolution` -- `ImportPathResolution` -- result of resolving a
  StyleX import path (relative, absolute, node_modules)
- `property_validation_mode` -- `PropertyValidationMode` -- controls whether
  unknown CSS properties raise errors or warnings
- `style_resolution` -- `StyleResolution` (ApplicationOrder,
  PropertySpecificity, LegacyExpandShorthands) -- selects the CSS property
  ordering strategy
- `style_vars_to_keep` -- `NonNullProp`, `NonNullProps` -- property-presence
  discriminants for dead-code elimination
- `sx_prop_name_param` -- `SxPropNameParam` -- discriminant for `sx` prop name
  configuration
- `theme_ref` -- `ThemeRefResult` (CssVar, Proxy, ToString) -- result of
  resolving a CSS custom-property theme reference
- `top_level_expression` -- `TopLevelExpressionKind` -- tags module-level StyleX
  API call expressions
- `value_with_default` -- `ValueWithDefault` -- wraps a CSS value with an
  optional fallback default
- `counter_mode` -- `CounterMode` -- controls UID generation strategy
- `data_structures` -- backward-compatible re-export module aliasing
  `css_syntax`, `import_path_resolution`, `style_vars_to_keep`,
  `top_level_expression`, `value_with_default`
