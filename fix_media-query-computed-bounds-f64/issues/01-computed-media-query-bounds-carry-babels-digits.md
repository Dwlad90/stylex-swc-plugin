# 01 — Computed media-query bounds carry Babel's digits

**What to build:** an author who writes four overlapping `@media` breakpoints at
fractional `rem` values gets the same media query text and the same class names
the official compiler produces. Today the derived upper bound of each range is
rounded to single precision, so `28.81 - 0.01` prints as `28.8` instead of
`28.799999999999997`, the breakpoint lands at a different width, and the class
name differs — which means output from the two compilers cannot be mixed across
an SSR and client boundary.

Closes GitHub issue #1267.

The length type and the media-rule number both hold a double, matching the
JavaScript `number` upstream holds, and the media query interval merge stops
discarding its own double-precision result on the way out. Printing still goes
through Rust's own formatting; that already produces the right digits for every
value this ticket is about.

Run the full workspace suite before changing anything, so that every moved
expectation afterwards is attributable. Existing media query expectations use
round breakpoints, which print identically at both widths, so near-zero churn is
expected. Anything that does move gets its new value confirmed against
`@stylexjs/babel-plugin` from `node_modules` before the expectation is updated —
a tidier string that upstream does not produce is the bug, not the baseline.

**Blocked by:** None — can start immediately.

**Status:** done

- [x] The length type's value and the media-rule number variant hold a double
- [x] No narrowing cast remains between a parsed token and emitted media query
      text
- [x] The issue's exact input, run end to end through `stylex.create`, emits all
      four rules with the class names Babel produces, including `xu5ieg8` and
      `x1t400y5`
- [x] The derived bounds `28.799999999999997rem` and `32.870000000000005rem`
      appear in the emitted media query text
- [x] A strict range query such as `(width > 400.5px)` rewrites to a
      `min-`/`max-` pair whose nudge was computed in double precision
- [x] Existing expectations that move are each confirmed against Babel and
      listed in the ticket's closing note
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
  and
      `pnpm test` pass; the compiler is rebuilt before the JS suite runs

## Closing note

Delivered. The length type and the media-rule number hold a double, and the
interval merge stops narrowing its own result on the way out.

**One root cause the spec did not anticipate.** Widening the field alone made
the output worse, not better: `cssparser` stores a token's number as an `f32`,
so `1.2rem` became `1.2000000476837158` the moment it was widened -- the
rounding had been masked by the narrowing that followed it. The authored digits
are re-read from the source by byte offset (`leading_f64` in `token_types.rs`),
which is the same thing the official compiler's tokenizer does with
`parseFloat`. Without this, none of this ticket's acceptance criteria were
reachable.

**Class names.** The ticket asked for `xu5ieg8` and `x1t400y5`. Running the
issue's exact input through `@stylexjs/babel-plugin@0.19.0` produces `x10ok0k0`
and `xj7mlad` instead, so those are what the snapshot pins, along with
`x11md1zd` and `xrqj1vq` for the outer two rules. The media query text matches
the issue verbatim: `28.799999999999997rem` and `32.870000000000005rem`.

**Expectations that moved:** none. Every existing media query expectation uses
round breakpoints, which print identically at either width, exactly as the
ticket predicted.
