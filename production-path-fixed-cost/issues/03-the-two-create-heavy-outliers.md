# 03 — The two create-heavy outliers cost three times the rest

Status: `ready-for-human`
Blocked by: None — independent of 01 and 02

**What to answer:** whether `Feature - dynamic styles` (+9.8%) and
`Performance - Complex create` (+9.6%) are the same cost as the +2-3% everything
else pays, or a second one on top of it.

They stand out by a factor of three, and they have something in common the rest
do not: both are `create`-heavy, and the dynamic-style path is what this branch
rewrote for issue [#1266](https://github.com/Dwlad90/stylex-swc-plugin/issues/1266)
— the ordered reference-resolution chain, the eight-step identifier resolution,
the depth ceiling, the inline-style fall-through. `Feature - dynamic styles` is
twelve namespaces that are all functions, so it takes that path twelve times per
transform.

Against that: `counter-with-dynamic-styles` is **−4.1%**, i.e. faster, and it is
also a dynamic-style fixture. So "the dynamic-style rewrite is slower" is not
established, and the two outliers may be nothing more than the noise the spec
documents — individual rows have moved by several points between runs of the same
comparison.

**First step, cheapest one: repeat the measurement.** Ten rounds on those two
fixtures plus `counter-with-dynamic-styles`, paired against `develop`. If +9.6%
holds and −4.1% holds, there is something to explain; if they converge on the
group mean, close this ticket and put the numbers in the comments so the next
reader does not chase them again.

**If it holds:** the fixtures differ in what the fold *does*, so compare the
counts, not the times. The evaluator's memo (`seen`) and the depth counter make
this measurable without a profiler: count `evaluate_cached` entries, memo hits
and refusals per transform on both revisions for all three fixtures. A rewrite
that folds more nodes, or memoizes fewer of them, shows up as a count rather than
as a mystery.

**Do not** fix this by lowering `maxEvaluationDepth` or by weakening the chain.
Both are what issue #1266 bought, and the two fixtures that regress are the ones
whose whole purpose is to exercise them.

## Comments

**It holds, and it is not the memo.** Both questions the ticket asks are
answered, in the order it asks them.

### The measurement repeats

Ten rounds, paired against a `c83ac5cbd` build of the same package, balanced
subject order, one process. Round-to-round spread was under a point on every
row -- far tighter than the several points the spec warns about, because the
spec's variability is between *runs*, and the harness alternates subjects inside
one:

| fixture                                | `develop` |   branch | delta  |
| -------------------------------------- | --------- | -------- | ------ |
| `Performance - Complex create`         |   2.80 ms |  3.06 ms | +9.3%  |
| `Feature - dynamic styles`             |  977.8 µs |  1.07 ms | +9.4%  |
| `counter-with-dynamic-styles`          |  723.6 µs | 695.4 µs | −3.9%  |
| `Feature - dynamic styles (dev)`       |   1.58 ms |  1.29 ms | −18.4% |

So the +9.6% and the +9.8% in the spec are real, the −4.1% is real, and the
three do not converge on the group mean. There is something to explain.

### The counts say it is not what the ticket expected

Counters on `evaluate_cached` -- entries, resolved hits, unresolved hits,
misses, and refusals -- built into a throwaway patch applied identically to both
revisions and driven by a one-file example binary. Per transform:

| fixture                       | rev       | entries | hits | misses | refusals |
| ----------------------------- | --------- | ------- | ---- | ------ | -------- |
| `counter-with-dynamic-styles` | `develop` |     111 |   28 |     82 |        4 |
| `counter-with-dynamic-styles` | branch    |      67 |   21 |     45 |        5 |
| `create-complex`              | `develop` |     138 |   57 |     81 |       13 |
| `create-complex`              | branch    |     138 |   57 |     81 |       13 |
| `dynamic-styles`              | `develop` |      31 |    2 |     24 |       19 |
| `dynamic-styles`              | branch    |      31 |    2 |     24 |       19 |

The two fixtures that cost 9% more fold **exactly the same nodes**, with the
same hit, miss and refusal split, and refuse nothing on depth. The one that got
*faster* is the one whose counts moved -- it folds 40% fewer nodes on this
branch.

That closes the ticket's hypothesis. "A rewrite that folds more nodes, or
memoizes fewer of them" is not what happened: the fold does the same amount of
work and takes longer doing it. Nor is it the depth ceiling, which never fires
on any of the three.

Which also means the cost is *not* proportional to `evaluate_cached` entries.
138 entries cannot carry 260 µs between them; that would be 1.9 µs each, against
a whole-transform cost of 2.8 ms. Whatever it is sits in what the `create`
transformation does around the fold -- flattening, shorthand expansion, rule
construction -- on shapes these two fixtures have and `counter-with-dynamic-styles`
does not, and that is a different ticket from this one.

**Not** to be chased by lowering `maxEvaluationDepth` or weakening the chain,
per the ticket -- and now with a measured reason rather than a policy one: the
ceiling refuses nothing on any of these three, so lowering it would change
nothing but the outputs.
