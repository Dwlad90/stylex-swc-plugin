# 06 — Dimensions spell numbers the way JavaScript does

**What to build:** a numeric CSS value at the edge of the double range is
spelled the way JavaScript spells it. Rust's own formatting never switches to
exponential form, so a value upstream writes as `1e+21` is written here with
twenty-two digits, `1e-7` as a long decimal, and negative zero keeps its sign
where JavaScript drops it. Because the printed spelling feeds the class-name
hash, the spelling itself is observable.

The workspace already has an ECMA-262 `Number::toString` port in the utils
crate, adopted by three other crates. This crate is the one that never got it.
Adopt it — do not write another one, and do not modify it.

This must land **after** the widening, not before. Adopting the helper while a
field is still single precision would widen the rounding error into the output:
a single-precision `28.8` becomes `28.799999237060547` when widened, where its
own formatting prints `28.8`.

**Blocked by:** 01, 03.

**Status:** done

- [x] The crate depends on the utils crate; no dependency cycle is introduced
- [x] Every dimension and number display path prints through the shared
      JavaScript-number helper rather than Rust's default formatting
- [x] Values at the exponential-form thresholds and negative zero are asserted
      against the spellings JavaScript produces
- [x] Every changed expectation is confirmed against Babel and listed in the
      ticket's closing note
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
  and
      `pnpm test` pass

## Closing note

Delivered, with one path deliberately left out.

**`value_parser.rs` must not adopt the formatter, but it is not correct
either -- and the first version of this note got the reason wrong.** Found in
review, corrected here.

The official compiler *echoes* the authored numeric token on that path:
`@stylexjs/babel-plugin@0.19.0` emits `1.50px`, `+1px`, `1E2px`, and
`1.7976931348623157e308px` exactly as written, applying only a leading-zero
strip. Routing the path through `String(Number)` would turn `1e21px` into
`1e+21px` where upstream writes `1e21px`, so the decision not to adopt the
formatter stands.

What the first version of this note wrongly claimed is that this crate echoes
too. It does not: it re-reads the digits into an `f64` and re-prints them, so
`1.50px` becomes `1.5px`, `1E2px` becomes `100px`, and
`1.7976931348623157e308px` becomes three hundred and nine digits. That is a
live divergence on an emission path, and `-0px` is worse than a spelling
difference -- the sign-carrying branch tests `value >= 0.`, which a negative
zero satisfies, so the crate emits `+-0px`, which is not a CSS value at all.

Closing that needs a third option neither this ticket nor 03 considered:
echoing the authored byte slice rather than any re-printing of the number.
Recorded as ticket 11 rather than folded in here, because it is a different
fix from this one and belongs behind its own revert boundary.

**One end-to-end confirmation, at the only seam where both compilers choose
the spelling.** The media query transform re-serializes the bound it derived,
so it does not echo. Babel emits
`@media (min-width: 1e+21px) and (max-width: 2e+21px)` for a two-breakpoint
`minHeight`, and the new snapshot matches it byte for byte, class names
included: `x2lwn1j`, `xvimvql`, `x18xutvv`.

**Two sites the ticket did not name, both on a numeric display path:** the
`calc()` bare-number leaf (two copies -- the `Display` impl and
`calc_value_to_string`) and the seven filter functions whose argument is a
bare `f64`.

**Expectations that moved: three, all of them tests that pinned Rust's
spelling as the known gap** and said so in their own names -- the largest
double and the smallest subnormal (long decimals to exponential form), an
overflow (`infpx` to `Infinitypx`), and a negative zero (`-0px` to `0px`, which
is also what Babel emits for `-0px`). No expectation moved that was not already
labelled as the gap this ticket closes.

**Six new edge cases failed on first run; all six were the test.** An angle
keeps the unit it was authored with rather than normalizing to `deg`; `kHz` is
not in the unit table but `KHz` is; a CSS escape takes at most six hex digits,
so `\0000070x` leaves a stray `0` in the unit; the tokenizer closes *every*
open group, not only the outermost; `Dimension` exposes `parse()` and not
`parser()`; and `1e-5 / 100` is not exactly `1e-7` in double precision --
JavaScript spells the same inexact quotient.
