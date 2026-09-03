# 15 — Bring the evaluator crate to the coverage gate

**What to build:** The `stylex-evaluator` crate that ticket 13 filled measures
66.76% of regions and 70.70% of lines against its own tests. The workspace
coverage gate demands zero uncovered lines and zero uncovered regions from every
crate that has a `src/lib.rs` and tests. Write the tests that close the gap.

The shortfall is not new code going untested. It is the same boundary effect
[ticket 11](./11-cover-the-state-crate.md) records for the state crate: the
evaluator was covered _transitively_, by compiling whole files through the
transform's integration suite under `crates/stylex-transform/tests/`, and the
transform is itself exempt from the gate. Moving the evaluator out made that
coverage stop counting for it. Ticket 13 moved all 569 unit tests down with the
code, which is what the 66.76% is; the missing third has no unit test anywhere.

`stylex-evaluator` is on the coverage exemption list, so CI is green in the
meantime. The exemption is the holding position, not the answer.

**Where the gap is**, from `pnpm run test:coverage` in the crate:

| File                                    | Uncovered regions | Cover          |
| --------------------------------------- | ----------------- | -------------- |
| `evaluate/nodes/call_expression.rs`     | 464               | 31.16%         |
| `evaluate/nodes/member_expression.rs`   | 282               | 49.10%         |
| `evaluate/engine_fold/transport.rs`     | 268               | 39.09%         |
| `evaluate/engine_fold/guard.rs`         | 201               | 81.54%         |
| `evaluate/helpers.rs`                   | 183               | 61.06%         |
| `evaluate/engine_fold/amplification.rs` | 135               | 74.77%         |
| `evaluate/mod.rs`                       | 125               | 66.22%         |
| `evaluate/engine_stylex_functions.rs`   | 102               | 16.39%         |
| `evaluate/nodes/global_conversion.rs`   | 92                | 17.12%         |
| `evaluate/nodes/object_expression.rs`   | 82                | 70.61%         |
| `evaluate/nodes/arrow_function.rs`      | 73                | 32.41%         |
| `evaluate/nodes/binary_expression.rs`   | 59                | 81.27%         |
| `evaluate/nodes/unary_expression.rs`    | 49                | 67.33%         |
| `evaluate/engine_fold/outward.rs`       | 37                | 82.04%         |
| `evaluate/engine_fold/theme.rs`         | 37                | 74.83%         |
| `convertors.rs`                         | 29                | 86.94%         |
| `evaluate/binding.rs`                   | 28                | 84.53%         |
| the remaining twelve files              | 63                | 68.42%--97.92% |

Three groups, in the order they are worth taking:

1. **The small whole-file gaps** -- `evaluate_result.rs` (the `refused`
   constructor) and `nodes/typescript_expression.rs` both read 0.00% and are a
   handful of lines each. `nodes/global_conversion.rs` and
   `engine_stylex_functions.rs` are under 20% and are each one concern.
2. **The two big node handlers** -- `call_expression.rs` and
   `member_expression.rs` carry 746 uncovered regions between them and are best
   taken one callee shape and one lookup shape at a time. Every shape is already
   reachable through `evaluate_source`, so no new scaffolding is needed.
3. **`transport.rs`** -- 268 regions at 39%. The carriage of a value in and of
   an engine value back is exercised end to end by the transform's suite and
   almost not at all directly.

Note that the coverage tool keeps only the best-covered instantiation of a
generic, so a generic helper can read as fully covered while one instantiation
is untested.

**Blocked by:** None — the evaluator crate is filled as of ticket 13.

**Status:** backlog

- [ ] `stylex-evaluator` reports zero uncovered lines and zero uncovered
      regions.
- [ ] `stylex_evaluator` is removed from all three exemption lists:
      `package.json`, `scripts/coverage-missing.sh` and
      `scripts/packages/test/coverage.sh`.
- [ ] Tests cover regular and irregular inputs, and the edge cases each function
      states, rather than only the path that makes the number go up.
- [ ] The full workspace suite stays green.

## Comments

**2026-09-02, ticket 25.** Re-measured on the tip of
`feat_split-transform-crate`: 66.86% of regions, 75.22% of functions and 70.93%
of lines, with 2347 unexercised regions across 28 files. Ticket 24 added two
test files here and moved the region figure by 0.1 of a point, which is the
point that ticket made: the holes it closed were behavioural, not lines. The
exclusion stays until this ticket lands, and `guidelines/STRUCTURE.md` now names
this ticket as its remover.
