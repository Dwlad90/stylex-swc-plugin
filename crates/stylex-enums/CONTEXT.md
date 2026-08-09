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

**Value with default**:
A configuration value that is either a bare value or a value plus its default
form — the shape `defineVars` accepts for a variable that varies by media query.
_Avoid_: optional value, defaulted value
