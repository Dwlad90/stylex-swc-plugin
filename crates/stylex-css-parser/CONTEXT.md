# stylex-css-parser

A CSS _value_ parser built from parser combinators. It parses what sits to the
right of a colon, and the prelude of an at-rule such as `@media` — never a
stylesheet, a selector, or a whole rule; those go through SWC's CSS parser in
[stylex-css](../stylex-css/CONTEXT.md).

## Language

**Token parser**:
`TokenParser<T>` — a parser combinator over a token list, composed with `map`,
`or`, `optional`, `zero_or_more`, `separated_by`. Everything in this crate is
built by composing these; there is no hand-written recursive descent.
_Avoid_: combinator, matcher, rule

**Token list**:
The mutable cursor a parser reads from. A parser that fails rewinds the cursor
to its saved index, which is what makes `or` work.
_Avoid_: stream, input, lexer

**CSS type**:
A parser for one value type from the CSS grammar — `Color`, `Length`, `Angle`,
`Calc`. Named for the specification's production, so `<length-percentage>` is
`LengthPercentage`.
_Avoid_: value parser, primitive

**Double-precision number**:
A numeric CSS type that holds an `f64` and whose `Display` prints through
`stylex_utils::number::to_js_string`, never through `{}`. Both halves are
required, and for the same reason: the printed spelling reaches the class-name
hash, so it is observable output rather than a debugging detail. An `f32` field
rounds `28.81 - 0.01` to `28.8` where the official compiler emits
`28.799999999999997`, and Rust's own formatting spells `1e21` with twenty-two
digits, names an overflow `inf`, and keeps the sign on a negative zero — three
strings the official compiler never writes. A new numeric type inherits both
rules or reintroduces the divergence. See
[the shared formatter](../stylex-utils/CONTEXT.md).

Two things are outside the rule rather than exceptions to it. A count is an
integer — a `steps()` count, which a round-trip check guards — and JavaScript
spells an integer the way Rust does. And the **echoed value** below is not
printed by a `Display` impl at all.

`Fraction`'s parts used to be listed here as counts, and they are not. A media
fraction is a **ratio**: CSS admits `(aspect-ratio: 16.5/9)` and the official
compiler holds both halves as `number`, so an `i32` truncated the numerator and
saturated past `i32::MAX`. Nothing about a ratio makes it whole, and the
carve-out read as permission not to look.
_Avoid_: single precision, f32 value

**Echoed value**:
Text emitted with the bytes the author wrote, rather than reprinted from a
value some crate computed. **No path in this crate echoes** — the entry is here
so that a reader who has met the term elsewhere does not go looking for it in
the wrong place. Every numeric type here holds a double and reprints it through
the formatter, which is what the reference compiler does too: it stores a
`number` and interpolates it, so `matrix(1.200, …)` is `matrix(1.2, …)` on both
sides and `.4` becomes `0.4`. The reference compiler's own transform and easing
cases are ported verbatim and assert exactly that.

Echoing happens one crate over, on the shorthand expansion path, where the unit
is a [value part](../stylex-css/CONTEXT.md) rather than a number — `1E2px` stays
`1E2px` there, because a double cannot hold a spelling. Reaching for the
formatter _there_ causes a divergence; reaching for it here closes one.
_Avoid_: passthrough, verbatim, raw

**Unreachable port**:
A type in this crate whose reference counterpart the plugin never runs, so its
behaviour cannot be settled by comparing output. The colour types are the case
that matters. Two separate reasons: the plugin normalizes a colour as _text_ and
never rebuilds it from parsed channels -- `lch(50 50% 180)` comes out as
`lch(50 50% 180)`, the percentage echoed rather than scaled -- and
`Oklch.parser`/`Oklab.parser` throw on every input anyway, because `lc` carries
a `.prefix(Whitespace.optional)` that eats the space the enclosing sequence then
demands.

What follows is a rule about evidence, not about width: a claim that some colour
grammar "matches the reference compiler" cannot be checked against the reference
compiler, so it is a design decision of this crate wearing a parity costume. One
such claim was made and reverted. Where a colour _does_ reach emitted text --
the comma spelling, an unbounded alpha, a fractional `rgb()` channel -- the
plugin can be run end to end, and was.
_Avoid_: dead code, unused type, aspirational port

**Precision suite**:
A test file named for what it pins rather than for an upstream test file —
`double_precision_test.rs`, `color_double_precision_test.rs`,
`js_number_spelling_test.rs`, `easing_function_precision_test.rs`,
`transform_function_precision_test.rs`, `token_types_precision_test.rs`. A third
category beside the two older ones: a `<subject>_test.rs` mirrors an upstream
test file case for case, and a `<subject>_coverage_test.rs` reaches the branches
that mirroring leaves untouched. A precision suite mirrors nothing, because the
divergence it pins cannot exist upstream — a JavaScript `number` is already a
double, so upstream has no reason to test that one is. Reach for this name only
where that is true; a case with an upstream counterpart belongs in the mirror.
_Avoid_: f64 test, widening test, precision coverage

**Property parser**:
A parser for one whole property's grammar — `Transform`, `BoxShadow`,
`BorderRadiusShorthand`. Composed from CSS types.
_Avoid_: shorthand parser, declaration parser

**Media query canonicalization**:
Rewriting a media query into its canonical text, so that two spellings of one
query hash to the same class. Three phases contribute, and naming only one of
them misplaces the behaviour: the range parsers turn `width >= 720px` into
`min-`/`max-` pairs, nudging a strict bound by 0.01; `MediaQuery::normalize`
flattens nested `and`s, merges each dimension's bounds into one interval, and
distributes a `not` over a two-clause `and` by DeMorgan; serialization collapses
an empty or single-child `or`, which is how a contradiction prints as `not all`.
A branch that contradicts is **retained, not pruned** — it reaches the bottom of
the distribution as an empty `or` and prints as `not all`, keeping the nesting
built around it, so a ladder of exclusive breakpoints canonicalizes to something
much longer than the queries the author wrote. That text is what the class name
hashes, so the wrapper is contract rather than noise. This is the media-query
counterpart of value [normalization](../stylex-css/CONTEXT.md) in `stylex-css`.
_Avoid_: minification, formatting, cleanup, pruning

**Media query grammar**:
What the rule parser accepts, which is the CSS Media Queries Level 4 condition
grammar rather than whatever parses. One condition takes `and`s or `or`s and
never both, a bare `not` is the whole condition rather than an operand in one,
and a comma binds more loosely than an `or` — so `(a) and (b), (c) or (d)` is
two disjuncts and not three, which matters because the
[last-media-query-wins transform](#last-media-query-wins-transform) distributes
its negations over the top-level `Or`. Two spellings are accepted here and refused
by the official compiler, on purpose: parentheses nested around a single
condition, and one bare `not` straight after a media type's `and`. The language
defines both and its `oneOf` chain has an alternative for neither. Refusing
valid CSS to match a stricter reference buys nothing — an author cannot get a
divergent class name from a query the other compiler will not compile at all.
Every other combinator shape agrees.

Nesting is bounded at sixty-four levels of parentheses, counted before the parse
by the same walk that checks the balance, because parsing recurses once per
level and a stack overflow aborts rather than panicking. The budget itself is
`stylex_utils::nesting::MAX_NESTING_DEPTH`, shared with the value guard in
`stylex-css`: the two scans differ, but the stack they are protecting is one.
_Avoid_: media syntax, query validation, condition parser

**Range merge boundary**:
`merge_and_simplify_ranges` — the single place media query canonicalization
crosses to merge an `and` list's ranges, named after the wrapper it mirrors in
the reference implementation. It exists to keep the pass's two failure modes
apart: the _inner recovery_ gives up merging and emits the author's rules as
written, while the _outer refusal_ rejects the declaration with the
invalid-media-query-syntax error. The function's own comment carries why that
distinction is load-bearing. The inner recovery is a depth bound, measured
before the distribution starts: past 18 levels of splitting the rules are handed
straight back, because each level doubles the query text. That number was chosen
against output size and is not arbitrary — its provenance is in
[docs/adr/0001](./docs/adr/0001-the-official-compilers-output-wins.md).
_Avoid_: merge wrapper, simplify wrapper, merge guard

**Last-media-query-wins transform**:
`last_media_query_wins_transform` — rewrites a set of media queries so that a
later one beats an earlier one, matching how authors expect overlapping queries
to behave rather than how the cascade actually resolves them. It rebuilds each
query through `MediaQuery`, so the keys it emits are canonicalized; it runs only
on nested `@media` keys, and only while `enableMediaQueryOrder` is on — its
default — so opting out hashes the authored spelling instead. Its rewritten
keys go into an insertion-ordered map, so two entries that canonicalize to one
query text leave **one** entry, at the earlier key's position and holding the
later key's value: one authored declaration is dropped, silently and on purpose.
See [docs/adr/0001](./docs/adr/0001-the-official-compilers-output-wins.md) for why
that and the retained branches are matched rather than improved on.
_Avoid_: media merge, query dedupe
