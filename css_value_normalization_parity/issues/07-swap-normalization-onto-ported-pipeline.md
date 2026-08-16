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

**Status:** resolved

- [x] Whitespace between value tokens is preserved: a transition keeps the space
      before its duration, a background position keeps the space after a
      percentage, a two-component translate stays two components, and a
      multi-value background position and an outline shorthand keep their spaces
      after leading-zero stripping
- [x] Spacing around multiplication in math functions is preserved exactly where
      the author put it, in both the negative-multiplier and the
      viewport-unit-multiplier forms, and inside a nested function within a
      shorthand — no space is moved to the other side of the operator
- [x] Six-digit hex colors are preserved, both standalone and inside a gradient
- [x] Single-quoted strings keep their quote character
- [x] Transform function name capitalization is preserved, without any
      restoration pass
- [x] Large numbers keep their plain decimal spelling rather than exponent
      notation, without any restoration pass
- [x] Every expectation changed in tickets 04 and 05 was one the harness
      predicted would change; any unpredicted change is treated as a defect in
      this ticket, not as an expectation to update
- [x] Values using syntax newer than the compiler's knowledge, and relative
      color syntax, normalize and emit rather than being rejected — with no
      allowlist and no fallback path
- [x] Exactly one code path serves every value; no bypass remains that could
      shadow it
- [x] No code remains whose purpose is to reverse an earlier step's
      canonicalization. This is the review signal for the whole ticket: any such
      code means the port has drifted back toward the old design
- [x] Degenerate input fails the way upstream fails, reported with a descriptive
      local message rather than an imitation of a foreign runtime error string;
      whether such input can reach normalization at all is confirmed via the
      harness first
- [x] The full test suite is run, including the JavaScript suite against a
      rebuilt native artifact — this is the first commit at which that suite is
      meaningful
- [x] The harness reports no divergence across the full corpus

## Answer

`normalize_css_property_value` is now two structural guards and a call to
`normalize_value`. The stylesheet parse survives only to feed custom-property
validation, which ticket 08 moves; nothing else in the function reads it.

**Harness, full corpus (781 declarations), `@stylexjs/babel-plugin` v0.19.0:**

| Verdict | Baseline | Now |
| --- | --- | --- |
| identical | 432 | 720 |
| **divergent** | **97** | **0** |
| structurally divergent | 15 | 15 |
| acceptance divergent | 12 | 25 |
| both reject | 14 | 21 |

The corpus grew from 570 to 781 as tickets 04, 05 and this one added test
sources for it to harvest, so only the divergent row compares directly.
**Nothing that reported `identical` at baseline reports anything else now** —
checked entry by entry against `baseline.json`, which is the assertion the
"unpredicted change is a defect" checklist item asks for.

All 97 divergences closed, and every one of them closed by construction: 97
`diverges` cases across the two parity modules were flipped onto the reference
spelling they already carried, and the expectation that had been recorded as
"what this compiler does today" is now the reference compiler's.

### Decisions this ticket had to make

**A `;` with a declaration behind it is rejected; a trailing one is not.** The
harness flagged the silent truncation at a `;` as something to decide on rather
than inherit. Eight corpus entries of the `color: 'red; margin: 10px'` shape
moved from `divergent` (only the first declaration survived extraction, with no
diagnostic) to a rejection, which is the whole reason the rule-breaking guard
exists.

Applying the guard to a *trailing* `;` was a different matter, and got caught by
the project's own large fixture, which carries 23 of them. That semicolon closes
its own declaration and opens nothing — a browser reads it the same way, and the
harness confirms the reference compiler emits it verbatim. So the guard now asks
whether anything follows the `;` that a second declaration could start with.
`var(--web-wash);`, `red ; ` and `red;;` all normalize and emit; `red; margin:
10px` and `red; /* x */` are rejected.

**A `url()` body is stepped over whole by the structural scan.** The scan used to
be reached only by the two bypass paths; now every value passes it, and it was
reading url bodies as CSS. `url(data:image/svg+xml;utf8,<svg/>)` — an entry the
suite already pinned as accepted — was rejected for a `;` no CSS parser will ever
see. The scan now mirrors the value parser: an unquoted url body runs to its
first unescaped `)`, and a body that is never closed swallows the rest of the
value. That last part is not tidiness — `url(a;b` carries no rule-breaking `;`,
only an unfinished url, and stopping short would report it as a rule terminator
instead of as the unclosed function the normalizers own. The change also moved
`url(it's-fine.png)` and `url(a/*b.png)` from rejected to `identical`.

The fixture's own 23 trailing semicolons now compile untouched, which is better
evidence than editing them would have been.

The scan's `url` name match is **case-sensitive**, which is not a typo and is
the one part of this worth reading twice. CSS function names are
case-insensitive and the first version of this scan matched them that way — but
the value parser compares this one name literally, so to the parser `URL(a}b)`
is an ordinary function and the `}` is an ordinary token that reaches the
declaration intact. The case-insensitive scan stepped over that body and waved
the value through, emitting a bare `}` into the stylesheet: the exact injection
the guard exists to stop. A scan that shortcuts a parser has to answer the
question the parser will actually be asked, not the question the language would
ask. `steps_over_only_the_bodies_the_parser_takes_whole` pins the agreement in
both directions, including the parity-of-backslashes rule that decides where a
body ends.

**A blank value still leaves the property undeclared.** Normalization rejects a
value that scans to no tokens, faithfully to upstream, which crashes on the same
input. Whether that leaves the property undeclared or fails the build is a
decision above normalization, and this compiler has always left it undeclared —
so `transform_value` answers it before normalization is reached, rather than
normalization carrying an exception. Making `color: ''` a hard compile error is
not in this ticket's scope.

### Also closed

Two tickets filed against defects in the old pipeline are fixed here as a
consequence, since neither defect has anywhere left to happen. Confirm and close
them rather than working them:

- **13 — escapes resolved inside strings.** `"\2014 A"` keeps the author's
  spelling.
- **15 — escaped quote breaks out of the rule.** `"a\"b#c"` keeps its quotes,
  so no manufactured `}` reaches the declaration.

### Left for ticket 09

`normalizers/base.rs` and `normalizers/whitespace_normalizer.rs` still compile,
called only by their own tests, under a module-level `#![allow(dead_code)]` and
a comment saying why. That is this ticket's deliberate instruction — the
migrated coverage proves itself against the new pipeline before its predecessor
is destroyed. Every restoration pass that lived in `css/common.rs` is gone,
which is the review signal the checklist names.

The `diverges` case constructor and the `Reference` enum are gone from
`tests/support.rs`: nothing could construct them once the corpus stopped
disagreeing, and an empty vocabulary reads as a claim that nothing was checked.
A future divergence is a defect the harness reports, not a spelling to enshrine.

The harvester no longer feeds selector or at-rule keys to the comparison. A
`:hover` key opens a nested block rather than naming a property, and asking both
compilers what `:hover: {color:red}` means produced a permanent one-entry
divergence that no change to normalization could ever close. Worth noting for a
reader comparing reports: this shrinks by one the corpus the "no divergence"
claim is measured over, so the oracle moved in the same change as the code.
