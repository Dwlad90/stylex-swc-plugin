# 08 — Move the evaluator core

**What to build:** Move the general JavaScript evaluator into its crate: the
dispatcher, every per-node handler, the **engine fold** with its guard,
amplification, **transport** and theme parts, the **deopt** helper, the
declaration check and the stylex function bindings. Roughly 9.5k lines.

This must land as **one atomic move**. The fold and the node handlers are
mutually recursive — the call and member handlers invoke the fold, and the fold
imports back out of the evaluator in several places — and the fold is a private
module. Moving either alone would make one of them call upward into the
transform, recreating the cycle. Splitting the handlers is equally impossible:
a handler moved ahead of its dispatcher calls upward too.

**Work mechanically.** This ticket fits a single context window only if it is
executed as a directory move plus a path rewrite, then iterating against
compiler errors — not by reading the code. Nothing here is being redesigned. Do
not invert the fold/handler edge: that trades the cycle for indirection on the
compiler's hottest path, which this work has explicitly rejected.

Behaviour must be identical. **Confident** results, **deopt** expressions,
**applied global** resolution and **declared binding** shadowing all keep
exactly their current semantics.

**Blocked by:** 07 — Create the evaluator crate and seed it with the dependency-free leaves.

**Status:** resolved

**Re-scoped** after ticket 07 reported this ticket unachievable as written. The
decision the report escalated is taken: the state manager moves down, into a
crate of its own rather than into the evaluator. See "The re-scope" below.

- [ ] The dispatcher, all node handlers, the engine fold, deopt, the declaration check and the stylex functions live in the evaluator crate.
- [ ] The fold and the handlers are in the same crate; the mutual recursion stays internal to it.
- [ ] No trait or callback indirection was introduced on the evaluation path.
- [ ] The embedded JS engine dependency moved with the fold; the transform no longer declares it.
- [ ] No function was renamed, split, merged or reordered.
- [ ] No re-export facade is left in the transform.
- [ ] The transform's source drops to roughly 20k lines.
- [ ] Benches diffed against the baseline; the fold and evaluation benches show no regression outside noise.
- [ ] Any temporary coverage exclusion from ticket 07 is removed.
- [ ] The crate's `CONTEXT.md` covers the vocabulary that moved with the code.


## The re-scope

Ticket 07 found that 34 of the 58 files in the evaluate subtree name
`StateManager`, which the spec pins to the transform, so a directory move makes
`stylex-evaluator` depend on `stylex-transform` -- a cycle. It listed two
escapes, both closed by this ticket's own criteria, and escalated.

**What was measured before deciding.** `StateManager` reaches into the transform
in exactly two places, `structures/types.rs` and `structures/seen_value.rs`.
Everything else it names is already an external crate, and every one of those
sits below the evaluator layer. Nothing closes a loop: `stylex-diagnostics`, the
one crate that could have, does not depend on `stylex-structures`.

`types.rs` in turn names `functions.rs`, `theme_ref.rs`,
`evaluate_result_value.rs` and `flat_compiled_styles_value.rs`, and those name
each other back. The knot is one unit and cannot be cut. But **nothing in the
knot names the evaluator core**, so the knot sits strictly below it.

**The decision.** The knot becomes `stylex-state`, a crate of its own below
`stylex-evaluator`, rather than travelling into the evaluator. The evaluator
crate then holds evaluation and nothing else, which is the mismatch ticket 06
spent a rename correcting; a 3k-line compilation-state manager inside a crate
named for evaluation would re-create it one ticket later.

`StateManager` stays one struct with an unchanged method surface, so the parity
constraint the spec set on it is kept, and no trait or callback indirection is
added to the evaluation path.

**What this costs.** Two of the criteria below were written against the
un-rescoped shape and no longer hold as stated:

- "The transform's source drops to roughly 20k lines" -- it drops to roughly
  16k, because the state crate takes about 4k more.
- The spec's "everything else stays: the state manager" is contradicted
  deliberately. It is the sentence that made this ticket unachievable.

**Landing shape.** Two commits: the `stylex-state` extraction, then the
evaluator core move it unblocks.

**Coverage.** The new crate measures 41.92% against its own tests, below the
gate's zero-uncovered requirement, so it joins the exemption list beside the
transform. The shortfall is not untested new code -- the state manager was
covered transitively through the transform, which is itself exempt, and the
boundary stopped that counting. Writing it direct tests and removing the
exemption is [ticket 11](./11-cover-the-state-crate.md), which is unblocked.


## Outcome

**Resolved by re-scope and split.** The ticket as written was unachievable, and
the decision it needed was taken (see "The re-scope"). What that decision made
possible landed here; what it turned out to require is
[ticket 12](./12-extract-declarations-crate.md) and
[ticket 13](./13-move-evaluator-core.md).

### Landed: `a1baab79e`

`crates/stylex-state` exists between nested config and evaluation, holding the
state manager, `types`, `seen_value`, `functions`, `theme_ref`,
`evaluate_result_value`, `flat_compiled_styles_value` and `common`, with the
five unit-test files that cover them. 4,550 source lines. The transform fell
from 30,099 to 25,428.

`StateManager` kept one struct and one method surface, so the parity constraint
held. No trait or callback indirection reached the evaluation path. No
re-export facade was left behind. Workspace `check`, `clippy -D warnings` and
`test` green, with `pnpm format:check`, `lint:check`, `typecheck` and `test`.

Three things the boundary forced:

- The `StyleqValue` implementation went to the type it is for. The orphan rule
  gives it no other home once that type crosses a boundary.
- Six accessors on the evaluated value were deleted. No crate reads them, the
  boundary is what made that visible, and CI runs `clippy -D warnings`.
- `for_test` stopped being `#[cfg(test)]`. Its callers are in the crates above,
  where a cfg set while compiling this crate is not set.

`EvaluationState` and `EvaluateResult` were tried in the state crate and put
back in the transform: nothing in the state crate names either, and both are
evaluator vocabulary.

`stylex_state` is on all three coverage exemption lists. Removing it is
[ticket 11](./11-cover-the-state-crate.md), which is backlogged.

### Not done, and why

The evaluator core did not move. Researching it turned up two edges this ticket
never listed -- the convertors cycle and `stylex_first_that_works` -- and
resolving them needs a crate this ticket never planned. Rather than grow the
re-scope a second time, the remaining work and the decisions taken for it are
[ticket 12](./12-extract-declarations-crate.md), which extracts the crate the
move needs, and [ticket 13](./13-move-evaluator-core.md), which makes the move.

The unchecked criteria at the top of this file are carried there verbatim; none
is abandoned.
