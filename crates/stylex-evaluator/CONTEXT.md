# stylex-evaluator

Helpers for the shapes `defineVars`, `defineConsts` and `createTheme` accept:
arbitrarily nested configuration objects, flattened into a single map of
dot-joined keys. The general JS expression evaluator is not here — it lives in
[stylex-transform](../stylex-transform/CONTEXT.md), because it needs the
transform's state.

## Language

**Nested config**:
An authored configuration object before flattening, with nesting standing for
grouping — `{ colors: { primary: 'red' } }`. Modelled per call site as
`NestedVarsValue`, `NestedStringValue` or `NestedConstsValue`, because each API
admits a different set of leaves.
_Avoid_: theme object, vars object, tree

**Leaf**:
A nested-config node that holds a value rather than more nesting. `is_vars_leaf`
decides, and the decision is not structural — a
[CSS type object](../stylex-structures/CONTEXT.md) and a conditional object are
both objects yet both are leaves.
_Avoid_: terminal, value node, scalar

**Flatten**:
Collapsing a nested config into one map whose keys join the path. Done once per
API — `flatten_nested_vars_config`, `flatten_nested_overrides_config`,
`flatten_nested_consts_config` — because the three differ in what they do with a
leaf, not in how they walk.
_Avoid_: normalize, resolve, expand

**Conditional object**:
An object whose keys are conditions (`default`, `@media …`) rather than
sub-groups. A leaf, since the compiler emits it whole.
_Avoid_: media object, variant object
