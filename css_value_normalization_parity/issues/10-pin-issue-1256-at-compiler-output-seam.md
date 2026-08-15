# 10 — Pin issue #1256 at the compiler-output seam

**What to build:** Permanent regression tests for all six divergences reported
in the parent issue, asserted where the contract actually lives.

The other tests in this effort assert normalized declaration text at the value
normalization entry point. That is the right seam for the bulk of coverage, but
it cannot see the thing the parent issue is actually about: the **class name**.
The class name is a hash of the canonical declaration, and it is the
compatibility contract between this compiler and the reference one. A test that
checks the declaration text but not the hash would miss a defect in hashing
itself.

So these six go through the full transform and assert class names and rule text
from the emitted style metadata, matching the reference compiler exactly.

All six are pinned, including the two that were already fixed before this effort
began. Those two were closed by restoration passes that ticket 07 deletes; if
the port regresses them, that must fail loudly here.

**Blocked by:** 07 — Swap normalization onto the ported pipeline.

**Status:** ready-for-agent

- [ ] All six reported cases are covered, each asserting both class name and
      rule text from the transform's style metadata
- [ ] Expected class names and rule text come from the reference compiler via
      the harness, never hand-written
- [ ] The whitespace case covers all six of its sub-inputs, including the
      gradient with percentage color stops
- [ ] The math-function spacing case covers all three of its sub-inputs
- [ ] The hex case covers both the standalone colors and the one inside a
      gradient
- [ ] The already-fixed cases — transform function capitalization and plain
      decimal spelling of large numbers — are pinned alongside the rest, so a
      regression introduced by deleting their restoration passes cannot pass
      silently
- [ ] Tests are placed with the existing value normalization transform coverage,
      following its established shape rather than introducing a new one
- [ ] Each test names the reported case it pins, so a future failure is
      traceable to the report without archaeology
