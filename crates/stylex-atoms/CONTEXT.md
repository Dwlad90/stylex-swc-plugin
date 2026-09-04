# stylex-atoms

Detection and compilation of the `@stylexjs/atoms` inline syntax —
`css.display.flex`, `css.color(value)` — inside `stylex.props(...)`. Every
detection and compile path answers `None` rather than raising, and the caller
leaves the original expression in place, which is what makes the crate safe to
run over any `stylex.props` argument.

## Language

**Atom**:
A single `{ property: value }` inline style written through the atoms syntax.
The unit this crate detects, compiles and replaces — never a whole style object.
_Avoid_: utility, class, token, style

**Static style**:
An atom whose value is written as a member — `css.display.flex`. It compiles to
a compiled-style object at build time. Three shapes count: two segments, a
computed segment (`css.width['calc(...)']`), and the single-segment `css.flex`,
where a named import makes the **imported name** the property while a namespace
or default import makes the member prop serve as both.
_Avoid_: literal style, constant style

**Dynamic style**:
An atom whose value is a call argument — `css.color(value)`. The property is
compiled against the literal `var(--x-{property})`, an
`@property --x-{property}` rule is emitted at priority 0, and the hoisted arrow
returns the compiled object beside an inline-vars object keyed on that custom
property; the `--x-` prefix reaches the stylesheet. A property that is empty or
holds whitespace, `;`, `{` or `}` is refused by
`is_safe_css_property_fragment`, the counterpart of the
[rule-breaking token](../stylex-css/CONTEXT.md) guard.
_Avoid_: runtime style, function style, computed style

**Compile trait**:
`Compile` — the seam this crate is shaped around. Every method on it is one the
atoms transform could not have without depending back on
[stylex-transform](../stylex-transform/CONTEXT.md), which depends on this crate.
Widening the trait is how that cycle gets reintroduced.
_Avoid_: adapter, backend, provider

**Atoms import**:
A local binding referring to `@stylexjs/atoms`, keyed by full SWC `Id` (`Atom`
plus `SyntaxContext`), so a shadowing local with the same text is not mistaken
for it. A namespace or default import is stored as `"*"`, and that sentinel is
what selects between the detection shapes above.
_Avoid_: alias, css import, binding

**Value normalization**:
Stripping one leading underscore from a value, so `css.display._flex` can name a
value that is a JS reserved word or starts with a digit. Exactly one is
stripped — a value that genuinely starts with one is written with two. Not
`stylex_css::normalize_value`, which is a different thing.
_Avoid_: sanitizing, unescaping, cleanup
