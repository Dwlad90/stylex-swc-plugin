# 16 — Measure the crate-type change, and re-read the earlier floors

**What to build:** Ticket 13 dropped `crate-type = ["cdylib", "rlib"]` from
nineteen crates, leaving only `stylex-rs-compiler` -- which needs one, for the
`.node` Node loads -- with a `cdylib`. The change landed on measured evidence for
one crate and by argument for the other eighteen. Close that gap.

The evidence is in [`../bench/ticket-13.md`](../bench/ticket-13.md). Dropping
the `cdylib` from `stylex-evaluator` alone moved the median of eighteen memo-key
benches from **+6.64%** to **+2.14%**, and took `StructuralKeyDepth` from +6.57%
to +0.14%. A `cdylib` exports its public symbols as preemptible and keeps Cargo
from handing the crate's bitcode to the fat LTO the `bench` and `release`
profiles ask for, so a caller cannot optimize into it. Nothing linked any of the
nineteen.

**Ticket 13's own A/B was never re-run after the change**, so no number in the
repo describes the branch as it now stands. That is the first job.

**The second is the more interesting one.** Ticket 07 measured a **+3.65%** floor
on a bench whose crate was byte-identical between its two legs, and ticket 12 a
**+3.04%** median; both attributed the shift to function placement under fat LTO
and called it a layout floor this effort should expect. Both moves also took code
out of the `rlib`-only transform and into a `cdylib` crate. If that floor was
partly the `cdylib`, then it is not a floor, and two tickets' conclusions want a
footnote.

`stylex-transform` was `rlib` only by accident rather than by intent: a comment
recorded that a `cdylib` broke the link there. That accident is why the effect
only appeared when code left it.

**Method.** Follow `../bench/ticket-13.md`: one target directory per leg, never
shared, criterion baselines copied between them. Three legs answer both
questions:

1. `f57526ebb` as it is -- the ticket-12 commit, `cdylib` everywhere.
2. `f57526ebb` with the nineteen `cdylib`s dropped and nothing else changed.
   Leg 2 against leg 1 is the crate-type change on its own, measured on a tree
   where no code moved.
3. This branch. Against leg 2 that is ticket 13's move with the build held
   constant, which is the attribution ticket 13 could not take.
4. All crates without a `crate-type` (like `crates/stylex-transform/Cargo.toml`
   on branch develop), and `stylex-rs-compiler` is the only crate without changes.

Leg 2 is the one that answers whether ticket 07 and ticket 12 measured layout or
a `cdylib`.

Build time is preferable than performance, because the crates not using
in production, only the `stylex-rs-compiler` crate is used in production,
so the other crates' performance is not relevant. For `stylex-rs-compiler`
performarce is critical.

**Additional check.**

Check the `stylex-rs-compiler` crate's `crate-type` is still needed and if the
value is correct for performance, check with nodejs benchmarks.

**Also worth measuring:** cold build time and artifact size. Nineteen crates
stopped emitting a dynamic library nobody linked, so both should fall, and the
baseline in [`../baseline.md`](../baseline.md) has the numbers to compare.

**One loose end from ticket 13.** `EvaluatePerfFixtures` moved +8.19% before the
`cdylib` change and +7.38% after, so the `cdylib` does not explain it. Its
per-fixture numbers also moved by up to 4 points between two runs of identical
code, so part of it is the group's own spread. Decide from leg 3 whether a real
cost remains there.

**The criterion measurements from ticket 13 are kept**, so leg 1 need not be
re-run unless the machine changed: `bench-13-criterion/parent-leg` holds the
`parent-clean` baselines taken on `f57526ebb`, and `bench-13-criterion/branch-leg`
holds the branch leg. Both sit beside the worktrees, at the same level as
`.bare`. The 11 GB of build output behind them was deleted, so every leg builds
from scratch.

**Blocked by:** None.

**Status:** ready-for-human

- [x] Ticket 13's A/B is re-run against the branch as it stands, and
      `../bench/ticket-13.md` carries the result: **+0.37%** median, 48 of 52
      inside +-4%, so the move costs nothing.
- [x] The crate-type change is measured on its own, on a tree where no code
      moved. Four control builds against three; the bands overlap.
- [x] Tickets 07 and 12 carry a note saying whether their floor was layout, the
      `cdylib`, or both. The answer is neither -- it was the method.
- [x] Cold build time and published artifact size are recorded against
      `../baseline.md`.
- [x] `EvaluatePerfFixtures` is cleared as spread: **+0.69%** once the build is
      held constant, and no effect across seven builds.
- [x] No crate but `stylex-rs-compiler` declares a `cdylib`, and
      `guidelines/STRUCTURE.md` still says why -- rewritten, because the reason
      it gave was not the reason the measurement supports.
- [~] Leg 4, "all crates without a `crate-type`", is measured on build cost and
      artifact size only, and the two are equivalent, so the explicit
      `["rlib"]` line stays. No criterion leg was run for it, because the
      configurations produce the same artifact. Nothing remains unless a later
      ticket wants the criterion run as well.

## Outcome

The ticket asked which of two mechanisms explained an earlier measurement. The
answer is that neither did, and the measurement itself was the artefact: one
build per leg cannot resolve an effect under about 5 points per group, and the
saved `parent-clean` baselines carry a −2% to −3% drift on top.

The crate type that did matter is the addon's own, which no earlier ticket
examined. `["cdylib", "rlib"]` kept fat LTO off the shipped `.node`. Dropping the
unused `rlib` makes all 64 benchmark fixtures faster, by a median of 16%.

Full record: [`../bench/ticket-16.md`](../bench/ticket-16.md).
