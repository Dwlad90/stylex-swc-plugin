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

**Status:** ready-for-agent

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
