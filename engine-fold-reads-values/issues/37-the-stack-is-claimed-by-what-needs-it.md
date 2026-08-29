# 37 — The stack is claimed by what needs it

**What to build:** The four-megabyte stack claim is made by the part of the fold
that recurses on it, so a call expression the guard declines pays nothing for a
stack the engine never entered.

**Where it is made now.** `grown_for_depth` wraps the whole of `fold`, including
`admit_call` — and `try_fold` runs for every `CallExpr` the evaluator visits. The
claim is `ceiling × 2 × 64 KiB`; at the default that is about four megabytes. On
a thread with less than that left, `stacker` mmaps and unmaps a segment per call
expression, folded or not. The four-to-six per cent recorded in the doc comment
is measured against folds, where the fold cost is real; on the no-fold path the
fold cost is approximately zero, so the ratio is not four per cent of anything.

The reason for an up-front claim is sound and stays: the engine's parser descends
through a nested literal without ever asking for room, so it cannot run on
`maybe_grow`. That argument covers the parse and the evaluation. It does not
cover the guard walk, which is this compiler's own recursion and already has a
growth mechanism.

**Two records go stale with it.** ADR 0004 still asserts that the descent runs
inside `stacker::maybe_grow`, with no mention of the up-front claim, the 8192
clamp or the deleted nesting bound — it is untouched by this effort and now says
something false. ADR 0001 says `superseded` and names no successor. Both are
fixed here rather than batched, because the claim's placement is the thing they
describe.

**And the claim is untested at the size it is sized for.** `growable_stack.rs`
has no tests; the deepest depth any test configures is 320, against a limit of
8192. Under-sizing does not fail as a diagnostic — it fails as a SIGSEGV, from
inside an evaluation whose whole contract is that it may fail.

**Blocked by:** 36.

**Status:** resolved

- [x] The claim is made around the parse and the evaluation; the guard walk runs
      on the per-level growth
- [ ] The no-fold fixture from 36 shows the improvement, measured — **it does
      not, and the measurement is why: see below**
- [x] A module nesting a fold near the configured depth, at
      `maxEvaluationDepth: 8192`, either folds or refuses with the nesting
      sentence — never aborts *(met at the evaluator seam; see below for why no
      module can be written that reaches that depth)*
- [x] ADR 0004 gains a revision section describing the claim, the clamp and the
      deleted ceiling; ADR 0001 names its successor; ADR 0008 links both

## Answer

**The claim now wraps the printing and the engine's own work, and nothing else.**
`try_fold` calls `fold` directly; `grown_for_depth` is made after `admit_call`
has returned, around `print_fold` and the engine block. Everything above that
line asks again at the next level, under one rule stated in `growable_stack`: *a
descent that can ask again at the next level does*. Six sites spend a level and
each now claims room for the one after it — the guard's value, statement and
pattern walks; the transport's value and expression walks; and the outward
conversion's object walk.

**The two that cannot ask are the two this compiler does not write.** The first
draft claimed around `apply` alone, which was wrong and the spec review caught
it: `print_fold` clones the expression and runs SWC's code generator over it, and
both recurse over the same nesting the engine's parser then reads back. It had
been inside the old claim and would have fallen out of every mechanism. The claim
now starts before the print. They do not overlap — the print has unwound before
the parse begins — so what is sized is the deeper of the two rather than the sum.

**The measurement, `x86_64-apple-darwin` (M1 Max, Node 24.11.0), 12 rounds, seed
1, branch-before against branch-after, twice:**

| fixture | run 1 | run 2 |
| --- | --- | --- |
| Feature - calls that do not fold | 1.000 (0.996, 1.006) | 1.000 (0.996, 1.005) |
| Feature - calls that do not fold (dev) | 1.002 (0.994, 1.003) | 1.007 (1.001, 1.009) |
| Feature - engine fold | 1.004 (1.000, 1.007) | — |
| Feature - engine fold (dev) | 1.013 (1.008, 1.014) | — |

**The improvement does not appear, and the reason is the condition the ticket
put on it.** The ticket says "on a thread with less than that left, `stacker`
mmaps and unmaps a segment per call expression". Where the room *is* there,
`maybe_grow` is a stack-pointer comparison and nothing else — no `mmap`, no
`munmap` — and the benchmark's thread has four megabytes under it, so the old
placement was already paying almost nothing on this corpus. Moving the claim
removes 111 comparisons and shows as 1.000.

So the ticket's argument stands and its number does not: a declined call no
longer claims a stack the engine never entered, and on a thread that has the
stack anyway there was nothing to save. The case it does save is the one no
fixture here reproduces — a worker deep in a transform with under four megabytes
left — and building a fixture that reproduces it means controlling the host's
thread stack, which the benchmark harness does not reach. **Recorded as a gap
rather than ticked.** An early draft of this ticket, claiming around `apply`
alone, reported 0.993 on the same fixture; the two runs above put that inside
the noise rather than in the change.

**Folds pay about a per cent for it in a development build**, 1.013 with the
interval clear of 1.0 — the six per-level checks the walks now make, on a fixture
that is all folds. Under the 1.10 warn threshold, and the production leg is 1.004.

**The sizing was measured rather than kept.** With only the engine on the claim,
a debug level of nested array literal costs about twenty kilobytes — sixty-four
megabytes carry 3300 levels and not 3500, measured by claiming nothing and
shrinking the thread instead. `BYTES_PER_LEVEL` stays at 64 KiB, which is three
times that.

**The factor of two stays, under a different name and a true reason.** It was
`DEEPEST_NESTED_WALKS` — two guard walks nested — and the guard is no longer on
the claim, so the reason was gone. Deleting the factor turned
`a_dead_operand_deeper_than_the_ceiling_is_never_entered` into a `SIGBUS`, which
is how the real reason was found: **the ceiling bounds the walk, not the text.**
A short-circuited operand is printed without being walked, so the parser descends
nesting no level was spent on. The constant is now `UNWALKED_NESTING`, and it
says in its own doc that it is a margin and not a bound. What it is not a bound on
is filed as [47](./47-a-dead-operand-is-printed-deeper-than-it-is-walked.md): at
the shipped ceiling that shape folds at 200 levels of dead nesting and aborts at
300, on this branch and on the merge base alike.

**What the tests reach that no source case could.**
`growable_stack_tests.rs` writes its recursion rather than parsing it — a frame of
a known size, a known number of times, on a thread of a known size — because the
claim is only interesting at numbers no expression can be written to. Five cases:
a walk carried eight megabytes past a one-megabyte thread, a claimed descent that
never asks again, the largest claim a project can configure asserted to be
underfoot, a covered claim asserted to allocate nothing, and a panic crossing both
kinds with its payload intact.

`a_fold_nested_near_the_largest_ceiling_answers_rather_than_aborting` folds at
8190 levels and refuses at 8300, under `maxEvaluationDepth: 8192`, and walks a
third case to the bottom before declining it. It runs on a thread of its own, and
that is not the fold's business: SWC's parser reads the test's own source first
and overflows a stock test thread around 1200 levels, which ADR 0004 already
attributes to the stages around the fold.

**Checkbox 3 says "a module", and the module half of it is the half that cannot
be written.** At 8192 levels a real compile aborts in the stages around the fold,
which is the residue ADR 0004 already owns — so no module reaches that depth to
either fold or refuse there. The checkbox is met in two pieces instead. The
evaluator's suite carries the nesting, at the ceiling and either side of it, with
nothing compiled around it.
`a_module_compiled_at_the_largest_ceiling_folds_rather_than_aborting` carries
what only a module can: `maxEvaluationDepth: 8192` set through the option
surface, folding 300 levels on a real compile, so the largest number the option
accepts is shown to reach the claim rather than stopping at the parser that
reads it.

**And one thing the outward direction cannot yet prove.** A value the engine
builds by looping converts at 1000 levels and is pinned there. Past roughly two
thousand the engine aborts while *building* it, before anything is converted —
its own limit, reached identically with the old claim in place, and not something
this ticket moved.
