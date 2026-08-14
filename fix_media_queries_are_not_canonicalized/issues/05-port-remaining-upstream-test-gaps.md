# 05 — Port the remaining upstream test gaps

**What to build:** Every executable upstream media-query test case has a Rust
counterpart with equivalent assertion strength, closing the 3 missing and the
unit/validation partial cases from the exhaustive upstream diff (178 of 202
already covered; the e2e partials are handled in ticket 01).

**Blocked by:** 04 — Align normalize AST shapes (new structural assertions
must target the final shapes).

**Status:** resolved

- [x] Missing: `@media (1000px > width >= 700px)` →
      `(min-width: 700px) and (max-width: 999.99px)`
- [x] Missing: `@media (1000px >= width > 700px)` →
      `(min-width: 700.01px) and (max-width: 1000px)`
- [x] Missing: `@media (not (max-width: 200px)) and (not (max-width: 300px))`
      → `@media (min-width: 300.01px)`
- [x] Partial: AST-structure assertions added for the five multi-clause `or`
      parses and the `(color) and (min-width: 400px), screen and …` case
      (currently string-only)
- [x] Partial: `@media (400px < width <= 700px)` asserts the canonical string
      with `400.01px`/`700px` boundaries (currently `is_ok()` only)
- [x] Partial: the two validation inputs with two unclosed parens
      (`…(max-width: 1000px` and `…calc(100% - 50px`) assert
      `UNBALANCED_PARENS`
- [x] Partial: the four unit-conflict transform tests restore the outer
      namespace wrapper so the extra nesting level is exercised
- [x] Skipped upstream tests are not ported as normative (noted in the spec)
- [x] Full workspace `cargo test` green

## Answer

All gaps closed as test-only changes; no production code needed to move, which
confirms the remaining diff was coverage rather than behaviour.

Touched files (all under `crates/stylex-css-parser/src/tests/at_queries/`):

- `parse_media_query_test.rs` — added the two missing double-inequality cases
  in `inequality_rule_tests` and the `not`/`not` collapse in
  `simplify_range_intervals`; gave the six previously string-only
  `or_combinator` tests full AST assertions, via four small local helpers
  (`expect_or_branches`, `expect_and_clauses`, `expect_length_pair`,
  `expect_string_pair`) so the six tests read as shape declarations instead of
  ~600 lines of repeated `match` nesting.
- `validation_media_query_test.rs` — the two unbalanced-paren inputs had one
  paren too many (`…(max-width: 1000px)` / `…calc(100% - 50px)`), so they
  exercised a single unclosed paren rather than upstream's two. Upstream's
  forms were **added alongside** the existing ones rather than replacing them:
  swapping would have dropped the singly-unmatched cases, which the sibling
  input `@media ((prefers-color-scheme: dark)` still covers. All four report
  `UNBALANCED_PARENS`.
- `media_query_coverage_test.rs` — `(400px < width <= 700px)` now asserts
  `@media (min-width: 400.01px) and (max-width: 700px)` instead of `is_ok()`.
- `media_query_transform_test.rs` — restored the `foo` namespace wrapper on
  `media_queries_with_em_units`, `media_queries_with_mixed_units`, and both
  `skips_range_simplification_…` tests, matching upstream's
  `{foo: {gridColumn: {...}}}` input shape. The rewritten media keys still
  appear nested two levels deep, so the extra level is genuinely traversed.

Verified: `cargo test --workspace` green (0 failures across every crate),
`cargo fmt --all --check` clean, `cargo clippy -p stylex_css_parser
--all-targets` warning-free.
