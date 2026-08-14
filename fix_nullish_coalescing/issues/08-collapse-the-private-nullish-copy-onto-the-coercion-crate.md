# 08 — Collapse the private nullish copy onto the coercion crate

**What to build:** No change an author can see, unless they write
`stylex.when(sel, void 0)` — which today is treated as a marker and after this
is treated as no marker, the way `null` and `undefined` already are.

Ticket 02 put the nullish question in the coercion crate, on the grounds that a
private copy in a consumer is how these things drift. A third copy already
exists and has already drifted:
`crates/stylex-transform/src/transform/stylex/transform_stylex_create_call/mod.rs`
answers the same question privately over the evaluator's value representation,
for the marker slot of a `when` call, and does not know the `void` spelling that
the coercion crate's copy does.

Collapsing it needs a *fifth* bridging helper — nullishness over
`EvaluateResultValue`, beside the four that now exist — because the private copy
answers for one variant the coercion crate cannot see: the absent-value variant
is nullish here, since an absent marker and a marker that evaluated to nothing
hand the slot to the options alike. That is a genuine difference from the
`ToBoolean` bridge, where the same variant refuses, and the reason for it
belongs in a comment rather than being left for the next reader to rediscover.

This was left out of ticket 02 deliberately: it changes a live call site, and
ticket 02's own acceptance required zero fixture movement.

**Blocked by:** None. Independent of 03–07 — it touches a different call site.

**Status:** resolved

- [x] A fifth bridging helper answers nullishness over the evaluator's value
      representation, beside the four that exist
- [x] The absent-value variant is nullish here, with a comment saying why this
      bridge parts company with the `ToBoolean` one rather than restating it
- [x] `transform_stylex_create_call`'s private `is_nullish` is deleted, not left
      as a fallback
- [x] A fixture covers `stylex.when` with a `void 0` marker, which is the one
      behaviour this changes
- [x] Compared against `@stylexjs/babel-plugin` before the fixture is pinned
- [x] `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`, `pnpm test` all
      pass
- [x] Lands as `refactor(stylex-transform):`, or `fix(stylex-transform):` if the
      `void 0` row turns out to be a divergence rather than an unreachable case

## Outcome

The ticket's premise about `void 0` was wrong in the author's favour, and
worse than described. `void 0` was not "treated as a marker": the unary node
answered with no value at all, and the evaluator's caller turns a confident
absence into a deopt, so **every** `void` expression failed the build with
`Unsupported expression: Known(Undefined)` — the `stylex.create` call carrying
the marker included. A dead `UnaryOp::Void` match arm further down the same
function said `undefined`, which is what made the early return look
deliberate.

So this landed as two commits rather than one:

1. `fix(stylex-transform): read \`void x\` as the undefined it is` — the
   divergence. `void` answers with the `undefined` identifier, still without
   evaluating its operand, and the unreachable arm is gone. Measured against
   `@stylexjs/babel-plugin@0.19.0`: the `void 0` marker produces the prefixed
   default marker byte for byte as `null` and `undefined` do, and
   `void 0 ?? 'red'` / `void 'blue' || 'red'` both fold to
   `.x1e2nbdu{color:red}`. `evaluates_void_unary_value_expressions`, which
   pinned the old failure, is now
   `void_evaluates_to_undefined_without_reading_its_operand`.
2. `refactor(stylex-transform): collapse the private nullish copy onto a
   bridge` — the ticket proper, with no fixture movement, as its acceptance
   asked.

The private copy had drifted in one way beyond the `void` spelling the ticket
named: it took **any** identifier named `undefined` for the value, including
one an author had shadowed. The bridge asks the coercion crate, which does
not.

A **Coercion bridge** entry was added to `crates/stylex-transform/CONTEXT.md`
— the concept now has five instances and no glossary entry.
