# 03 — Every dimension holds a double

**What to build:** an author writing any numeric CSS value — an angle, a
duration, a frequency, a resolution, a percentage, a flex fraction, an alpha, a
bare number, a length inside `calc()` — gets the value the official compiler
emits, not one rounded to single precision first.

Three of these compute at print time: a duration in milliseconds is rewritten to
seconds, a fractional frequency in Hz is rewritten to KHz, and a unitless number
is multiplied by 100 to become a percentage. At double precision these produce
upstream's exact results, which is sometimes a longer string than before — a
number times 100 can print as `7.000000000000001` where the single-precision
path printed `7`. That is the correct output; confirm each changed expectation
against `@stylexjs/babel-plugin` from `node_modules` before updating it.

**Blocked by:** 01. Not a logical dependency — a serialization guard, because
both tickets touch the dimension, length-percentage, and common-types modules.

**Status:** done

- [x] Angle, time, frequency, resolution, percentage, bare number, flex
      fraction, alpha, and the `calc()` dimension all hold a double
- [x] The millisecond-to-second, Hz-to-KHz, and times-100 conversions compute in
      double precision
- [x] No narrowing cast remains on any path from a parsed token to emitted text
      for these types
- [x] Every changed expectation is confirmed against Babel and listed in the
      ticket's closing note
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
  and
      `pnpm test` pass

## Closing note

Delivered, with one deliberate departure from the spec.

**The percentage decision is reversed, on evidence.** The spec said to accept
`7%` printing as `7.000000000000001%` and stated that value was confirmed
against Babel. It is not: `@stylexjs/babel-plugin@0.19.0`'s bundled tokenizer
runs `parseFloat` over the authored text and stores the authored percent, so
`7%` has value `7`, and Babel emits `7%` -- confirmed by running it. Keeping
the fraction-and-multiply-back-up round trip would have introduced a *new*
divergence under cover of fixing an old one. `SimpleToken::Percentage` now
carries the authored percent, and the `/100` survives only where a fraction is
genuinely wanted: an alpha, a filter argument, a scale factor.

This also settled a disagreement inside the crate: a `CssValue::Percentage`
built from a token printed `0.5%` where the same percentage built from its own
type printed `50%`.

**Expectations that moved:** the percentage ones above, plus test literals that
constructed `SimpleToken::Percentage` directly in the old fraction shape. No
emitted-CSS expectation moved.

**Colour channels** are still single precision, bridged at two marked sites in
`color.rs`, and widen in tickets 04 and 05.

**Six new edge cases failed on first run; all six were the test, not the
parser.** The nearest double to `0.12345678901234567` really is `...566` (as
JavaScript agrees); `1.1\70x` is a valid `px` because `\70` escapes to `p`; an
out-of-range alpha is carried through rather than refused, matching upstream;
an unclosed `calc(` is tolerated rather than rejected; `kHz` is not in the unit
table, `KHz` is; and `5e-324` prints as a 300-digit decimal, which is ticket
06's gap rather than this one's. Each is now pinned as the behaviour it
actually is.

**A second emission site, found in review.** `value_parser::parse_css` reads
`cssparser` tokens directly rather than through `SimpleToken`, so it was
untouched by the token-layer fix and still printed the `f32`. It is reached
from shorthand expansion, so it is on the emission path:
`1.2345678901234567px` came back as `1.2345679px` and
`1.7976931348623157e308px` saturated to `infpx`. Fixed the same way, in its own
commit (`0c0386009`), with the divergence confirmed against Babel first.
