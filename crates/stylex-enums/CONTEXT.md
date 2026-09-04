# stylex-enums

The closed sets of the compiler: every choice with a fixed list of alternatives
lives here as an enum. Three are user-facing configuration — style resolution,
property validation mode and CSS syntax — so their variant names are part of the
public surface.

## Language

**Style resolution**:
Which ordering strategy decides how one style overrides another —
`application-order`, `property-specificity` or `legacy-expand-shorthands`.
Chosen once per compile, and it drives shorthand expansion, not just sort order.
Implemented in [stylex-css](../stylex-css/CONTEXT.md).
_Avoid_: ordering mode, cascade mode, merge strategy

**Transformation cycle**:
The phase of the visitor pipeline for one file — `Discover`,
`TransformProducers`, `TransformConsumers`, `Finalize`, in that fixed order.
Which call belongs to which cycle is the
[producer / consumer](../stylex-transform/CONTEXT.md) split.
_Avoid_: pass, stage, phase, module cycle

**CSS syntax**:
The `@property`-style type of a variable — `<length>`, `<color>`, and the rest
of `CSSSyntax`. Declared by the author, and used to emit `@property` rules.
_Avoid_: value type, css type

**Property validation mode**:
What an unrecognised property does — `Throw`, `Warn` or `Silent`. `Silent` is
the default.
_Avoid_: strictness, error level

**Counter mode**:
Which counter a `UidGenerator` reads. Only `Local`, one per instance, and
`ThreadLocal`, shared across the thread, are constructed; `ThreadLocal` exists
so tests running in parallel do not observe each other's numbering.
_Avoid_: scope, counter scope

**Value with default**:
A configuration value that is either a bare value or a value plus its default
form — the shape `defineVars` accepts for a variable that varies by media query.
_Avoid_: optional value, defaulted value
