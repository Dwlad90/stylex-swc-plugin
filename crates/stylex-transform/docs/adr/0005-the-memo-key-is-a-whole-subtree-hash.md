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

| depth | fold     | vs previous | one key | vs previous |
| ----- | -------- | ----------- | ------- | ----------- |
| 30    | 18.9 µs  | —           | 0.67 µs | —           |
| 60    | 63.1 µs  | 3.3×        | 1.59 µs | 2.4×        |
| 120   | 220.8 µs | 3.5×        | 3.15 µs | 2.0×        |
| 240   | 823.7 µs | 3.7×        | 6.44 µs | 2.0×        |

One key is linear in the depth beneath it; the fold buys one per level, so its
curve converges on 4× per doubling. Priced per byte off the 240-level key and
summed over the levels the fold descends, the keys account for ~777 µs of the
824 µs the fold takes — ~94%, with the arms, the frames and the arithmetic
sharing the rest.

**The key stays a whole-subtree hash, and got wider.** Three things decide the
first half, and the third is what decided the second.

**It is bounded.** `maxEvaluationDepth` defaults to 32
([0004](./0004-the-fold-owns-its-own-ceiling-and-its-own-stack.md)), so the
quadratic term is a small constant unless a project raises it. Nothing in this
workspace's fixtures spends more than a handful of levels; the deepest real input
measured — a 6 885-line slice of `lotsOfStyles.js` — averages 14.5 nodes per key
over 7 755 keys, and the whole key walk for that file is ~2 ms of a 26 ms
transform.

**The clone is not the problem.** `stable_hash_unspanned` hands shapes its
in-place walk does not cover to `stable_hash_wide(&drop_span(path.clone()))`, a
deep clone and a second walk. Instrumented over a real corpus — that slice, the
dynamic-styles file, both perf themes, the six benchmark fixtures and every
transform fixture input — the arm is taken **6 times in 15 103 keys, 0.04%**, and
never once on a deep expression. Where it is taken it is selected by a shape, not
by a depth: an object literal past the 128-entry limit (2 of 1 121 keys on
`colorThemes.js`, a theme with a 130-colour palette) and a block-bodied arrow
inside a `useMemo` (2 of 33 keys on the `use-memo` fixture). Priced either side of
the boundary, the arm costs 8.5 µs against 2.7 for one extra property — 3.2×,
once, on 0.04% of keys. The per-level walk on the other 99.96% is the cost.

That pair only means anything while the two legs take different arms, and the
boundary is a private constant the bench cannot read — raise it and the gap
collapses to nothing, which reads exactly like a win. `key_fallback_benchmarks`
now asserts which arm each leg takes before timing it, so the figure above
cannot quietly stop being about the boundary.

**Its width was load-bearing and too narrow.** Eleven things key off a hash of
an expression, and they do not agree about how much the hash has to mean:

- the evaluator's `seen` memo returns a cached fold on a **hash hit alone**;
- `InsertionSlot::BeforeDecl` splices a declaration's style metadata on a
  **hash hit alone**;
- the code-frame `span_cache` returns a cached span on a **hash hit alone** —
  twice over, once keyed by `compute_cache_key` and once by
  `compute_key_span_cache_key`;
- the JSX-spread replacement map, the queued-decl dedup, the callee index behind
  `is_member_callee`, and three of the state manager's six `CandidateIndex`es --
  the ones that pin a call to its declarator, to its style variable and to its
  top-level expression -- narrow a bucket by hash and then confirm with
  `eq_ignore_span`;
- `all_call_expressions` confirms on read too, but a collision can evict the
  wrong entry when a call is replaced.

So for four of the eleven the key _is_ the equality test. At 64 bits and ten
thousand distinct expressions in a file that is a collision every `1e-12` files,
and they fail with different volumes: a wrong folded value or a misplaced
injection is silent, while a wrong cached span is **directly visible in the
output** as a style annotated with another style's `file:line`. All four are now
**128 bits**, which puts them past `1e-31`.

The span cache arrived at this decision late — it was missed on the first count
of consumers and found in review, which took the count from five to seven; the
three that pin a call to what holds it came later still. It keys a _positional_
hash rather than the structural one, because it
caches "where was this written" and two identical expressions at different
positions must not share an entry. That is why it has its own key derivation and
did not come along for free.

What the key does _not_ have to be is any particular number: no consumer persists
it, none derives a class name from it, and output order comes from source order
rather than from hash order. That is what made the width a contained decision
rather than a rename of every class in every project.

## Considered options

**Two salted `DefaultHasher` states, for the width.** The obvious way to get 128
bits out of the standard library, and the version that was built and measured
first: one walk feeding two SipHash states with different prefixes, so the
expensive half — descending the tree — is not duplicated. Rejected on the number.
It cost **+49% on the key and +5.8% on a whole production transform** of the
400-`create` corpus file (26.0 ms to 27.5 ms, 25 runs each), and paying that
forever to remove a failure that arrives once per `1e4` years is the wrong trade.

**Confirm the hit with `eq_ignore_span` instead of widening.** What the
confirming consumers do. Rejected for the evaluator's memo: a confirm costs a
subtree compare on _every hit_ — the same order as the walk just paid — and
`seen` would have to hold a deep clone of every memoized expression to have
something to compare against. It remains the right answer for a consumer whose
hits are rare; `InsertionSlot::BeforeDecl` would qualify, and is covered by the
width instead.

**xxh3, taken 128 bits wide.** What shipped. A single pass emits 128 bits, and it
is enough faster than SipHash that the wider key is also the _cheaper_ one:
against the 64-bit SipHash it replaced, one key is 30-43% faster and a deep fold
30% faster, with the end-to-end transform of the corpus file unchanged (26.0 ms
to 25.9). It costs one direct dependency, `xxhash-rust` (BSL-1.0, already on the licence
allow-list). Nothing depended on the old values, and `DefaultHasher`
was never stable across Rust releases anyway, so nothing could have depended on
it and been correct.

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

Since built and measured, and still rejected — now on the merits.
[0006](./0006-an-incremental-memo-key-was-built-and-measured-slower.md) has the
numbers: the curve flattens as predicted (−81% on the fold at 240 levels) and the
per-node constant doubles, which costs +10% at 30 levels and 14–42% on every
fixture `evaluate_bench` measures. The scope discipline held; the shape of
StyleX's input is what does not suit it.

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

**The width is not pinned by a test, because a collision cannot be constructed
on demand.** What is pinned is that the key is 128 bits wide and that both arms
agree on it — the fallback cases in `stylex_utils`' `hash_test.rs` compare
`stable_hash_unspanned` against `stable_hash_wide`, so an arm that quietly
narrowed to 64 would fail to compile rather than fail to notice.

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
residue as the other unbounded stages around the fold — which has since been
attributed, in [0004](./0004-the-fold-owns-its-own-ceiling-and-its-own-stack.md),
to SWC's parser in release and to the code frame's printer in debug. The key is
not among them.
