# Plan: issue #1250 — media query canonicalization parity

Status: awaiting approval
Parity reference: `~/Projects/Facebook/stylex` @ `5f51b2444` (the v0.19.0 release commit, clean tree)

## Findings

### F1 — Issue #1250 no longer reproduces on HEAD

Fixed by `501a91e96` ("always move media queries last, as upstream does"): a lone
`@media` key is no longer early-returned and now goes through `MediaQuery`
parse → normalize → re-stringify. Verified with the issue's exact inputs:

- `@media (max-height:120px) and (min-width: 720px)` → `@media (min-width: 720px) and (max-height: 120px)`, hash `x1gcnmh1` (Babel-identical)
- `@media (width >= 1460px)` → `@media (min-width: 1460px)`, hash `xju9v9y` (Babel-identical)

Remaining work for the headline symptom: tests only.

### F2 — Real parity bug in `merge_intervals_for_and` (`crates/stylex-css-parser/src/at_queries/media_query.rs:561`)

In the DeMorgan branch (Not(And(a, b)) → distribute into two recursive
branches), our code returns the built `Or` only `if !or_rules.is_empty()`
(media_query.rs:627). Upstream `mergeIntervalsForAnd` (media-query.js:356)
returns `[{type: 'or', rules: [left, right].filter(len > 0).map(...)}]`
**unconditionally** — even when both branches are contradictions and the `or`
is empty; its `#toString` then collapses empty/single-child `or`s.

When both branches are empty our version falls through, hits the
"non-width/height rule present" check on the un-distributed rule list, and
returns the rules verbatim — emitting a mangled query.

Trigger: two or more `not (And(min, max))` negations whose ranges contradict
the base range — i.e. ≥3 disjoint ranges in one style value. Proof: deleting
the disjoint shortcut (F3) makes
`mixed_min_max_width_with_many_disjoint_ranges` fail with

```
left:  @media ((min-width: 900px) and (max-width: 1440px) and (not ((min-width: 400px) and (max-width: 500px))) and (not (min-width: 600px))) or ((min-width: 900px) and (max-width: 1440px))
right: @media (min-width: 900px) and (max-width: 1440px)   <- upstream
```

### F3 — Compensating divergence: the disjoint shortcut in `media_query_transform.rs`

`transform_media_queries_in_result` short-circuits via
`are_media_queries_disjoint` → `normalize_media_query_syntax`
(media_query_transform.rs:160). Upstream `dfsProcessQueries` has no such
branch — it always builds negations and lets `MediaQuery.normalize`/`toString`
collapse them. The shortcut exists only to mask F2 and drags along ~130 lines
upstream doesn't have (`are_media_queries_disjoint`,
`extract_width_height_range`, `ranges_overlap`,
`normalize_media_query_syntax`) plus a silent `Err(_) => kv` fallback that is
unreachable today (every key was already parsed successfully) but is a
divergence hazard.

### F4 — `MediaQuery::normalize` AST-shape divergences (`media_query.rs:268`)

String output is currently equivalent, but the AST diverges from upstream
`normalize` (media-query.js:598):

- (a) extra `flattened.is_empty()` early return (upstream lets the merge run);
- (b) contradiction (`merged` empty) returns `And([MediaKeyword not-all])`;
  upstream returns the bare `{type: 'media-keyword', key: 'all', not: true}`;
- (c) a compensating special case in the `Not` arm (media_query.rs:315-326)
  unwraps exactly that `And`-of-one shape — needed only because of (b).

### F5 — Dead code

`last_media_query_wins_transform_internal` (media_query_transform.rs:40) is a
self-described no-op kept "for backwards compatibility with existing tests".

### F6 — Already-parity (no change)

`format_queries` matches upstream `#toString` exactly (empty-`or` filtering,
`not all`, single-child unwrap, spacing, top-level comma joins). The negation
bookkeeping (`accumulated_negations`) matches upstream index-for-index.
`enable_media_query_order` exists, defaults `true`, plumbed through NAPI.
Depth-0 (style-level `@media` keys) pass through verbatim on both sides.

### F7 — Test coverage

2184 css-parser tests green; the hard upstream cases (or-of-intervals,
epsilon boundaries, unit conflicts) are already ported. An exhaustive
upstream-vs-Rust test diff is running in the background; its MISSING/PARTIAL
list feeds step S5.

## Fixes (in order)

- **S1** `media_query.rs`: make the DeMorgan branch return the `Or`
  unconditionally, mirroring upstream: filter empty branches, wrap len>1
  branches in `And`, keep an empty `Or` as-is. 1:1 with media-query.js:376-396.
- **S2** `media_query_transform.rs`: delete the disjoint shortcut and its four
  helper fns so the negation path runs unconditionally like upstream
  `dfsProcessQueries`. Keep the `stylex_panic!` on parse failure (parity with
  upstream throw → `INVALID_MEDIA_QUERY_SYNTAX` via `catch_unwind`).
- **S3** `media_query.rs::normalize`: align to upstream shape — drop the
  `flattened.is_empty()` early return, return bare `MediaKeyword("all", not)`
  on contradiction, drop the compensating `Not`-arm special case. Fix any
  structural test assertions only where they contradict upstream expectations
  (checking each against the upstream test file).
- **S4** Remove `last_media_query_wins_transform_internal` (after confirming
  no callers outside its own tests).
- **S5** Port the MISSING/PARTIAL upstream test cases from the background diff
  into the corresponding Rust test files.
- **S6** End-to-end tests in `crates/stylex-transform/tests`:
  - issue #1250 exact inputs pinned in file snapshots (`stylex_test!`, runtime
    injection on) carrying the canonical queries and hashes
    `x1gcnmh1` / `xju9v9y`;
  - the same inputs with `enable_media_query_order: false` pinning verbatim
    pass-through (Babel opt-out parity).
- **S7** Verification: `cargo test` (workspace), `pnpm typecheck`,
  `pnpm format:check`, `pnpm lint:check`, rebuild `rs-compiler` + `pnpm test`
  for the JS suites; confirm app snapshots do not churn (expected: no output
  change — the shortcut's outputs already matched upstream where it engaged;
  S1 makes the general path produce the same strings).
- **S8** Docs: add a "media query canonicalization" term to
  `crates/stylex-css-parser/CONTEXT.md` (currently unnamed there).
- **S9** Commits (conventional, on this branch):
  1. `fix(stylex-css-parser): collapse contradictory negation branches like upstream` (S1)
  2. `refactor(stylex-css-parser): drop the disjoint shortcut the fixed merge no longer needs` (S2-S4)
  3. `test(stylex-css-parser): port the remaining upstream media query cases` (S5)
  4. `test(stylex-transform): pin issue #1250 canonical queries and hashes` (S6, S8)

## Risks

- S1-S3 could in principle change emitted strings for inputs that previously
  took the shortcut; the failing-test experiment showed shortcut and fixed
  general path agree on the expected upstream strings, and the full test +
  snapshot suites gate this.
- Structural (AST-shape) test assertions in `parse_media_query_test.rs` may
  encode the current Rust shape from F4; each adjustment must be justified by
  the upstream test file, never by "making the test pass".
