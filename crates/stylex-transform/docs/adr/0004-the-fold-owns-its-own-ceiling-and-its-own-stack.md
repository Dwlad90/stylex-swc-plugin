# The fold owns its own ceiling and its own stack

**Status:** accepted

The evaluator walks a nested expression recursively, so before this decision its
limit was the thread's stack and its failure was a process abort:
`fatal runtime error: stack overflow, aborting`. A `SIGABRT` gives a bundler no
message, no file and no chance to finish the rest of the build, and it is
indistinguishable from a crash in the compiler.

Two things are decided together, because neither works alone.

**The fold grows its own stack.** `evaluate_cached` runs the descent inside
`stacker::maybe_grow`, so a deep expression allocates a segment instead of
running into the thread's guard page. Without this the ceiling below could not be
a single number: the arms cost wildly different amounts of stack per level, and a
debug build ran out between 32 and 64 levels of nested `Math.max` where plain
arithmetic reached 384. A ceiling low enough for the worst arm would have refused
inputs the reference implementation folds by an order of magnitude.

**The ceiling is a counted depth, not a measured stack.** `maxEvaluationDepth`
bounds the levels the fold will descend, and crossing it is an ordinary
[refused fold](../../CONTEXT.md) carrying the depth it was willing to spend. The
counter lives on the state manager rather than on the evaluation state, because
the evaluation's confidence forks — a logical operand and a computed key each
get their own — while the stack it is accounting for does not; a counter that
forked would hand a nested fold a fresh allowance while the frames it was
counting were still standing.

**And it is configuration, with a default sized for hand-written styles.** 32,
resolvable per project as `maxEvaluationDepth` and per machine as
`STYLEX_MAX_EVALUATION_DEPTH`, precedence in that order. The default is low on
purpose: the value that keeps a build reporting rather than aborting is a
property of what a project generates, and the compiler cannot know it, so the
shipped number covers styles somebody wrote and anything past that says so out
loud. Measured across this workspace, nothing but three deliberate depth probes
and the ceiling's own tests spends more than a handful of levels.

Together these move the failure from "whatever this thread had left" to a number
the compiler states, and from an abort to a diagnostic naming a file and a key
path.

## Considered options

**Refuse on remaining stack instead of on depth.** `stacker::remaining_stack`
answers the question the abort actually asked, and it needs no per-arm reasoning
at all. Rejected because it makes the output depend on the build: the same input
folds in release and refuses in debug, and refuses or not depending on how deep
the caller already was. A compiler's answer to a source file has to be a
property of the file.

**Grow the stack and set no ceiling.** Nothing then refuses, which sounds like
the strictly better outcome, and is not: the stages around the fold recurse over
the same expression and answer to no ceiling of their own. Parsing, visiting and
printing a 1024-level expression abort in a debug build with the fold never
involved, so removing the fold's ceiling only moves the abort to a stage with
less to say about it. The ceiling sits well under those, so the fold is not the
reason a build reaches one of them at a depth it cannot print.

**Rewrite the recursion as an explicit stack.** The version with no depth limit
at all, and the one this ADR would be replaced by rather than argued with. It is
a rewrite of every arm in the evaluator -- the recursion is mutual across
seventeen of them, and several decide _between_ children rather than visiting
them all -- and it would still want a ceiling for the reason above, so it buys a
higher number for a change to all of the fold.

Measured before filing, in case it was also a performance argument: it is not.
Folding one tower of `(x + 1)` costs 1.7 ms at 60 levels, 4.4 at 120, 14.4 at 240
and 54 at 480 -- roughly quadratic in depth, with the output held constant. Call
frames are not what grows. The per-node structural hash that keys the memo is:
it walks the whole remaining subtree at every level. An explicit stack removes
the frames and leaves the quadratic exactly where it is.

**Panic instead of refusing.** Considered and rejected on
[0002](./0002-a-refusal-and-a-broken-invariant-are-separate-constructs.md): a
deep expression is author input, and a panic in the evaluator claims the broken
invariant is one this code established. The refusal reaches an author as a
diagnostic anyway wherever a static value is required, which is every position a
ceiling this high is reachable from.

## Consequences

**The ceiling is stated in fold levels, and those are not source levels.** A
member read descends to the object and then to the value under the key, a spread
descends to the object it copies, and a parenthesis is unwrapped before the fold
is asked at all. So the deepest accepted source nesting differs per shape — 317
levels of arithmetic, 316 of member reads, 315 of spreads, unbounded parentheses
— and each is pinned in
`tests/transform_stylex_create_test/evaluation_depth_budget.rs` rather than
derived. The message says _nested evaluation_ for the same reason.

**The ceiling is lower than the reference implementation's, deliberately.**
`@stylexjs/babel-plugin` 0.19.0 folds 1024 levels of arithmetic and throws
`RangeError: Maximum call stack size exceeded` past that. Between our ceiling and
theirs the two disagree: they fold, we refuse with a message. Both refuse
eventually, and a refusal at a stated depth was the point.

**In one shape ours is higher.** A dynamic style whose parameter shadows an
imported binding, nested 576 levels deep, throws a `RangeError` upstream and
folds here to the single custom property a shallow one folds to.

**A pathological input allocates.** A segment is allocated only when an
expression reaches the red zone, which nothing under the default ceiling comes
close to. Ad-hoc timing on the complex-theme fixture showed no change --
overlapping ranges over six runs each way, which is a sanity check rather than a
verdict; the gate that decides a regression is `bench:revisions` plus
`bench:verdict`, and it has not been run on this change.

**Depth is quadratic, and the ceiling is what bounds it.** The memo key is a
structural hash of the whole subtree, recomputed per level, so fold cost grows
about quadratically with depth -- 3.3x to 3.7x per doubling, converging on 4x,
and at 240 levels the keys are ~94% of the fold. At the default that is a small
constant; it only became worth knowing because this ADR made deep input
reachable at all. Decided separately rather than fixed here -- the hash is
deliberately span-insensitive and structural and two of its consumers act on a
hash hit alone, so changing it is a correctness question, not a refactor. Both
the measurement and the verdict are
[0005](./0005-the-memo-key-is-a-whole-subtree-hash.md).

**The residue is not the fold's.** A deep enough expression still aborts, in the
stages that recurse over it without a ceiling — measured in a debug build on a
2 MiB thread at 1024 levels with no `stylex` call involved at all, and at 608
with one. That is a separate limit with a separate owner, and it has since been
attributed to one:

- **In release, both numbers are 768 and the stage is SWC's parser.** The abort
  arrives before the module is parsed, so nothing the transform does after that
  is ever the constraint. The two cases converge, which means the gap above —
  where a refusal at 576 is a diagnostic and an abort at 608 is not — is a
  debug-build phenomenon. Release is not uniformly roomier: the no-`stylex` case
  _loses_ 256 levels against the debug build, because inlining merges a
  recursive-descent parser's callee frames into its caller.
- **In debug, the `stylex` case is our printer**, reached through the code
  frame's `get_source_code`, which prints the memoized module when no source
  text was stored beside it. Not the deep clone a refusal records: `deopt` was
  changed to record nothing and the whole table came back unmoved, so the clone
  and its drop glue cost no headroom at all.

Raising the release floor means growing the stack around the _parse_, which
happens in the host rather than anywhere this crate can reach. Named rather than
filed: the default ceiling refuses at 32, and a build that reaches 768 levels of
nesting has a generator loose in it.
