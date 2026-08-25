# A ladder of exclusive breakpoints compiles to redundant `not all` wrappers

**Version:** `@stylexjs/babel-plugin` 0.19.0, `@babel/core` 8.0.1, Node
v24.11.0.

## What happens

A conditional value map whose keys are a ladder of mutually exclusive
`min-width`/`max-width` queries compiles the earlier entries into disjunctions
of contradictory branches. The branches print as `not all`, and the nesting
built around them stays in the emitted query.

## Minimal reproduction

```js
import * as stylex from '@stylexjs/stylex';

export const styles = stylex.create({
  root: {
    color: {
      default: 'black',
      '@media (min-width: 1440px)': 'c1',
      '@media (min-width: 1200px) and (max-width: 1439px)': 'c2',
      '@media (min-width: 1024px) and (max-width: 1199px)': 'c3',
      '@media (min-width: 768px) and (max-width: 1023px)': 'c4',
      '@media (min-width: 480px) and (max-width: 767px)': 'c5',
      '@media (max-width: 479px)': 'c6',
    },
  },
});
```

## Observed output

```css
@media ((not all) or (not all)) or ((not all) or ((min-width: 1440px))) { ... }
@media (not all) or ((min-width: 1200px) and (max-width: 1439px)) { ... }
@media (min-width: 1024px) and (max-width: 1199px) { ... }
@media (min-width: 768px) and (max-width: 1023px) { ... }
@media (min-width: 480px) and (max-width: 767px) { ... }
@media (max-width: 479px) { ... }
```

## Expected

The first two queries are equivalent to `(min-width: 1440px)` and
`(min-width: 1200px) and (max-width: 1439px)` — the branches wrapped around them
are unsatisfiable and contribute nothing.

## Why it is worth fixing rather than tolerating

**It grows exponentially.** The query text doubles with every rung. Measured on
an Apple M1 Max, one process per ladder length: 14 rungs 15 371 characters,
20 rungs 983 051, 24 rungs 15 728 651 in 20.4 s, 28 rungs 251 658 251 in 435 s
and about 7.4 GB resident. A 30-rung run was abandoned. Design systems with
long breakpoint ladders reach the slow part of that curve.

**A mainstream minifier refuses the output.** lightningcss 1.33.0 rejects both
wrapped forms:

```text
@media ((not all) or (not all)) or ((not all) or ((min-width: 1440px)))
  → Unexpected token ParenthesisBlock
@media (not all) or ((min-width: 1200px) and (max-width: 1439px))
  → Unexpected token Ident("all")
```

## Where it comes from

`mergeAndSimplifyRanges` distributes `not (A and B)` into two branches and
filters the results, dropping only branches that come back empty. A branch whose
numeric constraints contradict does not come back empty: it recurses to the
bottom and yields a one-element result holding an empty disjunction, which the
filter keeps and serialization prints as `not all`.

`mergeAndSimplifyRanges` also wraps its merge in a `try`/`catch` that returns
the input rules on any throw, which reads as a guard against this growth. It
never fires here: the recursion depth is linear in ladder length while the
branch count doubles, so the call stack is not what gives out. The heap is a
fatal abort no `catch` sees, and the string-length `RangeError` is raised by
`combinedQuery.toString()`, outside the `try`.
