# 07 — Swap normalization onto the ported pipeline

**What to build:** The change the whole effort exists for. A developer writing
`transition: 'opacity 0.2s ease-in-out'`, `calc(-1 * var(--spacing))`,
`color: '#ffffff'`, or `gridTemplateAreas: "'sidebar content'"` gets the exact
declaration text — and therefore the exact class name — that
`@stylexjs/babel-plugin` produces.

Value normalization stops parsing into a CSS stylesheet and serializing through
a minifier. It parses with the ported value parser from ticket 03, folds the
nine normalizers from ticket 06 over the token list, and serializes back.
Everything no normalizer touches survives byte for byte, so the four live
divergences in the parent issue close by construction, and the two already
fixed stay fixed for the same reason rather than by the restoration passes that
currently hold them up.

Deleted in this ticket: the stylesheet-based serializer, the synthetic rule
wrapper built to feed the parser, the generic property name used to dodge
property-specific grammar, the parse-error handler, every restoration pass that
existed to undo minification, the allowlist that routed color functions around
the parser, and the fallback that preserved values the parser could not
understand. The last two become dead automatically: there is no longer a second
serialization path to route around, and the ported parser never rejects
anything.

Kept: the guard rejecting values containing characters that could terminate the
generated rule, and the unclosed-comment error. Both are plain string scans with
no parser dependency, so the rewrite does not touch them.

The CSS parse survives this ticket **only** to feed custom-property validation,
so that this change lands green on its own. Ticket 08 removes its last consumer.

The public entry point keeps its name, signature, crate and callers. The rewrite
is entirely internal — which is what makes the migrated coverage from tickets 04
and 05 valid across it.

**Blocked by:** 03 — Vendored value parser port; 04 — Migrate normalizing-visitor
coverage to the public seam; 05 — Migrate spacing-repair coverage to the public
seam; 06 — Port the nine value normalizers.

**Status:** ready-for-agent

- [ ] Whitespace between value tokens is preserved: a transition keeps the space
      before its duration, a background position keeps the space after a
      percentage, a two-component translate stays two components, and a
      multi-value background position and an outline shorthand keep their spaces
      after leading-zero stripping
- [ ] Spacing around multiplication in math functions is preserved exactly where
      the author put it, in both the negative-multiplier and the
      viewport-unit-multiplier forms, and inside a nested function within a
      shorthand — no space is moved to the other side of the operator
- [ ] Six-digit hex colors are preserved, both standalone and inside a gradient
- [ ] Single-quoted strings keep their quote character
- [ ] Transform function name capitalization is preserved, without any
      restoration pass
- [ ] Large numbers keep their plain decimal spelling rather than exponent
      notation, without any restoration pass
- [ ] Every expectation changed in tickets 04 and 05 was one the harness
      predicted would change; any unpredicted change is treated as a defect in
      this ticket, not as an expectation to update
- [ ] Values using syntax newer than the compiler's knowledge, and relative
      color syntax, normalize and emit rather than being rejected — with no
      allowlist and no fallback path
- [ ] Exactly one code path serves every value; no bypass remains that could
      shadow it
- [ ] No code remains whose purpose is to reverse an earlier step's
      canonicalization. This is the review signal for the whole ticket: any such
      code means the port has drifted back toward the old design
- [ ] Degenerate input fails the way upstream fails, reported with a descriptive
      local message rather than an imitation of a foreign runtime error string;
      whether such input can reach normalization at all is confirmed via the
      harness first
- [ ] The full test suite is run, including the JavaScript suite against a
      rebuilt native artifact — this is the first commit at which that suite is
      meaningful
- [ ] The harness reports no divergence across the full corpus
