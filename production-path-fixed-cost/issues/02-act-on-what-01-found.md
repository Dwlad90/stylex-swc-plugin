# 02 — Act on what 01 found

Status: `ready-for-human`
Blocked by: 01

**What to build:** Whatever `01` points at. The shape of the fix depends on the
answer, so this ticket records the three shapes rather than pretending to know.

**If it is the `Discover` walk** — the likeliest, given the cost is proportional
to module size and present with no StyleX import. Find the per-node method that
grew and make the common case cheap: a node that is not a StyleX call.
`add_call_expression` structurally hashes every call expression it sees, so a
cheap pre-filter on the callee shape before hashing would pay for itself if that
is the one.

**If it is `StateManager::new`.** The branch turned one binding-write set into
four collections and added the span cache, the framed-declaration map and the
depth counters. Empty `FxHashSet`s do not allocate, so the cost would be the
struct's size — construction, zeroing and moves. The fix is to group the
evaluator's per-module state behind one field constructed lazily, so a module
that never evaluates anything never builds it. Keep the accessors on
`StateManager`; only the storage moves.

**If it is `set_seen_module_source_code`.** The module is deep-cloned once per
transform on *both* revisions, so only a change in what is stored can matter.
`Rc` the module rather than cloning it into the state, if the clone is still
there.

**Acceptance in every case:** the paired harness over the `perf` and `transform`
categories shows the production-shape geometric **mean** within 1% of `develop`
— the mean, not a chosen fixture, because single rows move by several points
between runs —
**and** the development-shape fixtures keep their current advantage — 13-26% and
the 84% on `Debug data - lotsOfStyles.js`. A fix that gives back the debug-path
win is not a fix; that win is what the branch is for.

Re-run `pnpm run --filter=@stylexswc/rs-compiler parity` as well. A change to
per-module state can move what folds, and 0 changed verdicts over 1026 subjects
is the bar.

## Comments

**Nothing to act on in the shape this ticket imagined, and the reason is
measured.** `01` timed all three candidates on both revisions and every one of
them is at parity or faster on this branch -- the walk by 19-20%, the structural
key by 38-56%, the memoized-source clone by nothing at all, with
`StateManager::new` 29 ns dearer once per transform. The pass chain around the
transform is faster too. So the three fixes this ticket describes have no
target: there is no per-node method that grew, `StateManager` construction is
four hundredths of a percent of the smallest fixture, and the clone is unchanged.

What follows is what the chase found instead, so the next reader starts from the
evidence rather than from the three shapes above.

### The loop that reproduces it

`.node` level, not crate level, because that is the only place the cost is
visible.

- Two package directories, each a `dist/` holding `index.js`, `transform.js` and
  one `.node`. The JavaScript is byte-identical across the two revisions -- the
  only change to it on this branch is a comment -- so a `develop` package is a
  `develop` `.node` dropped into this branch's `dist/`.
- Four modules that import no StyleX at all: a two-line one and a
  25-component one, each as JavaScript and as TypeScript. The TypeScript pair
  matters because `verbatim_module_syntax` is false for a `.ts` input, which
  makes this branch's type-stripping configuration identical to `develop`'s
  `strip` -- so a regression there cannot be the pipeline change.
- 4000 transforms per sample, subjects alternating round by round inside one
  process, and the **process** repeated seven times with the median taken across
  processes. Repeating inside one process reproduces its own answer to a tenth of
  a point; a second process moves it by a point, so the process is the unit.

Same binary against itself reads −0.43% to +0.05%. This branch against
`c83ac5cbd` reads:

| module                | median |
| --------------------- | ------ |
| two-line `.js`        | +1.74% |
| two-line `.ts`        | +1.53% |
| 25-component `.jsx`   | +1.95% |
| 25-component `.tsx`   | +1.77% |

Twenty-eight process medians, every one positive. That is the spec's "a module
with no StyleX import at all regresses", pinned.

### What it excludes

The same four shapes, timed **inside the crate** through the whole pass chain,
are faster on this branch, not slower. A module with no calls and no StyleX
import is 20.68 µs against 20.72 µs through the StyleX pass, and 140.94 µs
against 154.16 µs through resolver, type stripping, StyleX, hygiene, fixer and
printer.

So: the same source, the same passes, in the same order, is at parity or faster
when linked into a bench and 1.5-2.0% slower when linked into the `.node`. What
differs between those two is the napi boundary and the whole-binary layout that
a fat-LTO, one-codegen-unit build produces, and neither is a call site anything
can be moved out of.

### It is a ramp, not a commit

Five points across the 148 commits, each built as a `.node` and measured against
`c83ac5cbd` with the loop above:

| point   | commit      | `.node` bytes | two-line `.js` | 25-component `.jsx` |
| ------- | ----------- | ------------- | -------------- | ------------------- |
| 0/147   | `c83ac5cbd` |     9 744 224 |         +0.02% |              −0.35% |
| 37/147  | `260a5db54` |     9 727 696 |         +0.18% |              +1.28% |
| 74/147  | `9474dac83` |     9 744 240 |         +3.58% |              +2.16% |
| 111/147 | `fa04d1964` |     9 826 992 |         +3.03% |              +1.46% |
| 147/147 | branch tip  |     9 777 312 |         +1.74% |              +1.95% |

No step. The cost appears somewhere in the first half, sits between one and
three points from there on, and does not track binary size -- the 37/147 build
is *smaller* than `develop` and already pays it. A bisect over these commits
would name whichever one it happened to land on; the ticket's own warning about
a false answer from a noisy predicate applies with more force now that the
predicate is known to be a ramp.

### The two outliers are separate, and are `03`

`03` re-measured them and they hold at +9.3% and +9.4% while
`counter-with-dynamic-styles` holds at −3.9%. Counters on the evaluator say the
two that cost more fold **exactly the same nodes** as `develop` does, with the
same hits, misses and refusals. So roughly seven points on those two shapes is a
second cost, in what the `create` transformation does around the fold rather
than in the fold, and it is not this ticket.

### Acceptance, honestly

Not met, and not meetable by this ticket as written. The production-shape
geometric mean is not within 1% of `develop`, and no change is proposed here,
because localizing produced an envelope rather than a call site. The
`develop`-side wins the branch exists for are intact and were re-measured with
everything else -- see the spec.

The decision the spec names -- accept the trade and record it, or keep chasing
into the napi boundary and the binary layout -- is still the maintainer's.
