# 05 — Extract the diagnostics crate

**What to build:** Building a code frame for an error, and finding the
declaration span an error should point at, are presentation concerns that
currently sit inside the transform and are reachable from anywhere in it. Give
them their own crate so error presentation can evolve independently of what
raises the error.

The diagnostics code reads exactly nine methods off the **state manager** —
filename, seen-module source get and set, cached span get and set, index access,
and the three framed-declaration methods. None of the nine has a counterpart, so
there is no parity risk in reshaping how they are reached. Declare them as a
trait owned by the diagnostics crate and implement it on the state manager.

This is established practice here: `stylex-atoms` already takes its compilation
utilities through an injected trait precisely to avoid depending on the
transform, which would be a cycle. The trait is consulted at a diagnostic site,
never on the evaluation path, so the indirection costs nothing measurable.

**Blocked by:** 03 — Move the three exported macros; 04 — Extract the state-index crate.

**Status:** ready-for-agent

- [ ] Code-frame building and declaration-span lookup live in the new crate.
- [ ] The crate reads state through its own trait and never names the state manager.
- [ ] The state manager implements that trait; its method surface is unchanged.
- [ ] Every consumer of the old module reaches the new crate directly, with no facade left behind.
- [ ] The unit tests covering the moved code move with it.
- [ ] The crate reaches zero uncovered lines and zero uncovered regions.
- [ ] The crate has a `CONTEXT.md` and a context-map row.
- [ ] The crate is placed in the documented layer list.
- [ ] Error output is byte-identical to the baseline for every diagnostic the suite exercises.
- [ ] Benches diffed against the baseline; no regression outside noise.
