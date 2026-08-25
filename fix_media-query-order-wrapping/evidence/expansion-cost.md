# Ticket 08 — what the restored expansion costs, and where it is stopped

Ticket 05 deleted the shortcut that recognized a disjoint breakpoint ladder and
skipped its expansion. This is the price of that, measured, and the bound that
caps it.

- machine: Apple M1 Max, 64 GB
- Node v24.11.0, `@stylexswc/rs-compiler` from `dist/`, release build, warm
- reproduced by `expansion-cost.cjs` beside this file, one ladder length per
  process:

  ```sh
  node evidence/expansion-cost.cjs 20
  ```

- the ladder is the reported shape at `n` rungs: exclusive `min-width` /
  `max-width` pairs from widest to narrowest, the first `min-width`-only and the
  last `max-width`-only, spaced so no two rungs touch — the arrangement that
  makes every distributed branch a contradiction, and so the largest expansion a
  ladder of that length can produce

## The curve

`chars` is the first rung's emitted query, which carries the whole accumulated
negation chain. Every length up to 20 matches `@stylexjs/babel-plugin` 0.19.0
character for character; the reference's own timings for the same lengths are in
`give-up-length.md`.

| Rungs | Wall clock | First rung's query | Resident |
| ----- | ---------- | ------------------ | -------- |
| 6     | 5 ms       | 71 chars           | 120 MB   |
| 10    | 3 ms       | 971 chars          | 119 MB   |
| 14    | 31 ms      | 15 371 chars       | 120 MB   |
| 16    | 131 ms     | 61 451 chars       | 122 MB   |
| 18    | 599 ms     | 245 771 chars      | 134 MB   |
| 20    | 2 634 ms   | 983 051 chars      | 179 MB   |
| 21    | 2 664 ms   | 1 078 chars        | 182 MB   |
| 24    | 2 809 ms   | 1 239 chars        | 179 MB   |
| 40    | 2 673 ms   | 2 135 chars        | 182 MB   |
| 100   | 2 690 ms   | 5 613 chars        | 185 MB   |

Twenty rungs is the last length that expands. At twenty-one the first rung's
query collapses from a megabyte to a kilobyte — the authored query with one
printed negation per later rung, which is what the merge hands back.

## Why the cost stops growing rather than moving

Past twenty rungs the wall clock sits at about 2.7 seconds and stays there, at
forty rungs and at a hundred. That is not the bound deferring the work: a ladder
of any length still contains a twenty-rung ladder among its later rungs, and
that one is under the bound and expands in full. So 2.7 seconds is the ceiling
the bound buys, not a step on the way to a worse one.

## What the bound does not cap

The boundary is crossed once per `and` node, so the ceiling above is per `and`
list rather than per compile. Lengthening a ladder stops costing more; widening
one still does. A comma-separated query is several `and` lists and each pays
separately — the same twenty-rung ladder, with the first key given `d`
disjuncts, measured by `disjunct-cost.cjs` beside this file:

| Disjuncts | Wall clock | First rung's query | Resident |
| --------- | ---------- | ------------------ | -------- |
| 1         | 2 598 ms   | 983 051 chars      | 182 MB   |
| 2         | 3 977 ms   | 1 966 097 chars    | 240 MB   |
| 4         | 6 641 ms   | 3 932 189 chars    | 354 MB   |
| 8         | 12 102 ms  | 7 864 373 chars    | 577 MB   |

Linear in the number of disjuncts, not exponential — and the same shape costs
the reference implementation the same way, with no bound at all. A budget spread
across a whole declaration would cap it. It is not built, because it answers a
different question from parity and nothing has asked for it.

## The bound

`MAX_DISTRIBUTION_DEPTH = 18` in `crates/stylex-css-parser/src/at_queries/
media_query.rs`. Each `not (A and B)` clause splits the rule list in two, so a
list carrying `d` of them costs `2^d` branches; an `n`-rung ladder reaches depth
`n - 2`, because its first and last rungs are single bounds and split nothing.
The depth is measured before the expansion starts, at the boundary
`merge_and_simplify_ranges`, and a list too deep is handed straight back.

Eighteen is chosen against output size, not against stack depth. Stack depth is
not what runs out in either compiler — a bound generous enough for 26 rungs
would permit a 63 MB single query long before the stack noticed.

## Where this compiler stops and the reference does not

Reported rather than left implicit, because it is a real divergence and it is
deliberate: **from twenty-one rungs on, `@stylexjs/babel-plugin` 0.19.0 still
merges and this compiler no longer does.** The reference was measured merging at
21, 22, 23, 24, 26 and 28 rungs — the last of those taking 435 seconds, emitting
252 MB of query text and reaching about 7.4 GB resident. It has no give-up of
its own to match: its `try`/`catch` is never reached, and past 28 rungs it dies
rather than degrades. Byte parity past the bound is therefore unattainable in
principle, and what is above the bound is a compile nobody wants to finish.

Below the bound the two agree character for character, which is where every
ladder anyone writes lives.
