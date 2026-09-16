# 67 — Put the transform crate on the coverage gate

**What to build:** The removal of `stylex_transform` from every coverage
exclusion list, so the workspace gate measures the crate and fails on the first
region a future change leaves unexercised. Tickets 63 to 66 close the 741
regions; this ticket is what makes closing them permanent.

**Five lists must agree**, and
`scripts/git/coverage-exclusions.test.mjs` compares all five and names the one
that disagrees — so removing four of them fails that test rather than leaving a
crate half-excluded:

- the `test:coverage:workspace` script in the root `package.json`
- `EXCLUDED_CRATES` in `scripts/coverage-missing.sh`
- the `case` in `scripts/packages/test/coverage.sh`
- `EXCLUDED` in `scripts/git/crate-coverage-runner.test.mjs`, which asserts that
  `case`
- the row under "Excluded from Coverage" in `guidelines/STRUCTURE.md`

The list in `scripts/packages/test/coverage.sh` holds crate **directory** names
and the others hold Cargo package names, so `stylex-transform` comes out of that
one and `stylex_transform` out of the rest.

**Two things to settle while the row comes out.** The `Temporary:` heading
[ticket 62](./62-record-the-transform-coverage-baseline.md) adds holds only this
row, so it goes with it, and the prose above the permanent rows — which names
the evaluator as the last crate to have carried a temporary row — has to name
this crate instead. And with the transform measured, the workspace coverage job
in `.github/workflows/pr-validation.yml` grows by the crate's own run; the
uninstrumented suite is 2865 tests in 18 seconds, so record what the job costs
after the change rather than assuming it is free.

**Blocked by:** [63](./63-cover-the-transform-shared-utils.md) and
[64](./64-cover-the-transform-call-handlers.md), which are both part-met, and
[73](./73-read-a-variable-reference-key-the-way-the-reference-does.md), which
holds what 63 left.
[69](./69-inject-the-rules-an-array-bound-create-declares.md),
[70](./70-read-first-that-works-inside-a-keyframes-step.md),
[71](./71-close-the-guards-over-a-compiler-built-object.md) and
[74](./74-write-the-inline-style-a-props-call-merges.md) are settled: 70 held
no region of its own, and the count is now 36, with no region counted for a
per-instantiation gap. Every region must be closed before the gate can be
turned on, because the gate fails on one.
[65](./65-cover-the-transform-walk.md) and
[66](./66-cover-the-transformers-and-structures.md) are met. Also
[68](./68-stop-the-performance-fixture-timing-the-clock.md): the crate's suite
holds a wall-clock assertion, and instrumentation moves it, so the gate would
stand over a suite that can go red on a busy machine. The measurement is in
the comment below.

**Status:** ready-for-agent

- [ ] `stylex_transform` is removed from all five lists, and
      `scripts/git/coverage-exclusions.test.mjs` is green.
- [ ] `pnpm run test:coverage:workspace` reports zero uncovered lines and zero
      uncovered regions with the transform measured. 36 regions are open, held
      by [63](./63-cover-the-transform-shared-utils.md),
      [64](./64-cover-the-transform-call-handlers.md) and
      [73](./73-read-a-variable-reference-key-the-way-the-reference-does.md).
- [ ] `guidelines/STRUCTURE.md` keeps no `Temporary:` heading with no row under
      it, and its prose no longer names the evaluator as the last temporary row.
- [~] The cost the coverage CI job takes after the change is recorded in this
  ticket. The two local runs are in the comment below; the CI figure waits
  on the gate being turned on, which this ticket does.
- [x] The full workspace suite stays green, and so is the coverage gate, which
      [77](./77-restore-the-workspace-coverage-gate.md) repaired.

## Comments

### The count after ticket 71

**36 unexercised regions over fourteen files**, from the 84 measured below, and
none counted for a per-instantiation gap. `scripts/coverage-missing.sh -p
stylex_transform` on 2026-09-16:

| Where                                                  | Regions | Held by |
| ------------------------------------------------------ | ------- | ------- |
| `src/transform/stylex/create_call/helpers.rs`          | 7       | 64      |
| `src/transform/stylex/transform_stylex_atoms.rs`       | 5       | 64      |
| `src/transform/stylex/create_call/mod.rs`              | 3       | 64      |
| `src/transform/stylex/create_theme_nested_call.rs`     | 3       | 64      |
| `src/transform/stylex/transform_define_marker_call.rs` | 3       | 64      |
| the other seven files under `src/transform/stylex/`    | 6       | 64      |
| `src/shared/utils/core/`                               | 6       | 63, 73  |

### The gate was turned on, measured, and turned off again

The five lists were edited and the gate was run. It fails: the transform holds
**84 unexercised regions over twenty files**, and two more counted against the
gate for a per-instantiation gap. `scripts/coverage-missing.sh` on
2026-09-16, `rustc 1.100.0-nightly`:

| Where                                                         | Regions | Held by    |
| ------------------------------------------------------------- | ------- | ---------- |
| `src/transform/stylex/create_call/dynamic_style_functions.rs` | 19      | 71         |
| `src/transform/stylex/define_vars_call/helpers.rs`            | 15      | 71         |
| the other fourteen files under `src/transform/stylex/`        | 41      | 64, 69, 70 |
| `src/shared/utils/core/`                                      | 8       | 63, 73, 74 |
| `visit_mut_var_declarator.rs`, per-instantiation              | 2       | 71         |

Tickets 63 and 64 record this themselves -- both acceptance criteria are `[~]`,
not `[x]` -- and ticket 66 states that 83 regions remain. This ticket's
`Blocked by` line named neither, which is the correction above. The edit to the
five lists was reverted; it is four one-line deletions and is cheap to redo
when the regions are closed.

### The gate was already red before the change

Measured on the way in, and filed as
[77](./77-restore-the-workspace-coverage-gate.md): with `stylex_transform`
still excluded, `pnpm run test:coverage:workspace` failed on seventeen regions
in `stylex_state`, `stylex_structures` and `stylex_diagnostics`. Sixteen of
them are API the transform alone calls, which the per-crate reading the batch
used counts and the gate does not. They are closed, and the gate is green at
100.00% of regions, functions and lines over 6189 tests.

### What the coverage job costs

Not yet measured with the transform on the gate, because the gate cannot be
turned on. What is known: the workspace run without the transform takes 36
seconds locally, 6189 tests; with the transform it takes 62 seconds, 9455
tests. Record the CI figure when the gate is turned on.
