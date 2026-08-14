# stylex-enums

The closed sets of the compiler: every choice with a fixed list of alternatives
lives here as an enum. Most of them are user-facing options, so the variant
names are part of the public configuration surface.

## Language

**Style resolution**:
Which ordering strategy decides how one style overrides another —
`application-order`, `property-specificity`, or `legacy-expand-shorthands`. The
strategy is chosen once per compile and drives shorthand expansion, not just
sort order. Implemented in [stylex-css](../stylex-css/CONTEXT.md).
_Avoid_: ordering mode, cascade mode, merge strategy

**Transformation cycle**:
The phase of the visitor pipeline for one file — `Discover`,
`TransformProducers`, `TransformConsumers`, `Finalize`, in that fixed order. A
producer creates styles (`stylex.create`, `defineVars`); a consumer spends them
(`stylex.props`, `stylex.attrs`). Driven by
[stylex-transform](../stylex-transform/CONTEXT.md).
_Avoid_: pass, stage, phase, module cycle

**CSS syntax**:
The `@property`-style type of a variable — `<length>`, `<color>`, and the rest
of `CSSSyntax`. Declared by the author, used to emit `@property` rules.
_Avoid_: value type, css type

**Property validation mode**:
What an unrecognised property does — `Throw`, `Warn` or `Silent`.
_Avoid_: strictness, error level

**Counter mode**:
Whether a `UidGenerator`'s counter is `Local` to one instance or `ThreadLocal`
and shared. Chosen so a name generated during a compile is stable regardless of
how the host schedules files.
_Avoid_: scope, counter scope

**Callable global**:
A JavaScript global the compiler folds when it is _called_ — `String`, `Number`,
`Array`, `Object` — as `CallableGlobalJS`. Distinct from the per-type method
enums (`ArrayJS`, `MathJS` and the rest), which map a _method_ name: `Math` is a
valid callee because its methods fold, but calling `Math` itself is not a fold,
so it is deliberately absent here. `name()` is the inverse of the `TryFrom<&str>`
that recognises one, so a diagnostic naming the callee never repeats a literal.
_Avoid_: builtin, wrapper, global function

**Value with default**:
A configuration value that is either a bare value or a value plus its default
form — the shape `defineVars` accepts for a variable that varies by media query.
_Avoid_: optional value, defaulted value
