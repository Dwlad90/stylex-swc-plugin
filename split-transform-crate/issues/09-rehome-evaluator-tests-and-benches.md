# 09 — Re-home the evaluator's tests and benches

**What to build:** The evaluation unit tests — roughly 5k lines — and the three
criterion benches that measure evaluation, folding and evaluation depth still
sit with the transform after the core moved. Move them to the crate they now
cover, so the evaluator can be judged and profiled on its own.

Criterion baseline identities are per-crate, so the three moved benches cannot
be diffed automatically against the pre-split baseline. Take a manual
before-and-after on the same machine in the same session and record it beside
the baseline. The remaining four benches stay with the transform and diff
normally. These benches are not gated in CI, so this is a local verification
step.

**Blocked by:** 08 — Move the evaluator core.

**Status:** ready-for-agent

- [ ] The evaluation unit tests live with the evaluator crate.
- [ ] The three evaluation benches are declared as targets on the evaluator crate.
- [ ] The four remaining benches still build and run on the transform.
- [ ] Test assertions, inputs and fixtures are unchanged; only import lines differ.
- [ ] A manual same-machine before/after is recorded for the three moved benches.
- [ ] The evaluator crate reaches zero uncovered lines and zero uncovered regions.
- [ ] Debug workspace build and test green.
