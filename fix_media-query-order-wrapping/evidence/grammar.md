# Ticket 13 — which media queries are valid, decided

The ticket carried three inputs where this compiler was more permissive than
`@stylexjs/babel-plugin` 0.19.0. Widening the comparison to every combinator
shape turned three into twelve, in three families rather than one — including
two shapes both compilers accepted and rendered *differently*, which the
three-input list could not have found.

`grammar-sweep.cjs` beside this file runs all forty-seven and prints the table.

| Run    | Shapes | Agreeing |
| ------ | ------ | -------- |
| before | 50     | 33       |
| after  | 50     | 45       |

The sweep runs over features the range merge cannot read — `(orientation:
portrait)`, `(monochrome)`, `(hover: hover)` — on purpose. Width bounds would be
intersected into a single interval and most of these shapes would emit the same
string, which would make "agree" mean far less than it looks. Set `MERGEABLE=1`
to run the same shapes over width bounds instead; the verdict is the same either
way, which is the check that the choice of feature is not doing the work.

## What was decided, and why

### Matched: comma binds more loosely than `or`

`(a) and (b), (c) or (d)` and two more like it were accepted by both compilers
and emitted **differently** — three disjuncts here, two there. Comma and `or`
both reach the same `Or` node, but `or` groups inside a comma segment and the
segments group above it, and the last-media-query-wins transform distributes its
negations over whatever the top-level `Or` holds.

This was the worst of the twelve and the least visible: no error, no warning, a
different class name. Fixed by giving comma a parser above the `or` one.

### Matched: a disjunction inside parentheses is accepted

`((a) or (b))`, that shape beside an `and` on either side, and `not ((a) or (b))`
were **refused here** and accepted upstream. `( <media-condition> )` is a media
query in its own right and a condition may hold an `or`, so all four are valid
CSS — the parser simply read only `and` inside parentheses. Rejecting valid
input is a divergence too, and a louder one.

Worth recording for the second half: the parentheses do not survive
serialization. `((a) or (b)) and (c)` prints as `(a) or (b) and (c)`, which
reads as though the `and` bound tighter. That is upstream's output, the class
name hashes it, and the tests say so rather than quietly pinning it.

### Matched: an undefined combinator spelling is refused

CSS defines a condition as `<media-not> | <media-in-parens> [ <media-and>* |
<media-or>* ]`. Two consequences, and this compiler honoured neither:

- one condition takes `and`s or `or`s, never both — `(a) and (b) or (c)` is not
  a query, and we were guessing a precedence for it
- a bare `not` is the *whole* condition — `not (a) and (b)` and `(b) or not (a)`
  are not queries either, and we were compiling them

Five shapes, all now refused, matching upstream. The last two were not in the
ticket; the widened sweep found them.

### Not matched, deliberately: a bare `not` after a media type's `and`

`screen and not (orientation: portrait)` and `not screen and not (...)` are
accepted here and refused upstream. `<media-query> = [not | only]?
<media-type> [ and <media-condition-without-or> ]?` and
`<media-condition-without-or> = <media-not> | <media-in-parens> <media-and>*`,
so one bare negation straight after a media type's `and` is exactly what the
language defines.

Only there, and only immediately: `screen and (a) and not (b)` is back inside a
condition, where an operand must be parenthesized, and that one is refused by
both.

Kept for the same reason as the case below — see it for the argument.

### Not matched, deliberately: nested parentheses around one condition

| Input                     | Upstream | Here                        |
| ------------------------- | -------- | --------------------------- |
| `@media ((min-width: 1px))` | refuses  | `@media (min-width: 1px)`   |
| `@media (((min-width: 1px)))` | refuses | `@media (min-width: 1px)`  |
| `@media (((((min-width: 1px)))))` | refuses | `@media (min-width: 1px)` |

`<media-in-parens> = ( <media-condition> ) | <media-feature> | <general-enclosed>`
and `<media-condition>` may itself be a `<media-in-parens>`, so a condition
wrapped twice is valid CSS. Upstream's `oneOf` chain has no alternative for it,
which is a gap in its grammar rather than a rule of the language.

Refusing these to match would mean rejecting correct CSS an author is entitled
to write, and the class-name argument does not apply: nobody gets a divergent
hash from a query the other compiler will not compile at all. So they stay
accepted, and this is the one place the two disagree by choice.

It is also the shape where matching would have been most expensive. Upstream's
parser backtracks exponentially in nesting depth. This compiler walks each level
once. Measured by `nesting-depth.cjs` beside this file, one depth per process:

| Levels | Upstream            | Here                     |
| ------ | ------------------- | ------------------------ |
| 2      | 20 ms               | 1 ms                     |
| 4      | 55 ms               | 1 ms                     |
| 8      | 1 120 ms            | 1 ms                     |
| 12     | 19 836 ms           | 1 ms                     |
| 16     | not finished in 40 s | 1 ms                    |
| 200    | —                   | 1 ms, refused            |
| 5000   | —                   | 1 ms, refused            |

## The budget that came out of measuring it

Walking each level once is linear in time and still recursive in stack, and at
five thousand levels this compiler **aborted** — a stack overflow is not
unwindable, so nothing downstream could have turned it into a diagnostic.

A query may now nest sixty-four levels of parentheses. The depth is counted by
the same walk over the raw text that checks the parentheses balance, before the
parse rather than during it, because the parse is what would abort. Sixty-four
is the same number and the same reasoning as `MAX_VALUE_NESTING_DEPTH` in
`stylex-css`, which guards the identical exposure for values: stated rather than
left to whatever stack the host provides, set well below the observed cliff, and
far above real CSS.

Past the budget the query is refused in 1 ms rather than compiled or crashed.
That is a third place the two compilers differ — upstream has no budget and
would still be backtracking — but not one anybody reaches: an author writes one
or two levels.

## Where the answers live now

- the four matched families are pinned by unit tests over the transform, each
  refusal paired with the same operands bracketed
- two parity corpus subjects carry the comma binding and the parenthesized
  disjunction, so they move if upstream does
- the deliberate difference is stated in the crate glossary, next to the term
