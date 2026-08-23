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

Four groups, two module sizes for the walk, on an Apple-silicon laptop, full
criterion runs (not `--quick`):

| leg                                 | `c83ac5cbd` |    branch |  delta |
| ----------------------------------- | ----------- | --------- | ------ |
| `ModuleWalk/calls/1x`               |   142.24 µs | 113.72 µs | −20.1% |
| `ModuleWalk/calls/4x`               |   561.08 µs | 453.80 µs | −19.1% |
| `ModuleWalk/no-calls/1x`            |    20.72 µs |  20.68 µs |  −0.2% |
| `ModuleWalk/no-calls/4x`            |    78.45 µs |  78.94 µs |  +0.6% |
| `ModuleWalk/imported/1x`            |   209.06 µs | 180.73 µs | −13.6% |
| `ModuleWalk/imported/4x`            |   789.67 µs | 676.93 µs | −14.3% |
| `SeenModuleSource` clone, no-calls/1x |   39.93 µs |  38.57 µs |  −3.4% |
| `SeenModuleSource` clone, no-calls/4x |  154.92 µs | 153.68 µs |  −0.8% |
| `SeenModuleSource` clone, calls/4x  |   208.33 µs | 204.22 µs |  −2.0% |
| `StructuralKey/call/shallow`        |    92.77 ns |  55.70 ns | −40.0% |
| `StructuralKey/call/member`         |    99.90 ns |  61.79 ns | −38.1% |
| `StructuralKey/call/nested`         |   277.68 ns | 121.89 ns | −56.1% |
| `StateManager/new`                  |   121.69 ns | 151.18 ns | +24.2% |
| `FullPipeline/no-calls/1x`          |   154.16 µs | 140.94 µs |  −8.6% |
| `FullPipeline/calls/1x`             |   295.87 µs | 258.24 µs | −12.7% |
| `FullPipeline/imported/1x`          |   367.94 µs | 329.73 µs | −10.4% |

The `SeenModuleSource` rows are the *difference* between that group's two legs
(`use_real_file_for_source` off against on), which is what the module clone
costs. Identical on both revisions to within a couple of points, so the `Rc` and
the `OnceCell` this branch wrapped it in cost nothing.

Taking them in the order the ticket asked for them:

1. **The `Discover` walk.** Faster, by a lot, and the reason is the third group.
   `add_call_expression` structurally hashes every call expression in every
   module, and the structural key is 38-56% cheaper on this branch because
   `DefaultHasher` became xxh3. A module with no calls at all is unchanged, which
   is what says the walk itself did not move -- only the per-call work inside it.
   The spec's *Ruled out* table has the hasher change in it for the reason "xxh3
   is the faster hasher"; that is now measured rather than reasoned, and the
   direction is right.
2. **`set_seen_module_source_code`.** No difference. Excluded, as the ticket said
   a null result here would exclude it.
3. **`StateManager::new`.** 29 ns dearer per transform. Real, and far too small:
   0.00004% of the 72 µs fixture, less of every other one.

**A fourth leg was added, and it is the one that matters.** `FullPipeline` runs
the whole chain `stylex_rs_compiler::transform` builds -- resolver, type
stripping, the StyleX pass, hygiene, the fixer, the printer -- because if none
of the three carries the cost the next place to look is the passes on either
side. It is faster too, on every shape.

So the cost is not in the crate. It is only observable through the built
`.node`, which is where it was found in the first place -- see the comments on
`02` for the loop that reproduces it there and what it does and does not
localize.
