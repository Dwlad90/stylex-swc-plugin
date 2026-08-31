# 09 — Re-home the evaluator's tests and benches

**What to build:** The evaluation unit tests — roughly 5k lines — and the three
criterion benches that measure evaluation, folding and evaluation depth still
sit with the transform after the core moved. Move them to the crate they now
cover, so the evaluator can be judged and profiled on its own.

Criterion baseline identities are per-crate, so the three moved benches cannot
be diffed automatically across the move. Take a manual before-and-after on the
same machine in the same session and record it beside the baseline. The
remaining four benches stay with the transform and diff normally. These benches
are not gated in CI, so this is a local verification step.

**There is no `pre-split` baseline to diff against.** Ticket 07 recorded it as
destroyed by something outside this work, and only its `head-attrib` baselines
survive in `target/criterion`. Every bench comparison from ticket 12 onward is an
A/B against `develop` on one machine in one session, which ticket 07 established
is the stricter test anyway -- it removes the drift of the commits in between.
Expect an LTO-layout floor around +4%.

**Two copies of the test scaffolding converge here.** Ticket 07 moved the
growable stack but could not move the five helpers its tests read off
`tests/source_evaluation.rs` -- a parser, a thread of a stated size, the two
thread sizes and the nested literal -- because `source_evaluation.rs` as a whole
builds a `StateManager`. They were duplicated into
`crates/stylex-evaluator/src/tests/scaffolding.rs`, and the transform kept its
copy for `short_circuited_walk_tests`, `engine_fold_tests` and
`applied_global_tests`. Once those suites arrive here, the duplication has no
reason to exist. Delete one copy; do not leave both.

**Blocked by:** 13 — Move the evaluator core. (Was 08, which resolved by
re-scope; the move it would have provided is now 13.)

**Status:** ready-for-agent

- [ ] The evaluation unit tests live with the evaluator crate.
- [ ] The three evaluation benches are declared as targets on the evaluator crate.
- [ ] The four remaining benches still build and run on the transform.
- [ ] Test assertions, inputs and fixtures are unchanged; only import lines differ.
- [ ] A manual same-machine before/after is recorded for the three moved
      benches, A/B against `develop`.
- [ ] `crates/stylex-evaluator/src/tests/scaffolding.rs` and the transform's
      copy of the same five helpers are one copy, not two.
- [ ] The evaluator crate reaches zero uncovered lines and zero uncovered regions.
- [ ] Debug workspace build and test green.
