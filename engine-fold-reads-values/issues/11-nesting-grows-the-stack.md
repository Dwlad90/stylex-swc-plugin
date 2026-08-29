# 11 — Nesting grows the stack; the ceiling is the configured depth

**What to build:** A deeply nested expression folds up to the depth the
project configured, rather than a second, lower limit nobody set.

The fold carries its own nesting ceiling because the evaluation runs on the
bare thread stack and the engine's parser recurses — measured, it overflows
around a hundred levels and aborts the process from inside an evaluation whose
whole contract is that it may fail. Refusing earlier is what turns that crash
back into a diagnostic.

But the evaluator's own descent already solved this: it runs inside a growable
stack, and that pairing — grow the stack, and state the ceiling as a
configured number rather than a measured cliff — is a recorded architectural
decision. The fold copied that decision's *number* without its *mechanism*,
and its own documentation says the two are not the same setting for exactly
that reason. Growing the stack is what makes that sentence obsolete.

Verify empirically at the depths the existing tests already probe before
relying on it. A ceiling that is wrong here is a process abort, not a failed
assertion.

**Blocked by:** 05.

**Status:** resolved

- [x] The evaluation runs inside the same growable stack the evaluator's own
      descent uses
- [x] Nesting well past the old ceiling folds rather than refusing, verified
      at the depths the existing tests probe
- [x] The fold's separate nesting ceiling is deleted, and the limit is the
      project's configured evaluation depth
- [x] Raising that configured depth raises what the fold accepts, and a
      project that raised it does not lose the diagnostic
- [x] Past the ceiling the result is a refusal naming the depth, never an
      abort
- [x] The corpus entry that pinned the nesting divergence is re-measured

## Note from ticket 04

There are **two** engine nesting bounds now, not one, and both are the same
`Depth` budget spending `MAX_ENGINE_NESTING`.

The second is on the way *out*. A loop inside the engine can nest a value deeper
than any expression the guard admits on the way in —
`'x'.repeat(40).split('').reduce((a, c) => [a], [])` builds forty levels from a
two-level expression — and the conversion back recurses on the bare thread
stack, so it aborts the process where the input walk would have refused. It was
added with ticket 04 rather than deferred here because it closes a crash rather
than a gap.

**The inward bound costs a fold, measured.** It refuses rather than handing the
expression back, so under a raised `maxEvaluationDepth` it declines input the
older path would have folded — `"a".concat(<40 levels of &&>)` at a ceiling of
512 is the measured case, pinned in `engine_fold_tests`. Handing it back instead
is not simply better: the older path *aborts* on a nested array reaching its
`join`, with `Array element must evaluate to a string for join()` from
`native_functions.rs`, which is a `stylex_panic!` rather than a deopt. That
panic is older than this effort and is only reachable with a raised ceiling.
**Fix it before making the inward bound a fall-through**, or unifying the
ceilings will trade a readable refusal for a process abort.

So this ticket's "the fold's separate ceiling is deleted" has two sites. The
inward one becomes the configured evaluation depth as written. The outward one
is a property of the conversion's own stack, not of the evaluator's descent, so
whether it follows the configured depth or keeps a number of its own is a
question this ticket has to answer rather than assume.

## Comments

### From 07

The statics moved to the engine, so `Math` and `Object` answer to the fold's
ceiling now instead of the evaluator's. Measured, a nested `Math.max(x + 1, 0)`
folded 158 levels deep before and folds 16 now — two fold levels per source
level, against `MAX_ENGINE_NESTING`'s 32 — and raising `maxEvaluationDepth` does
not move it, which is this ticket's whole subject. Upstream folds all of 32, 40,
158 and 300.

Pinned at the new boundary in
`transform_stylex_create_test::evaluation_depth_budget::a_builtin_call_folds_at_the_deepest_accepted_nesting`
and `::a_builtin_call_refuses_one_level_past_the_ceiling`, with the reason and a
pointer here written beside them. Both numbers move when this ticket lands.

## How it landed

`MAX_ENGINE_NESTING` is gone. `Depth` carries the ceiling it was opened with,
`try_fold` opens it at `StateManager::evaluation_ceiling()` -- the same accessor
the evaluator's own descent reads -- and the refusal names that number.

**The two ways of growing a stack.** The evaluator asks for room at every level,
so a small red zone is right for it: it only has to cover the levels between one
question and the next. The engine's parser asks *never* -- it descends through
the printed source on whatever stack it was handed -- so the fold claims room
for its whole descent before it starts. `growable_stack` names both:
`grown_per_level` and `grown_for_depth`.

**The outward bound follows the configured depth**, which was this ticket's open
question. It can now, because `grown_for_depth` claims the stack for the ceiling
rather than hoping the thread had one: the conversion out recurses on a stack
sized for exactly the levels the ceiling admits. There was no need to make the
inward bound a fall-through, so the `native_functions.rs` panic ticket 04 warned
about is not on this path and stays where it is.

**Two walks nest, so the claim is for two.** The guard walks the expression, and
where it reaches a name it walks the *value* that name resolved to -- a walk of
its own, against the whole ceiling, whose frames sit on top of the guard's. So
`grown_for_depth` claims `ceiling * 2 * BYTES_PER_LEVEL`, and the room is there
by construction rather than by the margin on the per-level number. Pinned by
`evaluation_depth_budget::a_deep_expression_reading_a_deep_value_folds_rather_than_aborting`,
which is 90 levels of each and agrees with upstream on `.xbqvfa8`.

**Measured, on a debug build.** The guard's walk and the conversion back cost
about 4 KiB a level; the engine's parse of the printed source costs most of the
rest -- 800 levels of nested array literal fold inside 16 MiB and do not fold
inside 8. `BYTES_PER_LEVEL` is 64 KiB, roughly three times that.

**`MAX_EVALUATION_DEPTH_LIMIT` dropped from `1 << 20` to 8192.** A configured
depth is only honest if a stack can be claimed for it, and a ceiling of a
million would have asked for 128 GiB. 8192 levels of two nested walks at 64 KiB
is a gigabyte, which is the largest claim this is willing to make; a
`const _: () = assert!(...)` in `growable_stack` pins the arithmetic, because
the numbers live in two crates. Documented in the compiler README, because the
clamp is silent.

**Boundaries that moved**, all re-measured against `@stylexjs/babel-plugin`
0.19.0 and agreeing with it on class name and rule text on the folding side:

| shape | before | after (ceiling 320) | upstream |
| --- | --- | --- | --- |
| `Math.max(x + 1, 0)` nested | folded 16, refused 17 | folds 160, refuses 161 | folds both, `.xj7c5yd` at 160 |
| `Array((x + 1)...).length` | folded 15, refused 16 | folds 159, refuses 160 | folds both, `.xf8gxui` at 159 |
| `[...["a"]...].join("")` | folded 30, refused 31 at any ceiling | folds `ceiling - 2` | folds all |

`modules-06-deep-nesting` stays acceptance-divergent: a hundred levels is still
past the shipped default of 32. What changed is why, and the row now says so --
raise `maxEvaluationDepth` past the nesting and it folds to the `.x16319ns`
upstream reaches.

**What it costs, measured.** `cargo bench --bench engine_fold_bench`, paired
against this branch's parent on one machine: +4.0/+5.5/+7.2%, +1.4/+3.0/+4.6%,
+5.0/+6.0/+7.1%, +2.3/+4.7/+7.9% and +1.1/+2.5/+4.2% on five of the nine
fixtures, the rest inside noise. The cost is the mapping, not the check --
re-running with a claim too small to ever grow puts every fixture back inside
noise (mixed +/-3%), which is what says so.

It is the fold mapping a segment and unmapping it again on a thread that did not
have the room left, so it is paid per fold rather than per file. Paid rather
than avoided: it buys the whole ticket -- crossing the ceiling is a diagnostic
rather than an abort -- and both ways of paying it less often (claim once per
top-level evaluation, or grow the evaluator's own segment sooner) move the cost
onto files that fold nothing at all. Under the 10% advisory threshold in
`guidelines/PERFORMANCE.md`, and well under the 20% that blocks; the release
comparison in CI is the gate that decides.

## Still open, and not this ticket's

**Source nested past roughly a thousand levels aborts before it is evaluated.**
Measured: `[...["a"]...].join("")` at 1198 levels overflows at the *default*
ceiling of 32, so nothing the fold does reaches it. The parser recurses without
a budget of its own and no option here is consulted on the way in, which is why
the cap above is described as bounding what the fold will be asked for rather
than what a build survives. It predates this effort and is unchanged by it; a
depth in the thousands is only reached in practice by a value the engine
*builds* in a loop, which is the direction the claim does serve. Written into
`MAX_EVALUATION_DEPTH_LIMIT`'s own docs and into the README so nobody reads the
cap as a promise it does not make.

**A deep folded value is dropped outside the claimed stack.** The conversion
out builds inside the segment and hands the value back; whoever drops it
recurses on an ordinary stack. The same is true of any deep value the evaluator
answers without the fold, so it is a property of the ceiling rather than of this
change.
