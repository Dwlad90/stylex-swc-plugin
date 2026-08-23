# An incremental memo key was built, measured, and not kept

**Status:** rejected

[0005](./0005-the-memo-key-is-a-whole-subtree-hash.md) left one option open. Its
_Compose the key from its children's, on the way back up_ paragraph described a
sound mechanism, said the numbers were not in, and rejected it **for now**
rather than on the merits. This is the measurement that closes it. The key stays
exactly what 0005 decided it is; read that first, because everything here is
stated against it.

## What was built

Both halves 0005 named, complete and green across the whole workspace — 3 969
tests, including the 989-case transform suite, which is the statement that the
composed key produces byte-identical output.

**The walk composes.** `stylex-utils`' `hash.rs` stopped writing a subtree into
one `Hasher`. Each expression node opened a hasher of its own, wrote its variant
discriminant and its own scalar fields, wrote each child's _digest_ — a fixed 16
bytes — and finished. A node's cost became independent of what its children
hold, which is what makes a child's digest reusable.

**The cache is keyed by address and scoped to a level.** `evaluate_cached`
recorded the cache's length on the way in and truncated back to it on the way
out. The invariant that makes an address a usable identity, which is the part
0005 could not have without building it: _every entry was recorded inside a
level still on the stack, for a node reachable from the `path` that level was
handed, and `path` outlives that level's call._ So no entry names a freed node,
and no live node's entry is dropped while a level that could reuse it is still
running. It held. The scope discipline is not why this was rejected.

Two details that only appear once it is built. A cloned `StateManager` has to
start with an empty cache, because the copy's entries name nodes the original
holder's stack keeps alive. And a panic unwinding out of a fold skips every
truncation below it, so the outermost level — the one place it is knowable that
no scope is open — has to clear rather than truncate.

## What it measured

Release, this machine, against the same tower of `(MY_CONST + 1)` 0005 used.
`EvaluateDepth` is the fold; `StructuralKeyDepth` is one key with no cache
beneath it.

| depth | fold, 0005 | fold, composed | one key, 0005 | one key, composed |
| ----- | ---------- | -------------- | ------------- | ----------------- |
| 30    | 18.1 µs    | **19.8 µs**    | 0.67 µs       | **4.36 µs**       |
| 60    | 61.6 µs    | 39.3 µs        | 1.74 µs       | 8.18 µs           |
| 120   | 217.9 µs   | 78.6 µs        | 3.80 µs       | 17.2 µs           |
| 240   | 821.5 µs   | 159.1 µs       | 6.49 µs       | 37.0 µs           |

**The curve flattened exactly as predicted, and the constant went the other way
by more.** The fold is −81% at 240 and −36% at 60, and **+10% at 30** — the last
depth before the crossover, and the only column under the shipped ceiling of 32.
One key, uncached, costs **5.7×** more, because the walk went from one hasher
per key to one per node.

That per-node cost is close to its floor rather than an implementation defect.
The first version opened an `Xxh3Default` per node — a streaming state with 64
bytes of accumulators and a 256-byte staging buffer, zeroed on construction —
and cost +40% on the fold at depth 30. Replacing it with a stack-buffered
one-shot `xxh3_128` recovered most of that (22.8 µs → 19.8), and shrinking the
buffer to 64 bytes and inlining the three hot methods recovered the rest of what
was available (40.9 µs → 37.0 on the 240 key). A 128-bit finalization per node
is ~13 ns measured in isolation, against ~13.5 ns for a whole node of the
streamed walk. **Doubling the uncached per-node cost is what composition is, and
no implementation gets under it.**

## Why that settles it

The saving is proportional to the depth a fold descends past a node it has
already hashed. StyleX's input is **wide and shallow**: an object literal of
style keys, each holding a short value. Most nodes are visited once whatever the
key does, so most of the tree pays the doubled constant and collects nothing
back. The deep spine the cache is built for is the shape the ceiling forbids —
`maxEvaluationDepth` defaults to 32, and the crossover is above it.

`benches/evaluate_bench.rs`, over the six perf fixtures and the two transform
fixtures, in µs.

Two caveats found afterwards, in review, and recorded here rather than in the
commit that found them. Neither changes the decision — every leg regressed, and
both harnesses paid the same overheads on both sides — but the absolute numbers
below are not what a reader would assume.

- The loop built its `StateManager` and walked the module twice **inside**
  `b.iter`, so every figure includes setup the fold does not pay. This is the
  same defect `module_path_bench` was fixed for, where removing it moved a
  published figure from +29 ns to +9.1 ns. The bench now batches that setup out.
- Three legs fold **nothing** confidently — `create-complex.js` and both
  `dynamic-param-shadows-import` fixtures — because they import a theme, and
  resolving one needs a filename a bench cannot supply (`set_plugin_pass` is
  `pub(crate)`). Those three rows price the refusal path, not a fold. They still
  exercise the reference chain the shadowing fixtures were added for, since it
  runs per reference, but they are not measuring what the column header says.

The counts are now pinned in `EXPECTED_CONFIDENT_FOLDS`, so a leg that stops
folding what it used to fails the bench rather than reporting a win.

| fixture                                | 0005  | composed | change   |
| -------------------------------------- | ----- | -------- | -------- |
| `create-complex.js` †                  | 41.61 | 57.30    | **+38%** |
| `dynamic-param-shadows-import-edges` † | 13.98 | 19.88    | **+42%** |
| `dynamic-param-shadows-import` †       | 3.00  | 3.88     | +29%     |
| `create-basic.js`                      | 10.91 | 13.56    | +24%     |
| `sizes.stylex.js`                      | 19.82 | 23.37    | +18%     |
| `colors.stylex.js`                     | 35.13 | 40.50    | +15%     |
| `createTheme-basic.js`                 | 30.31 | 34.43    | +14%     |
| `createTheme-complex.js`               | 3 244 | 3 318    | +2%      |

† Folds nothing confidently; prices the refusal path. See the caveats above.

Every fixture regressed. Nothing the repo measures on real input got faster, and
the two that regressed worst are the ones this effort's own defect was filed
against. `bench:revisions` and `bench:verdict` were not run: the gate they apply
is a 10% warn and a 20% fail, and six of eight fixtures are already past the
fail threshold on a cheaper harness.

The quadratic 0005 measured is real and this removes it. It is also, on the
evidence, **not a term any project pays** — the ceiling bounds it at 32 levels,
and buying its removal costs 14–42% on every input that exists.

## Considered options

**Land it behind the ceiling** — compose only when a project raises
`maxEvaluationDepth` past the crossover, and stream otherwise. It works, and it
is the only version that keeps the win without the cost. It buys two
monomorphizations of a forty-arm walk, a mode flag, and two possible key values
in one process, to serve a configuration no project has been shown to set. Left
unbuilt on that ratio rather than on a measurement; if a project ever reports a
raised ceiling and a slow build, this is the change to make and this ADR is the
head start.

**A cheaper mixing function per node** — a couple of multiply-xor rounds instead
of a 128-bit hash. Would close most of the constant. Rejected on 0005's own
grounds: four consumers act on a hash hit _alone_, so a weaker mix is a wrong
fold rather than a slower one, and a wrong cached span is directly visible in
the output as a style annotated with another style's `file:line`.

**Keep the composed key only for the evaluator, and stream for everyone else** —
`stable_hash_unspanned`'s other consumers never descend, so they gain nothing
from composition and pay all of it. Two walks over one source is achievable by
parameterising the arms. It leaves two different structural keys for the same
expression alive in one process, distinguishable only by which function was
called, and it does not fix the fold's own +10% at depth 30 — which is the
number that matters, because that is where the fold runs.

## Consequences

**`key_cost_scaling_tests` stays as it is.** It counts the bytes the walk feeds
one hasher, and it still describes the shipped key: linear per key, quadratic
per fold. Under a composed walk it would have had to be restated in nodes,
because there is a hasher per node and no single byte stream to count. The
restatement is in the patch below if it is ever needed.

**The work is described, and the base has to be rebuilt.** The composed walk,
the scoped cache, the `StateManager` field, the `evaluate_cached` bracket and the
tests that pin the scope discipline are all described above and in the rejected
options below. An earlier revision of this file pointed at a patch under
`.scratch/`, which is a symlink outside the repository and never committed — so
it was never reachable for the reader this paragraph is addressed to. The next
person to ask this question should expect to rebuild the base before measuring a
variant.

**0005's `for now` is spent.** Its rejected-options section now points here.
Reopening this needs a new reason — a project on a raised ceiling, or an input
shape that is deep rather than wide — not a fresh reading of the same quadratic.
