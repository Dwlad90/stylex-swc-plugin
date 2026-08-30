# 03 — Move the three exported macros to the macro crate

**What to build:** The transform crate exports three macros whose bodies name
hard-coded paths rooted at the defining crate, and those paths point at three
different future destinations: one into the diagnostics code, one into the
evaluator, and one into *both* the AST convertors (which stay) and the evaluator
(which leaves) inside a single expansion.

Because an exported macro publishes at its defining crate's root, leaving them
in place would force the evaluator crate to depend back on the transform — a
cycle Cargo rejects outright. The first breakage lands at the diagnostics
extraction, not the evaluator one, so this must be cleared before either.

Move all three to the macro crate, which the context map already describes as
the error and panic vocabulary every crate raises failures through. That crate
sits at a low layer and cannot name types in the upper layers, so each macro
takes the function it calls **as a parameter** — macros expand at the call site,
so the caller supplies the path. Same injection principle as everywhere else in
this work.

These macros are Rust-only machinery with no counterpart, so the parity
constraint permits changing their shape. This is the one ticket in the sequence
where code shape genuinely changes rather than merely moving, which is why it is
isolated: a failure here stops the work early and cheaply.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] All three macros live in the macro crate.
- [ ] No macro body names a path in a layer above the macro crate.
- [ ] Each call site supplies the function the macro invokes.
- [ ] The macro crate gains no new dependency.
- [ ] Behaviour at every expansion site is identical — the emitted code is the same.
- [ ] Test files change import lines only; no assertion, input or fixture is touched.
- [ ] Debug workspace build and test green.
- [ ] Coverage gate still passes.
