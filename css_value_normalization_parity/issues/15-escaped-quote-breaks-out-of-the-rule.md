# 15 — An escaped quote inside a string breaks out of the rule

**What to build:** A value containing an escaped quote —
`fontFamily: '"a\\"b"'`, `backgroundImage: 'url("a\\")b.png")'` — normalizes to
a declaration that stays inside its own rule, instead of one carrying the
generated rule's closing brace.

Today it does not. Three things go wrong in sequence:

1. SWC's minifying codegen emits a double-quoted string containing an escaped
   quote **without its quotes**: `font-family: "a\"b#c"` comes back as
   `*{font-family:a"b#c}`. What was a string is now a bare token sequence with
   an unbalanced `"` in it.
2. Value extraction reads that as an unterminated string, so the `}` that ends
   the generated rule is not recognised as a terminator and the extraction runs
   to the end of the input.
3. The declaration this compiler returns is therefore `a"b#c}` — a value
   carrying a `}`.

A `}` in a value is precisely what the structural guard in
`normalize_css_property_value` exists to reject: without it,
`height: "1px solid } color: red"` would escape its own declaration and inject
arbitrary CSS. The guard is not wrong, it is simply in the wrong place for this
input — it reads the author's value, which is well-formed, and this `}` is
manufactured two stages downstream of it.

The single-quoted spelling fails differently and more quietly: `'a\'b#c'`
normalizes to `"ab#c"`, with the escaped quote silently deleted. No brace
escapes, but a character the author wrote is gone from both the CSS and the
hash.

`@stylexjs/babel-plugin` returns all three values as written, so every one of
them is also a class-name divergence.

Surfaced by ticket 05's migrated coverage rather than by a report: the
spacing-repair unit tests asserted that an escaped quote does not prematurely
end a string, and that assertion only becomes observable once it is made at the
public entry point, where the codegen round trip is part of the path.

Nothing new is expected to be needed for it: a lossless round-trip preserves the
source spelling by construction, and a string that stays a string cannot leak a
brace. The work is to confirm that after the swap and re-verdict the three
`Reference::Diverges` cases below. If they do not flip on their own, that is a
gap in the port — and the brace case is the one to check first, because it is
the only one in this effort where a divergence is also a correctness bug rather
than a spelling difference.

**Blocked by:** 07 — Swap normalization onto the ported pipeline.

**Status:** resolved

- [x] A double-quoted string containing an escaped quote normalizes to a
      declaration with no `}` in it
- [x] A single-quoted string containing an escaped quote keeps that character
- [x] The same holds inside a `url()` body
- [x] The parity harness reports `identical` for the three `fontFamily` and
      `backgroundImage` escaped-quote entries
- [x] `keeps_an_escaped_quote_inside_a_string`,
      `keeps_an_escaped_quote_inside_a_url_body` and
      `keeps_an_escaped_quote_around_a_protected_decimal` in
      `crates/stylex-css/src/css/tests/spacing_repair_parity_test.rs` are
      `unchanged` cases

      Written against `Reference::Same` when this ticket was filed. Ticket 07
      retired that vocabulary along with `diverges`, so the three tests were
      renamed off `diverges_on_*` and now carry the same claim through
      `unchanged` — which asserts both compilers spell the value as written.
- [x] A doubled backslash (`"a\\\\b"`), which is an escaped backslash rather
      than an escaped quote, still round-trips as it does today

## Resolution

Closed by ticket 07 rather than by work of its own. The defect lived in the
round trip through the CSS stylesheet serializer, and normalization no longer
makes that round trip: a value is parsed losslessly and spelled back out, so a
string the normalizers do not name reaches the declaration as the author wrote
it.

Confirmed against the harness, `@stylexjs/babel-plugin` v0.19.0 — verdict
`identical` for every value this ticket names.
