# 02 — Teach the coercion crate `ToBoolean` and nullishness

**What to build:** Nothing an author can observe yet. This ticket supplies the
two questions ticket 03 needs to ask, and puts them where the project already
keeps questions of that kind.

The coercion crate answers what ECMAScript says a value converts to when
another type is asked for — it already covers `ToString`, `ToNumber` and
`ToObject` over an already-evaluated expression. It gains two more:

- `ToBoolean`, following the same refusal contract its siblings document: a
  value with no compile-time form of that type is refused, and the caller
  [deopts](../../../crates/stylex-transform/CONTEXT.md) rather than inventing
  an answer.
- A nullish predicate — `null`, `undefined`, and `void` — a plain question about
  an expression rather than a coercion, but sitting beside them because that is
  where the reference implementation's `??` reaches.

The evaluator's own value representation is bridged to `ToBoolean` by a fourth
helper alongside the three bridging helpers that already exist. Do not write
this mapping privately inside the consumer in ticket 03 — the variant partition
it needs is already written down twice in the bridging helpers, and a third
private copy is how these things drift.

The partition to follow is the one the object bridge already draws: every
variant the evaluator has of its own stands for a JavaScript object or a
function upstream, so all of them are truthy. The absent-value variant refuses
rather than answering, for the same reason the object bridge records — it can
mean "absent" or it can mean "unknown", and those two would answer differently.
Mirror that reasoning in a comment rather than restating the conclusion.

The dependency edge stays one-way: the coercion crate must not learn about the
evaluator's value representation.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] The coercion crate answers `ToBoolean` over an evaluated expression,
      refusing where it has no compile-time boolean form
- [x] The coercion crate answers whether an expression is nullish, covering
      `null`, `undefined` and `void`
- [x] Both are covered in the coercion crate's own test module, at the boundary
      the crate publishes
- [x] A fourth bridging helper maps the evaluator's value representation to
      `ToBoolean`, beside the three that exist and drawing the same partition
- [x] The absent-value variant refuses, with a comment giving the reason rather
      than repeating the object bridge's prose
- [x] The coercion crate gains no dependency on the transform crate
- [x] Nothing consumes the new helpers yet — the suite passes with zero fixture
      or snapshot edits
- [x] `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`, `pnpm test` all
      pass
- [x] Lands as `feat(stylex-js):` for the coercions and `feat(stylex-transform):`
      for the bridge

## Comments

Landed as three commits:

- `2ba07883a feat(stylex-js)` — `to_js_boolean` and `is_nullish`, with 12 tests.
- `3cef68bea feat(stylex-transform)` — `evaluate_result_to_js_boolean`.
- `5d5c4a2ba fix(stylex-js)` — a defect the review caught before ticket 03
  could inherit it.

Every expected value in the new tests was checked against a JavaScript
runtime rather than reasoned about, and each one matches.

**The bug the review caught.** `ToBoolean` on a number was written as
`num.value != 0.0`. That answers the two zeroes correctly and `NaN` wrongly —
every comparison against `NaN` is false *except* the inequality, so `NaN` came
back truthy. It was covered only through `ident_expr("NaN")`, which takes the
identifier arm and was always right; the number arm is the one a fold reaches,
via `Number('10px')`. Left in, ticket 03 would have folded `NaN || x` to `NaN`
where upstream folds to `x`. Fixed, and both spellings of the value are now
asserted side by side.

**`void` in `ToBoolean` as well as in the nullish predicate.** The two are
asked about the same value by the same guard, so a `void x` that `??` folds and
`||` refuses would be a disagreement the language does not have.

**`expect(dead_code)`, not `allow`.** Nothing consumes the bridge yet, which
this ticket requires. `expect` fails the build the moment ticket 03 wires it
up, so the attribute cannot outlive its reason.

**Follow-up worth filing.** A third copy of the nullish question already exists,
privately, at `transform_stylex_create_call/mod.rs:128` — over
`EvaluateResultValue`, for the `when` marker slot. It predates this ticket and
has already drifted: it does not know `void`. Collapsing it onto
`coercions::is_nullish` needs a *fifth* bridging helper and changes a live call
site, so it is out of scope here — but it is exactly the duplication this
ticket exists to prevent, and should not be left unrecorded. See issue 08.
