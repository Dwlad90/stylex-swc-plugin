# 14 — Deeply nested values abort the process

**What to build:** A value nested past whatever depth the pipeline can handle
produces a StyleX diagnostic naming the file and the declaration, instead of
killing the compiler.

Value normalization recurses once per nesting level with no depth limit. Past
roughly a hundred levels of `calc(calc(…))` it exhausts the stack and the
process aborts with `fatal runtime error: stack overflow`. A stack overflow is
not a panic: it cannot be caught, so the `catch_unwind` the compiler wraps
normalization in does not see it, no diagnostic is emitted, and a build tool
driving the compiler in-process dies with it rather than reporting a bad file.

The exact cliff depends on stack size — the figure above is a 2 MiB test thread;
the main thread survives deeper — which is precisely the problem: the limit is
whatever the host happens to give, so the same source compiles on one machine
and kills the build on another.

Hand-written CSS does not reach these depths. Generated CSS and hostile input
do, and the failure mode is disproportionate either way.

Filed from ticket 04, whose migrated coverage hit it. **Out of the parent
spec's scope** — that spec is about which bytes get emitted, not about recursion
limits — and deliberately not folded into ticket 07, because a depth guard is a
new behaviour rather than a port. Ticket 07 changes which parser recurses, so
the cliff moves; it does not remove it.

**Blocked by:** 07 — Swap normalization onto the ported pipeline, so the guard
is written against the parser that survives.

**Status:** ready-for-agent

- [ ] A value nested past the supported depth is rejected with a StyleX
      diagnostic, not a process abort
- [ ] The diagnostic names the property and the value, like the other
      normalization rejections
- [ ] The supported depth is a stated constant rather than whatever the host's
      stack allows, so the same source compiles the same way everywhere
- [ ] A test asserts rejection at the limit and acceptance just under it
- [ ] `survives_deep_function_nesting` in
      `crates/stylex-css/src/css/tests/value_normalization_parity_test.rs` is
      raised to the new stated depth and its comment about the cliff removed
