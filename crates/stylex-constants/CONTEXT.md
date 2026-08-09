# stylex-constants

Static lookup tables and compile-time constants, shared by every crate above it.
The values are the contract with `@stylexjs/stylex` at runtime, so a term here
usually names something a browser or a runtime helper also sees.

## Language

**Priority**:
The number that decides where an injected rule lands in the stylesheet, and so
which rule wins. Carried per rule and summed from the tables here —
`PSEUDO_CLASS_PRIORITIES`, `AT_RULE_PRIORITIES`, `PSEUDO_ELEMENT_PRIORITY`. It
is StyleX's own ordering scheme, not the cascade's.
_Avoid_: specificity, weight, rank

**Compiled key**:
`$$css`, the marker property that tells the runtime an object is compiled output
rather than an authored style.
_Avoid_: css marker, compiled flag

**Split token**:
`__$$__`, the separator embedded in a generated key so a composite key can be
taken apart again.
_Avoid_: delimiter, separator token

**Shorthand of shorthands**:
A property that expands into other shorthands rather than into longhands —
`border`, `background`, `animation`. Tracked apart from
`SHORTHANDS_OF_LONGHANDS` because expansion order depends on which it is.
_Avoid_: nested shorthand, compound shorthand

**Logical-to-physical map**:
`PROPERTY_TO_LTR` / `PROPERTY_TO_RTL`, mapping `*-start`/`*-end` onto
`*-left`/`*-right`. A polyfill reached only under the
`legacy-expand-shorthands` [style resolution](../stylex-enums/CONTEXT.md);
modern output emits the logical property directly.
_Avoid_: direction map, rtl table
