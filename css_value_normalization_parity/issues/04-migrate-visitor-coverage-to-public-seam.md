# 04 — Migrate normalizing-visitor coverage to the public seam

**What to build:** The tests covering the CSS-AST normalizing visitor currently
assert against that visitor directly — they build a stylesheet, run the visitor
over it, and check the serialized result. That code is deleted in ticket 07, and
if these tests go with it, years of accumulated regression knowledge goes too.

Re-express that coverage against the public value normalization entry point
instead: same inputs, same expectations, asserted one level up at a boundary
that survives the rewrite.

Do this **now, while the old implementation is still live**. That is the point
of the ticket. These tests pass before the swap and after it, which turns ticket
07 from an unguarded rewrite into a change with a net under it. Migrating this
coverage after the swap would mean landing the riskiest change in the effort
with nothing watching.

Each migrated case carries a verdict from the ticket 01 harness: either this
expectation matches what the reference compiler produces, or it is one of the
known divergences and will change at ticket 07. Recording that verdict up front
is what makes the ticket 07 diff reviewable — every changed expectation was
predicted, and an unpredicted change is a bug.

**Blocked by:** 01 — Babel differential parity harness.

**Status:** resolved

- [x] Every input covered by the existing visitor tests is covered by a case at
      the public normalization entry point
- [x] Expectations assert current behaviour, so the suite is green before any
      pipeline change
- [x] Each case records whether it matches the reference compiler or is a known
      divergence scheduled to change, sourced from the harness rather than from
      judgement
- [x] The behaviours preserved include: millisecond-to-second conversion and its
      below-threshold exception, zero-dimension unit handling across angles,
      timings, fractions and percentages, zero handling inside functions, the
      custom-property exemption from zero normalization, camel-case value
      conversion for the properties that get it, and font-size conversion under
      both settings of its option
- [x] No new assertion references the visitor, the stylesheet type, or any other
      internal that ticket 07 removes
- [x] The original visitor test files are left in place for now — deletion is
      ticket 09, once the swap has proven the migrated coverage holds

## Answer

`crates/stylex-css/src/css/tests/value_normalization_parity_test.rs` — 31 tests
over 72 cases, all calling `normalize_css_property_value` and nothing else.

**How a verdict is recorded.** Each case is a `Case { property, value, expected,
reference }`, built by one of two constructors: `same(...)` when the harness
reported `identical`, `diverges(..., upstream)` when it did not. `expected` is
what this compiler produces today; `upstream` is the reference compiler's
spelling, copied from `entries[].babel.declarations`.

The verdicts are self-policing rather than a comment: the shared `check` runner
asserts `expected != upstream` for every case claiming a divergence. When ticket
07 adopts a reference spelling, that case fails and has to be re-verdicted — it
cannot be quietly left carrying a stale claim. So the 18 `diverges` cases are a
list of the expectations ticket 07 is *predicted* to change, and any other
changed expectation in that diff is a bug.

**Not migrated, deliberately.** Two of the 46 tests in
`crates/stylex-css/src/css/normalizers/tests/base.rs` have no seam equivalent
because their subject is the visitor itself:
`empty_function_value_is_visited_without_panicking` hand-builds an AST state no
input string can produce, and `base_normalizer_with_no_property` exercises a
`None` property that the entry point has no way to express. Both guard code
ticket 09 deletes. The `zero_unit` and `restore_negative_leading_zero` unit
tests at the bottom of that file test helpers, not the visitor;
`restore_negative_leading_zero` is spacing-repair machinery and belongs to
ticket 05. `zero_unit`'s three assertions (`%` and `fr` kept, `dpi` dropped) are
covered at the seam by `normalizes_zero_dimensions_by_unit_kind`.

**A trap in the harness, now documented.** The harness runs the whole transform,
and `content` / `hyphenateCharacter` are returned verbatim by `transform_value`
whenever their value already carries matching quotes — they never reach value
normalization. A verdict on one of those describes the transform path, and the
two disagree: `content: "\2014 A"` passes through the transform untouched but
has its escape resolved at the seam. The three `content` escape cases were
re-expressed on `fontFamily`, and `parity/README.md` now warns about this under
"Using a report to write test expectations".

**Corpus additions.** Seventeen new `edge.json` entries carry verdicts for the
extra coverage: deep nesting, a custom-property fallback chain, vendor prefixes
(keyword, function, and camel-case value), escapes and non-ASCII on a property
that reaches normalization, an unclosed line-name bracket, an opening brace, a
nonsense operator sequence, and a long comma-separated list. This is corpus work
that properly belongs to ticket 01 rather than to a migration, and is called out
as such: the migration itself needed no new corpus, because the visitor tests'
inputs were already harvested.

**A sixth harvest shape.** The new case tables are exactly the shape
`harvest-corpus.ts` could not see, so a case added to this module in future
would silently fall out of the corpus the harness runs. Shape 1's extractor is
now generalized over three call names — `normalize_css_property_value`, `same`,
`diverges` — taking the first two literal arguments and never the expected
output. `findCallSites` gained an identifier-boundary check, without which a
name as short as `same` would match the tail of `is_same`. Unit-tested in
`parity/__tests__/harvest.test.ts` under "shape 6", which is the extension
point `parity/README.md` names.

Corpus is now 594 declarations — 448 identical, 101 divergent, 15 structurally
divergent, 15 acceptance divergent, 15 rejected by both. The ticket 01 baseline
table still describes the 570 it measured. The four new value-normalization
divergences are the two escape cases (ticket 13), hex shortening inside a
vendor-prefixed function, and one more surfaced by the wider harvest.

Beyond the migration, the module also covers vendor prefixing, unicode and
escapes, malformed input that is inert versus rule-breaking, and two generated
robustness cases (nesting depth, a 500-entry list). Those two are the only
tests here with no harness verdict, because their inputs are generated rather
than checked in; both say so in their doc comments. The brace rejection has a
verdict (`acceptance divergent`) but is asserted on the diagnostic text rather
than through the case table, since a rejection has no spelling to compare and
`is_err()` alone would pass on any panic at all.

**Two findings, filed rather than fixed here.**

- **Escapes resolved inside strings** (ticket 13). `"\2014 A"` → `"—A"`,
  `"\1F600"` → `"😀"`; upstream preserves both. Same family as the six reported
  divergences, absent from the ticket 01 baseline only because the corpus
  reached escapes through `content`, which bypasses normalization. Ticket 07's
  lossless round-trip should close it for free.
- **Deep nesting aborts the process** (ticket 14). Normalization recurses per
  nesting level with no limit; past ~100 levels of `calc(calc(…))` on a 2 MiB
  test thread it overflows the stack, which no `catch_unwind` can catch — the
  process dies with no diagnostic. Out of the parent spec's scope, and the
  reason `survives_deep_function_nesting` asserts depth 50 rather than a depth
  that would demonstrate the limit.
