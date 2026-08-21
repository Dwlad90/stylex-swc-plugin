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
hash to one class. One path serves every value: it is scanned into a token
list, a fixed sequence of value passes is folded over it, and the list is
spelled back out. There is no second route and no allowlist — syntax the
compiler has never heard of, relative color syntax included, goes the same way
as `color: red`.
_Avoid_: minification, formatting, cleanup

**Value pass**:
One member of that fixed sequence — a single walk of the token list that
either rewrites it in place or rejects the value outright. Its position in the
sequence is behaviour, not arrangement: a pass that reads a token another pass
has already rewritten sees different input, and a rejecting pass placed after
another decides which of two diagnostics an author gets.
_Avoid_: normalizer (that is the narrower term for the ported ones), step,
stage, visitor

**Reference verdict**:
What the parity harness recorded when it ran a declaration through both this
compiler and the reference compiler — `identical`, or a divergence, and in the
latter case the reference compiler's own spelling. A normalization expectation
carries one so a pipeline change can be read as predicted or unpredicted. Always
taken from a harness run, never from judgement.
_Avoid_: baseline, golden value, upstream expectation

**Reference compiler**:
`@stylexjs/babel-plugin` at the version the parity harness pins. It is the
oracle a verdict is measured against, because a class name is a hash of the
declaration text and both compilers have to produce the same one.
_Avoid_: upstream, the Babel side, the other compiler

**Marker**:
The class name that `when.*` selectors observe on an ancestor or descendant, so
a rule can react to a pseudo-class active on another element. Set per call by
the second argument to `when.*`, which reaches this crate as a
[when marker value](../stylex-types/CONTEXT.md); the default marker when that
argument is absent.
_Avoid_: sentinel, flag class, hook class

**Default marker**:
The marker a `when.*` selector falls back to, `{prefix}-default-marker`. The
separator is always present, so an explicitly empty `classNamePrefix` yields
`-default-marker` -- an unset one arrives here already defaulted to `x`. A
second argument that resolves to no known marker shape carries no prefix at
all and so falls back to a bare `default-marker`.
_Avoid_: fallback class, base marker

**Pseudo element**:
A selector segment carrying a `::` prefix — `::before`, `::thumb` — as against
the single colon of a pseudo class. The test is `is_pseudo_element`
(`utils::pseudo`); every site that classifies a segment calls it rather than
spelling the prefix out, so the rule moves in one place. Not a synonym for
pseudo selector, which is the wider term covering both kinds.
_Avoid_: double-colon check

**Pseudo class**:
A selector segment carrying exactly one leading colon — `:hover`,
`:nth-child(2)`. Tested by `is_pseudo_class`, which is the colon prefix minus
the pseudo elements.
_Avoid_: state selector, single-colon check

**Pseudo selector**:
A key opening with a colon, whichever kind of pseudo follows — the test
(`is_pseudo_selector`) that a key introduces a nested selector rather than a
declaration. Distinct from pseudo class: `::before` is a pseudo selector and
not a pseudo class. Narrower than conditional key, which also admits at-rules
and attribute selectors.
_Avoid_: nested key, colon key

**Pseudo run**:
A maximal stretch of a selector's pseudo list holding no pseudo element — the
unit `sort_pseudos` sorts. A pseudo element pins its own position and closes the
run before it, because it names which part of the element the rule targets
rather than a state the element is in; everything else, pseudo classes and
attribute selectors alike, joins the run it sits in. A run is sorted whole at
whatever length it reached, which is why the same set of keys hashes one class
name however the author nested them.
_Avoid_: pseudo group, sort group, pseudo pair

**Conditional key**:
A key that opens a nested block rather than declaring a property: a pseudo
selector, an at-rule (`@media ...`), or an attribute selector
(`[data-active]`). The test is `is_conditional_key` (`utils::condition`).
Sites deliberately admitting only some of the three spell those out rather
than widening to this term.
_Avoid_: condition key, nested key, at-or-pseudo

**Nested CSS rule**:
The final rule string, built by wrapping a declaration in its at-rules and
pseudo selectors. Pseudos and at-rules are each sorted first (`sort_pseudos`,
`sort_at_rules`), so the same set always nests in the same order. The sorted
pseudo list is what the class-name hash reads; the printed selector spells the
pseudo classes in that order and every pseudo element after all of them, so the
two are not the same sequence wherever an element sits mid-list.
_Avoid_: selector, wrapped rule, block
