# Remediation plan: review findings on `fix_media-query-computed-bounds-f64`

Status: ready-for-agent
Spec: [spec.md](./spec.md)
Branch reviewed:
`fix_media-query-order-computed-upper-bounds-use-f32-arithmetic`
Base: `develop` (`578e11f61`) -- 40 commits, 107 files, +9652/-2516
Review date: 2026-08-24

Source of findings: a two-axis `/code-review` (Standards, Spec) plus seven
further passes -- regressions, removed tests, performance, upstream parity,
porting notes, core numeric correctness, and the parity harness itself.

## Baseline measured at review time

Everything below is green **before** any remediation, so any number that moves
is attributable:

- `cargo build --workspace` clean; `cargo clippy --all-targets` clean;
  `cargo fmt --check` clean; `pnpm lint:shell` clean.
- Rust: 6000+ tests, 0 failures (`stylex_css_parser` 2313, `stylex_css` 1032,
  `stylex_transform` 1870+860+19+3, `postcss_value_parser` 1032).
- JS: 24 files / 645 tests pass.
- `parity-values.ts`: 1085 subjects, 0 divergent, 0 changed, 0 unexpected.
- `fuzz-shorthand-split.ts`: 153 624 subjects, 18 893 divergences, all pinned,
  0 unexpected.
- `fuzz-pseudo-order.ts`: 1000 pairs, 0 disagreements vs `Intl.Collator('und')`.

## Correction to the review

The reported `$=` / `*=` coverage gap is **false**. Both are asserted at
`crates/stylex-css/src/tests/values_parser_tests.rs:364-365`. No action.

## Framing that decides the order

The only production consumer of `stylex-css-parser` anywhere is
`last_media_query_wins_transform`
(`crates/stylex-transform/src/shared/utils/core/flatten_raw_style_object.rs:4`).
`crates/stylex-css/Cargo.toml:20` declares the dependency and never uses it
(pre-existing on `develop`). So **every finding in `color.rs` is latent**: it
cannot reach emitted CSS today. That does not make them non-findings -- one of
them is pinned by a new test -- but it does mean they belong in their own
commits, and arguably their own branch, because spec story 17 requires the
#1267 commit to stay independently revertable.

Sequencing follows three properties, not severity alone:

1. A test that pins a wrong answer is worse than no test. Fix it before
   anything else touches that file, or later work inherits the wrong baseline.
2. Doc fixes are zero-risk and nothing can regress. Batch them.
3. Latent-path parity fixes are churn with no live benefit today. Separate
   commits.

Order: correctness-of-tests -> docs -> harness -> robustness -> latent parity.

---

## Phase 1 -- Blocking: the test that pins the wrong answer

`fix(stylexswc/css-parser): omit the alpha tail the reference compiler omits`

Upstream `packages/style-value-parser/src/css-types/color.js:485,547,601` uses a
**truthy** alpha test:

    return `lch(${this.l} ${this.c} ${this.h.toString()}${this.alpha ? ` / ${this.alpha}` : ''})`;

so `alpha === 0` prints no tail. This crate prints `/ 0` for any `Some`.

| Change | Site |
| --- | --- |
| Gate the tail on `Some(a) if a != 0.0`, routing zero to the existing no-tail arm | `css_types/color.rs:1972` (Lch), `:2107` (Oklch), `:2222` (Oklab) |
| Correct the assertion that pins the divergence | `tests/css_types/color_double_precision_test.rs:613` -> `"oklab(0 0 0)"` |
| Add the `lch` / `oklch` siblings of that case | same file |

Both `Display` arms already exist (`color.rs:1976`/`:1984`, `:2111`/`:2119`,
`:2226`/`:2234`), so this is a match-guard change, not a restructure.

Confirm the new expectation against `@stylexjs/babel-plugin@0.19.0` directly
rather than by reasoning -- the spec's rule for every moved expectation.

**This is the only item worth holding the merge for.**

## Phase 2 -- Docs and traceability (zero risk, one pass)

`docs(guidelines): say that the parity harness now runs in CI`

- `guidelines/SCRIPTS.md:113-117` still reads "Not a test and not wired into CI
  -- except `parity:harvest:check`". Commit `80b6a719b` falsified that: it wires
  `parity`, `parity:positions` and `fuzz:pseudo-order` into
  `.github/workflows/pr-validation.yml:203-211`, and `fuzz:shorthand` into the
  nightly `parity-sweep` job. The doc edit landed one commit *before* the CI
  change and was never brought forward.
- Same file: document `fuzz:pseudo-order` and `fuzz:shorthand`, the two new
  `crates/stylex-rs-compiler/package.json` scripts.

`docs(stylexswc/css-parser): name the test-suite suffix convention`

- `crates/stylex-css-parser/CONTEXT.md` -- the branch invented a third naming
  category (`*_precision_test.rs`, `color_double_precision_test.rs`,
  `js_number_spelling_test.rs`) beside the established `*_test.rs` upstream
  mirror and `*_coverage_test.rs` sibling. Record it, and say why it has no
  upstream counterpart: the divergence it pins cannot exist upstream.

`docs(stylexswc/css): cite the upstream identifier each module ports`
(naming parity is a stated repo goal)

- `crates/stylex-css/src/values/parser.rs` -- 60-line header that never names
  `splitValue` (`split-css-value.js:30`).
- `is_list_style_type` -- cite `listStyleTypeRegex`
  (`legacy-expand-shorthands.js:97`).
- `order/constants/legacy_expand_shorthands_order.rs:236` -- say *why* the
  dropped `!part.is_empty()` guard is unreachable: `TRawValue`
  (`crates/stylex-structures/src/raw_value.rs:12`) has no array variant, so
  upstream's `part != null` serves the `if (Array.isArray(str)) return str;`
  arm (`split-css-value.js:33-39`) that this port dropped.

**ADR for the collation dependency** -- `crates/stylex-transform/docs/adr/`.
Move the substance of `pre_rule.rs:311`'s `pub(super) mod collation_cost {}`
there and leave a pointer. Retires the empty-module idiom and puts a Rust-only
dependency with no upstream analogue where the repo already records such
decisions. Include the +1 222 800 byte (+12.5%) addon figure: `budget.json` is
`pending-calibration`, so nothing else will catch it.

**Tracker hygiene** (edit through `.scratch/...`; never `git add` it from a code
worktree -- commit only from the `scratch/` worktree):

- Tickets 22-28 carry `Status: done` with every acceptance box unchecked and no
  closing note, unlike 01-21. Fill them in.
- Amend `spec.md` to cover what tickets 11-28 added. The deletion of
  `crates/stylex-css-parser/src/value_parser.rs` is sanctioned by tickets
  11/13/19 but by no line of the spec, and the Out of Scope list never
  contemplated it.
- Amend story 20 to the class names Babel 0.19.0 actually produces
  (`x10ok0k0` / `xj7mlad`), so the criterion is met as written rather than
  met-with-a-footnote. Ticket 01's closing note already explains why `xu5ieg8` /
  `x1t400y5` were not reproducible; the media query *text* matches verbatim.

> commitlint note: a body line beginning `word:` parses as a footer. Bodies here
> will want to quote CSS (`rgba:`, `oklab:`) -- indent or reword those lines.

## Phase 3 -- Harness guards

The harness is the oracle every expectation was derived from. A silent pass is
its worst failure mode.

`fix(stylexswc/rs-compiler): fail a pair the ordering fuzz could not measure`

- `parity/fuzz-pseudo-order.ts:247` --
  `selector.indexOf(left) < selector.indexOf(right)` never checks for `-1`. If
  the emitted selector escapes a key, both lookups return `-1`, `-1 < -1` is
  `false`, and the pair passes silently whenever root collation agrees. Count
  unmeasured pairs and exit non-zero if any -- the same argument
  `compared === 0` already makes at `:268`.

`fix(stylexswc/rs-compiler): refuse a sweep whose filter selected nothing`

- `parity/fuzz-shorthand-split.ts:228-232` -- `--property bogus` filters
  `PROPERTIES` to empty with no guard, so `unexpected === 0` and the run exits
  0. The only failure condition is `unexpected.length > 0` (`:404`). Mirror the
  guards at `parity-values.ts:174-177` and `parity-positions.ts:297`.
- Same file: add the `unreached` families check it lacks, so a family that stops
  claiming rows in the sweep is visible.

`ci(stylexswc/rs-compiler): bound the nightly sweep`

- `parity-sweep` has no `timeout-minutes`, so it inherits the 360-minute
  default.

Optional, same area:

- Add `--seed` to the pseudo-order fuzz. Its 1000 pairs are fixed forever
  (`fuzz-pseudo-order.ts:168`) and the printed seed is decorative.
- Print `process.versions.icu` so ICU/CLDR skew between machines surfaces; the
  data version is currently uncosted.

**Open decision, not a defect** -- `parity/lib/report.ts:155`: `fails()`
deliberately does not fail on `unexpected`, so a brand-new divergence prints a
count and exits 0 in CI. The reasoning at `:143-153` is sound and explicit, but
the sweep takes the opposite policy. Pinning the expected count at 0 would make
a new row loud without weakening the argument. See Decisions.

## Phase 4 -- Robustness and test fidelity

`fix(stylexswc/css-parser): fall back rather than abort on a bad offset`

- `token_types.rs:261,309` -- `&input[start..]` becomes
  `input.get(start..).unwrap_or_default()`, folding into the `leading_f64`
  fallback three lines down. `start` comes from
  `parser.position().byte_index()` inside a *nested* block; a slice panic here
  is a process abort across NAPI, not a diagnostic.
- Same commit: replace the six `.unwrap()`s in
  `tests/at_queries/media_query_coverage_test.rs` and the one at
  `crates/stylex-css/src/order/tests/legacy_expand_shorthands_constants_test.rs:6639`
  with `match ... panic!`, per `guidelines/stack/RUST.md`.

`test(stylexswc/css): move the normalization suite onto the shipped entry point`

- The `normalize_value` / `normalize_value_guarded` split is **new on this
  branch** (`d1e5c604e`). On `develop` the 1047-line
  `css/tests/normalize_value_test.rs` tested the shipped function; at HEAD it
  tests a wrapper production bypasses -- the shipped path is
  `normalize_css_property_value` (`css/common.rs:695`) ->
  `normalize_value_guarded`, and `normalize_value` (`normalize_value.rs:97`) is
  now called only from that suite and one bench.
- Repoint the suite at `normalize_css_property_value`. Partially mitigated
  already by the new `css/tests/value_normalization_parity_test.rs`, which does
  target the shipped entry point -- but repointing is the difference between
  testing the compiler and testing a test helper.

`refactor(stylexswc/css-parser): collapse the epsilon to one constant`

- `at_queries/media_query.rs:557` already declares
  `const EPSILON: f64 = 0.01` at module scope; `:1303`, `:1461`, `:1680` shadow
  it locally. Upstream has two (`media-query.js:54,359`). Pure tidy.

`test(stylexswc/css): port the reference compiler's own split cases`

- `packages/@stylexjs/babel-plugin/src/shared/utils/__tests__/split-css-value-test.js:12-45`
  has five cases with no counterpart here. Add as one
  `upstream_split_css_value_cases()`:
  - `'0 1 2 3'` -> `['0','1','2','3']`
  - `'0, 1, 2, 3'` -> `['0','1','2','3']`
  - `'rgb(255 200 0)'` -> `['rgb(255 200 0)']`
  - `'rgb(255 200 / 0.5)'` -> `['rgb(255 200/0.5)']` (div inside a function)
  - `'calc((100% - 50px) * 0.5) var(--rightpadding, 20px)'` -> 2 parts
    (anonymous nested group)

  The last two are the only upstream cases exercising exactly the node-kind
  logic this branch rewrote.
- Also port `easing-function-test.js:62` `'cubic-bezier( 1.5 , 1 , .1 , 1 )'`
  and `:75`; and `transform-function-test.js:43`
  `'matrix(\n.4,0,0.5,1.200,60,10   )'` and `:96` `matrix3d(...)` -- the latter
  is precisely the `leading_f64` prefix scanner's edge.

## Phase 5 -- Latent upstream divergences (own commits; consider own branch)

All in `color.rs`, all unreachable from the compiler path today. Fix so the next
person to wire `Color` into production does not inherit them.

| Finding | Change | Cost |
| --- | --- | --- |
| Comma spelling. Upstream `color.js:314,428` emits `rgba(${r},${g},${b},${a})` with **no** spaces; `color.rs:1205,1756` emit `", "` | Drop the spaces | ~12 output assertions across 6 test files. The other ~114 `rgba(` occurrences in tests are inputs and unaffected |
| Bounded legacy alpha. `css_types/alpha_value.rs:97` `parse_alpha_in_unit_range` refuses outside `0..=1`; upstream's reader (`color.js:243`) is **unbounded** -- only *channels* are bounded (`color.js:237`), so `rgba(0,0,0,2)` parses upstream and is refused here | Delete it; call `alpha_as_number()` at `color.rs:1016,1127,1546,1651` | Reverts a consolidation this branch just made; the four coverage tests move back |
| LCH grammar. `parse_lch_chroma_token` (`color.rs:1920`) takes `Number` only; upstream `color.js:498` also accepts `Percentage.parser.map((p) => (150 * p.value) / 100)`. Upstream lightness (`color.js:488`) guards `value >= 0` and accepts `none`; `parse_lch_lightness_token` (`color.rs:1903`) does neither | Accept the percentage; add the guard | Port `color-test.js:77` `lch(50% 100 270deg)` with it |

Also here, informational from the correctness pass: `color.rs:37` keeps
`rgb`/`rgba` channels as `u8`, so `rgb(2.5, 0, 0)` truncates to `2` where
upstream (`color.js:260`) prints `2.5`. Same dead path. Widening it is the
honest completion of story 16 -- "no `as f32` cast left on any path between a
parsed token and emitted text, so that the class of bug #1267 belongs to is
closed rather than fixed at one site" -- because the same argument applies to a
`u8`.

## Phase 6 -- Deferred, with reasons

- **`write_js_number(&mut impl fmt::Write, f64)`** to halve `to_js_string`'s two
  allocations per number (`crates/stylex-utils/src/number.rs:103` builds a
  `String`; `:167` does `format!("{:e}")` first). Worthwhile -- the branch
  widened its reach to every `Display` in `stylex-css-parser`, all of which are
  `write!("{}", to_js_string(..))`, allocating only to copy into a formatter.
  **But the spec puts `to_js_string` out of scope** ("It is adopted, not
  modified") and the path is cold. Follow-up branch.
- **`evaluate_stylex_create_arg.rs:333-357`** -- the reason-drop sits inside the
  spread branch; upstream keeps it there (`babel-plugin/lib/index.js:3075`) and
  drops it at the **caller** (`:3030-3040`). Same observable output today,
  confirmed against Babel on five refusal shapes. Cosmetic faithfulness.
- **Dead dependency** -- `crates/stylex-css/Cargo.toml:20` declares
  `stylex_css_parser` and never uses it. Pre-existing on `develop`, so not this
  branch's finding. Remove opportunistically, not here.
- **Double byte scan** per shorthand value (`values/parser.rs:107` calls
  `nests_too_deeply`, then normalization scans again at `css/common.rs:700`).
  Already priced and accepted in the comment; only reached from the opt-in
  legacy expansion. Leave.
- **`padding: '1px /*'`** now emits `padding-top` + `padding-bottom` only, where
  `develop` emitted all four sides. Babel *throws* on this input
  (`Cannot read properties of undefined (reading 'type')`), so there is no
  reference answer to match. The branch documents the choice at length. Leave,
  but say so in the PR body: it is a net loss of two declarations relative to
  `develop` on an input no author writes.

## Decisions needed

1. **Phase 5 here or a follow-up branch?** Spec story 17 wants the #1267 commit
   independently revertable. Phase 1 must land here (a test pins a wrong
   answer). Phases 2-4 are safe here. Phase 5 is ~12 expectation edits on a dead
   path -- a follow-up keeps this branch's revert story clean.
2. **`report.ts` policy** -- pin the curated harness's expected `unexpected`
   count at 0 so a new divergence fails CI, or keep the documented "a divergence
   is information, not a regression" stance?
3. **`guidelines/coding/PLAN.md` ritual** -- it prescribes asking about a
   PRD and running three competing planning agents plus a reviewer. Skipped
   here: the
   session barred unrequested agents, and "three fundamentally different
   approaches" does not fit a remediation list. Re-run properly on request.

## Verification gate, every phase

    cargo test -p stylex_css_parser -p stylex_css -p stylex_transform
    cargo clippy --all-targets
    cargo fmt --all -- --check
    pnpm typecheck && pnpm lint:check && pnpm lint:type-aware
    pnpm test
    pnpm run --filter=@stylexswc/rs-compiler parity
    pnpm run --filter=@stylexswc/rs-compiler parity:positions
    pnpm run --filter=@stylexswc/rs-compiler fuzz:pseudo-order

Tests that import `@stylexswc/rs-compiler` exercise `dist/*.node`: rebuild
before the JS suite or the report is about nothing.

Any harness number that moves from the baseline above gets named in the PR body
with its Babel-confirmed value, per the spec's baseline rule.

## Findings ledger

| # | Kind | Severity | Phase | Item |
| --- | --- | --- | --- | --- |
| 1 | Upstream parity | Blocking | 1 | Zero-alpha tail printed; a new test pins the wrong answer |
| 2 | Standards | Hard | 2 | `SCRIPTS.md:113` contradicted by its own branch |
| 3 | Standards | Hard | 2 | `46b54b8aa` typed `fix(guidelines)` for a docs-only edit; also the branch's one unrelated hunk |
| 4 | Spec | Partial | 2 | Tickets 22-28 `done` with no acceptance record |
| 5 | Spec | Partial | 2 | `value_parser.rs` deletion sanctioned by tickets, not by the spec |
| 6 | Spec | Partial | 2 | Story 20 class names unmet as written (documented, not silent) |
| 7 | Porting | Note | 2 | New test-suffix convention unrecorded in `CONTEXT.md` |
| 8 | Porting | Note | 2 | Module headers never name the upstream identifier they port |
| 9 | Standards | Judgement | 2 | `collation_cost` empty module; belongs in an ADR |
| 10 | Performance | Report | 2 | +12.5% addon size, uncaught by any budget |
| 11 | Harness | Confirmed | 3 | `indexOf === -1` silently passes pairs |
| 12 | Harness | Confirmed | 3 | Empty-corpus vacuous pass via `--property` |
| 13 | Harness | Gap | 3 | Sweep has no `unreached` families check |
| 14 | Harness | Gap | 3 | `parity-sweep` unbounded; fuzz seed not settable |
| 15 | Standards | Hard | 4 | Unchecked slicing at `token_types.rs:261,309` |
| 16 | Standards | Hard | 4 | `.unwrap()` in new test code (7 sites) |
| 17 | Standards | Judgement | 4 | Normalization suite drifted onto a wrapper (new on this branch) |
| 18 | Upstream parity | Tidy | 4 | `EPSILON` declared four times |
| 19 | Upstream parity | Tests | 4 | Upstream's own `splitValue` / easing / matrix cases unported |
| 20 | Upstream parity | Latent | 5 | `rgba`/`hsla` comma spelling |
| 21 | Upstream parity | Latent | 5 | Bounded legacy alpha invented |
| 22 | Upstream parity | Latent | 5 | LCH chroma/lightness grammar |
| 23 | Upstream parity | Latent | 5 | `rgb` channels held as `u8` |
| 24 | Performance | Deferred | 6 | `to_js_string` allocates twice per number |
| 25 | Upstream parity | Deferred | 6 | Reason-drop at the wrong seam |
| 26 | Regression | Plausible | 6 | `padding: '1px /*'` emits two declarations, not four |
| -- | Removed tests | None | -- | No lost coverage requiring revert; `$=`/`*=` gap was false |

## What the review confirmed as correct

Recorded so remediation does not "fix" any of it:

- `to_js_string` is a faithful ECMA-262 `Number::toString` port -- 0 diffs vs
  `node` over the edge set plus a 20 000-value random-bit-pattern fuzz.
- The root cause was deeper than the spec anticipated: `cssparser` itself stores
  a token's number as `f32`, so widening alone made output *worse*
  (`1.2rem` -> `1.2000000476837158`) until `leading_f64` (`token_types.rs:78`)
  re-read the authored digits from source by byte offset. Verified correct
  across comments, multibyte strings, `url()`, nested functions, ident escapes.
- The nudge matches `media-query.js:359,435-437` line for line at `f64`;
  unit conflicts bail the merge.
- No `Eq`/`Hash`/`Ord` on any f64-bearing type; no sort key depends on an `f64`.
- All 144 shared `application-order` entries expanded recursively on both sides:
  **zero** mismatches. The five spread relocations match upstream.
- `split_value_parts` matches `split-css-value.js:29-53` line for line;
  `js_trim` is a real fix (JS trims `U+FEFF`, not `U+0085`).
- The collation mirrors *both* upstream comparators -- `rule-utils.js:57`
  `localeCompare` for pseudos, `:46` bare sort for at-rules -- and
  `sort_unstable_by` -> `sort_by` is required for JS sort stability. Root vs
  host locale is deliberate, documented and instrumented.
- Two changed expectations were fixes, not churn: `list_style_basic_shorthand`
  (`develop`'s `serialize_string` re-quoted; Babel emits `'=='` / `x1tpmu87`),
  and the `SimpleToken::Percentage` flip repaired a latent bug where
  `css_value.rs:308` printed `50%` as `0.5%`.
- The `5fb8dcfa1` guard re-derivation *was* reported, and correctly flags that
  the spec cited the wrong commit -- `043a1de9c` was the real one.

---

## Outcome

Status: applied, 2026-08-24. Fifteen commits, `bb9340c58..d17877d0d`.

Gate green at each commit and at the end: 6143 Rust tests, 645 JS tests, clippy
and rustfmt clean, and all three PR-time harnesses at the pre-work baseline
(parity 864 identical / 0 divergent / 0 changed / 0 unexpected; positions 13/18
with 0 unexpected; ordering 1000 pairs / 0 disagreements / 0 unreadable). The
nightly sweep was run twice by hand: 153 624 subjects, 18 893 divergences, all
pinned, 0 unexpected.

### Two findings the work overturned

Both were wrong, and running them is what proved it. Recorded because the
plan above still states them as defects.

- **Finding 13 (sweep should check unreached families).** Implemented, then
  reverted: it fires on four families every run. The sweep is broader in volume
  but *narrower in kind* -- it generates shorthand values for eight properties,
  so it cannot produce a custom-property name, a lone surrogate in a key,
  nesting past the recursion budget, or a key off `Object.prototype`. The check
  belongs to the corpus that can reach every family, which already has it. The
  sweep now carries a comment saying so.
- **Finding 17 (normalization suite drifted onto a wrapper).** Repointing the
  suite at `normalize_css_property_value` fails two tests, which is the proof
  that `normalize_value` is a seam rather than a leftover: `a } .x { color: red`
  and `red /* unterminated` are normalized by the fold and rejected by the entry
  point, and that suite pins the former deliberately so the division of labour
  is written down. Both ends now say so instead.

### One finding that grew

Porting the reference compiler's own transform and easing cases (finding 19)
settled a documentation defect nothing in the review had caught: the crate
glossary's **Echoed value** entry claimed a transform's arguments and an easing
curve's control points are echoed from source, and warned that reaching for the
formatter there would *cause* a divergence. Both claims are backwards --
`matrix(1.200, ...)` prints `matrix(1.2, ...)`, because the reference compiler
stores a `number` and interpolates it and so does this crate. Nothing in the
crate echoes at all. Verified by probe, then pinned by the ported cases.

### Scope widened by decision

Phase 5 was planned as "latent, consider a follow-up branch". The maintainer
directed that any disequality with the reference compiler be fixed regardless,
so all four landed here, each in its own commit. Three were pre-existing on
`develop` rather than introduced by this branch:

- the `", "` comma spelling in all four legacy colour forms (56 expectations);
- the invented `0..=1` alpha bound, whose own doc comment asserted the opposite
  of what the reference compiler does (9 expectations);
- the `lch()` chroma percentage (`(150 * p) / 100`), its `none` lightness, the
  `value >= 0` guards on the number arms only, and the missing percentage arm on
  every `oklch`/`oklab` channel (11 expectations);
- `rgb()` channels held as `u8`, which truncated `rgb(2.5, 0, 0)` to
  `rgb(2,0,0)` -- the same narrowing as #1267, one type over.

Every moved expectation was taken from the reference compiler's own expression
or from a run against `@stylexjs/babel-plugin@0.19.0`, never from reasoning
about floating point. Inputs kept their authored whitespace throughout, so the
suites still assert that it is tolerated on the way in.

### Left undone, deliberately

- **`prepend_key_to_reason`** (`evaluate_stylex_create_arg.rs:66`) is a
  **confirmed observable divergence**: the reference compiler emits
  `unknown error` where this compiler emits `a > flexGrow > unknown error`,
  measured by running both. Not changed, because it is the one divergence where
  matching upstream makes the compiler worse -- the text is a build-failure
  message, nothing hashes it, no CSS or class name differs, and three tests pin
  the richer form on purpose. Needs a maintainer decision, not a patch.
- **The reason-drop seam.** The reference compiler returns the spread result
  intact and drops `reason` at the caller; this port drops it in the branch.
  Same observable output, confirmed against Babel on five refusal shapes.
  Cosmetic, and entangled with the item above.
- **`timeout-minutes` on `parity-sweep`** (finding 14). No job in
  `pr-validation.yml` sets one, so adding it to a single job would invent a
  convention rather than follow one.
- **`report.ts`'s `unexpected` policy** (finding 3 in Decisions). Untouched: the
  stance is documented and reasoned, and flipping it is a maintainer call.
- **`to_js_string`'s double allocation** (finding 24). The spec puts the helper
  out of scope, and the path is cold.
- **The other 184 `unwrap()`s** in the legacy-expand test file. Older than this
  branch; a cleanup of its own.

---

## Second review, and one revert

A `/code-review` of the fifteen remediation commits (fixed point `047020f02`)
found a defect in them that the first review's nine agents had not, because it
required running the reference compiler rather than reading it.

**`889673747` is reverted.** It changed `lch()` to scale a percentage chroma by
`(150 * p) / 100` and taught every `oklch`/`oklab` channel to read a percentage,
citing `style-value-parser`'s `Lch`/`Oklch`/`Oklab` classes. Two things are
wrong with that citation:

- `Oklch.parser` and `Oklab.parser` **throw on every input**, including
  `oklch(0.7 0.1 200deg)`. `lc` carries `.prefix(Whitespace.optional)`, which
  consumes the space the enclosing sequence then demands. Nothing reaches the
  channel readers, so there is no behaviour there to be parity with.
- More decisively, **the plugin does not use those classes for a colour at
  all.** Run end to end it emits `lch(50 50% 180)` for `lch(50 50% 180)` -- the
  percentage echoed, not scaled to `75` -- and `oklch(.5 .5 180)` for
  `oklch(0.5 0.5 180)`. A colour value is normalized as text and never rebuilt
  from parsed channels.

So that commit's eleven expectations asserted numbers the reference compiler
does not produce, which is the exact fault `bb9340c58` -- the first commit of
this whole effort -- existed to fix. Mid-review a further change compounded it
by making the `oklch` hue `n * 360`, reading upstream's `lc.map(num => new
Angle(num * 360, 'deg'))`; that was discarded before it was committed, along
with the six pre-existing tests it would have flipped.

**The three commits either side stand, and were confirmed by running the
plugin**: the comma spelling (`rgba(255,0,0,1.5)`), the unbounded alpha (the
same, carried rather than refused) and the fractional `rgb()` channel
(`rgb(2.5,0,0)`).

The durable lesson is now a glossary entry, **unreachable port**: where the
plugin never runs a type's reference counterpart, a claim that some grammar
"matches the reference compiler" cannot be checked against it, and is a decision
of this crate wearing a parity costume. Reading the reference source is not
evidence about the reference compiler's behaviour; running it is.

**Also fixed, from the Standards axis.** Unbounding the alpha and widening the
channels left three doc comments asserting the opposite of the code beneath
them, the worst being an old comment left above a `#[test]` with the new one
inserted below it, so one test carried both readings. A near-duplicate refusal
test appended at file top level was folded into the mod-resident test it
duplicated. And `parse_rgb_number_token`/`parse_rgba_number_token` -- identical
bar one word, the second's doc admitting "same as RGB", both restating a bound
`rgb_number_parser` already carried -- became the one reader the reference
compiler has.

Final state: seventeen commits, `bb9340c58..a33a89341`. Gate green on a freshly
rebuilt addon -- 6139 Rust tests, 645 JS tests, clippy and rustfmt clean, all
three harnesses at baseline.

### Second-review findings: final disposition

Closed: the contradictory doc on one test, the stale "is a byte" doc and test
name, the top-level duplicate refusal test, the duplicated legacy channel
readers, the expectation asserting a number the reference never emits (reverted),
and the `let parsed = parsed(…)` shadowing.

Dissolved rather than fixed: the two one-line delegating channel readers were a
**Middle Man** only because of the reverted commit. Reverting it restored their
bodies, so the smell went with it.

**Void, not pending.** The plan asked to port `color-test.js:77`
`lch(50% 100 270deg)`. Running the plugin on it emits
`lch(50% 100 270deg)` -- the percent echoed. This crate's `Lch` stores the
authored percent as a number and would print `lch(50 100 270deg)`, dropping the
`%`. So porting that case as a parity assertion would pin a value the plugin
never produces, which is the trap the **unreachable port** rule was written for.
The case belongs to whoever puts these types on an emission path, if anyone ever
does.

**Declined, with reasons already recorded**: `timeout-minutes` on `parity-sweep`
(no job in that workflow sets one); `--seed` and `process.versions.icu` on the
ordering fuzz (the run is reproducible as it stands); the two over-width lines
(both single unbreakable ADR path tokens, one of them pre-existing and untouched).

**Resolved, as a kept divergence**: `prepend_key_to_reason`
(`evaluate_stylex_create_arg.rs:66`) plus its sibling at
`object_expression.rs:256`. Removing the prefix was implemented -- both sites,
two inverted unit tests, 32 expectations across seven files -- and then reverted
on the maintainer's call: the key path stays. Both sites now carry the reasoning,
so the next reviewer meets it where the code is rather than rediscovering the
divergence and closing it.

The measurement stands either way: the reference compiler emits `unknown error`
and `Unsupported expression: SpreadElement` bare, with no key path anywhere in
it, and a nested key does not change that. What tipped the decision is that the
text is a build-failure message -- nothing hashes it, no CSS or class name moves
-- and that most diagnostics carrying a path are this compiler's own additions,
`Expression is too deeply nested` among them, which upstream raises no error for
at all. There is nothing there to be faithful to, and the shorter sentence buys
an author nothing.

Story 20's wording is closed: `spec.md` now names `x11md1zd`, `x10ok0k0`,
`xj7mlad` and `xrqj1vq` in the story line itself, and says the issue's
`xu5ieg8`/`x1t400y5` are not reproducible against `@stylexjs/babel-plugin@0.19.0`.

**Closed from the declined list.** `parity-sweep` now carries
`timeout-minutes: 20` -- it is the only job here whose work is a cross product,
so the alphabet's square is what a runaway would grow. The ordering fuzz takes
`--seed` and refuses a non-numeric one, so a reported pair can be re-run instead
of only read, and it prints the Node ICU version, which is the number the
collation ADR left uncosted. The ADR itself is renamed: its path was 81
characters, so no reflow could bring a reference to it inside the 80-column
markdown rule, and the parity README is back to the over-width count it had
before this branch touched it. The one long line left in `pre_rule.rs` points at
another crate's ADR and predates this work.

**Closed from the void list, as characterization.** The reference compiler's
`lch(50% 100 270deg)` case is ported with the input verbatim and an expectation
that is *this crate's* answer, `lch(50 100 270deg)`, labelled as such and
pointing at the unreachable-port rule. The plugin emits `lch(50% 100 270deg)`,
the percent echoed; pinning our answer is worth doing for the parser's
consistency, and pinning it as the reference's answer is the mistake this branch
already made once.

Final state: twenty-one commits, `bb9340c58..8abfdf48a`.
