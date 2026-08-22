# 33 — Make the memo key incremental

Status: `wontfix`
Blocked by: None — 29 answered the design question this implements.

**What this is.** The implementation 29 decided was possible and deferred. Read
`crates/stylex-transform/docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md`
first; it carries the measurement, the mechanism and the reasons the obvious
variants do not work. This file is only the plan.

**The cost being removed.** `evaluate_cached_within_budget` keys its memo on
`stable_hash_unspanned(path)`, which walks the whole remaining subtree, at every
level. Release, one tower of `(MY_CONST + 1)`: 25.4 µs at 30 levels, 85.9 at 60,
309 at 120, 1179 at 240, of which ~1.13 ms at 240 is the keys. Linear per key,
quadratic per fold, and ~96% of the fold at depth.

**The mechanism.** Two halves, neither of which works alone:

1. **A compositional walk.** `hash.rs` changes from "write the subtree into a
   `Hasher`" to "return a `u64` for this node, mixed from its children's". The
   values all change; nothing depends on them (no consumer persists a key, none
   derives a class name, and output order comes from source order).
2. **A cache keyed by address, scoped to the level that owns the nodes.** The
   address is a valid identity only while the node is alive, and entries added
   while hashing level `L`'s subtree belong to nodes that outlive every child
   level beneath `L`. So `evaluate_cached` records the cache's length on entry
   and truncates to it on the way out: nothing dead is kept, nothing live is
   dropped, and a node synthesized at a sibling level cannot inherit a freed
   node's entry.

The gain is that a descent into a node already hashed as part of its parent's
subtree is O(1), which is the whole of the tower case.

- [x] Land the compositional walk with the existing `hash_test.rs` cases green —
      the span-insensitivity and fallback-agreement cases are the contract, and
      they must not be relaxed to fit the new walk
- [x] Add the scoped cache and prove the scope discipline: a case where a level
      synthesizes an expression, a sibling level allocates over it, and the two
      must not share a key
- [x] Confirm the collision resistance did not drop. Two consumers act on a hash
      hit alone (34), so a mixing function weaker than the current SipHash walk
      is a wrong fold rather than a slower one
- [x] Re-measure `benches/evaluate_depth_bench.rs` — both groups. The curve
      flattening is the whole claim
- [x] Re-measure the JSX-spread bucket, carried over from 29: it keys off the
      same hash and is not reached through `evaluate_cached`, so it gains the new
      values without the new cache and has to be shown not to have lost anything
- [x] Update `key_cost_scaling_tests`, whose asserted ~4× ratio is a statement
      about the current key. It should become a statement about the new one, not
      be deleted
- [x] Re-run `bench:revisions` plus `bench:verdict` on a real project, since the
      point of the change is a curve nothing in this workspace's fixtures reaches

## Answer

Built, measured, not kept. The mechanism works exactly as 29 predicted and the
numbers say not to ship it.
[`0006-an-incremental-memo-key-was-built-and-measured-slower.md`](../../../crates/stylex-transform/docs/adr/0006-an-incremental-memo-key-was-built-and-measured-slower.md)
is the record; the whole diff is [`33-composed-key.patch`](./33-composed-key.patch)
beside this file, and it applies to the commit that added that ADR.

**Both halves landed and the suite was green** -- 3 969 tests across the
workspace, including the 989-case transform suite, so the composed key produces
byte-identical output. Clippy clean over `--workspace --all-features
--all-targets`. The scope discipline held and is not why this was rejected; the
two things it turned up that the plan did not name are that a cloned
`StateManager` has to start with an empty cache, and that a panic unwinding out
of a fold skips every truncation below it, so the outermost level has to clear
rather than truncate.

**The curve flattened, and the constant went the other way by more.**

| depth | fold before | fold after | one key before | one key after |
| --- | --- | --- | --- | --- |
| 30 | 18.1 µs | **19.8 µs** | 0.67 µs | **4.36 µs** |
| 60 | 61.6 µs | 39.3 µs | 1.74 µs | 8.18 µs |
| 120 | 217.9 µs | 78.6 µs | 3.80 µs | 17.2 µs |
| 240 | 821.5 µs | 159.1 µs | 6.49 µs | 37.0 µs |

−81% at 240 and **+10% at 30**, which is the only column under the shipped
ceiling of 32. One uncached key costs 5.7× more, because the walk went from one
hasher per key to one per node. That is close to its floor: a 128-bit
finalization is ~13 ns against ~13.5 ns for a whole node of the streamed walk,
so doubling the uncached per-node cost is what composition *is*. Two rounds of
optimisation are in the ADR (streaming `Xxh3Default` per node → stack-buffered
one-shot `xxh3_128`, then a 64-byte buffer and inlined accessors) and recovered
what was available.

**Every real fixture regressed**, `benches/evaluate_bench.rs`, µs:

| fixture | before | after | change |
| --- | --- | --- | --- |
| `create-complex.js` | 41.61 | 57.30 | **+38%** |
| `dynamic-param-shadows-import-edges` | 13.98 | 19.88 | **+42%** |
| `dynamic-param-shadows-import` | 3.00 | 3.88 | +29% |
| `create-basic.js` | 10.91 | 13.56 | +24% |
| `sizes.stylex.js` | 19.82 | 23.37 | +18% |
| `colors.stylex.js` | 35.13 | 40.50 | +15% |
| `createTheme-basic.js` | 30.31 | 34.43 | +14% |
| `createTheme-complex.js` | 3 244 | 3 318 | +2% |

The reason is structural rather than an implementation defect: StyleX's input is
wide and shallow, so most nodes are visited once whatever the key does and
collect nothing back from a cache, while the deep spine the cache is built for is
the shape `maxEvaluationDepth` forbids at 32. The crossover sits above the
ceiling.

**On the last checkbox.** `bench:revisions` / `bench:verdict` were not run. Their
gate is a 10% warn and a 20% fail per fixture, and six of eight fixtures are
already past the fail threshold on a cheaper harness that agrees with them in
direction; building two `dist/*.node` subjects to confirm a decided answer is not
what that gate is for. If the decision is ever reopened, run it then.

**Reopening this needs a new reason** -- a project on a raised ceiling, or an
input shape that is deep rather than wide -- not a fresh reading of the same
quadratic. The variant to build in that case is named in the ADR: compose only
when the ceiling is raised past the crossover, and stream otherwise.
