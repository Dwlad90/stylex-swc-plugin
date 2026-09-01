# 09 — Re-home the evaluator's tests and benches

**What to build:** The three criterion benches that measure evaluation, folding
and evaluation depth still sit with the transform after the core moved. Move them
to the crate they now cover, so the evaluator can be profiled on its own.

**The unit tests are already done.** Ticket 13 had to move them with the code:
workspace coverage runs `--exclude stylex_transform`, so a test left there never
runs under the gate, and reaching into the evaluator from the transform would
have made the fold and the node handlers `pub`. All 9.9k lines travelled, and the
scaffolding de-duplication below travelled with them. See ticket 13's Comments.

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

**The two copies of the test scaffolding have converged.** Ticket 07 could not
move the five helpers its tests read off `tests/source_evaluation.rs` -- a
parser, a thread of a stated size, the two thread sizes and the nested literal --
because `source_evaluation.rs` as a whole builds a `StateManager`. They were
duplicated into `crates/stylex-evaluator/src/tests/scaffolding.rs`. Ticket 13
brought both copies into one crate and deleted the second: `source_evaluation.rs`
now re-exports the five from `scaffolding`, which RUST.md permits as a test
prelude. Nothing is left to do here.

Two dev-dependencies wait on this move. The transform still declares
`boa_engine` and `stylex_js` under `[dev-dependencies]` solely for
`engine_fold_bench` and one test helper; ticket 13 could not remove them while
the benches stayed. Check both when the benches leave.

**Blocked by:** 13 — Move the evaluator core. (Was 08, which resolved by
re-scope; the move it would have provided is now 13.)

**Status:** ready-for-agent

- [x] The evaluation unit tests live with the evaluator crate. (Ticket 13.)
- [ ] The three evaluation benches are declared as targets on the evaluator crate.
- [ ] The four remaining benches still build and run on the transform.
- [ ] Test assertions, inputs and fixtures are unchanged; only import lines differ.
- [ ] A manual same-machine before/after is recorded for the three moved
      benches, A/B against `develop`.
- [x] `crates/stylex-evaluator/src/tests/scaffolding.rs` and the transform's
      copy of the same five helpers are one copy, not two. (Ticket 13.)
- [ ] `boa_engine` and `stylex_js` leave the transform's `[dev-dependencies]`
      with the benches, or a reason to keep them is recorded.
- [ ] The evaluator crate reaches zero uncovered lines and zero uncovered
      regions. This is [ticket 15](./15-cover-the-evaluator-crate.md) rather
      than this ticket: the shortfall is 33% of the crate and has nothing to do
      with where the benches live.
- [ ] Debug workspace build and test green.
