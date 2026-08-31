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

Two notes on the baseline. The `pre-split` criterion baseline no longer exists,
so bench comparisons are A/B runs against `develop` rather than diffs against a
saved baseline. And the incremental-check measurement in
[`../baseline.md`](../baseline.md) is *"add an item to `state_manager.rs`"* --
that file now lives in `crates/stylex-state/`, not in the transform. The
measurement stays valid and is the one the split most directly aims at, so keep
taking it; just do not go looking for the file where the baseline recorded it.

**Blocked by:** 09 — Re-home the evaluator's tests and benches.

**Status:** ready-for-agent

- [ ] Any stale snapshots are regenerated, in a change containing no source edits.
- [ ] The generated value-parser fixture is regenerated through its generator and the pre-test check passes.
- [ ] The documented layer list is renumbered and every crate appears exactly
      once, including the two the spec did not plan: `stylex-state` and
      `stylex-declarations`.
- [ ] The context map lists every new crate and no stale row remains.
- [ ] No artifact anywhere asserts a porting or mirroring relationship with another implementation.
- [ ] The full workspace suite is green in debug.
- [ ] The compiler addon is rebuilt and the JavaScript suite passes against it.
- [ ] The public entry points the compiler consumes are confirmed unchanged.
- [ ] Coverage passes, and every crate still on an exclusion list has a reason
      recorded. This will not be the transform alone: `stylex-state` is excluded
      in all three lists and stays there until
      [ticket 11](./11-cover-the-state-crate.md) comes off the backlog. Say so
      in the close-out rather than leaving the list looking accidental.
- [ ] A before/after table is recorded: largest crate size, excluded-from-coverage lines, cold build, incremental check after touching the state manager.
