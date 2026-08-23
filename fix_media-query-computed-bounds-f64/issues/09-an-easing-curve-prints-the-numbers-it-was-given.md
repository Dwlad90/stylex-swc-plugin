# 09 — An easing curve prints the numbers it was given

**What to build:** an author writing a high-precision `cubic-bezier(...)` gets
the curve they wrote. The easing function display carries the same six-decimal
rounding helper as ticket 08, with the same provenance and the same absence of
justification, so a control point beyond six decimals is silently moved.

**Blocked by:** 06.

**Status:** done

- [x] The easing function display's rounding helper is gone
- [x] Every control point prints through the shared JavaScript-number helper
- [x] A `cubic-bezier` carrying more than six decimal places round-trips its
  full
      precision, asserted against Babel's output for the same input
- [x] Every changed expectation is confirmed against Babel and listed in the
      ticket's closing note
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
  and
      `pnpm test` pass

## Closing note

Delivered.

**There were two helpers here, not one.** The ticket says "the same
six-decimal rounding helper"; the file carried two copies of it, one closure
inside `CubicBezierEasingFunction`'s `Display` and a second inside
`LinearEasingFunction`'s. Both are gone. A ticket naming only `cubic-bezier`
would have left `linear()` rounding, so the count is worth recording.

**Babel confirmed first.** `@stylexjs/babel-plugin@0.19.0` emits
`cubic-bezier(.1234567890123,.5000005,.98765432109,1)` and
`linear(0,.2500005,.5000005,1)` for the same source.

**The same two further bugs as ticket 08 went with them:** `{:.6}` rounded
anything below 1e-6 to `0`, and the whole-number branch's `as i64` saturated,
so a control point of `1e19` printed as `9223372036854775807`.

**Expectations that moved: none.** As with ticket 08, no test depended on the
rounding.

**A `steps()` count and the keyword curves take no numeric path at all** --
the count is a `u32` -- and both are now pinned so the formatter's reach stays
bounded by the types that hold a double.

**One new test was wrong twice before it was right,** and the correction is
the useful part: a CSS hex escape stops at the first character that is not a
hex digit, so `\63 ubic-bezier` and `\63ubic-bezier` are *both*
`cubic-bezier` -- the space is not required, it is merely one way to terminate
the escape. The counterexample lives in the transform companion file, where
`\6datrix` is not `matrix`.
