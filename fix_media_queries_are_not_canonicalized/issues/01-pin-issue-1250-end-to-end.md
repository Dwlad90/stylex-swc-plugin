# 01 — Pin issue #1250 end-to-end

**What to build:** A user compiling the exact inputs from GitHub issue #1250
gets Babel-identical output, and the test suite proves it with snapshots that
carry the emitted CSS text, so a rehash shows up as a query-string diff in
review rather than a bare class-name change. The same inputs compiled with
`enableMediaQueryOrder: false` keep their media keys verbatim, matching the
Babel plugin's opt-out.

This is the guard rail for every refactor ticket that follows — it lands
first.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] Transform-seam test (file snapshot, runtime injection enabled) for
      `'@media (max-height:120px) and (min-width: 720px)'` asserting the
      emitted rule `@media (min-width: 720px) and (max-height: 120px){…}` and
      class hash `x1gcnmh1`
- [x] Same for `'@media (width >= 1460px)'` asserting
      `@media (min-width: 1460px){…}` and class hash `xju9v9y`
- [x] Base classes pinned alongside: `x1s85apg` (display) and `x1e2nbdu`
      (color)
- [x] The same two inputs under `enable_media_query_order: false` pin the
      authored query strings passing through verbatim
- [x] The seven partial end-to-end media cases from the upstream diff are
      strengthened so the injected CSS text (not just class names) is
      asserted — the static media tests currently build their transform
      without runtime injection
- [x] Full workspace `cargo test` green; no existing snapshot churn

## Answer

Landed as tests only — the transform already produced Babel-identical output;
nothing pinned it.

- `crates/stylex-transform/tests/transform_stylex_create_test/media_query_canonicalization.rs`
  (new): both issue #1250 inputs via `stylex_test!`, whose snapshots record the
  full injected rule text — the repo pins every transform-seam case in a
  snapshot file, and runtime injection is what keeps a rehash reviewable there.
  Canonical output pinned character-for-character against the Babel 0.19.0
  block in the issue: `.x1s85apg{display:none}` /
  `@media (min-width: 720px) and (max-height: 120px){.x1gcnmh1…{display:block}}` /
  `.x1e2nbdu{color:red}` /
  `@media (min-width: 1460px){.xju9v9y…{color:blue}}`, priorities 3000/3200.
  The opt-out test (`enable_media_query_order: false`) pins the authored
  strings passing through verbatim, reproducing the 0.18.3 hashes `x4ob7n2`
  and `xy2bn39` the issue reports — that is the Babel opt-out's contract, and
  it is self-derived rather than taken from the issue, which carries no
  Babel-side opt-out output.
- Seven end-to-end media cases strengthened with runtime injection so the
  injected CSS text is asserted, not just class names: the six in
  `static_styles.rs` (`media_queries`, `media_queries_with_last_query_wins`,
  `media_queries_without_last_query_wins`, `…_v2`,
  `media_query_with_pseudo_classes`, `media_query_with_array_fallbacks`) plus
  `metadata_test::stylex_metadata_is_correctly_set`, whose depth-0 `@media`
  key is exactly the case where pinning the rule text proves the verbatim
  pass-through.

All seven snapshot diffs are **additions only** — every class name is
unchanged, so there is no behavioural churn to review.

Note on `media_queries_without_last_query_wins_v2`: its newly visible CSS
records the nested `or`-of-`or`s with `(not (screen))` clauses. Ticket 02 was
expected to change this snapshot and did not, so it was verified directly
against Babel 0.19.0 — the same input run through `@stylexjs/babel-plugin` with
`runtimeInjection` emits this snapshot byte for byte, class names
(`x1qc147k`, `x9qmkci`, `x17z8iku`) included. The `screen` media type is not a
width/height rule, so the interval merge correctly declines to simplify these
branches; the snapshot is upstream's output, not a mangling left behind.

Gate: workspace `cargo test` green (1055 transform + 2184 css-parser, 0
failed), `cargo fmt --check`, `cargo clippy --all-targets` clean, `pnpm
typecheck` / `format:check` / `lint:check` green, `rs-compiler` rebuilt and
`pnpm test` green (63/63 tasks). No app snapshot churn.
