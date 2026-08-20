# 29 — The memo key makes the fold quadratic in depth

Status: `needs-triage`
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
| --- | --- | --- |
| 60 | 1.68 ms | — |
| 120 | 4.35 ms | 2.6× |
| 240 | 14.4 ms | 3.3× |
| 480 | 54.1 ms | 3.8× |

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

**Why it is not a refactor.** The key is a *structural, span-insensitive* hash on
purpose. Two expressions that differ only in position must land on the same memo
entry, and the same hash buckets the JSX-spread replacement map, where a
collision is guarded by an `eq_ignore_span` check rather than by the hash being
unique. Any cheaper key has to keep both properties, so this is a correctness
question wearing a performance question's clothes.

- [ ] Measure how often the `drop_span(path.clone())` fallback arm is taken on a
      real project, since a deep clone per level is the worse of the two costs if
      it is common
- [ ] Decide whether the key can be made incremental — a parent's hash composed
      from its children's, computed on the way back up rather than on the way
      down — without losing span-insensitivity
- [ ] If it can, the JSX-spread bucket has to be re-measured against it too, not
      only the evaluator's memo
- [ ] Pin the scaling as a test or a bench, so a future change to the key reports
      as a changed curve rather than as nothing
