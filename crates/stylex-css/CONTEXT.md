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

**Spacing repair**:
The stage of whitespace normalization that puts back the spaces SWC's minifying
codegen removed — between a function result and the word after it, around a `/`,
before a hex colour. It reads the codegen's output as text, not as an AST, and
only ever inserts: a space the minifier swallowed is not recovered.
_Avoid_: whitespace fix, respacing, pretty-printing

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
`sort_at_rules`), so the same set always nests in the same order.
_Avoid_: selector, wrapped rule, block
