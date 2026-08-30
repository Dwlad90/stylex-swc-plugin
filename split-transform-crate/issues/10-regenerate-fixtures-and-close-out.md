# 10 — Regenerate fixtures, renumber the layer list, and close out

**What to build:** Settle everything that can only be settled once all the moves
have landed, and prove the work achieved what it set out to.

Two generated artifacts may be stale. Snapshot files are keyed by the path of
the test that produced them, so any test that moved may need its snapshots
regenerated — mechanically, with no source change alongside. Separately, a
generation chain crosses crates: Rust test sources feed a harvested parity
corpus in the compiler package, which generates a committed fixture in the value
parser, and that package's pre-test step checks it. Moving Rust test files
therefore invalidates a fixture in a crate this work never touched. Regenerate
it; never hand-edit it.

Then renumber the documented layer list end to end — the new crates shift the
numbering, and it can only be made consistent once they all exist — and record
the closing measurements against the baseline.

**Blocked by:** 09 — Re-home the evaluator's tests and benches.

**Status:** ready-for-agent

- [ ] Any stale snapshots are regenerated, in a change containing no source edits.
- [ ] The generated value-parser fixture is regenerated through its generator and the pre-test check passes.
- [ ] The documented layer list is renumbered and every crate appears exactly once.
- [ ] The context map lists every new crate and no stale row remains.
- [ ] No artifact anywhere asserts a porting or mirroring relationship with another implementation.
- [ ] The full workspace suite is green in debug.
- [ ] The compiler addon is rebuilt and the JavaScript suite passes against it.
- [ ] The public entry points the compiler consumes are confirmed unchanged.
- [ ] Coverage passes with the exclusion list confirmed shrunk; the remaining exclusion is the transform alone.
- [ ] A before/after table is recorded: largest crate size, excluded-from-coverage lines, cold build, incremental check after touching the state manager.
