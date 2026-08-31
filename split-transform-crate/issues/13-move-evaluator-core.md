# 13 — Move the evaluator core

**What to build:** The move [ticket 08](./08-move-evaluator-core.md) set out to
make and could not. Ticket 08 landed `stylex-state`;
[ticket 12](./12-extract-declarations-crate.md) lands `stylex-declarations`.
With both below it, the evaluator can leave the transform without a cycle.

Move into `stylex-evaluator`: the dispatcher, every per-node handler, the
**engine fold** with its guard, amplification, **transport** and theme parts,
the **deopt** helper, the declaration check and the stylex function bindings --
and, established by ticket 08's research, also:

- the cycle half of `convertors.rs` that ticket 12 left behind, with its share
  of `convertors_tests.rs`;
- `transformers/stylex_first_that_works.rs`, whose only non-test caller
  anywhere is `engine_stylex_functions.rs`. It moves whole, tests included, and
  the transform loses it;
- `structures/state.rs` (`EvaluationState`) and `structures/evaluate_result.rs`
  (`EvaluateResult`), both pure evaluator vocabulary that ticket 08 parked in
  the transform for exactly this.

Roughly 10.5k lines. It must land as **one atomic move**: the fold and the node
handlers are mutually recursive, and the fold is a private module, so moving
either alone recreates the cycle one level down. Splitting the handlers is
equally impossible -- a handler moved ahead of its dispatcher calls upward too.

**Work mechanically** -- a directory move plus a path rewrite, then iterate
against compiler errors, not by reading the code. Nothing here is being
redesigned. Do not invert the fold/handler edge: that trades the cycle for
indirection on the compiler's hottest path, which this work has explicitly
rejected.

Behaviour must be identical. **Confident** results, **deopt** expressions,
**applied global** resolution and **declared binding** shadowing all keep
exactly their current semantics.

- [ ] The dispatcher, all node handlers, the engine fold, deopt, the declaration
      check and the stylex functions live in the evaluator crate.
- [ ] The fold and the handlers are in the same crate; the mutual recursion
      stays internal to it.
- [ ] No trait or callback indirection was introduced on the evaluation path.
- [ ] The embedded JS engine dependency moved with the fold; the transform no
      longer declares it.
- [ ] No function was renamed, split, merged or reordered.
- [ ] No re-export facade is left in the transform.
- [ ] The transform's source drops to roughly 15k lines.
- [ ] Benches diffed; the fold and evaluation benches show no regression outside
      noise. A/B against `develop`, per decision 4 below.
- [ ] The evaluator crate still reports zero uncovered lines and regions after
      ~9.5k lines land in it. (Ticket 08 carried a criterion about removing a
      ticket-07 exclusion; there is none -- that crate reached 100% -- so this
      replaces it. The live exclusion is `stylex-state`'s, and removing it is
      [ticket 11](./11-cover-the-state-crate.md).)
- [ ] The evaluator's `CONTEXT.md` covers the vocabulary that moved with the
      code. This is the bulk of the transform's glossary, not a handful of
      entries: of the 36 terms it defines today, roughly 28 are evaluation
      vocabulary -- _confident_, _deopt_, _applied global_, _declared binding_,
      _engine fold_, _fold memo_, _fold guard_, _transport_, _carried value_,
      _conversion behind the fold_, _engine-callable StyleX function_, _named
      callback_, _speculative read_, _refused fold_, _hole_, _member lookup_,
      _written slot_, _declared length_, _measured receiver_, _spread refusal_,
      _own enumerable properties_, _own key order_, _object method receiver_,
      _winning operand_, _dead operand_, _coercion bridge_, _string operand_,
      _reference resolution chain_, _folded function map_, _early reference_,
      _evaluation depth_, _measured string_.
- [ ] What stays in the transform is the visitor's own vocabulary: _pre-scan_,
      _pre-rule_, _blank value_, _producer / consumer_, _transformer_, _property
      registration_, _runtime binding_. _Synthesized node_ is a judgement call --
      shorthand expansion and the injected function mappers both produce them, so
      it may belong lower than either crate.
- [ ] ADRs 0005 and 0006 describe the memo key, so they travel with `cache.rs`.
      Fix the inbound links in `stylex-state/CONTEXT.md` and
      `stylex-utils/CONTEXT.md`, which point at the transform's copies.
- [ ] The full workspace suite is green, with `pnpm format:check`, `lint:check`,
      `lint:shell`, `typecheck` and `test`.

## Decisions already taken

Do not re-litigate these; they were settled before this ticket was written.

1. **The state manager moved down rather than the parity constraint bending.**
   Ticket 08's re-scope, taken deliberately against the spec's "everything else
   stays". `StateManager` is one struct with an unchanged method surface.
2. **`stylex_first_that_works` moves with the evaluator.** A `transformers/`
   module that no transformer calls is already filed under the wrong family.
3. **This is its own ticket, not a commit beside ticket 12.** A new crate
   boundary and a 10.5k-line relocation in one reviewable unit cannot be
   reviewed, and bench movement could not be attributed between them.
4. **Benches: A/B against `develop`, the base branch.** The `pre-split`
   criterion baseline was destroyed outside this work and is four commits stale;
   ticket 07 established that an A/B on one machine in one session is the
   stricter test. Expect an LTO-layout floor around +4% -- ticket 07 measured
   +3.65% on a bench whose crate was byte-identical between legs.

**Tooling.** Ticket 08 was executed mechanically with three scripts kept at
[](../tools/README.md): a use-tree expander (a literal path rewrite
misses nested brace groups and reports a false clean), a compiler-driven
visibility narrower, and an import re-nester that corrupts comments and globs --
read its caveats before running it.

**Blocked by:** 12 — Extract the declarations crate.

**Status:** ready-for-agent
