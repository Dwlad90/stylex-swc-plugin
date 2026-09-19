# 62 — Re-declare the transform's exclusion as temporary and record the baseline

**What to build:** The transform crate is the last large crate off the coverage
gate, and its row in the five exclusion lists is **permanent**: "SWC transform,
tested through snapshot tests". That reason describes how the crate is tested,
not why its coverage cannot be measured. Measured, it reads 89.03% of regions
and 90.09% of lines. The gate demands zero uncovered lines and zero uncovered
regions, so the row is a temporary one written as a permanent one.

This ticket makes the record true and gives the three batch tickets a baseline
to work against. It writes no test and turns no gate on.

**The measurement**, from `scripts/coverage-missing.sh -p stylex_transform` on
the tip of `fix_benchmarks` (rustc 1.100.0-nightly, 2865 tests, 18s of run
time):

| Figure                                   | Value              |
| ---------------------------------------- | ------------------ |
| Regions                                  | 89.03%             |
| Functions                                | 92.78%             |
| Lines                                    | 90.09%             |
| Regions unexercised by every test        | 741, over 58 files |
| Uncovered lines                          | 630, over 58 files |
| Files with a per-instantiation gap alone | 7                  |

The 741 fall into the four groups the batch tickets take:

| Group                                                         | Regions | Files | Ticket |
| ------------------------------------------------------------- | ------- | ----- | ------ |
| `shared/utils/**`                                             | 334     | 20    | 63     |
| `transform/stylex/**`                                         | 198     | 18    | 64     |
| `transform/visit_mut/**`, `transform/mod.rs`                  | 123     | 7     | 65     |
| `shared/transformers/**`, `shared/structures`, `shared/enums` | 86      | 13    | 66     |

**Why the coverage is there to be had.** This is not the boundary effect
[ticket 15](./15-cover-the-evaluator-crate.md) records for the evaluator, where
moving code out of the transform stopped the transform's own suite from counting
for it. Here the suite and the code are in one crate, so every integration test
under `tests/` already counts. The 741 regions are what 2865 tests never reach:
refusal arms, the second half of a two-mode helper, and the branch a fixture has
no spelling for.

**The performance fixture has to stop measuring the clock.**
`tests/performance_fixture/performance_fixture_test.rs` asserts a ratio between
two wall-clock timings. A coverage run instruments every branch and slows both
sides by different amounts, and the test is already recorded as flaky in a full
parallel run. The gate is worthless if the suite under it is not repeatable, so
this ticket settles that assertion: the fixture comparison it also makes
(`assert_eq!(output, output_fixture)`) is the part that has value, and the
timing part must either be removed, made a reported figure with no assertion, or
skipped under instrumentation. Choose one and say why in the file.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] The `stylex_transform` row moves from `Permanent:` to a `Temporary:`
      heading in `guidelines/STRUCTURE.md`, and names
      `67-remove-the-transform-coverage-exclusion` as its remover. The prose
      above the permanent rows that says "There is no temporary row" is
      corrected. **Superseded by 67**, which took the crate off the exclusion
      list entirely: the temporary row and its heading went with it, the prose
      reads "There is no temporary row" again, and the paragraph under it
      records that `stylex_transform` held the last one. So the file holds no
      `Temporary:` heading today, and that is the end state this ticket wanted.
- [x] The reason on the row says why the coverage is not there yet, not how the
      crate is tested.
- [x] The other four lists keep the crate excluded and
      `scripts/git/coverage-exclusions.test.mjs` stays green (20 of 20). Two
      comment lines moved in `scripts/coverage-missing.sh`, neither of them a
      row: the row comment said `# permanent`, and a comment further down said
      the list holds no temporary row. `coverage-exclusions.mjs` strips `#` to
      the end of the line before it reads a name, so a comment is for a reader
      only.
- [ ] `performance_fixture_test` no longer asserts on wall-clock time, and the
      file says what replaced the assertion and why. **Not done at the owner's
      instruction**: the change was written (the ratio dropped, the two timings
      kept as a printed line, the fixture comparison kept) and then reverted on
      request. Ticket
      [68](./68-stop-the-performance-fixture-timing-the-clock.md) carries it,
      and blocks ticket 67 — the gate cannot stand over a suite that measures
      the machine.
- [x] The baseline above is reproducible: two runs of
      `scripts/coverage-missing.sh -p stylex_transform` gave byte-identical
      reports. Every figure in the table holds.
- [x] The full workspace suite stays green.

## Comments

### The baseline reproduces, figure for figure

Two runs on this machine, `rustc 1.100.0-nightly (4b6d04e70 2026-09-13)`, 2865
tests. The reports are identical byte for byte from the summary table down, so
the only thing that moved between them was the nextest run id and the clock.

| Figure                                   | Ticket | Measured |
| ---------------------------------------- | ------ | -------- |
| Regions                                  | 89.03% | 89.03%   |
| Functions                                | 92.78% | 92.78%   |
| Lines                                    | 90.09% | 90.09%   |
| Regions unexercised by every test        | 741    | 741      |
| Files holding them                       | 58     | 58       |
| Uncovered lines                          | 630    | 630      |
| Files with a per-instantiation gap alone | 7      | 7        |

The four groups reproduce too, so tickets 63 to 66 divide the 741 with no
overlap and no remainder:

| Group                                                         | Ticket | Regions | Files |
| ------------------------------------------------------------- | ------ | ------- | ----- |
| `shared/utils/**`                                             | 63     | 334     | 20    |
| `transform/stylex/**`                                         | 64     | 198     | 18    |
| `transform/visit_mut/**`, `transform/mod.rs`, `visit_mut.rs`  | 65     | 123     | 7     |
| `shared/transformers/**`, `shared/structures`, `shared/enums` | 66     | 86      | 13    |

One addition to the group table in the body: the third group is spelled
`transform/visit_mut/**`, `transform/mod.rs` there, and a third file belongs to
it -- `transform/visit_mut.rs`, which is not the same file as `mod.rs`. The
group is 91 regions under `transform/visit_mut/`, 29 in `transform/mod.rs` and
3 in `transform/visit_mut.rs`: 123 over 7 files, as the body says. Ticket 65
already names all three.

The full report is kept at
[`baseline/transform-coverage.log`](../baseline/transform-coverage.log), so the
batch tickets have the `file:line:col` list to work from and can diff against
it.

### The gate counts more than the 741

Three counts come out of one run, and they are three different sums. Do not add
them together:

- **741** regions that no test runs at all. These are what tickets 63 to 66
  divide, and a test closes each one.
- **753** regions that llvm-cov calls uncovered. The 12 above the 741 did run,
  but never all of them in one monomorphization.
- **31** regions that the gate counts, over 7 files. The gate scores a function
  on its best-covered instantiation, so a region counts unless one instantiation
  runs it. This sum is taken over whole functions, so it is not the 12 and it is
  not a part of the 741.

The 31 need a different fix from the 741: drive a single instantiation through
the whole function. A case per type does not work, because each type is a
separate instantiation and no one of them covers everything.

### The machine was not idle

Fourteen orphaned busy-loop shells from an earlier session held this machine at
0% idle, load average 357. They were killed before the measurement. The
coverage figures do not depend on the clock, so the numbers stand either way,
but nothing here was timed until the machine was quiet.

### The timing assertion was reverted on request

The ticket asked for the wall-clock assertion in
`performance_fixture_test.rs` to be settled. The first of the three ways was
taken -- drop the ratio, keep both timings as a printed line, keep
`assert_eq!(output, output_fixture)` -- and the owner asked for the revert. The
test is unchanged on this branch. Ticket 68 holds the work and the reasoning.

## Re-measured after the legacy polyfill suites were registered

The batch figures below were all taken while six files holding 42
`stylex_test!` cases sat under `tests/legacy/` with no `mod` line. Nothing
compiled them, so every region they alone reach was counted as reached by
something else, or not reached at all.

They are registered now, and the gate was re-measured with a clean profile:

| | Regions | Functions | Lines |
| --- | --- | --- | --- |
| Workspace | 28,858 / 0 missed | 3,657 / 0 missed | 28,529 / 0 missed |
| `stylex-transform` | 6,096 / 0 missed | | |

The per-batch numbers above are left as they were read. They record what each
batch measured at the time, which is the thing this ticket exists to hold; the
figures that describe the branch as it stands are these.
