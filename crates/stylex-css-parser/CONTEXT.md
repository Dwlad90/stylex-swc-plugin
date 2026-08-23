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
integer — `Fraction`'s parts, a `steps()` count — and JavaScript spells an
integer the way Rust does. And the **echoed value** below is not
printed by a `Display` impl at all.
_Avoid_: single precision, f32 value

**Echoed value**:
Text emitted with the bytes the author wrote, rather than reprinted from a
value this crate computed — what `value_parser::parse_css` does for the shorthand
expansion path. The official compiler echoes here too, so the target is the
author's own bytes: `1E2px` stays `1E2px`, and adopting the number formatter
would _cause_ a divergence rather than close one. This is the one numeric path
that must not follow the double-precision rule above, and it does not yet meet
its own: it still reprints. Do not "fix" it by reaching for the formatter.
_Avoid_: passthrough, verbatim, raw

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
This is the media-query counterpart of value
[normalization](../stylex-css/CONTEXT.md) in `stylex-css`.
_Avoid_: minification, formatting, cleanup

**Last-media-query-wins transform**:
`last_media_query_wins_transform` — rewrites a set of media queries so that a
later one beats an earlier one, matching how authors expect overlapping queries
to behave rather than how the cascade actually resolves them. It rebuilds each
query through `MediaQuery`, so the keys it emits are canonicalized; it runs only
on nested `@media` keys, and only while `enableMediaQueryOrder` is on — its
default — so opting out hashes the authored spelling instead.
_Avoid_: media merge, query dedupe
