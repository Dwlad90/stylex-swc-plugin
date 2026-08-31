# 11 — Bring the state crate to the coverage gate

**What to build:** The `stylex-state` crate that ticket 08 extracted measures
41.92% of regions and 40.50% of lines against its own tests. The workspace
coverage gate demands zero uncovered lines and zero uncovered regions from every
crate that has a `src/lib.rs` and tests, and `stylex-state` is not on the
exemption list. Write the tests that close the gap.

The shortfall is not new code going untested. It is a boundary revealing what
was already true: the state manager was covered *transitively*, by compiling
whole files through the transform, and the transform is itself exempt from the
gate. Extracting the state manager made that coverage stop counting for it.

`stylex-state` is on the coverage exemption list, so CI is green in the
meantime. The exemption is the holding position, not the answer: the code is
worth direct tests, and this ticket removes the exemption when they exist.

**Where the gap is**, from `pnpm run test:coverage` in the crate:

| File | Uncovered regions | Uncovered lines | Cover |
| --- | --- | --- | --- |
| `state_manager.rs` | 1373 | 963 | 38.92% |
| `common.rs` | 114 | 67 | 71.28% |
| `evaluate_result_value.rs` | 77 | 53 | 31.86% |
| `theme_ref.rs` | 65 | 58 | 46.28% |
| `flat_compiled_styles_value.rs` | 58 | 46 | 0.00% |
| `evaluate_result.rs` | 30 | 30 | 0.00% |
| `state.rs` | 9 | 12 | 0.00% |
| `functions.rs` | 6 | 5 | 0.00% |

The five small files are mostly constructors, accessors and the `StyleqValue`
implementation, and are reachable directly. `state_manager.rs` is the real work
and is best taken a method group at a time.

Note that the coverage tool keeps only the best-covered instantiation of a
generic, so a generic helper can read as fully covered while one instantiation
is untested.

**Blocked by:** None — the state crate exists as of `a1baab79e`.

**Status:** not now, backlog

- [ ] `stylex-state` reports zero uncovered lines and zero uncovered regions.
- [ ] `stylex_state` is removed from all three exemption lists:
      `package.json`, `scripts/coverage-missing.sh` and
      `scripts/packages/test/coverage.sh`.
- [ ] Tests cover regular and irregular inputs, and the edge cases each method
      states, rather than only the path that makes the number go up.
- [ ] The full workspace suite stays green.
