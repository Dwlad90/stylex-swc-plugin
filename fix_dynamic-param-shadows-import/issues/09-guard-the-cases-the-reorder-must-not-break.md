# 09 — Guard the cases the reorder must not break

Status: `ready-for-agent`
Blocked by: 02, 03, 05, 06, 07, 08

**What to build:** The agreeing half of the audit, recorded as corpus guards, so
the next change to the identifier chain reports a regression as a *changed
verdict* rather than as silence.

Every ticket before this one carries the corpus entry for the case it fixes.
What none of them owns is the set of inputs that already agree with the
reference implementation and only needed to keep agreeing across a reordered
chain:

- a dynamic style parameter shadowing a module-level `const`
- a parameter named `firstThatWorks`
- a reference bound to a function declaration, and to a class declaration
- member mutation of a `const` read in a style value
- both passing shapes from 01: the dynamic style alone, and with an unrelated
  static prop

Each records the verdict it is known to read, which is what turns a future
regression into a reported change rather than a quiet one. These are guards, not
demonstrations — none of them is expected to change, and any that does is the
finding.

- [ ] Every case above is a corpus entry with the verdict it is known to read
- [ ] The parity run is clean end to end, no unexplained verdict
- [ ] Any case that turns out *not* to agree is recorded here rather than
      quietly given an expected-divergence verdict
