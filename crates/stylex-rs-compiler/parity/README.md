# CSS value parity harness

Runs a corpus of CSS declarations through `@stylexswc/rs-compiler` and through
a pinned `@stylexjs/babel-plugin`, and reports — per declaration — whether the
two produce the same class name and the same rule text.

A StyleX class name is a hash of the canonical declaration text, so that text
is a compatibility contract. Any setup that mixes the two compilers (SSR built
by one and client bundles by the other, cached HTML, an incremental migration,
snapshot tests written against either) breaks silently when they disagree: the
markup names a class the stylesheet does not define, and nothing errors.

This is a developer tool, not a test. It lives outside the Rust test suite so
`cargo test` never needs a Node toolchain, and it is not wired into CI.

## Running it

```sh
pnpm run --filter=@stylexswc/rs-compiler build     # the harness reads dist/
pnpm run --filter=@stylexswc/rs-compiler parity
```

The harness loads the compiler from `dist/`, not from the Rust sources. **A
report is only about the last build.** Rebuild after touching a crate or the
verdicts are stale.

| Flag                    | Effect                                                  |
| ----------------------- | ------------------------------------------------------- |
| `--only-mismatches`     | print only the declarations that disagree               |
| `--set <name>`          | limit to `reported`, `edge`, or `harvested`; repeatable |
| `--filter <substring>`  | limit to declarations whose value contains it           |
| `--json <path>`         | also write the full machine-readable report             |
| `--font-size-px-to-rem` | enable the font-size conversion in both compilers       |

## Reading a verdict

| Verdict                  | Meaning                                                                                                                                                                     |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `identical`              | Both accepted and agreed byte for byte.                                                                                                                                     |
| `divergent`              | Both emitted the same properties but spelled a value differently — and therefore hashed a different class name. This is what the harness is for.                            |
| `structurally divergent` | Both accepted but emitted different properties, or a different number of them. The divergence is in shorthand expansion or property validation, not in value normalization. |
| `acceptance divergent`   | One accepted and the other rejected.                                                                                                                                        |
| `both reject`            | Both rejected. Messages may differ; only the outcome is compared.                                                                                                           |

Both compilers receive the same module text and the same option object,
constructed once in `lib/compare.ts`. Option drift would surface as a
normalization divergence and send the reader chasing the wrong thing, so
options are never spelled out per subject.

Right-to-left rule text is compared but never printed. An RTL-only difference
is a value difference all the same, and without comparing it a verdict would
call such a pair identical; the left-to-right spelling shows what changed more
plainly, so that is what the report displays.

The upstream version is held by the lockfile, not by an exact range in the
catalog. The subject block prints the version actually resolved, so a report is
attributable either way — but after a `pnpm update`, read that block before
comparing against an older report.

## The corpus

Three checked-in JSON files under `corpus/`, loaded in this order, with
duplicate declarations collapsed onto the first entry seen:

- **`reported.json`** — the six divergences reported in issue #1256, one entry
  per illustrating value. Hand-written.
- **`edge.json`** — non-ASCII content, escape sequences, URL bodies containing
  CSS-looking syntax, comments, importance annotations, empty values, and
  unclosed constructs. Hand-written.
- **`harvested.json`** — the CSS declarations the Rust test suites carry, as
  far as the scan below recognizes them. **Generated — do not edit.**

Adding a case means editing `reported.json` or `edge.json`. Entries there take
an optional `note`, which the report prints next to a mismatch.

### Regenerating the harvest

```sh
pnpm run --filter=@stylexswc/rs-compiler parity:harvest
# --check reports staleness and writes nothing:
pnpm run --filter=@stylexswc/rs-compiler parity:harvest -- --check
```

`harvest-corpus.ts` scans the test sources of `stylex-css` and
`stylex-transform` for five literal shapes that carry a CSS declaration — direct
`normalize_css_property_value` calls, case tables looped through one property,
`* {{ prop: value; }}` rule literals, minified `*{prop:value}` literals, and
`stylex.create` objects embedded in transform tests. Each entry records the
`<path>:<line>` it came from, so an unexpected one can be traced back to the
test that motivated it. Run this after adding tests that carry CSS values.

The scan is a heuristic over Rust sources, so the corpus contains some values
that are not valid CSS — degenerate inputs to the whitespace-repair unit tests,
keys that are pseudo-selectors rather than properties. They are kept: they cost
two compiler runs each, and filtering them by guesswork risks dropping a real
divergence. They land in the `structurally divergent`, `acceptance divergent`,
and `both reject` buckets, which is why those are counted apart from
`divergent`.

## Checking a future upstream release

1. Bump `@stylexjs/babel-plugin` in this package's `devDependencies` and
   `pnpm install`.
2. Rebuild the compiler, then run
   `pnpm parity --json parity/results/<version>.json`.
3. The printed subject block states both versions, so a report is always
   attributable to a specific release.
4. Diff the new report's `divergent` set against the previous one. Anything new
   is an upstream normalization change this compiler has not adopted.

## Using a report to write test expectations

Never write or update an expectation by eye — a hand-edited expectation in this
area is just the bug re-encoded. Take it from `--json` output instead. Each
report entry carries, for both compilers:

- `classNames` — the generated class names, in emission order
- `rules` — the full LTR rule text
- `declarations` — the `property:value` text inside the braces, which is what
  value normalization produces and therefore what the value-normalization tests
  assert against

`entries[].babel.declarations` is the expectation; `entries[].rust` is what this
compiler produces today.

**A verdict with no declarations is not a verdict.** Both compilers expand or
drop shorthands such as `background` and `border`, so a subject written on one
produces an empty `declarations` array on both sides and the entry is reported
`identical` — agreement about nothing. Check that `entries[].rust.declarations`
and `entries[].babel.declarations` are non-empty before reading an `identical`
as evidence, and pick a longhand (`backgroundImage`, `boxShadow`) when writing
the subject.

**One caveat when the expectation is for a value-normalization test.** The
harness runs the whole transform, and a few properties never reach value
normalization at all: `content`, `hyphenateCharacter` and `hyphenate-character`
are returned verbatim when their value already carries matching quotes, names a
content function, or is a content keyword. A verdict on one of those describes
the transform path, not the normalization seam, and the two disagree — `content:
"\2014 A"` survives the transform unchanged but has its escape resolved at the
seam. Pick a property that reaches normalization (`fontFamily` covers the same
string shapes) when sourcing an expectation for a normalization test.
