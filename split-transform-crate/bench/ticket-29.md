# Ticket 29 — engine fold, before and after

> **Measured before the allocator change.** Every number in this record was
> taken when a bench measured the system allocator. Commit `aa2925f06` made
> every bench link `swc_malloc`, which is the allocator the published addon
> runs. Numbers here do not compare with numbers taken after that commit.
> The re-baseline is [`ticket-17.md`](./ticket-17.md); compare against that
> one. See "The allocator a bench measures" in
> `guidelines/PERFORMANCE.md`.

Bench: `cargo bench -p stylex_evaluator --bench engine_fold_bench`, one build
per leg, same machine, back to back. Baseline log
[`ticket-29-baseline.log`](./ticket-29-baseline.log), result log
[`ticket-29-after.log`](./ticket-29-after.log).

Ten measurements. The deltas run from −1.5% to +1.3%, and three of the ten
cross the p = 0.05 line in opposite directions. That band is the same one a
control leg produces with no change at all, so the numbers show no movement
either way.

That agrees with what the change does. It replaces three indirect calls
through a vtable with three direct calls, and removes a downcast from each
one. Nothing on the fold path gained work, so a regression has no mechanism.
