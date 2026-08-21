# The memo key is a whole-subtree hash

**Status:** accepted

The evaluator memoizes what it folds. The key is
`stable_hash_unspanned(path)`: a structural, span-insensitive hash of the
expression about to be folded, taken at every level of the descent. Because the
key covers the whole remaining subtree, folding an `n`-deep expression hashes
`n + (n−1) + (n−2)` … nodes. The memo that exists to avoid repeated work pays
O(subtree) to decide whether it can avoid it.

**That cost is not a term in the fold's cost; it is nearly all of it.** Measured
in release on one tower of `(MY_CONST + 1)`, output held constant:

| depth | fold    | vs previous | one key | vs previous |
| ----- | ------- | ----------- | ------- | ----------- |
| 30    | 24.8 µs | —           | 1.17 µs | —           |
| 60    | 83.9 µs | 3.4×        | 2.32 µs | 2.0×        |
| 120   | 306 µs  | 3.6×        | 4.62 µs | 2.0×        |
| 240   | 1164 µs | 3.8×        | 9.15 µs | 2.0×        |

One key is exactly linear in the depth beneath it; the fold buys one per level,
so its curve converges on 4× per doubling. Priced per byte off the 240-level key
and summed over the levels the fold descends, the keys account for ~1.10 ms of
the 1.16 ms the fold takes — ~95%, with the arms, the frames and the arithmetic
sharing the rest.

**The key stays as it is.** Three things decide that, and the third is the one
that matters.

**It is bounded.** `maxEvaluationDepth` defaults to 32
([0004](./0004-the-fold-owns-its-own-ceiling-and-its-own-stack.md)), so the
quadratic term is a small constant unless a project raises it. Nothing in this
workspace's fixtures spends more than a handful of levels; the deepest real
input measured — a 6 800-line slice of `lotsOfStyles.js` — averages 14.5 nodes
per key over 7 755 keys.

**The clone is not the problem.** `stable_hash_unspanned` hands shapes its
in-place walk does not cover to `stable_hash(&drop_span(path.clone()))`, a deep
clone and a second walk. Instrumented over a real corpus — that slice, the
dynamic-styles file, both perf themes, the six benchmark fixtures and every
transform fixture input — the arm is taken **6 times in 15 103 keys, 0.04%**,
and never once on a deep expression. Where it is taken it is selected by a
shape, not by a depth: an object literal past the 128-entry limit (2 of 1 121
keys on `colorThemes.js`, a theme with a 130-colour palette) and a block-bodied
arrow inside a `useMemo` (2 of 33 keys on the `use-memo` fixture). Priced either
side of the boundary, the arm costs 12.3 µs against 4.9 for one extra property —
2.5×, once, on 0.04% of keys. The per-level walk on the other 99.96% is the
cost.

**A cheaper key is a correctness question.** Four things consume this hash, and
they do not agree about how much it has to mean:

- the evaluator's `seen` memo returns a cached fold on a **hash hit alone**;
- `InsertionSlot::BeforeDecl` splices a declaration's style metadata on a
  **hash hit alone**;
- the JSX-spread replacement map and the queued-decl dedup narrow a bucket by
  hash and then confirm with `eq_ignore_span`.

So the key is load-bearing for two consumers and only a bucket for two others. A
key with more collisions is a wrong fold and a misplaced injection, not a slower
one. It must also stay span-insensitive, because a synthesized expression has to
land on the entry its parsed twin landed on.

What the key does _not_ have to be is any particular number: no consumer
persists it, none derives a class name from it, and output order comes from
source order rather than from hash order. An incremental key is free to produce
entirely different values.

## Considered options

**Compose the key from its children's, on the way back up.** The obvious fix,
and it does not work on its own: a hash composed from child hashes still visits
every node unless the children's hashes are _retained_ between levels, and
retaining them needs something that identifies a node. The memo's own key cannot
be that — it is what is being computed.

SWC's `Expr` carries no node id. Two candidates stand in for one, and both fail
alone:

- **The span.** Not an identity: the evaluator folds expressions it synthesized,
  which carry `DUMMY_SP`, and a member read or a function return re-emits nodes
  it did not parse. Every synthesized node would share one entry.
- **The address.** Valid only while the node is alive. A node synthesized at one
  level, hashed, and dropped when that level returns leaves its address free for
  the next level's synthesized node — a stale hit, which is a wrong key and then
  a wrong fold.

The sound version is the address _scoped to the level that owns the nodes_:
entries added while hashing level `L`'s subtree all belong to nodes that live
until `L` returns, so a cache truncated to its entry length as `L` returns holds
no dead entry and drops no live one. With a compositional walk that makes a
tower linear, and the cache is the fold's, not the key's.

Rejected **for now**, not on the merits: it rewrites the walk in
`stylex-utils/src/hash.rs` from "write into a `Hasher`" to "return a `u64` per
node" across all forty of its arms, changes every key's value, and adds a
lifetime discipline the fold has to maintain, all for a term the default ceiling
already bounds at 32 levels. Filed as its own work rather than done here.

**Key by node identity instead of by structure, as the reference
implementation does.** `@stylexjs/babel-plugin` 0.19.0 declares
`seen: Map<t.Node, Result>` and looks up `seen.get(node)`
(`src/utils/evaluate-path.js`, `evaluateCached`) — JavaScript hands it object
identity for free, so upstream never asks this question at all. That is the same
missing capability seen from the other side: what costs us a subtree walk costs
upstream a pointer compare. It would also drop the sharing between
structurally-identical expressions at different positions, which this memo has
and upstream does without.

One behavioural difference falls out of the same line, and it is not a
divergence. Because entries are shared, an expression can meet an
_in-progress_ entry recorded for a structurally-identical but distinct node and
refuse where upstream would evaluate. Reaching that state needs the two nodes to
be on one evaluation stack, which needs them mutually reachable through their
bindings — a cycle, which both compilers refuse anyway. Ours refuses it one
level earlier. Reasoned rather than exhausted: no input has been found that
separates them.

**Hash a bounded prefix of the subtree.** O(1) per level, and it moves the cost
rather than removing it: the collisions it invites have to be resolved by
`eq_ignore_span`, which walks the subtree on a _hit_, and two consumers do not
confirm at all. That is the correctness question above, answered wrong.

**Remove the recursion.** Answered while closing
[0004](./0004-the-fold-owns-its-own-ceiling-and-its-own-stack.md): the frames
are not what grows. An explicit work stack removes O(1) per node and leaves the
O(n) key on every one of them.

## Consequences

**The curve is pinned twice, in two units.** `benches/evaluate_depth_bench.rs`
measures the fold and one key across four doublings, and
`stylex_utils`' `key_cost_scaling_tests` counts the bytes the walk feeds its
hasher — deterministic, machine-independent, and asserting the ratio rather than
a time. A change to the key that flattens the curve reports in both; one that
keeps it reports the same numbers.

**A synthesized literal does not share a parsed one's entry.** `Number` and
`Str` carry the raw text they were written as, `None` for a literal nothing
wrote, and the key covers it. So `1` read out of a file and `1` the compiler
built fold to the same value under two entries. That costs a duplicated entry,
never a wrong one, and the raw text is what distinguishes `1` from `1.0` and
`'\u0041'` from `'A'` — which a diagnostic quotes back. Pinned, not fixed.

**The key's own limit is the thread's stack, not the ceiling.** It recurses over
whatever it is handed, and it is handed expressions before the fold's counter has
an opinion. 1024 levels is pinned as answering; past that it belongs to the same
residue as the other unbounded stages around the fold.
