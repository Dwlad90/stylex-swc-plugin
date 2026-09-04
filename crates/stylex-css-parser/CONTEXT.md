# stylex-css-parser

A CSS _value_ parser built from parser combinators. It parses what sits to the
right of a colon, and the prelude of an at-rule such as `@media` — never a
stylesheet, a selector or a whole rule.

Only one item is reachable from the rest of the workspace:
`last_media_query_wins_transform`. Every CSS type and property parser here is a
port with no caller outside the crate, which is what
**unreachable port** below is about.

## Language

**Token parser**:
`TokenParser<T>` — a parser combinator over a token list, composed with `map`,
`or`, `optional`, `zero_or_more`, `separated_by`. Most of the crate is composed
this way, but not all of it: `TokenParser::new` also wraps hand-written
token-consuming closures, and the media-query parser recurses.
_Avoid_: combinator, matcher, rule

**Token list**:
The mutable cursor a parser reads from. A parser that fails rewinds the cursor
to its saved index, which is what makes `or` work. It also carries a
**parser-frame** depth, the second of the two nesting guards below. Not the
loose node list of the
[value scanner](../postcss-value-parser/CONTEXT.md).
_Avoid_: stream, input, lexer

**CSS type**:
A parser for one value type from the CSS grammar — `Color`, `Length`, `Angle`,
`Calc`. Named for the specification's production, so `<length-percentage>` is
`LengthPercentage`.
_Avoid_: value parser, primitive

**Double-precision number**:
A numeric CSS type that holds an `f64` and whose `Display` prints through
`stylex_utils::number::to_js_string`, never through `{}`. Both halves are
required: an `f32` field rounds `28.81 - 0.01` to `28.8` where the reference
compiler emits `28.799999999999997`, and Rust's own formatting spells `1e21`
with twenty-two digits, names an overflow `inf`, and keeps the sign on a
negative zero. A new numeric type inherits both rules or reintroduces the
divergence. See [the shared formatter](../stylex-utils/CONTEXT.md).

A count is outside the rule rather than an exception to it — a `steps()` count,
guarded by a round-trip check — because JavaScript spells an integer the way
Rust does. A media `Fraction` is not a count: CSS admits
`(aspect-ratio: 16.5/9)`, so both halves are `f64`.
_Avoid_: single precision, f32 value

**Echoed value**:
Text emitted with the bytes the author wrote, rather than reprinted from a
computed value. **No path in this crate echoes**: every numeric type holds a
double and reprints it through the formatter, which is what the reference
compiler does too, so `.4` becomes `0.4` on both sides. Echoing happens one
crate over, on the shorthand expansion path, where the unit is a
[value part](../stylex-css/CONTEXT.md) rather than a number — `1E2px` stays
`1E2px` there, because a double cannot hold a spelling. Reaching for the
formatter _there_ causes a divergence; reaching for it here closes one.
_Avoid_: passthrough, verbatim, raw

**Unreachable port**:
A type in this crate whose reference counterpart the plugin never runs, so its
behaviour cannot be settled by comparing output. The rule is about evidence: a
claim that some colour grammar matches the reference compiler cannot be checked,
because the plugin normalizes a colour as _text_ and never rebuilds it from
parsed channels — `lch(50 50% 180)` comes out unchanged, the percentage echoed
rather than scaled. Where a colour _does_ reach emitted text — the comma
spelling, an unbounded alpha, a fractional `rgb()` channel — the plugin can be
run end to end, and was.
_Avoid_: dead code, unused type, aspirational port

**Precision suite**:
A test file named for what it pins rather than for an upstream test file, with
the suffix `_precision_test.rs` or `_double_precision_test.rs`. The third
category: a `<subject>_test.rs` mirrors an upstream file case for case, a
`<subject>_coverage_test.rs` reaches the branches mirroring leaves untouched,
and a precision suite mirrors nothing — the divergence it pins cannot exist
upstream, since a JavaScript `number` is already a double.
_Avoid_: f64 test, widening test, precision coverage

**Property parser**:
A parser for one whole property's grammar — `Transform`, `BoxShadow`,
`BorderRadiusShorthand`. Composed from CSS types.
_Avoid_: shorthand parser, declaration parser

**Media query canonicalization**:
Rewriting a media query into its canonical text, so two spellings of one query
hash to the same class. Three phases contribute, and naming only one misplaces
the behaviour: the range parsers turn `width >= 720px` into `min-` / `max-`
pairs, nudging a strict bound by 0.01; `MediaQuery::normalize` flattens nested
`and`s, merges each dimension's bounds into one interval, and distributes a
`not` over a two-clause `and` by DeMorgan; serialization collapses an empty or
single-child `or`.

A branch that contradicts is **retained, not pruned**: it reaches the bottom of
the distribution as an empty `or`, prints as `not all`, and keeps the nesting
built around it. That text is what the class name hashes, so the wrapper is
contract rather than noise.
_Avoid_: minification, formatting, cleanup, pruning

**Media query grammar**:
What the rule parser accepts: the CSS Media Queries Level 4 condition grammar.
One condition takes `and`s or `or`s and never both, a bare `not` is the whole
condition rather than an operand in one, and a comma binds more loosely than an
`or` — so `(a) and (b), (c) or (d)` is two disjuncts, which matters because the
last-media-query-wins transform distributes its negations over the top-level
`Or`. A few spellings are accepted here and refused by the official compiler, on
purpose: an author cannot get a divergent class name from a query the other
compiler will not compile at all.

Nesting is bounded by two guards, because neither can see what the other counts
and each closed a crash. `scan_query_structure` counts parentheses in the query
_text_ before tokenizing, since thousands of them abort inside
`TokenList::new`; the token list's parser-frame depth counts the rest, because a
bare `not`'s operand is a whole rule and a text scan cannot see it — `n\6ft`
decodes to `not`. The budget is `stylex_utils::nesting::MAX_NESTING_DEPTH`,
shared with the value guard in [stylex-css](../stylex-css/CONTEXT.md): 64 levels
accepted, the 65th refused.
_Avoid_: media syntax, query validation, condition parser

**Range merge boundary**:
`merge_and_simplify_ranges`, the single place canonicalization crosses to merge
an `and` list's ranges. It keeps the pass's two failure modes apart: the _inner
recovery_ gives up merging and emits the author's rules as written, while the
_outer refusal_ rejects the declaration with the invalid-media-query-syntax
error. The inner recovery is a node budget measured before the distribution
starts: past `2^18` branch nodes the rules are handed straight back. Nodes
rather than levels, because a depth bound cannot see a query that is wide and
shallow. Its provenance is
[ADR 0001](./docs/adr/0001-the-official-compilers-output-wins.md).
_Avoid_: merge wrapper, simplify wrapper, merge guard

**Last-media-query-wins transform**:
`last_media_query_wins_transform` — rewrites a set of media queries so a later
one beats an earlier one, matching how authors expect overlapping queries to
behave rather than how the cascade resolves them. It runs only on nested
`@media` keys, and only while `enableMediaQueryOrder` is on, which is its
default.

Two consequences are contract. It rebuilds each query through `MediaQuery` into
an insertion-ordered map, so two entries that canonicalize to one query text
leave **one**, at the earlier key's position and holding the later key's value:
an authored declaration is dropped silently and on purpose. And a rewritten key
is removed and re-inserted, which puts every media key after all the other
properties. See
[ADR 0001](./docs/adr/0001-the-official-compilers-output-wins.md) for why the
drop is matched rather than improved on.
_Avoid_: media merge, query dedupe
