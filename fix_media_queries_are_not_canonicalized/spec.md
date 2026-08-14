# Spec: media query canonicalization parity (issue #1250)

Status: ready-for-agent

Upstream reference: `~/Projects/Facebook/stylex` @ `5f51b2444` (the v0.19.0
release commit). GitHub issue:
https://github.com/Dwlad90/stylex-swc-plugin/issues/1250. Companion analysis:
[PLAN.md](./PLAN.md) in this directory.

## Problem Statement

Users migrating between `@stylexjs/babel-plugin` and this compiler need both
to emit byte-identical CSS and class hashes for the same source. The Babel
plugin canonicalizes media queries — reorders conditions width-first,
normalizes spacing after the colon, and rewrites range syntax
(`(width >= 1460px)` → `(min-width: 1460px)`) — so any query this compiler
emits as authored hashes to a different class name, silently breaking style
deduplication and cross-compiler consistency.

The headline symptom (issue #1250) is already fixed on the current branch, but
nothing in the test suite pins it, and the audit performed for this spec found
a live parity bug the fix's shortcut was masking: with three or more disjoint
width/height ranges on one property, the interval merge fails to collapse
contradictory negation branches and emits a mangled query that upstream would
collapse to a simple range.

## Solution

Bring the media-query canonicalization pipeline (the last-media-query-wins
transform and the `MediaQuery` normalize/serialize path in the CSS parser
crate) to exact 1:1 logic parity with upstream v0.19.0, remove the
compensating code upstream does not have, and pin the behavior with tests at
the same seams upstream tests it — including the issue's exact inputs and
hashes, and the `enable_media_query_order: false` opt-out.

## User Stories

1. As a StyleX user migrating from `@stylexjs/babel-plugin`, I want authored media queries to canonicalize identically in both compilers, so that my class hashes do not change when I switch compilers.
2. As a StyleX user, I want `@media (width >= 1460px)` rewritten to `@media (min-width: 1460px)`, so that my emitted CSS keeps the broader browser support of the min-width form.
3. As a StyleX user, I want `@media (max-height:120px) and (min-width: 720px)` emitted as `@media (min-width: 720px) and (max-height: 120px)`, so that condition order and spacing match the official compiler.
4. As a StyleX user with several media conditions on one property, I want later queries to win over earlier ones exactly as upstream computes it, so that responsive overrides resolve the same in both compilers.
5. As a StyleX user with three or more disjoint width ranges on one property, I want each range emitted as a clean simple range rather than a mangled `or` expression, so that my stylesheet stays valid and minimal.
6. As a StyleX user with intersecting ranges, I want the interval subtraction to produce the same `or`-of-intervals string as upstream, so that hashes match even for complex responsive layouts.
7. As a StyleX user who opts out via `enableMediaQueryOrder: false`, I want my media keys passed through verbatim, so that I keep control over query text exactly as the Babel plugin allows.
8. As a StyleX user using legacy style-level `@media` keys, I want them left untouched, so that legacy code keeps its existing hashes (matching upstream's depth rule).
9. As a StyleX user who writes an invalid media query, I want the same `INVALID_MEDIA_QUERY_SYNTAX` error the Babel plugin raises, so that mistakes fail loudly instead of leaking broken CSS.
10. As a StyleX user with mixed units across queries (`px` vs `em`), I want range simplification skipped exactly as upstream skips it, so that unit-conflicting queries keep upstream's negation form.
11. As a StyleX user using strict/inclusive double inequalities (`(1000px > width >= 700px)`), I want the epsilon-adjusted boundaries (`999.99px`, `700.01px`) to match upstream digit-for-digit, so that boundary behavior and hashes agree.
12. As a StyleX user negating ranges (`(not (max-width: 300px))`), I want double negations and contradictions collapsed the way upstream collapses them, so that equivalent queries converge on one canonical string.
13. As a maintainer of this repo, I want the Rust transform to have no logic branches upstream lacks, so that future upstream changes port mechanically without re-deriving compensations.
14. As a maintainer, I want the issue's exact inputs, canonical outputs, and class hashes pinned by tests that assert the emitted CSS text and not just class names, so that a rehash is visible in review rather than silent.
15. As a maintainer, I want the upstream test suites' remaining gaps ported, so that parity claims rest on executable evidence rather than sampling.
16. As a maintainer, I want the CSS parser crate's glossary to name media-query canonicalization, so that future contributors find the concept before renaming or splitting it.
17. As an agent working a future ticket in this area, I want dead compatibility shims removed, so that the code I read reflects only behavior that runs.
18. As a user of `defineVars`/`createTheme`/`defineConsts` at-rules, I want their current pass-through behavior unchanged, so that this fix does not silently rehash variable themes (upstream does not canonicalize these either).

## Implementation Decisions

- Parity target is the executable behavior of upstream v0.19.0; skipped
  upstream tests are not normative. Where the Rust output for such an input
  differs from a skipped upstream expectation, actual Babel runtime output
  decides — verify before changing anything.
- Fix the interval merge's DeMorgan distribution in the CSS parser crate's
  `MediaQuery` module: return the distributed `or` unconditionally (filtering
  empty branches, wrapping multi-rule branches in `and`, keeping an empty `or`
  as-is) exactly as upstream `mergeIntervalsForAnd` does, and let the
  serializer collapse empty/single-child `or`s — the serializer already
  matches upstream.
- Delete the disjoint-ranges shortcut from the last-media-query-wins
  transform (`are_media_queries_disjoint`, `extract_width_height_range`,
  `ranges_overlap`, `normalize_media_query_syntax`), so the negation path runs
  unconditionally like upstream `dfsProcessQueries`. This also removes the
  silent keep-authored fallback on re-parse failure, which upstream does not
  have.
- Align `MediaQuery::normalize` AST shapes with upstream: no early return for
  an empty flattened list; contradiction yields the bare `not all` media
  keyword (not an `and`-wrapped one); drop the `not`-arm special case that
  existed only to unwrap the wrapped shape.
- Keep the panic-on-unparseable-query behavior in the transform: it surfaces
  through the existing catch as `INVALID_MEDIA_QUERY_SYNTAX`, matching
  upstream's throw.
- Remove the vestigial no-op `last_media_query_wins_transform_internal` after
  confirming nothing outside its own tests calls it.
- `enable_media_query_order` plumbing (option struct, NAPI field, default
  `true`) is already correct; no interface changes.
- Preserve upstream naming and structure throughout; no Rust-idiomatic
  restructuring beyond what compilation requires. One exception, forced by
  RUST.md's ban on `.unwrap()`/`.expect()`: upstream keeps per-dimension
  interval and unit maps and indexes them directly, which in Rust is a fallible
  map lookup on every read. `dimension_constraint` and `DimensionIntervals`
  carry the same state without a fallible index. Same inputs, same outputs,
  same bail-out order — only the container differs.
- Record the "media query canonicalization" term in the CSS parser crate's
  domain glossary.
- Conventional commits on the existing branch, ordered: the merge fix, the
  shortcut removal, ported test gaps, issue-pinning end-to-end tests.

## Testing Decisions

Good tests here assert external behavior only: the canonical query string, the
emitted CSS rule text, and the class hash — never intermediate AST shapes
except where upstream's own tests assert them (structural assertions are then
ported like-for-like).

Seams, all existing (no new seams introduced):

1. **Transform seam (highest)**: the whole-file transform macros in the
   transform crate's test suite, with runtime injection enabled so the
   injected `@media …{…}` strings and hashes are visible. Prior art: the
   dynamic-styles media tests, whose snapshots carry full `ltr:` strings. New
   tests here use file snapshots like every other test at this seam; runtime
   injection is what makes a rehash reviewable, since the snapshot then carries
   the query string beside the class it hashes to.
2. **Module seam**: `last_media_query_wins_transform` unit tests over
   key-value props. Prior art: the existing transform test file with its
   JSON-comparison helpers.
3. **Parser seam**: `MediaQuery` parse-to-end plus display round-trips. Prior
   art: the existing parse and coverage test files, ported from upstream.

Coverage to add (from the exhaustive upstream diff — 178 of 202 upstream cases
already covered):

- Issue #1250 inputs end-to-end with exact canonical queries and hashes
  (`x1gcnmh1`, `xju9v9y`) captured in file snapshots, plus the same inputs under
  `enable_media_query_order: false` pinning verbatim pass-through.
- A regression test for the ≥3-disjoint-ranges collapse (currently passes only
  via the shortcut being removed and the merge being fixed).
- The 3 missing parser cases: `(1000px > width >= 700px)` →
  `(min-width: 700px) and (max-width: 999.99px)`;
  `(1000px >= width > 700px)` → `(min-width: 700.01px) and (max-width: 1000px)`;
  `(not (max-width: 200px)) and (not (max-width: 300px))` →
  `(min-width: 300.01px)`.
- Strengthen the partial cases: AST structure for the five multi-clause `or`
  parses; boundary values for `(400px < width <= 700px)`; the two
  two-unclosed-paren validation inputs; restore the extra nesting wrapper in
  the four unit-conflict transform tests.
- Verification gate: full workspace `cargo test`, `pnpm typecheck`,
  `pnpm format:check`, `pnpm lint:check`, rebuilt `rs-compiler` before
  `pnpm test`, and no app-snapshot churn (expected: none — the shortcut's
  outputs already matched upstream where it engaged).

## Out of Scope

- The `@property … inherits: true` vs upstream `inherits: false` divergence
  found in dynamic pseudo-class output during the audit — a real parity bug,
  but in variable declaration emission, not media queries. File separately.
- Canonicalizing at-rules on the `defineVars`/`createTheme`/`defineConsts`
  paths, `@supports`, and `@container` — upstream does not canonicalize these
  either.
- Depth-0 (style-level) `@media` key canonicalization — upstream leaves these
  verbatim by design.
- The upstream `test.skip`ped keywords-plus-rules combination expectation —
  not executable upstream behavior; only verify against live Babel output if
  it becomes contentious.
- Closing GitHub issue #1250 — deliberate maintainer act after the tests land.

## Further Notes

- The branch is even with `origin/develop`; no rebase needed.
- The audit experiment (shortcut removed, merge unfixed) fails exactly one
  test, `mixed_min_max_width_with_many_disjoint_ranges`, reproducing the
  mangled-`or` output — a ready-made red/green harness for the merge fix.
- Structural test assertions that encode the current Rust AST shapes may need
  adjustment under the normalize alignment; every such change must be
  justified by the upstream test file, never by making the test pass.
