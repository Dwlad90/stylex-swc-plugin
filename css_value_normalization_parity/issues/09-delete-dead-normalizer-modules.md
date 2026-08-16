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

**Status:** resolved

- [x] The CSS-AST normalizing visitor and its test file are gone
- [x] The whitespace repair pass, its helpers, and its test files are gone
- [x] Any remaining restoration pass, value-extraction helper, or rule-structure
      helper that no longer has a caller is gone
- [x] Nothing is left behind that is unreferenced but still compiles — an unused
      helper reads as intentional to the next maintainer
- [x] Test count and coverage are compared before and after, and any drop is
      accounted for by a case that genuinely moved rather than one that
      vanished
- [x] The full test suite passes, including the JavaScript suite against a
      rebuilt native artifact
- [x] The harness reports no divergence across the full corpus

## Outcome

Deleted: the normalizing visitor (`normalizers::base`) with `base_normalizer`,
`zero_unit` and `restore_negative_leading_zero`; the whitespace repair pass
(`normalizers::whitespace_normalizer`) with `normalize_spacing`,
`extract_css_value` and `is_css_unit`; the CSS parse and serialize helpers
`swc_parse_css`, `stringify` and `css_codegen_unreachable`; and the
`CLEAN_CSS_VAR` regex, whose only purpose was undoing an escape `stringify`
applied.

The crate's test count goes 1134 → 987. The 148 deleted cases are 46 from the
visitor's test module, 64 and 27 from the whitespace pass's two, and 11 from
the `swc_parse_css` / `stringify` modules in `common_test`. One of the eleven
asserted at the public entry point rather than through `stringify`, and moved
into `normalize_css_property_value_tests` — hence 987 rather than 986.

The parity harness reports 0 divergent over 790 declarations.

`fancy-regex` went too — `stringify`'s `Captures` type was the crate's only use
of it. The `swc_core` CSS features stay: switching them off is ticket 12, and
it is that build succeeding that proves no call site was missed.

The `Spacing repair` glossary entry is deleted rather than reworded. Its
subject no longer exists, and a term defined by its own absence is the entry
`docs/agents/domain.md` says to remove. The migrated cases still carry the
phrase, and `spacing_repair_parity_test.rs` explains it at length in its own
header.

## Review findings, all addressed

A two-axis review raised four open findings after the deletion commit. Each was
fixed and committed on its own.

**`normalizer` where the glossary says `pass`** (`c0f2f9c`). Three comments on
the structural guard used the narrower term for the ordered list as a whole,
including for two members that reject and never rewrite.

**Caller-less value-parser wrappers** (`4026937`). `format_ident`,
`_format_quoted_string` and `join_css` in `values/parser.rs` were one-line
delegations to `stylex_css_parser` whose only callers were tests — the same
assertions running twice against one implementation. Deleted, with the seven
cases the copies had and the originals lacked moved to the owning crate; four
of those stop settling for `is_empty()` and name the string they expect.
`parse_css` stays, it has a real caller. stylex-css 987 → 978,
stylex-css-parser 2186 → 2188.

**Dead `swc_core` dependency** (`895feb7`). Nothing under `src/` or `benches/`
named it any more, yet the crate pulled nine features of it. Removed entirely —
which is the check the deleted `swc_parse_css` comment described, and it holds:
the crate builds and all 978 tests pass with no swc at all. The workspace sweep
(`stylex-css-parser`, `stylex-transform`, `stylex-rs-compiler` still enable the
CSS features) remains ticket 12's.

**Stale harvester and corpus** (`f4dd6fb`). Resolved rather than deferred — see
below.

## Corpus: re-harvested, nothing lost

The dead property-agnostic shape is gone from `harvest.ts`, and the corpus is
re-harvested: 746 declarations to 674. Of the 107 pairs that lost their source,
86 are already present under a real property name; the 21 that would have
vanished moved to `edge.json` by hand, so **no value the corpus held before is
missing from it now** (verified pairwise across all three corpus files). Three
property names were corrected on the way in — the harvester had read them off
minified expected-output strings, so `boxShadow` and `transitionProperty` had
entered as lowercase spellings no author writes.

Seven of the rescued values are declaration lists: the injection shape this
compiler rejects and the reference compiler emits verbatim. They carry a note
saying so and sit beside the hand-written case that already pinned it. That is
what moves `acceptance divergent` 26 to 31. **Value normalization is unchanged
at 0 divergent.**

The re-harvest also gained 35 cases added in tickets 07 and 08 that had never
been picked up, and five of the rescued 21 traced to `validator_test.rs`,
deleted in ticket 08 — so the corpus had been stale before this ticket touched
it.

## Left for ticket 12

The CSS features on `stylex-css-parser`, `stylex-transform` and
`stylex-rs-compiler`. `stylex-css` no longer has an swc dependency at all,
which is one crate's worth of that proof already in hand.
