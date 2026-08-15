# 02 — JavaScript-semantics float parsing utility

**What to build:** Three of the nine ported normalizers decide what to do with a
token by parsing it as a number the way JavaScript's lenient float parsing does
— read as much of a leading number as possible, ignore whatever trails it, and
report failure only when no number could be read at all. That behaviour has no
equivalent in Rust's standard parsing, which rejects a string with any trailing
characters.

This ticket adds that utility to the shared utils crate. The complementary
direction — spelling a float back out the way JavaScript does — already exists
there and is reused unchanged.

This is called out separately, rather than folded into the normalizer port,
because it is the highest silent-divergence risk in the whole effort. A float
parsed or spelled one digit differently produces a different class hash, and it
surfaces at every other seam as an unrelated-looking wrong character with no way
to localize it. It deserves its own exhaustive table of cases.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] Leading whitespace is skipped before parsing begins
- [x] An optional leading sign is accepted
- [x] Digits, an optional fractional part, and an optional exponent are consumed
- [x] Parsing stops at the first character that cannot continue the number, and
      the number read so far is returned rather than the whole string being
      rejected
- [x] The infinity literal is recognized, with sign
- [x] Input from which no number can be read reports failure, corresponding to
      the JavaScript not-a-number result — callers must be able to distinguish
      this without inspecting a sentinel value
- [x] Behaviour is asserted by a table of input/output pairs generated from a
      JavaScript runtime, not hand-written from memory
- [x] The table covers the shapes the normalizers actually encounter: bare
      integers, decimals with and without a leading zero, negative decimals,
      values with unit suffixes, values with percent suffixes, exponent
      notation with and without a signed exponent, and values that are units
      only
- [x] No use of unchecked unwrapping, per the repository's error-handling rules

## Answer

`parse_js_float` in `crates/stylex-utils/src/number.rs`, next to the
`to_js_string` it complements. `Option<f64>`, so the NaN case is visible in the
type and a caller cannot let "no number here" pass as zero.

The scan is ECMA-262's `StrDecimalLiteral` under the longest-prefix rule: skip
`StrWhiteSpace`, optional sign, then either the `Infinity` literal or digits
with an optional fraction and an optional *complete* exponent. An exponent that
never completes is not part of the literal, so `1e`, `1e+` and `1epx` all read
back as `1`. The matched prefix is then handed to Rust's own `f64` parse, whose
accepted grammar is a superset of what the scan admits.

Two details a reader would otherwise reach for the standard library for:

- **Whitespace is hand-rolled.** `char::is_whitespace` disagrees with JS at both
  ends -- it admits U+0085, which `parseFloat` does not skip, and omits U+FEFF,
  which it does.
- **`-0` is asserted bit-for-bit.** `-0.0 == 0.0` under `==`, so an equality
  assertion would have let a lost sign through.

**Expectations are generated, never written by eye.**
`crates/stylex-utils/scripts/generate-parse-float-cases.mjs` prints the Rust
table straight out of `Number.parseFloat`; the checked-in
`src/tests/number_parse_float_cases.rs` carries its **100 cases**. Both are
wired into the crate's `package.json`:

```sh
pnpm run --filter=@stylexswc/utils generate:parse-float-cases
# diffs a fresh run against what is committed
pnpm run --filter=@stylexswc/utils generate:parse-float-cases:check
```

The `:check` variant is what stops the table drifting from the generator; the
`crates/*/scripts/` convention it establishes is recorded in
`guidelines/STRUCTURE.md`.

The 100 cases cover every shape the checklist lists -- bare integers, decimals
with and without a leading zero, negative decimals, unit and percent suffixes,
signed and unsigned exponents, units only -- plus the degenerate inputs (`""`,
`"-"`, `"."`, `".e3"`), the overflow/underflow edges (`1e400`, `1e-400`,
`5e-324`), the non-decimal prefixes JS truncates rather than rejects (`0x10`,
`1_000`), the `Infinity` literal in every casing, **every one of the nineteen
`StrWhiteSpace` codepoints one per entry**, and the three near misses (U+0085,
U+180E, U+200B) that are whitespace to Rust or to neither but not to JS.

**The whitespace table is load-bearing, and proven so.** Swapping
`is_js_whitespace` for `char::is_whitespace` now fails on `"\u{feff}1"`, and
adding U+0085 to the set fails on `"\u{85}1"`. Before those cases existed the
whole hand-rolled set could be reverted with the suite still green.

**Coverage.** `cargo llvm-cov nextest -p stylex_utils` reports 100% of lines,
regions and functions for `number.rs`, meeting the gate in
`guidelines/STRUCTURE.md`. Getting there caught a real defect: a first pass at
the whitespace additions silently dropped the `Infinity` and non-decimal-prefix
entries from the generator, and the uncovered early-return was the only thing
that showed it.

**Cross-checked beyond the table.** Two independent randomized corpora, neither
sharing the checked-in inputs, were run through both `Number.parseFloat` and
`parse_js_float` and compared bit-for-bit: 2,991 and 3,091 distinct inputs built
from numeric fragments, unit suffixes, every whitespace kind, multi-byte leads
(`é`, an emoji, Arabic-Indic and fullwidth digits, U+2212 minus, U+FFFD) and
malformed pieces, plus digit strings up to 800 characters. **0 mismatches, 0
panics** -- including no char-boundary panic from the byte-index slicing.

The upstream plugin's three call sites (`parseFloat(dimension.number)`,
`parseFloat(node.value)`, and the source-slice form) all use bare `parseFloat`,
so this is the function parity is owed to.
