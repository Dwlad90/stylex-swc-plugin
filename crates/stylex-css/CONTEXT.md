# stylex-css

Turns a resolved `{ property: value }` pair into the CSS text that gets
injected: expansion, normalization, direction flipping and priority. By the time
a value reaches here it is already a string.

## Language

**Order strategy**:
The implementation of one [style resolution](../stylex-enums/CONTEXT.md) —
`ApplicationOrder`, `PropertySpecificityOrder`, `LegacyExpandShorthandsOrder`.
Each implements `stylex_structures::order::Order`, which answers, per property,
which expansion function applies.
_Avoid_: resolver, sorter, mode

**Expansion**:
Rewriting one authored property into the
[order pairs](../stylex-structures/CONTEXT.md) actually emitted — `margin` into
four longhands. Which expansion runs depends on the order strategy in force.
_Avoid_: longhand split, desugaring, flattening

**Value part**:
One piece of a shorthand's value — `10px` and `20px` in `margin: '10px 20px'`.
Where a part ends is decided on the
[value scanner](../postcss-value-parser/CONTEXT.md)'s node kinds rather than on
characters, because `/` and `:` end a part at the top level and are ordinary
characters inside a function. `values::parser::split_value_parts` is the only
producer, which is what enforces the three rules of a part: it is echoed and
never re-spelled, a trailing importance annotation belongs to every part, and an
empty part is still a part.
_Avoid_: token, side, segment, value fragment

**Alias**:
An authored property name that is not a CSS property but stands for one.
`Aliases::get` is checked before `Shorthands::get`, so an alias resolves before
shorthands.
_Avoid_: synonym, shim, polyfill

**LTR / RTL generation**:
Producing the left-to-right rule and, where the property is direction-sensitive,
its mirrored counterpart. An absent `rtl` is the normal case, not a gap.
_Avoid_: bidi, flip, mirroring

**Normalization**:
Rewriting a value into its canonical text so two spellings of one value hash to
one class. One path serves every value: scanned into a token list, a fixed
sequence of value passes folded over it, spelled back out. No second route and
no allowlist.

Two entry points, and only one is safe. `normalize_css_property_value` is the
production entry and applies the structural guards. `normalize_value` is the
ported fold without them, `pub` for the bench target alone, and will normalize a
`}` straight into a stylesheet.
_Avoid_: minification, formatting, cleanup

**Value pass**:
One member of that sequence — a single walk of the token list that either
rewrites it in place or rejects the value. Its position is behaviour, not
arrangement: `normalize_timings` runs before `normalize_leading_zero`, so
`100ms` becomes `0.1s` and then `.1s`, and reordering the two changes a class
name. A rejecting pass placed after another decides which of two diagnostics an
author gets.
_Avoid_: normalizer, step, stage, visitor

**Structural guard**:
A rejection that reads the raw bytes of a value rather than the token list.
Three exist, and only one preempts the passes. The **nesting budget** must speak
before the value is parsed, because parsing recurses once per level and past the
budget the process aborts with no diagnostic; `nests_too_deeply` is the shared
depth answer, read by normalization and the shorthand splitter both.

The other two are handed to the sequence as a **deferred refusal** and fire
after the shared rejections: the unclosed comment, and a **rule-breaking
token** — a `{`, `}` or `;` outside strings and comments, which would let a
value break out of its declaration. That one has two carve-outs, each a real
program: a **trailing** `;` is allowed, in any number and with any trailing
whitespace, and an **escaped** one is allowed, so `A\;B` compiles as a
`fontFamily`. Both are deferred so a value carrying two faults earns the same
complaint from both compilers — `calc(1px /*` must be refused for the unclosed
function, as the reference compiler refuses it.
_Avoid_: validation, sanity check, precheck, injection check

**Shared rejection**:
A value pass that rejects for something the reference compiler rejects for too —
the unclosed function and the unclosed string, and only those. It is the
boundary a structural guard may not fire ahead of, or the two compilers hand an
author different sentences for one value.
_Avoid_: common guard, upstream check, shared validator

**Reference verdict**:
What the parity harness recorded when it ran a declaration through both
compilers — `identical`, or a divergence plus the reference compiler's own
spelling. A normalization expectation carries one, always from a harness run and
never from judgement.
_Avoid_: baseline, golden value, upstream expectation

**Reference compiler**:
`@stylexjs/babel-plugin` at the version the parity harness pins. It is the
oracle, because a class name is a hash of the declaration text and both
compilers have to produce the same one.
_Avoid_: upstream, the Babel side, the other compiler

**Marker**:
The class name that `when.*` selectors observe on an ancestor or descendant, so
a rule can react to a pseudo-class active on another element. Set by the second
argument to `when.*`, which arrives as a
[when marker value](../stylex-types/CONTEXT.md).
_Avoid_: sentinel, flag class, hook class

**Default marker**:
The marker a `when.*` selector falls back to, `{prefix}-default-marker`. The
separator is always present, so an explicitly empty `classNamePrefix` yields
`-default-marker`; an unset one arrives already defaulted to `x`. A second
argument resolving to no known marker shape carries no prefix, and falls back to
a bare `default-marker`.
_Avoid_: fallback class, base marker

**Pseudo element**:
A selector segment carrying a `::` prefix — `::before`. The test is
`is_pseudo_element`.
_Avoid_: double-colon check

**Pseudo class**:
A selector segment carrying exactly one leading colon — `:hover`. The test is
`is_pseudo_class`.
_Avoid_: state selector, single-colon check

**Pseudo selector**:
A key opening with a colon, whichever kind follows — `is_pseudo_selector`, the
test that a key introduces a nested selector rather than a declaration.
`::before` is a pseudo selector and not a pseudo class.
_Avoid_: nested key, colon key

**Pseudo run**:
A maximal stretch of a selector's pseudo list holding no pseudo element — the
unit `sort_pseudos` sorts. A pseudo element pins its own position and closes the
run before it, because it names which part of the element the rule targets
rather than a state it is in; pseudo classes and attribute selectors join the
run they sit in. A run is sorted whole, which is why one set of keys hashes one
class name however the author nested them.
_Avoid_: pseudo group, sort group, pseudo pair

**Primary weight**:
What a character contributes to the order a pseudo run sorts in, before an
accent or a case is read. `primary_weight` reads `ASCII_PRIMARY_RANK`, the
compile-time inversion of `ASCII_PRIMARY_ORDER`, itself taken from the ordering
`localeCompare` produces: whitespace, symbols, digits, letters, with a letter's
two cases sharing one weight. It is not byte order — `{` weighs below `z`, and
`_` below `-`.
_Avoid_: collation weight, sort key, rank

**ASCII fast path**:
The half of `pseudo_comparator` that reads the rank table, taken per pair and
only when every byte of both keys is printable ASCII. The boundary is printable
ASCII and not `is_ascii()`: the table ranks an unnamed byte above every named
one while root collation weighs a control character not at all, so admitting one
produces an intransitive comparator rather than a different order. Anything else
goes to root collation.
_Avoid_: the ASCII branch, the table path

**Root collation**:
The ordering `icu_collator` produces at the root locale, for every key the fast
path does not claim. It places an accented letter beside its base letter, weighs
a symbol below every letter, ignores a completely ignorable character, and lets
one character weigh as several. Root and not the host's locale: the reference
implementation calls `localeCompare` bare, so a Swedish or Danish machine sorts
`ö` after `z` where every other locale sorts it beside `o`. See
[ADR 0001](./docs/adr/0001-root-collation-orders-a-non-ascii-key.md).
_Avoid_: ICU collation, locale-aware ordering, Unicode order

**Case tiebreak**:
What the fast path falls back to when two keys tie on every primary weight and
on length: the first position where they differ, lowercase first. It has no
accent pass in front of it, which is why it is only ever asked about ASCII.
_Avoid_: tertiary weight, case fold

**At-rule comparator**:
The comparator `sort_at_rules` uses, deliberately not the one a pseudo run uses:
plain byte order with `default` pulled to the front. Upstream sorts pseudo keys
with `localeCompare` and at-rules with a bare `.sort()`, so a locale-aware
at-rule sort would be a new divergence rather than a fix.
_Avoid_: locale-aware at-rule sort

**Conditional key**:
A key that opens a nested block rather than declaring a property: a pseudo
selector, an at-rule, or an attribute selector. The test is
`is_conditional_key`.
_Avoid_: condition key, nested key, at-or-pseudo

**Nested CSS rule**:
The final rule string, built by `build_nested_css_rule` and `generate_css_rule`
wrapping a declaration in its at-rules and pseudo selectors. Both lists are
sorted first, by callers in
[stylex-transform](../stylex-transform/CONTEXT.md). The sorted pseudo list is
what the class-name hash reads; the printed selector spells the pseudo classes
in that order and every pseudo element after all of them, so the two are not the
same sequence wherever an element sits mid-list.
_Avoid_: selector, wrapped rule, block
