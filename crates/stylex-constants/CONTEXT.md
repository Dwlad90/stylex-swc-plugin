# stylex-constants

Static lookup tables and compile-time constants, shared by every crate above it.
The values are the contract with `@stylexjs/stylex` at runtime, so a term here
usually names something a browser or a runtime helper also sees.

## Language

**Priority**:
The number that decides where an injected rule lands in the stylesheet, and so
which rule wins. Summed per rule — the property's own priority plus every
pseudo, at-rule and const priority that applies, from
`PSEUDO_CLASS_PRIORITIES`, `AT_RULE_PRIORITIES` and
`PSEUDO_ELEMENT_PRIORITY`. StyleX's own ordering scheme, not the cascade's.
_Avoid_: specificity, weight, rank

**Compiled key**:
`COMPILED_KEY`, the literal `$$css`. It tells the runtime an object is compiled
output rather than an authored style.
_Avoid_: css marker, compiled flag

**Split token**:
`SPLIT_TOKEN`, the literal `__$$__`. It separates the halves of a generated key,
so a composite key can be taken apart again.
_Avoid_: delimiter, separator token

**Shorthand of shorthands**:
A property expanding into other shorthands rather than into longhands —
`border`, `background`, `margin` and the rest of `SHORTHANDS_OF_SHORTHANDS`.
Held apart from `SHORTHANDS_OF_LONGHANDS` because the two carry different
default priorities: 1000 against 2000.
_Avoid_: nested shorthand, compound shorthand

**Logical-to-physical map**:
The tables mapping a logical property onto a physical one — `PROPERTY_TO_LTR`
and `INLINE_PROPERTY_TO_LTR`, against `LOGICAL_TO_RTL`, `INLINE_TO_RTL` and
`LOGICAL_VALUE_TO_RTL`. Only the inline tables are a polyfill:
[stylex-css](../stylex-css/CONTEXT.md) reads `INLINE_PROPERTY_TO_LTR` under the
`legacy-expand-shorthands`
[style resolution](../stylex-enums/CONTEXT.md) with
`enableLogicalStylesPolyfill` on, while `PROPERTY_TO_LTR` applies under every
style resolution.
_Avoid_: direction map, rtl table
