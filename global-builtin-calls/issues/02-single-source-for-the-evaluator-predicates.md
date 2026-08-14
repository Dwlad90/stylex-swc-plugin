# 02 — Source the evaluator predicates from one place

**What to build:** One definition of each JavaScript-semantics predicate, so
that a future change to the set of foldable callees cannot land in one copy and
silently miss the other.

The valid-callee test, the invalid-method test, and both mutating-method tests
exist twice: once in the JS-semantics crate, and once verbatim in the
evaluator's own helpers. The crate was extracted for exactly this purpose — its
glossary already defines all four concepts — but nothing in the workspace
depends on it, so the two copies have been free to drift unobserved.

This is a prefactor: make the change easy, then make the easy change. Ticket 03
hangs off the valid-callee predicate and makes the evaluator depend on this
crate for the first time, which puts both copies in scope simultaneously.
Consolidate before that happens rather than after.

Behaviour-preserving and mechanical. No predicate changes shape and no constant
set changes membership — if a test needs changing, something went wrong.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] The transform crate depends on the JS-semantics crate
- [x] The duplicated predicates are gone from the evaluator's helpers and every
      call site reads them from the crate
- [x] Any evaluator-side test covering a predicate the crate's own tests miss
      has moved across
- [x] The full suite passes with no test modified

## Answer

Eight functions were duplicated, not four: the four the ticket names, plus the
mutation-expression test and the three identifier extractors
(`get_callee_name`, `get_method_name`, `is_id_prop`). All eight are now
defined only in `stylex_js`.

Seven were byte-identical. `is_id_prop` had already drifted — exactly the
failure mode the ticket was written to prevent, arrived at before the
consolidation. On a computed string key that is not valid UTF-8 the crate copy
answered `None`, letting the caller treat it as not an id prop; the evaluator
copy panicked with `INVALID_UTF8`. The evaluator's is the copy with a
consumer, so the crate's arm was dead code, and promoting the panicking body is
what leaves compiler behaviour untouched. It also matches what the convertors
and the shared string helpers already do with the same input. Pinned by a new
crate test; the lone surrogate is built through `CodePoint::from_u32`, so the
crate needs no `unsafe` block.

The evaluator had no unit tests for any of the eight, so nothing moved across.

The call sites needed no edits: `evaluate/mod.rs` imports the eight from
`stylex_js::helpers`, and `nodes/call_expression.rs` picks them up through the
`use super::super::*` it already had. `VALID_CALLEES`, `INVALID_METHODS`, both
mutating-method sets, `INVALID_UTF8`, and `AssignTarget` / `SimpleAssignTarget`
/ `UnaryOp` are no longer named in the evaluator.

Verification: `cargo test --workspace` 5657 passed / 0 failed with no test
modified; clippy and `pnpm format:check` clean; JS package and app suites pass
against a rebuilt `dist/*.node`.

Equality against `@stylexjs/babel-plugin` 0.19.0 is unchanged. Seven inputs
exercising the consolidated predicates were compared before and after; both
runs give the same five matches and the same two mismatches, so neither
divergence is caused by this work. Both are pre-existing gaps unrelated to the
fold: a string `===` comparison this compiler rejects, and a different deopt
message for `Object.assign` over locals. Worth filing separately.

Deliberately not done: renaming `is_id_prop`, whose `is_` prefix reads oddly
for an extractor that returns `Option<&Atom>` and can panic. The rename is
sound, but this ticket is a prefactor whose whole value is that no predicate
changes shape.
