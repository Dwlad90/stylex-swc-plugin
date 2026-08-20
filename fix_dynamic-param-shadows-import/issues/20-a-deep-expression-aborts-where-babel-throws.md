# 20 — A deep expression aborts the process where Babel throws

Status: `needs-triage`
Blocked by: None

**What was measured:** The evaluator walks a nested expression recursively, so
its limit is stack depth rather than a checked bound. Around a shadowing dynamic
parameter, `(((zIndex + 1) + 1) …)`:

| depth | Babel 0.19.0 | rs-compiler |
| --- | --- | --- |
| 256 | accepts, one custom property | accepts, one custom property |
| 384, 512, 768 | accepts | **512 aborts: `has overflowed its stack` / `fatal runtime error: stack overflow, aborting`** |
| 1024 | `RangeError: Maximum call stack size exceeded` | — |

Two differences, and the second is the one that matters:

1. Our ceiling is lower — somewhere in (256, 512] against Babel's in (768, 1024].
2. Our failure mode is a **process abort**, not a catchable error. Babel raises a
   `RangeError` a caller can catch and report against a file and a line. A
   `SIGABRT` gives a bundler nothing: no diagnostic, no file, and no chance to
   continue with the rest of the build. Every other refusal in this compiler is
   a panic carrying a StyleX message; this one is not a refusal at all.

Nesting this deep is not something a person writes, but it is something codegen
produces, and the failure is indistinguishable from a compiler crash.

**Why it is not part of ticket 10.** Found while measuring the boundary that
ticket's edge coverage asked for, and recorded there at
`two_hundred_and_fifty_six_levels_of_arithmetic_around_a_shadowing_param` — which
asserts the agreeing depth and stops, because a test that crosses the boundary
aborts the test binary rather than failing. Fixing it means giving the evaluator
a depth budget that refuses before the stack runs out, which is a change to the
evaluator's contract and wants its own decision.

**Not specific to the shadowing.** Measured through a shadowed parameter because
that is what was under test, but nothing in the mechanism depends on it — an
equally deep expression over a module-level `const` should abort the same way.
Worth confirming as the first step, since it decides whether this belongs to
this feature's tracker or the evaluator's.

- [ ] Confirm the abort reproduces without any shadowing
- [ ] Decide whether a depth budget refuses with a message, or the recursion
      becomes an explicit stack
- [ ] Pin the boundary as a test that can survive being wrong
