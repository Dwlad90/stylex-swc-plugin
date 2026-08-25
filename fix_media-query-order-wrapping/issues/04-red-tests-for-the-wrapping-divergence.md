# 04 — The wrapping divergence, asserted red at all three seams

**What to build:** Three failing tests that describe what an author should get
for the reported input — a ladder of exclusive minimum and maximum width
queries ending in a maximum-width-only rung, over a variable defined in a
separate module. Every expectation is taken from a run of the reference
implementation, not written by hand.

The three seams, deliberately all of them: the transform's own unit seam, which
asserts the rewritten query keys without invoking the compiler; the end-to-end
seam over the style-creation call, which asserts emitted CSS text and class
names, so that a rehash reads as a query-string diff rather than an opaque
class-name change; and the parity corpus consulted by the parity harness, which
is the only seam that can fail when the reference implementation changes rather
than when this compiler does.

Commit these red. The history has to show the divergence failing before the
change that fixes it, because the claim ticket 05 rests on is that one deletion
is its sole cause.

**Blocked by:** 02.

**Status:** done

- [x] The reported input is asserted at the transform unit seam, the end-to-end
      seam, and the parity corpus.
- [x] Every expectation is quoted from ticket 02's reference run, and the table
      row it came from is identified.
- [x] All three fail before any production change, and the failure output shows
      the retained contradictory branches missing from our text.
- [x] The end-to-end expectations carry emitted CSS text, not class names alone.
- [x] The parity corpus subject needs no change to the harness — evidence: the
      harness runs it unmodified.
