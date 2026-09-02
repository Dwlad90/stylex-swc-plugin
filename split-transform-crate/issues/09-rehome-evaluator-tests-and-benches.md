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

**Status:** resolved

- [x] The evaluation unit tests live with the evaluator crate. (Ticket 13.)
- [x] The three evaluation benches are declared as targets on the evaluator crate.
- [x] The four remaining benches still build and run on the transform.
- [x] Test assertions, inputs and fixtures are unchanged; only import lines differ.
- [x] A manual same-machine before/after is recorded for the three moved
      benches. See the Outcome below for why it is a before/after across the
      move rather than an A/B against `develop`.
- [x] `crates/stylex-evaluator/src/tests/scaffolding.rs` and the transform's
      copy of the same five helpers are one copy, not two. (Ticket 13.)
- [x] `boa_engine` and `stylex_js` leave the transform's `[dev-dependencies]`
      with the benches, or a reason to keep them is recorded.
- [ ] The evaluator crate reaches zero uncovered lines and zero uncovered
      regions. This is [ticket 15](./15-cover-the-evaluator-crate.md) rather
      than this ticket: the shortfall is 33% of the crate and has nothing to do
      with where the benches live.
- [x] Debug workspace build and test green.

## Outcome

The three files moved unchanged except for one line. `evaluate_bench` reads two
transform fixtures, and its `transform_fixtures_dir` now points at the sibling
crate rather than at its own. A sibling path rather than a copy: the transform's
own tests already pin those two files, and a second copy would drift from them
silently. Nothing is compiled across the boundary -- the bench reads the files.

`boa_engine` left the transform's `[dev-dependencies]` with `engine_fold_bench`,
which was its only reader. **`stylex_js` stays, and the reason is recorded in
the manifest**: `tests/utils/ast.rs` reads the language's truthiness table out
of the coercion crate, so the dependency outlives the bench that shared it.

`swc_malloc` stays with the transform too, and did not need to travel. Only
`transform_consumers_bench` names it, and a crate links the allocator only where
a target names it -- none of the three moved benches does.

**The A/B is a before/after across the move, not a comparison with `develop`.**
Criterion baseline identities are per crate, so the two legs cannot be diffed
automatically; both were measured on one machine in one session and paired by
benchmark id. 28 of 28 measurements paired. Median +1.90%, range −2.10% to
+7.71%, 25 of 28 slower.

**No evaluator code changed, so the +1.90% cannot be a code cost.** The diff
settles that, not the numbers: two of the three bench files are byte-identical
to their previous versions, the third differs by one path expression, and
`crates/stylex-evaluator/src` is untouched.

**The shift is real all the same, and it is the link graph.** 25 of 28 slower is
p≈2e-4 under a sign test. A bench target links its own crate's dependency
subgraph, so these binaries used to link `stylex_transform` and twenty crates
behind it and now link the evaluator's subgraph alone. Both crates are `rlib`
only, so ticket 13's `cdylib` hazard cannot recur -- but fat LTO is handed a
different unit, with different inlining and placement.

**So the pre-move criterion series for these three ids is closed, not
continued.** The shipped `.node` still links `rs-compiler → transform →
evaluator`, which is what the before leg resembled; these benches no longer do.
Discard the old baselines rather than diffing them, and read ratios inside a leg
rather than absolute times across the move. That is the price of profiling the
evaluator on its own, which is what the move is for.

An earlier draft of this section argued the move was free because the
`EngineFoldRoundTrip/engine` legs -- which enter no evaluator code -- moved
*more* than the `fold` legs. That reasoning was wrong and is recorded here so it
is not repeated: those legs control for a source change, and a source change was
never the mechanism. The link recomposition reaches them too.

Full numbers, the two outliers and the sample-size caveat:
`../bench/ticket-09.md`.
