# 01 — Localize the production cost with a micro-bench

Status: `needs-triage`
Blocked by: None — can start immediately

**What to build:** A criterion bench in `crates/stylex-transform/benches` that
times the candidates from the spec separately, run on this branch and on
`c83ac5cbd`, so the +2.4% is attributed to one of them rather than to "the
module-level path".

The spec's shape reading — proportional to module size, present with no StyleX
import — puts the walk first and construction last:

1. **The `Discover` walk** over a module with no StyleX import: construct the
   transform, run `discover_module`, stop. Run it over two module sizes, because
   the whole question is whether the cost scales with node count. If it does,
   the next cut is per-node: time `add_call_expression` (it structurally hashes
   every call), `find_top_level_expr` and `is_member_call_callee` against their
   `develop` versions.
2. **`set_seen_module_source_code`** over a parsed module. It does the same
   `module.clone()` on both revisions, so a difference here means the `Rc` and
   the `OnceCell` around it cost something; no difference excludes it.
3. **`StateManager::new(StyleXOptions::default())`** construction and drop.
   Fixed rather than proportional, so it cannot explain the shape — measured
   anyway because it is one more line in the same bench.

**Why a bench and not a profile.** A sampling profiler was tried: `sample` at
1 ms over a 14-second loop gave 7 215 main-thread samples, fat LTO inlined the
module-level path into a handful of frames, and a quarter of the samples were
`malloc` on both revisions. Nothing localized. A bench that calls one function
does not have that problem.

**Running it on both revisions.** `cargo bench -p stylex_transform` needs the
bench file to exist in the tree being measured, and it does not exist on
`c83ac5cbd`. Cherry-pick the bench commit onto a detached `c83ac5cbd`, or keep
the bench in a patch applied to both. Say which was done in the comments,
because a bench compiled against two different crate versions is only comparable
if it is the same bench.

**Acceptance:** a number per candidate per revision, at two module sizes for the
walk, and a statement of which one carries the difference. "None of the three" is
a real answer — it would mean the cost is in parsing, printing, or the napi
boundary, and the next step is a bisect of the 148 commits driven by the paired
harness. If it comes to that, bisect on a *group* mean rather than one fixture:
individual rows move by several points between runs (see the spec), and the
first attempt at this bisect produced a false answer for exactly that reason
plus a broken predicate.

**Do not** re-test anything in the spec's *Ruled out* table without new evidence.

## Comments

**Answered: none of the three.** All three candidates are at parity or faster
on this branch, and the pass chain around them is faster too.

The bench is `crates/stylex-transform/benches/module_path_bench.rs`, committed
on this branch. It was **copied** into a detached `c83ac5cbd` worktree rather
than cherry-picked -- the same file, byte for byte, compiled by both trees --
because it only touches API that is public on both: `StyleXTransform::test`,
`StyleXOptionsParams`, `StateManager::new` and
`stylex_utils::hash::stable_hash_unspanned_call`. Nothing in either tree was
changed to make it compile.

Five groups, two module sizes for the walk, on an Apple-silicon laptop, full
criterion runs (not `--quick`):

| leg                               | `c83ac5cbd` |    branch |  delta |
| --------------------------------- | ----------- | --------- | ------ |
| `ModuleWalk/calls/1x`             |   138.63 µs | 112.00 µs | −19.2% |
| `ModuleWalk/calls/4x`             |   549.53 µs | 445.18 µs | −19.0% |
| `ModuleWalk/no-calls/1x`          |    20.68 µs |  20.97 µs |  +1.4% |
| `ModuleWalk/no-calls/4x`          |    78.70 µs |  79.56 µs |  +1.1% |
| `ModuleWalk/imported/1x`          |   208.39 µs | 181.58 µs | −12.9% |
| `ModuleWalk/imported/4x`          |   776.75 µs | 672.27 µs | −13.5% |
| `SeenModuleSource/kept/calls/1x`  |   189.63 µs | 162.72 µs | −14.2% |
| `SeenModuleSource/kept/calls/4x`  |   755.04 µs | 652.82 µs | −13.5% |
| `SeenModuleSource/kept/no-calls/1x` |  59.90 µs |  58.65 µs |  −2.1% |
| `SeenModuleSource/kept/no-calls/4x` | 234.78 µs | 235.03 µs |  +0.1% |
| `StructuralKey/call/shallow`      |   100.32 ns |  57.96 ns | −42.2% |
| `StructuralKey/call/member`       |   101.26 ns |  63.75 ns | −37.0% |
| `StructuralKey/call/nested`       |   266.17 ns | 122.77 ns | −53.9% |
| `StateManager/new`                |    70.35 ns |  79.48 ns | +13.0% |
| `FullPipeline/no-calls/1x`        |   141.34 µs | 138.89 µs |  −1.7% |
| `FullPipeline/no-calls/4x`        |   545.39 µs | 543.81 µs |  −0.3% |
| `FullPipeline/calls/1x`           |   281.12 µs | 255.04 µs |  −9.3% |
| `FullPipeline/calls/4x`           |    1.110 ms |  1.006 ms |  −9.3% |
| `FullPipeline/imported/1x`        |   349.26 µs | 325.82 µs |  −6.7% |
| `FullPipeline/imported/4x`        |    1.335 ms |  1.235 ms |  −7.5% |

The `SeenModuleSource/kept` rows are the same transform as the matching
`ModuleWalk` row with the clone left in, so the clone costs the difference
between them: 39.2 µs against 37.7 µs on `no-calls/1x`, 156.1 µs against
155.5 µs on `no-calls/4x`, 205.5 µs against 207.6 µs on `calls/4x`. Identical
on both revisions to within a couple of points, so the `Rc` and the `OnceCell`
this branch wrapped it in cost nothing.

**What this table can resolve.** Criterion's interval inside one run is a couple
of tenths of a percent; two *builds* of the same source do not agree that
closely. An earlier pair of full runs read `no-calls/1x` at −0.2% and
`no-calls/4x` at +0.6% where this pair reads +1.4% and +1.1%, and
`FullPipeline/no-calls/1x` at −8.6% where this pair reads −1.7%. Anything inside
about a point and a half either way is **unresolved here**, not measured at
parity. What survives that is the call-heavy walk, the structural key, and the
pass chain over anything holding calls -- all of them faster on the branch by an
order of magnitude more than the noise.

**Two numbers in an earlier version of this comment were wrong and are
withdrawn.** `StateManager::new` was published at +29 ns / +24.2%; the bench was
constructing `StyleXOptions::default()` inside the timed closure, which
allocates and, on this branch only, reads the evaluation-depth environment. With
the options hoisted into batched setup it is +9.1 ns / +13.0%. And
`FullPipeline` was handing the printer a fresh empty `SourceMap` with no
comments, so it was timing a printer with no source file to resolve against; it
now gets the fixture's own. Both were caught in review, and both corrections are
in the committed bench.

Taking them in the order the ticket asked for them:

1. **The `Discover` walk.** Over a module holding calls, faster by a lot, and
   the reason is the third group.
   `add_call_expression` structurally hashes every call expression in every
   module, and the structural key is 37-54% cheaper on this branch because
   `DefaultHasher` became xxh3. A module with no calls at all reads inside the
   file's own resolution, which is what says the walk itself did not move --
   only the per-call work inside it.
   The spec's *Ruled out* table has the hasher change in it for the reason "xxh3
   is the faster hasher"; that is now measured rather than reasoned, and the
   direction is right.
2. **`set_seen_module_source_code`.** No difference. Excluded, as the ticket said
   a null result here would exclude it.
3. **`StateManager::new`.** 9.1 ns dearer per transform. Real, and far too
   small: about a hundred-thousandth of the 72 µs fixture, less of every other
   one.

**A fourth leg was added, and it is the one that matters.** `FullPipeline` runs
the whole chain `stylex_rs_compiler::transform` builds -- resolver, type
stripping, the StyleX pass, hygiene, the fixer, the printer -- because if none
of the three carries the cost the next place to look is the passes on either
side. It is faster too, on every shape.

So the cost is not in the crate. It is only observable through the built
`.node`, which is where it was found in the first place -- see the comments on
`02` for the loop that reproduces it there and what it does and does not
localize.
