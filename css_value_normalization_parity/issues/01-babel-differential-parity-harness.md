# 01 — Babel differential parity harness

**What to build:** A maintainer can run one command and get a report of every
CSS value whose normalized declaration differs between this compiler and a
pinned `@stylexjs/babel-plugin` release. Today that comparison is done by hand,
one value at a time, which is why the divergences in the parent spec went
unnoticed until a production codebase surfaced them.

The harness feeds a corpus of values through both compilers with identical
options and diffs the resulting style metadata — class name and rule text. It
reports matches and mismatches, and its mismatch list is the authoritative
answer to "what will change when the normalization pipeline is replaced."

This ships first because everything downstream depends on it: it is the oracle
that generates the expectations for tickets 04, 05, 07 and 10. No expectation in
this effort is ever written by hand or updated by eye.

It is a developer tool, not a test. It lives outside the Rust test suite so that
running `cargo test` never requires a Node toolchain, and it is not wired into
continuous integration. It is checked in rather than discarded because this
class of divergence recurs at every upstream release, and a throwaway means the
next person rebuilds it from nothing.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] A single command runs the whole corpus through both compilers and prints a
      per-value verdict: identical, or both spellings side by side
- [x] The upstream plugin version is pinned and stated in the output, so a
      report can be attributed to a specific release
- [x] Both compilers receive identical options, including the module resolution
      settings, so option drift cannot be mistaken for a normalization
      divergence
- [x] The corpus includes every value literal harvested from the existing Rust
      test suites
- [x] The corpus includes all six cases reported in the parent issue
- [x] The corpus includes a hand-written edge set: non-ASCII content, escape
      sequences, URLs whose bodies contain CSS-looking syntax, comments,
      importance annotations, empty values, and unclosed constructs
- [x] The report can be emitted in a form that a later ticket can consume to
      annotate test expectations, rather than only as prose for a human
- [x] A baseline report is captured and recorded on this ticket, establishing
      the divergence set as it stands before any change
- [x] Running it requires no change to the Rust test suite and adds no
      dependency to `cargo test`
- [x] Documented well enough that a maintainer can re-run it against a future
      upstream release without reading its source

## Answer

Shipped as `crates/stylex-rs-compiler/parity/`, alongside the existing
`benchmark/` developer tooling in the same package — which is where the
`@stylexjs/babel-plugin` devDependency and the built `dist/` both already live.

```sh
pnpm run --filter=@stylexswc/rs-compiler build     # the harness reads dist/
pnpm run --filter=@stylexswc/rs-compiler parity
pnpm run --filter=@stylexswc/rs-compiler parity:harvest
```

Full usage in `parity/README.md`.

**Corpus — 570 declarations across three checked-in sets.** `reported.json`
(14 hand-written entries covering all six reported symptoms), `edge.json`
(43 hand-written entries), and `harvested.json` (528 declarations extracted
from the `stylex-css` and `stylex-transform` test sources by
`parity:harvest`, each carrying its `<path>:<line>` origin). Duplicates across
sets collapse onto the first entry seen, so the hand-written note and origin win.

The checklist says "every value literal harvested from the existing Rust test
suites"; what ships recognizes five literal shapes across the two crates that
carry CSS declarations. A shape nobody anticipated is silently dropped, so the
README states the scan is a heuristic rather than a guarantee. The extractors
are unit-tested in `parity/__tests__/`, which is where a sixth shape gets
added.

**Verdicts.** A fifth verdict beyond the ticket's wording — `structurally
divergent` — separates declarations that expanded into *different properties*,
or a different number of them (shorthand expansion, property validation), from
ones that differ only in how a value was spelled. Without it, 15
shorthand-validation differences inflated the number this effort is scoped to.
Right-to-left rule text is compared but not printed, so an RTL-only divergence
cannot be reported as identical; none occurs in this corpus.

**Pinning.** The upstream version is held by the lockfile rather than by an
exact range in the catalog, which still carries `^0.19.0`. Every report prints
the version actually resolved, so a verdict stays attributable to a release;
after a `pnpm update` that block is the thing to read before comparing reports.

**`--json` output** carries, per entry and per compiler, `classNames`, `rules`,
`rtlRules`, and `declarations` (the `property:value` text inside the braces).
`entries[].babel.declarations` is the expectation tickets 04, 05, 07 and 10 read
from; nothing is written by eye.

## Baseline

`@stylexswc/rs-compiler` v0.18.4-rc.1 (branch `fix_CSS-value-normalization`)
against `@stylexjs/babel-plugin` v0.19.0, `@babel/core` v8.0.1, options
`{dev: false, enableFontSizePxToRem: false, unstable_moduleResolution: {type: "haste"}}`.

| Verdict | Count |
| --- | --- |
| identical | 432 |
| **divergent** | **97** |
| structurally divergent | 15 |
| acceptance divergent | 12 |
| both reject | 14 |

### The 97 value-normalization divergences

| Symptom | Count |
| --- | --- |
| whitespace between tokens (identical once whitespace is stripped) | 50 |
| single quotes rewritten to double | 13 |
| leading zero not stripped where upstream strips it | 7 |
| comment dropped from the value | 6 |
| 6-digit hex shortened | 4 |
| exponent notation | 1 |
| other (below) | 16 |

All four live symptoms from the parent issue reproduce exactly as the spec
describes them, including the *displaced* space in
`max(4.8125rem, 100vw * 0.12)` → `max(4.8125rem,100vw* .12)`. Both
already-fixed symptoms — `translateX`/`translateY` capitalization and
`-10000px` — report `identical`, so the rewrite must not trade them back.

The 16 "other" divergences, which the harness surfaced and #1256 does not name:

- **Value corruption, not just respelling.** `fontFamily: '"Helvetica \"Neue", sans-serif'`
  emits `font-family:Helvetica "Neue,sans-serif}` — a stray brace, and the
  outer quotes gone. `height: 'url("a\")b.png") 10px'` emits
  `url(a")b.png)10px}` the same way. An escaped quote inside a string is being
  mis-scanned, and the repair passes then splice the rule's own terminator into
  the value.
- **Zero-dimension conversion.** `transform: 'rotate(0rad)'` stays `0rad` here;
  upstream emits `0deg`.
- **Leading-zero stripping inside colour functions.** `hsla(120, 100%, 50%, 0.5)`,
  `oklab(40.101% 0.1147 0.0453)`, and the relative-colour forms keep `0.`
  here and lose it upstream. These are the values routed around the CSS parser
  by the colour-function allowlist, which is why they also keep their spaces
  after commas.
- **Function-argument spacing.** The same allowlist leaves `clamp(200px,  40%,     400px)`
  spaced where upstream emits `clamp(200px,40%,400px)`.
- **Unit letter case.** `CALC(100% - 20PX)` keeps the function's case but
  lowercases the unit; upstream keeps both.
- **Silent truncation at a `;`.** Six entries of the form
  `color: 'red; margin: 10px'` emit `color:red` here and the full multi-
  declaration text upstream. This is *not* the spec's injection guard firing:
  that guard **rejects** a value carrying a rule-terminating character, and
  these are accepted. The value parses cleanly into two declarations and only
  the first survives extraction, so author input is dropped with no diagnostic.
  Worth deciding on explicitly during the swap rather than inheriting.

### Out-of-scope buckets, for the record

- **structurally divergent (15)** — `borderBottom`, `borderInline`, `font`,
  etc.: shorthands this compiler expands and upstream rejects. Property-level,
  not value-level; tracked outside this spec.
- **acceptance divergent (12)** — the local guards and the degenerate inputs.
  Notably, **an empty value (`color: ''`) is accepted here and crashes
  upstream** with `Cannot read properties of undefined (reading 'type')` — which
  confirms the spec's reading of upstream's unguarded whitespace normalizer, and
  answers its open question: an empty value *does* reach normalization.
  `var(x)`, the rule-breaking token, and the unclosed comment are the three
  retained local guards, all rejected here and accepted upstream.

Regenerate with:

```sh
pnpm run --filter=@stylexswc/rs-compiler parity -- --json parity/results/baseline.json
```

`parity/results/` is gitignored; the numbers above are the record.
