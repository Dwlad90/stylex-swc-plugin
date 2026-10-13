# stylex-stylesheet

The stylesheet step: the rules a compile emits (`metadata.stylex`), assembled
into one sheet. A port of `processStylexRules` from `@stylexjs/babel-plugin`,
function by function and in its order, so a change upstream maps onto one
function here. The reference is `packages/@stylexjs/babel-plugin/src/index.js`
at facebook/stylex `1fcb9a51cbecad2c82255e0f4b45cc072fa1a95a`, the commit
`@stylexjs/babel-plugin` 0.19.1 was published from: upstream changes enter this
project once they are on npm, so the port follows the published version. The
output is the string that version writes, byte for byte.

Today the crate holds the sort order of the sheet. The assembly itself
(`process_stylex_rules`) and the NAPI export beside `transform` follow in their
own changes; when the addon links this crate it sits on rung 3 of the ladder in
[STRUCTURE.md](../../guidelines/STRUCTURE.md), above
[stylex-css-parser](../stylex-css-parser/CONTEXT.md).

## Language

**Width bound**:
The one `min-width` or `max-width` a media query imposes in px, read off the
parsed and normalized query: a bound in another unit, a `not`, an `or`, a
word rule or two bounds on different sides leave the query with none, and so
does a prelude the parser refuses (a block left open, a stray parenthesis).
Zero is a bound in any unit or none, and `-0` is zero: the reference compares
bounds by subtraction. Two bounds of one side are the merged bound, because the
parser's normalization merges them before the bounds are read, as the
reference's does.
_Avoid_: breakpoint value, media bound

**Width sort key**:
A rule's width bound plus the at-rule **context** it sits in: the chain of its
preludes with the `@media` prelude spelled `@media`, so a breakpoint under
`@supports` compares only with breakpoints under the same `@supports`. A chain
with no `@media` prelude, or two, has no key.
_Avoid_: media key, sort key alone

**Chain**:
A rule's leading at-rule preludes, outermost first, read up to the first rule
block: `@supports (x){@media (y){.a{…}}}` is `["@supports (x)", "@media (y)"]`.
_Avoid_: at-rule stack, nesting

**Declared property**:
The property name of a rule's innermost declaration, the text between its last
`{` and the first `:`. Breakpoints are ordered within a property, so the
comparator reads it before the width key.
_Avoid_: property name, declaration key

**Collation**:
`localeCompare` as the reference calls it with no locale: root collation, as
[stylex-css](../stylex-css/CONTEXT.md) answers it for pseudo-class order. A
table for printable ASCII (two levels: each character by its base letter, case
folded, punctuation before digits before letters, a prefix before the string it
opens; then the first case difference, lowercase first) and `icu_collator` at
the root locale for any text carrying another character, so a `content: '·'`
weighs where ICU weighs it. Byte order is not it, and the sheet's order depends
on it.
_Avoid_: lexicographic order, string compare
