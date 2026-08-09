# stylex-atoms

Detection and compilation of the `@stylexjs/atoms` inline syntax —
`css.display.flex`, `css.color(value)` — inside `stylex.props(...)`. It takes
the style-compilation utilities it needs through a trait rather than depending
on [stylex-transform](../stylex-transform/CONTEXT.md), which would be a cycle:
the transform depends on this crate.

## Language

**Atom**:
A single `{ property: value }` inline style written through the atoms syntax.
The unit this crate detects, compiles and replaces — never a whole style object.
_Avoid_: utility, class, token, style

**Static style**:
An atom whose value is written as a member — `css.display.flex`. Compiles to a
compiled-style object at build time.
_Avoid_: literal style, constant style

**Dynamic style**:
An atom whose value is a call argument — `css.color(value)`. Compiles to a
hoisted dynamic-style function call, because the value is not known until
runtime.
_Avoid_: runtime style, function style, computed style

**Compile trait**:
`Compile` — the seam this crate is shaped around; its rustdoc lists what the
consumer must supply. Every method on it is one the atoms transform could not
have without depending back on `stylex-transform`, so widening it is how the
cycle gets reintroduced.
_Avoid_: adapter, backend, provider

**Atoms import**:
A local binding that refers to `@stylexjs/atoms`, keyed by full SWC `Id`
(`Atom` plus `SyntaxContext`) so a shadowing local with the same text is not
mistaken for it. A namespace or default import is stored as `"*"`.
_Avoid_: alias, css import, binding

**Value normalization**:
Stripping one leading underscore from a value, so `css.display._flex` and
`css.zIndex._1` can name values that are JS reserved words or start with a
digit. Exactly one underscore is stripped — a value that genuinely starts with
one is written with two.
_Avoid_: sanitizing, unescaping, cleanup
