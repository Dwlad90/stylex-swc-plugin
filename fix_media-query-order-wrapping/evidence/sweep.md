# Ticket 09 — the verification sweep

Three questions, each asked by compiling the same module through both compilers
and comparing every emitted rule. `sweep.cjs` beside this file runs them and
prints the report this summarizes.

- reference implementation: `@stylexjs/babel-plugin` 0.19.0
- this compiler: `@stylexswc/rs-compiler` from `dist/`, built after ticket 08
- 27 of 30 comparisons agree; the three that do not are named below

## 1. At-rule order — no change needed

The rewritten keys are much longer than the ones an author wrote, and at-rule
sorting compares the final text, so a key can sort to a different place than it
would have. It does not: four arrangements were compared and all four agree,
rule for rule and class for class.

| Arrangement                                            | Verdict |
| ------------------------------------------------------ | ------- |
| a ladder beside `@supports` and `@container`            | agree   |
| a rewritten key that sorts before its authored spelling | agree   |
| rewritten keys nested under a pseudo-class              | agree   |
| non-media properties on both sides of a ladder          | agree   |

The comparator is untouched. That is the deliverable for this half: the order
was checked rather than assumed.

## 2. The invalid-syntax refusal — one family closed, one named

Twenty-two malformed or unusual queries. Sixteen agreed on arrival. Six did not,
in two families.

### Closed: unbalanced parentheses

`(min-width: 100px`, `(width: calc(100px)`, and `((min-width: 100px)` were all
accepted here and refused by the reference implementation. The cause is the
tokenizer: `cssparser` synthesizes a closing parenthesis at end of input, so the
parse succeeds and a query the author never wrote reaches the stylesheet. The
reference's tokenizer synthesizes nothing, so its parse fails outright.

Fixed by routing the transform's parse through `validate_media_query`, which
already carried the balanced-parenthesis check and mirrors the reference's own
`validateMediaQuery` — it was simply not on the path the transform took. All
three now refuse, with the same invalid-media-query-syntax error.

The check had to be made CSS-aware to do it without refusing too much. Counting
raw characters, as the reference's own `_hasBalancedParens` does, calls
`@media (min-width: 100px) and (\(: 1)` unbalanced — but that counter never runs
on this path there, so the reference accepts the query and we would have started
refusing it. The check now skips a parenthesis written as an escape and one
inside a string, and treats an unterminated string as unbalanced in its own
right, since it swallows whatever would have closed the parenthesis it sits in.
Four inputs cover the corners and all four agree:

| Input                                          | Both compilers |
| ---------------------------------------------- | -------------- |
| `@media (min-width: 100px) and (\(: 1)`         | accept         |
| `@media (min-width: 100px) and (foo: "(")`      | refuse         |
| `@media (min-width: 100px) and (foo: "()")`     | refuse         |
| `@media (min-width: "100px)`                    | refuse         |

### Named, not closed: a more permissive grammar

Three inputs the reference implementation refuses and this compiler still
accepts:

| Input                                                   | We emit                              |
| ------------------------------------------------------- | ------------------------------------ |
| `@media (min-width: 100px) and (max-width: 200px) or (color)` | a comma-joined `or` of the two sides |
| `@media ((min-width: 100px))`                           | `@media (min-width: 100px)`          |
| `@media (((( min-width: 100px ))))`                     | `@media (min-width: 100px)`          |

These are grammar-level acceptance differences, not consequences of this work:
our rule parser accepts a parenthesized single condition and an unparenthesized
mix of `and` and `or`, where the reference's `oneOf` chain accepts neither.
Closing them means changing the media query grammar, which is a different change
from this one and carries its own regression surface. Filed as ticket 13.

Worth recording beside them: **the reference implementation's parser backtracks
exponentially in parenthesis nesting depth.** Measured on the same machine, one
query per process — 2 levels 22 ms, 4 levels 52 ms, 8 levels 1.18 s, 12 levels
20.5 s, 16 levels did not finish in 30 s. This compiler is flat: 200 levels in
2 ms. Any future work on the grammar should not reach parity by reaching for
that behaviour too.

## 3. The ordering option — no change needed

| Case                                   | Verdict |
| -------------------------------------- | ------- |
| default, option unset                  | agree   |
| explicitly enabled                     | agree   |
| opted out                              | agree   |
| opted out, authored spelling preserved | agree   |

The default matches, and opting out hashes the query the author wrote —
`@media (max-height:120px) and (min-width: 720px)` stays spelled that way, with
its own class name, in both. Nothing changed.
