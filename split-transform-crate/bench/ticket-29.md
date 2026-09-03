# Ticket 29 — engine fold, before and after

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
