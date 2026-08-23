# 08 — A matrix prints the numbers it was given

**What to build:** an author writing a `transform` with more than six decimal
places of precision gets the matrix they wrote. Today the transform function
display rounds every argument to six decimals and trims trailing zeros, so
`matrix(1.0000005, …)` is silently emitted as `matrix(1.000001, …)`. Upstream
interpolates each number directly, which is plain JavaScript number
stringification — no rounding step exists there.

The rounding helper arrived in the bulk commit that added all the CSS parsers,
with no issue behind it, no rationale in the diff, and no test asserting the
rounding. It is a porting artifact, not a decision. Delete it and print through
the shared JavaScript-number helper.

Kept separate from ticket 09 because the test surface is roughly twice the size.

**Blocked by:** 03, 06.

**Status:** done

- [x] The transform function display's rounding helper is gone
- [x] Every transform function argument prints through the shared
      JavaScript-number helper
- [x] A transform carrying more than six decimal places round-trips its full
      precision, asserted against Babel's output for the same input
- [x] Every changed expectation is confirmed against Babel and listed in the
      ticket's closing note
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
  and
      `pnpm test` pass

## Closing note

Delivered.

**The ticket's claim about provenance held under test.** No test asserted the
rounding: the helper was deleted and the full workspace suite passed unchanged
before a single new test was written.

**Babel confirmed first.** `@stylexjs/babel-plugin@0.19.0` emits
`matrix(1.0000005,2.00000049,.1234567890123,4,5,6)` and
`scale(1.0000005,2.0000005) rotate3d(.5000005,.5000005,.5000005,45deg)` for the
same source -- every digit intact, no rounding step anywhere. Its comma spacing
and stripped leading zeros come from a later normalizing pass and are not this
type's business.

**Two further bugs went with the helper, neither in the ticket.** `{:.6}`
rounded anything below 1e-6 to `0`, so a small scale factor became no scale
factor at all. And the whole-number branch printed via `as i64`, a *saturating*
cast in Rust, so `scaleX(1e19)` came out as `scaleX(9223372036854775807)`. Both
are now pinned.

**Expectations that moved: none.** Nothing in the suite depended on the
rounding.

**One new test was wrong on first run:** an unclosed `matrix(` is tolerated,
not refused -- the tokenizer synthesises the closing paren, exactly as it does
for `calc(`. Pinned as the tolerance it is. A second, on escaped function
names, was refined rather than corrected: `\6d atrix(` *is* `matrix`, while
`\6datrix(` is not, because `a` is a hex digit the escape swallows. Both are
now pinned, so the distinction is recorded rather than implied.
