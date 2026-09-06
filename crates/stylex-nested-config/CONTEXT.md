# stylex-nested-config

Helpers for the shapes `defineVars`, `defineConsts` and `createTheme` accept:
arbitrarily nested configuration objects, flattened into a single map of
dot-joined keys.

## Language

**Nested config**:
An authored configuration object before flattening, where nesting stands for
grouping — `{ colors: { primary: 'red' } }`. Modelled per call site as
`NestedVarsValue`, `NestedStringValue` or `NestedConstsValue`, because each API
admits a different set of leaves.
_Avoid_: theme object, vars object, tree

**Leaf**:
A nested-config node that holds a value rather than more nesting. `is_vars_leaf`
decides, and the decision is not structural: a **conditional object** and a CSS
type object — both keys `syntax` and `value`, per `is_css_type_object`, which
becomes a [base CSS type](../stylex-structures/CONTEXT.md) — are objects, and
both are leaves.
_Avoid_: terminal, value node, scalar

**Conditional object**:
An object whose keys are conditions rather than sub-groups, per
`is_conditional_object`: a `default` key is required, and every other key must
start with `@`. A leaf, since the compiler emits it whole.
_Avoid_: media object, variant object

**Flatten**:
Collapsing a nested config into one map whose keys join the path with `.`. Once
per API — `flatten_nested_vars_config`, `flatten_nested_overrides_config`,
`flatten_nested_consts_config`. An authored key containing a `.` panics, since
the separator would make the flattened path ambiguous.
_Avoid_: normalize, resolve, expand
