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
limits.

**Blocked by:** None. The guard turned out not to need the ported parser: it
reads the value rather than parsing it, so it sits in front of every path and
is unaffected by which parser runs behind it.

**Status:** resolved

- [x] A value nested past the supported depth is rejected with a StyleX
      diagnostic, not a process abort
- [x] The diagnostic names the property and the value, like the other
      normalization rejections
- [x] The supported depth is a stated constant rather than whatever the host's
      stack allows, so the same source compiles the same way everywhere
- [x] A test asserts rejection at the limit and acceptance just under it
- [x] `survives_deep_function_nesting` in
      `crates/stylex-css/src/css/tests/value_normalization_parity_test.rs` is
      raised to the new stated depth and its comment about the cliff removed

## Answer

`MAX_VALUE_NESTING_DEPTH = 64` in `crates/stylex-css/src/css/common.rs`, checked
in `normalize_css_property_value` alongside the unclosed-construct guards.

**Where the depth comes from.** `scan_value_structure` already walked the value
once and already tracked paren depth, to answer whether a function was left
unclosed. It now also records the peak, so the guard costs nothing beyond a
`max` per `(` — no second pass, no new scanner.

**Why the guard is not blocked on ticket 07.** The ticket assumed a depth limit
had to be written against whichever parser recurses. It does not: the check
reads the source text and rejects before anything recursive is entered. A test
at 5000 levels — well past the depth that used to abort — returns a diagnostic
rather than dying, which is the proof that no recursion happens first. Ticket 07
can replace the parser underneath without touching this.

**Why 64.** Below the observed cliff with room to spare (a 2 MiB thread, the
smallest in play, survived past a hundred levels in a debug build, which uses
more stack per frame than release), and far above real CSS — the deepest value
in the project's own corpus nests eight. Stating it as a constant is the point:
the depth a value may reach is now a property of the compiler rather than of
which thread it happens to run on.

**The guard fires before the branch.** Placed ahead of the colour-function
allowlist, so the limit does not depend on which path a value takes. Covered by
`rejects_deep_nesting_on_the_colour_function_path`.

**Verified end to end**, not just at the seam: `edge-nesting-past-the-depth-limit`
in the parity corpus reports `acceptance divergent` with the full diagnostic,
where the run previously took the process down. Upstream has no limit and
accepts the value, which is the divergence being accepted deliberately — the
alternative is an abort with no diagnostic at all.
