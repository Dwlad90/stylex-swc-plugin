# Ticket 06 — the collision, and why it could not land red

Two entries of one conditional value map can canonicalize to the same query
text once ticket 05 stops pruning contradictory branches. The reference
implementation writes its rewritten keys into a plain object, so the second
assignment replaces the first entry's value while keeping that entry's
position, and one authored declaration disappears.

Ticket 06 asked for that as a failing test. It cannot fail: this compiler
already produces the same output, byte for byte, for every collision that could
be found or constructed.

- reference implementation: `@stylexjs/babel-plugin` 0.19.0
- this compiler: `@stylexswc/rs-compiler` 0.18.4, from `dist/`, built after
  ticket 05

## Where the semantics already come from

The transform hands back a sequence and does keep both colliding entries in it.
Its only consumer, `flatten_raw_style_object_logic` in `stylex-transform`,
immediately writes that sequence into an `IndexMap` keyed by the property key —
one map per nesting level, matching the object the reference builds per
recursion level. Inserting a key already present keeps its position and replaces
its value, which is what a JavaScript object assignment does. So the loss is
reproduced one layer below where the reference produces it.

## The search

`collision-search.cjs`, beside this file, enumerates ordered conditional value
maps over an alphabet chosen for collisions: queries that contradict outright,
queries already spelled in canonical form, a `not all` written by hand, and a
non-numeric rule that makes the merge bail out. Each map is compiled by both
compilers and the full rule list — class name and rule text, in emission order —
is compared.

| Run                          | Maps | Disagreements |
| ---------------------------- | ---- | ------------- |
| 3 keys                       | 504  | 0             |
| 4 keys                       | 3024 | 0             |
| 3 keys, nested under `:hover` | 504  | 0             |

An earlier run over a second alphabet, 5040 ordered four-key maps, also found
none.

## The one case a search cannot settle

The reference deletes the old key and assigns the new one *inside* its loop, so
a rewritten key could in principle land on a later authored key that has not
been processed yet — overwriting that entry's value before it is read. This
compiler reads every value first, so it would keep both.

That case is unreachable, and by argument rather than by sampling. The rewritten
key for entry `i` carries `not q_j` for every later entry `j`. Either the merge
reads those constraints numerically, in which case intersecting with `not q_j`
leaves a range disjoint from `q_j` and the two texts cannot be equal; or a rule
blocks the merge, in which case `not q_j` survives verbatim in the text and
makes it longer than `q_j`; or the intersection empties and the key becomes
`not all`, which equals a later authored key only if that key is written
`@media not all` — and negating *that* is a non-numeric rule, which blocks the
merge and puts the case back in the second branch.

## What ticket 06 landed instead

The tests it asked for, passing rather than failing: the end-to-end seam pins
the rule count, which of the two colliding declarations survives, and the
position it survives at; the parity corpus pins the same subject against the
reference implementation, which is the seam that speaks up if the reference ever
changes its mind.
