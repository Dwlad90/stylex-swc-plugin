# 09 — Delete the dead normalizer modules

**What to build:** Nothing new. This ticket removes the old normalization
machinery now that nothing calls it: the CSS-AST normalizing visitor, the
hand-rolled whitespace repair pass, the helpers that existed only to serve them,
and the test files bound to all of the above.

Held back from ticket 07 deliberately. Keeping the dead code alive across the
swap means the migrated coverage in tickets 04 and 05 gets to prove itself
against a working pipeline before its predecessor is destroyed. Deleting it in
the same commit as the swap would remove the fallback at the exact moment it
might be needed.

By the time this runs, every input those tests covered is already asserted at
the public normalization entry point, so this is pure subtraction with no
coverage loss.

**Blocked by:** 07 — Swap normalization onto the ported pipeline; 08 — Move
custom-property validation onto the value AST.

**Status:** ready-for-agent

- [ ] The CSS-AST normalizing visitor and its test file are gone
- [ ] The whitespace repair pass, its helpers, and its test files are gone
- [ ] Any remaining restoration pass, value-extraction helper, or rule-structure
      helper that no longer has a caller is gone
- [ ] Nothing is left behind that is unreferenced but still compiles — an unused
      helper reads as intentional to the next maintainer
- [ ] Test count and coverage are compared before and after, and any drop is
      accounted for by a case that genuinely moved rather than one that
      vanished
- [ ] The full test suite passes, including the JavaScript suite against a
      rebuilt native artifact
- [ ] The harness reports no divergence across the full corpus
