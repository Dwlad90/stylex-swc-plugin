# 06 — Pin the reproduction matrix as fixtures

**What to build:** The complete divergence matrix from the spec, as one
`stylex.create` fixture file, so that a future refactor of the evaluator cannot
quietly undo any of this.

Tickets 03, 04 and 05 each land their own fixtures for the behaviour they fix.
This ticket is the sweep-up: the rows none of them owned individually, and the
combinations that only appear when the operators meet.

Two categories to cover that the earlier tickets have no natural home for:

- **The already-matching rows**, kept as regression coverage rather than
  assumed: `'a' + 'b'`, `1 + 'px'`, `'solid' + ''`, `'' + 'solid'`, `1 + 2`,
  nested `+`, `+` inside a template literal, and `??` on a dynamic style
  function parameter. These agreed with upstream before the effort started and
  must still agree after it.
- **Cross-operator combinations**: a logical operator whose winning operand is
  itself an addition, an addition whose operand is a logical expression, a
  logical operator nested inside another, and a logical operator in a style
  value that also carries a static part.

Test one claim per fixture, at the seam the rest of the evaluator is tested
from: StyleX source in, CSS metadata out. Do not reach into the evaluator's
value representation and do not assert which internal path an operator took —
those are the details this effort rearranged, and a test coupled to them would
have to be rewritten by the very commit it exists to guard.

Carry forward the trap ticket 03 records: for any case where the expected
outcome is a refusal, assert the **diagnostic**, not merely that the build
failed. Several of these cases failed before the effort for entirely different
reasons, so a bare failure assertion would have passed all along.

**Blocked by:** 03, 05.

**Status:** resolved

- [x] One fixture file under the create-transform suite carries the matrix
- [x] Every row of the spec's divergence table is represented, including the
      rows where the expected outcome is a refusal
- [x] The already-matching rows are present as regression coverage
- [x] Cross-operator combinations are covered: logical over addition, addition
      over logical, nested logical, and logical beside a static part
- [x] Refusal cases assert the diagnostic rather than the bare failure
- [x] No fixture reaches into the evaluator's value representation or asserts
      an internal path
- [x] Each fixture states one claim, named for the claim it states
- [x] `pnpm run --filter=@stylexswc/rs-compiler build` before any suite that
      reaches the compiler through the Node package
- [x] `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`, `pnpm test` all
      pass
- [x] Lands as `test(stylex-transform):`

## Answer

The sweep-up is
`crates/stylex-transform/tests/transform_stylex_create_test/operator_interaction.rs`,
six fixtures covering the four cross-operator combinations — the nested-logical
combination splits into the two shapes it has, and the refusal edge gets its
own. Every class name, rule text, priority and property key in their snapshots
is measured output of `@stylexjs/babel-plugin@0.19.0` for the same input, and
the refusal is measured there too.

**The matrix was re-measured whole, not only the new rows.** A throwaway script
extracted the source out of every fixture in this effort, ran it through the
reference implementation, and compared the emitted rules against the committed
snapshot. Thirty-four of thirty-seven agree exactly. The three that do not are
all already-recorded, deliberate states:

- `and_returns_a_falsy_left_side` — the reference implementation crashes with a
  bare `TypeError`. This is upstream defect 2, the subject of ticket 07.
- `an_operator_with_no_string_result_deopts_rather_than_failing` and
  `a_left_side_with_no_numeric_form_still_refuses_under_other_operators` — the
  reference implementation *folds* `'a' * 'b'` to `NaN`, `null - 1` to `-1`, and
  `({}) * props.x` to a dynamic style, where this compiler deopts the first two
  and refuses the third. Both are non-`Add` arms of the number path, which the
  spec puts out of scope; neither is reachable from issue #1254 and neither is a
  wrong *value* — they are a refusal and a deopt where upstream has a fold.
  Worth its own ticket, not this one.

**Two already-matching rows are covered elsewhere and not restated here.** The
`+` rows all belong to `string_concatenation.rs`, which ticket 04 landed with
them; `??` on a dynamic style function parameter is
`dynamic_styles.rs::nullish_coalescing_safe_left_side`, whose snapshot the whole
effort left untouched — which is itself the claim that row exists to make. A
second snapshot of identical output would record the same claim twice.
