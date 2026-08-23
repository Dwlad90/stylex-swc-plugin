# 07 — Colours spell numbers the way JavaScript does

**What to build:** the same JavaScript number spelling, applied to every colour
display path, so that no default-formatted number is left behind in the crate to
reintroduce the divergence.

**Blocked by:** 04, 05, 06.

**Status:** done

- [x] Every colour display path prints through the shared JavaScript-number
      helper
- [~] No numeric display path in the crate uses Rust's default formatting. Every
      `Display` impl does; `value_parser.rs`'s raw echo path deliberately does
      not, and should not -- see the closing note and ticket 11.
- [x] Every changed expectation is confirmed against Babel and listed in the
      ticket's closing note
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
  and
      `pnpm test` pass

## Closing note

Delivered. The `rgba` and `hsla` alphas, every `lch`, `oklch`, and `oklab`
channel and their optional alphas, and the bare-number arm of the `lch` hue all
print through the shared formatter.

**The second criterion is met for every `Display` impl, and one path is
deliberately outside it.** After this commit the transform and easing functions
were the only `Display` impls left on Rust's default formatting, and tickets 08
and 09 removed those. Verified by enumerating every `f64` field and every
`Display` impl rather than by grepping for the symptom -- and that enumeration
is exactly why the exception was missed on the first pass: `value_parser.rs`
writes into a `String` rather than through a `Display` impl, so it is not in
either list. It stays on `{}` because adopting the formatter there would be a
*new* divergence, and it needs a different fix entirely; ticket 11 carries it.

Two integer paths are also outside the criterion and always were: `Fraction`'s
`i32` numerator and denominator, and `StepsEasingFunction`'s `u32` count.
JavaScript spells an integer the way Rust does, so there is nothing to adopt.

**Nothing changed for the integer channels**, and nothing for the `hsl` hue and
percentages: those already reached the formatter through the angle and
percentage types, which ticket 06 converted.

**Expectations that moved: three, all pinned as the gap.** An alpha of
`0.0000001` prints as `1e-7`; a negative zero on any channel loses its sign;
and the two authored spellings of one alpha collapse onto one printed form,
because the alpha is one double and the formatter names a double one way. As
ticket 04 recorded, the colour types are not on the official compiler's
emission path -- it echoes an authored colour with whitespace normalized -- so
`String(Number)` is the reference here, which is what `to_js_string` is.

**One expectation of the new tests was wrong on first run:** `Lch`'s hue is a
number *or* an angle, and a bare number prints without a unit, unlike
`Oklch`'s, which is always an angle. Now pinned as the difference it is.
