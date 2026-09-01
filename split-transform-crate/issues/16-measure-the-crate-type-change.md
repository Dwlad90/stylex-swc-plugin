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

Leg 2 is the one that answers whether ticket 07 and ticket 12 measured layout or
a `cdylib`.

**Also worth measuring:** cold build time and artifact size. Nineteen crates
stopped emitting a dynamic library nobody linked, so both should fall, and the
baseline in [`../baseline.md`](../baseline.md) has the numbers to compare.

**One loose end from ticket 13.** `EvaluatePerfFixtures` moved +8.19% before the
`cdylib` change and +7.38% after, so the `cdylib` does not explain it. Its
per-fixture numbers also moved by up to 4 points between two runs of identical
code, so part of it is the group's own spread. Decide from leg 3 whether a real
cost remains there.

**Blocked by:** None.

**Status:** ready-for-agent

- [ ] Ticket 13's A/B is re-run against the branch as it stands, and
      `../bench/ticket-13.md` carries the result.
- [ ] The crate-type change is measured on its own, on a tree where no code
      moved.
- [ ] Tickets 07 and 12 carry a note saying whether their floor was layout, the
      `cdylib`, or both.
- [ ] Cold build time and published artifact size are recorded against
      `../baseline.md`.
- [ ] `EvaluatePerfFixtures` is either cleared as spread or filed as a cost.
- [ ] No crate but `stylex-rs-compiler` declares a `cdylib`, and
      `guidelines/STRUCTURE.md` still says why.
