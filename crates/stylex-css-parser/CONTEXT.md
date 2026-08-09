# stylex-css-parser

A CSS _value_ parser built from parser combinators. It parses what sits to the
right of a colon — never a stylesheet, a selector, or a rule; those go through
SWC's CSS parser in [stylex-css](../stylex-css/CONTEXT.md).

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

**Property parser**:
A parser for one whole property's grammar — `Transform`, `BoxShadow`,
`BorderRadiusShorthand`. Composed from CSS types.
_Avoid_: shorthand parser, declaration parser

**Last-media-query-wins transform**:
`last_media_query_wins_transform` — rewrites a set of media queries so that a
later one beats an earlier one, matching how authors expect overlapping queries
to behave rather than how the cascade actually resolves them.
_Avoid_: media merge, query dedupe
