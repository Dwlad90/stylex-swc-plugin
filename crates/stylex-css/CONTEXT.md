# stylex-css

Turns a resolved `{ property: value }` pair into the CSS text that gets
injected: expansion, normalization, direction flipping and priority. It knows
nothing about JavaScript — by the time a value reaches here it is already a
string.

## Language

**Order strategy**:
The implementation of one [style resolution](../stylex-enums/CONTEXT.md) —
`ApplicationOrder`, `PropertySpecificityOrder`, `LegacyExpandShorthandsOrder`.
Each is a unit struct implementing the `Order` trait, whose single job is to
answer, per property, which expansion function applies.
_Avoid_: resolver, sorter, mode

**Expansion**:
Rewriting one authored property into the
[order pairs](../stylex-structures/CONTEXT.md) actually emitted — `margin` into
four longhands, or an alias into its canonical
name. Which expansion runs depends entirely on the order strategy in force.
_Avoid_: longhand split, desugaring, flattening

**Alias**:
An authored property name that is not a CSS property but stands for one —
resolved during expansion, before shorthands.
_Avoid_: synonym, shim, polyfill

**LTR / RTL generation**:
Producing the left-to-right rule and, when the property is direction-sensitive,
its mirrored counterpart. A pair with no directional meaning yields no RTL rule
at all — an absent `rtl` is the normal case, not a gap.
_Avoid_: bidi, flip, mirroring

**Normalization**:
Rewriting a value into its canonical text so two spellings of the same value
hash to one class. Whitespace normalization is the general path; values SWC's
CSS parser cannot handle — relative color syntax such as `rgb(from red r g b)` —
are normalized by spacing alone rather than parsed and re-serialized.
_Avoid_: minification, formatting, cleanup

**Marker**:
The class name that `when.*` selectors observe on an ancestor or descendant, so
a rule can react to a pseudo-class active on another element. Configurable;
`defaultMarker` when it is not.
_Avoid_: sentinel, flag class, hook class

**Nested CSS rule**:
The final rule string, built by wrapping a declaration in its at-rules and
pseudo selectors. Pseudos and at-rules are each sorted first (`sort_pseudos`,
`sort_at_rules`), so the same set always nests in the same order.
_Avoid_: selector, wrapped rule, block
