# 12 — Drop the swc CSS features

**What to build:** Remove CSS parsing, code generation, AST and visitor support
from the compiler's dependency configuration. After tickets 07 through 09,
nothing in the workspace uses any of them — the whole CSS half of that
dependency is compiled and linked for no reason.

This is the last ticket for a reason beyond tidiness: it is the **proof the
migration is complete**. If the workspace builds and every suite passes with CSS
support switched off, then nothing is quietly still on the old path. There is no
other check that gives that guarantee — a stray call site would otherwise sit
there compiling happily and reading as intentional.

Splitting this to a follow-up was considered and rejected. A dependency that is
declared but unused is indistinguishable, to the next reader, from one that is
load-bearing.

The dependency itself stays — the compiler still needs it for JavaScript
parsing. Only the CSS feature set comes off.

**Blocked by:** 08 — Move custom-property validation onto the value AST; 09 —
Delete the dead normalizer modules.

**Status:** ready-for-agent

- [ ] The CSS parsing, code generation, AST and visitor features are removed
      from the crate's dependency configuration
- [ ] Any other crate in the workspace that enabled them for this pipeline has
      them removed too
- [ ] The workspace builds clean with them gone — this is the acceptance signal
      for the whole effort
- [ ] The full test suite passes, including the JavaScript suite against a
      rebuilt native artifact
- [ ] The JavaScript-parsing capability of the same dependency is untouched
- [ ] Build time and artifact size are compared before and after, and recorded
      on this ticket alongside the ticket 11 numbers
- [ ] The harness reports no divergence across the full corpus
