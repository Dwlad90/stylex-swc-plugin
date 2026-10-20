# stylex-stylesheet

The stylesheet step: the rules a compile emits (`metadata.stylex`), assembled
into one sheet. A port of `processStylexRules` from `@stylexjs/babel-plugin`,
function by function and in its order, so a change upstream maps onto one
function here. The reference is `packages/@stylexjs/babel-plugin/src/index.js`
at facebook/stylex `1fcb9a51cbecad2c82255e0f4b45cc072fa1a95a`, the commit
`@stylexjs/babel-plugin` 0.19.1 was published from: upstream changes enter this
project once they are on npm, so the port follows the published version. The
output is the string that version writes, byte for byte.

`process::process_stylex_rules` is the assembly; `width_sort_key` holds the order
it sorts by. [stylex-rs-compiler](../stylex-rs-compiler/CONTEXT.md) exports it
beside `transform` as `processStylexRules`. The crate sits on rung 3 of the
ladder in [STRUCTURE.md](../../guidelines/STRUCTURE.md), above
[stylex-css-parser](../stylex-css-parser/CONTEXT.md),
[stylex-js](../stylex-js/CONTEXT.md) (the JavaScript value it reads and V8's
sort), [stylex-constants](../stylex-constants/CONTEXT.md),
[stylex-regex](../stylex-regex/CONTEXT.md) and
[stylex-utils](../stylex-utils/CONTEXT.md).

The fixtures in `tests/fixtures/` have readers outside this crate:
`@stylexswc/rs-compiler` reads `process-stylex-rules-cases.json` from
`__test__/processStylexRules.spec.ts` and `width-sort-key-matrix.json` from
`parity/parity-sheets.ts`, through the one constant in its `parity/lib/fixtures.ts`;
a rename here moves that constant.

## Language

**Rule**:
One emitted rule as the reference's `Rule` tuple: `[className, { ltr, rtl?,
constKey?, constVal? }, priority]`, the rows of a compile's `metadata.stylex`.
A rule with both `constKey` and `constVal` not `null` is a **constant** (a
`defineConsts` value) and injects no text; every other rule is a style. A rule
is read from a JavaScript value as the reference destructures it: the first
three items of an iterable (an array, or a string's characters), the style
spread (so only an object gives an `ltr`; `null` and `undefined` throw), the
priority as `Number(...)` reads it (a missing one is `NaN`). The class name is
kept as it arrived: `String(...)` spells it in a constant's name, and it is a
`Map` key by `SameValueZero` in the dedupe. A constant's value of any kind
substitutes as `String(...)` spells it. A style rule's `ltr` must be a string
where it is read (in the sort, and where it is written), and a truthy `rtl`
too; a rule that is never read there is never refused.
_Avoid_: injectable style (the producer-side type in stylex-types), entry

**Value reading**:
The reading of the two arguments from a `JsValue`, in the reference's order:
the early return on `rules.length === 0` (an empty array, an empty string, or
an object whose `length` is 0), then the config, then the rules, with a hole
skipped as `filter` skips it. A value the reference throws on is refused with
a `ProcessStylexRulesError` of the port's own wording, and so is a text that
the reference would cut in two (`CharacterCutInTwo`: a text with no `{` that
ends in an astral character, which `slice` splits). These are the errors of
the Rust API; the addon never shows them (see **Fallback**).
_Avoid_: deserialization, parsing, validation

**Fast path**:
The addon's native `processStylexRules`: the rules a JavaScript gate proved
plain, unpacked into `Rule`s and assembled by `process::process_stylex_rules`.
It takes the shapes a build emits only, and answers no sheet where the
reference throws.
_Avoid_: native mode, Rust path

**Fallback**:
Upstream's own `processStylexRules`, copied out of the published plugin into
the addon, which runs for every input the fast path does not take, and for
every `ProcessStylexRulesError`. It writes the reference's sheet, or throws the
reference's error, by construction.
_Avoid_: slow path, JS mode, reference mode

**Constant substitution**:
Before the sort, every constant's value is resolved through the constants it
refers to (`var(--key)`, a chain that returns to itself is refused by name) and
written into every style's text, `ltr` then `rtl`, one constant at a time in
declaration order; a constant whose value is itself a `var(...)` also renames a
declaration of the constant (`--key:`) to the variable it names. The
replacements are `String.prototype.replace`/`replaceAll` with a string
replacement, dollar patterns included, as the reference calls them.
_Avoid_: inlining, expansion

**Priority group**:
The sorted rules cut at each change of `floor(priority / 1000)` (a `NaN` level
equals no level, so each `NaN` rule is a group of its own); one group is
one `@layer priorityN` when layers are on (a group whose first rule has
priority 0 stays outside every layer), else one run of rules carrying N - 1
copies of the **specificity level** `:not(#\#)` unless `legacyDisableLayers`
asks for neither. Within a group a class name appears once, at its first
position with its last style, as `new Map(...)` keeps it. The reference's
grouping starts at level -1 with no group, so a first rule at level -1 (a
priority from -1000 to less than 0) is refused, as the reference throws.
_Avoid_: layer bucket, tier

**Layer options**:
`useLayers` as an object: layers to declare `before` and `after` the priority
layers in the `@layer` header, and a `prefix` for their names
(`prefix.priorityN`). `true` is the object with nothing set; `false`, `null` and
an absent key are no layers. `before` and `after` are read as the header
spreads them (an array gives its elements and a string its characters; any
other value throws) and spelled as `join` spells them (`null` as nothing). The
prefix is read by truthiness and then spelled with `String(...)`.
_Avoid_: layer config

**Config**:
The reference's `config` argument, read as the reference reads it
(`typeof config === 'boolean' ? { useLayers: config } : config ?? {}`, then one
property read each): a boolean is `useLayers`; `null` and a value that is not an
object have no properties, so every option is at its default. `useLayers` is on
for every value but `false` and `null` (`!== false`), with the defaults when
the value is not an object; the three flags are read by JavaScript truthiness
(`null`, `0`, `NaN` and `""` are off). Inside the object, a `null` property is
its default (`?? []`, `?? ''`).
_Avoid_: options bag, loose config

**Sort**:
`Array.prototype.sort` with the reference's comparator, which answers a number:
the priority difference, then -1, 0 or 1 from `localeCompare`, or a width bound
difference. A priority difference that is `NaN` (a missing priority, or two
equal infinities) is returned as it is and read as a tie, so the comparator is
not a consistent order for such rules. The port sorts with V8's TimSort
([stylex-js](../stylex-js/CONTEXT.md), "Array sort"), so each comparison is the
one Node makes and the order is the one Node gives.
_Avoid_: stable sort, ordering

**RTL twin**:
A style whose `rtl` text differs is written twice, under
`html:not([dir='rtl'])` and `html[dir='rtl']` (inside the innermost at-rule;
never into a `@keyframes` block), or between `/* @ltr begin */` and
`/* @rtl begin */` comments under `enableLTRRTLComments`. An empty `rtl` is as
absent as a missing one.
_Avoid_: direction pair, bidi rule

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
