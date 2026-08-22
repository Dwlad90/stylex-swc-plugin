# 29 — The memo key makes the fold quadratic in depth

Status: `resolved`
Blocked by: None

**What was measured.** `evaluate_cached_within_budget` opens every level with

```rust
let cleaned_path_hash = stable_hash_unspanned(path);
```

and `stable_hash_unspanned` (`stylex-utils/src/hash.rs:173`) walks the **whole
remaining subtree**. So folding an n-deep expression hashes n + (n−1) + (n−2) …
nodes: the memo that exists to avoid repeated work pays O(subtree) to decide
whether it can avoid it.

Timed on one tower of `(MY_CONST + 1)` with the ceiling raised, output held
constant at ~286 bytes so nothing but the fold is being measured:

| tower depth | fold time | vs previous |
| ----------- | --------- | ----------- |
| 60          | 1.68 ms   | —           |
| 120         | 4.35 ms   | 2.6×        |
| 240         | 14.4 ms   | 3.3×        |
| 480         | 54.1 ms   | 3.8×        |

Doubling the depth costs between 2.6× and 3.8×, converging on 4×. That is
quadratic, and the constant is large: 54 ms to fold a single expression that
produces one declaration.

There is a second cost on the same line. `stable_hash_unspanned` falls back to
`stable_hash(&drop_span(path.clone()))` for shapes its in-place hasher does not
cover — a full deep clone of the subtree, again per level. How often that arm is
taken has not been measured.

**Why this is not urgent.** The ceiling from 20 is 32 by default, so depth is
bounded at a small constant and the quadratic term never grows. Nothing in this
workspace's fixtures spends more than a handful of levels. This became worth
knowing only because 20 made deep input reachable at all: before it, the same
input aborted.

**Why it is not the recursion.** Asked and answered while closing 20, because
"remove the recursion to make it faster" is the obvious first move and it is the
wrong one. The frames are not what grows — the numbers above are with the
recursion in place and the output constant, and an explicit work stack removes
O(1) per node while leaving the O(n) hash on every one of them. Recorded in
`crates/stylex-transform/docs/adr/0004-the-fold-owns-its-own-ceiling-and-its-own-stack.md`
so the measurement is not repeated.

**Why it is not a refactor.** The key is a _structural, span-insensitive_ hash on
purpose. Two expressions that differ only in position must land on the same memo
entry, and the same hash buckets the JSX-spread replacement map, where a
collision is guarded by an `eq_ignore_span` check rather than by the hash being
unique. Any cheaper key has to keep both properties, so this is a correctness
question wearing a performance question's clothes.

- [x] Measure how often the `drop_span(path.clone())` fallback arm is taken on a
      real project, since a deep clone per level is the worse of the two costs if
      it is common — **6 keys in 15 103, 0.04%**, and never on a deep expression
- [x] Decide whether the key can be made incremental — a parent's hash composed
      from its children's, computed on the way back up rather than on the way
      down — without losing span-insensitivity — **it can, and composition is
      not the hard part; node identity is**. Deferred to 33 with the mechanism
      written down
- [x] ~~If it can, the JSX-spread bucket has to be re-measured against it too,
      not only the evaluator's memo~~ — nothing to re-measure until 33 lands;
      carried into it as a checkbox, along with the two consumers that do not
      confirm a hit
- [x] Pin the scaling as a test or a bench, so a future change to the key reports
      as a changed curve rather than as nothing — both:
      `benches/evaluate_depth_bench.rs` in wall-clock and
      `stylex_utils`' `key_cost_scaling_tests` in counted bytes

## Answer

**The clone is not the cost, and the walk is nearly all of it.** Both halves
measured; the verdict is recorded as
`crates/stylex-transform/docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md`,
which is the document to read rather than this section.

**The fallback arm: 0.04%.** Instrumented over a real corpus — a 6 885-line
slice of `apps/rollup-large-example/lotsOfStyles.js`, `lotsOfStylesDynamic.js`,
both performance-fixture themes, the six benchmark perf fixtures and every
`tests/fixture/*/input.stylex.js` — the arm is taken **6 times in 15 103 keys**,
and not once on a deep expression. It is selected by a shape rather than by a
depth, and only two shapes reach it in practice: an object literal past the
128-entry limit (2 of 1 121 keys on `colorThemes.js`, whose palette has 130
colours) and a **block-bodied arrow inside a `useMemo`** (2 of 33 keys on the
`use-memo` fixture, which is post-Fast-Refresh React output — the one real-world
producer found). Priced either side of the boundary the arm costs 12.3 µs against
4.9 for one extra property, so 2.5× on 0.04% of keys. The instrumentation was
temporary and is not in the tree; what replaces it is the boundary and
per-level-fallback cases in `key_edge_case_tests`.

**The walk: ~95% of the fold.** Re-measured in release, which is where the
earlier debug numbers were misleading about the constant but right about the
curve. One tower of `(MY_CONST + 1)`: 24.8 µs at 30 levels, 83.9 at 60, 306 at
120, 1164 at 240 — 3.4× to 3.8× per doubling. One key over the same tower is
_exactly_ linear: 1.17 µs, 2.32, 4.62, 9.15. Priced per byte and summed over the
levels the fold descends, the keys account for ~1.10 ms of the 1.16 ms at 240
levels. The arms, the frames and the arithmetic share the remainder, so this is
not one term in the fold's cost at depth — it is the fold's cost at depth.

**Incremental: possible, and blocked on something the AST does not have.**
Composition alone buys nothing; a hash composed from its children's still visits
every node unless the children's hashes are _retained_ between levels, and
retaining them needs a node identity. SWC's `Expr` has none: the span is not one
(the evaluator folds nodes it synthesized, all carrying `DUMMY_SP`), and the
address is only valid while the node is alive (a node synthesized at one level
and dropped when it returns frees its address for the next level's, which is a
silent wrong hit). The sound version is the address _scoped to the level that
owns the nodes_ — entries added while hashing level `L`'s subtree all outlive
`L`'s children, so a cache truncated as `L` returns holds nothing dead and drops
nothing live. That plus a compositional walk makes the tower linear. It is a
rewrite of all forty arms of `stylex-utils/src/hash.rs` and a lifetime
discipline for the fold to maintain, for a term the default ceiling bounds at 32
levels, so it is 33 rather than this ticket.

**Found on the way, and filed rather than fixed.** Two of the key's four
consumers act on a hash hit _without_ confirming with `eq_ignore_span` — the
evaluator's own memo and `InsertionSlot::BeforeDecl` — which the
`stylex-utils` glossary claimed none of them did. The glossary is corrected here
and the consequence is 34: a collision is a wrong folded value or a misplaced
injection, not a slower fold, and it is the constraint any cheaper key has to
respect.

**A smaller finding, pinned in place.** A literal carries the raw text it was
written as, and the key covers it, so `1` read out of a file and `1` the
compiler synthesized do not share a memo entry. That costs a duplicated entry
and never a wrong one — and the raw text is what tells `1` from `1.0` — so it is
pinned as behaviour in `key_edge_case_tests` rather than treated as a bug.

**Superseded in one respect.** The timings above are of the 64-bit SipHash key
this ticket measured. 34 replaced it with a 128-bit xxh3 key, which is _faster_
than what was measured here -- a deep fold by ~30%, one key by 30-43% -- so every
absolute number in this ticket is now an upper bound. The curve, the fallback
frequency and the verdict on incrementality are unchanged; only the constant
moved. Current numbers live in
`crates/stylex-transform/docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md`.
