# 08 — Move custom-property validation onto the value AST

**What to build:** A developer who mistypes a custom property reference — naming
it without the leading double hyphen — still gets the compile-time error they
get today, instead of a declaration that silently resolves to nothing at
runtime.

The rule itself does not change: a custom-property reference whose first
argument does not begin with a double hyphen is rejected. What changes is where
it reads from. Today it walks a CSS stylesheet, and that walk is the only
surviving reason the compiler parses CSS at all after ticket 07. The token list
answers the same question directly and more cheaply — a function token with the
reference's name whose first word child lacks the prefix.

Once this lands, nothing in the workspace consumes the CSS parse, and ticket 12
can remove the dependency.

Worth recording for whoever picks this up: this rule has **no upstream
equivalent**. The reference compiler accepts a malformed custom-property
reference without complaint. It is a deliberate local addition, knowingly
retained, because it changes only which programs are rejected and never the
bytes of an accepted program — so it cannot affect class-name parity — and it
catches a mistake that otherwise fails silently in a browser.

**Blocked by:** 07 — Swap normalization onto the ported pipeline.

**Status:** ready-for-agent

- [ ] A malformed custom-property reference is rejected with the same error and
      message as before this change
- [ ] A correctly prefixed reference is accepted, including inside nested
      functions and alongside a fallback argument
- [ ] The check runs off the token list, not off a CSS stylesheet
- [ ] The CSS parse has no remaining consumers anywhere in the workspace
- [ ] Existing tests for this rule pass unchanged, or are re-expressed at the
      public entry point where they referenced the stylesheet type
- [ ] The harness reports no divergence across the full corpus
