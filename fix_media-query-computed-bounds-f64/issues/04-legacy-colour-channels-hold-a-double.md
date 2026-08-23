# 04 — Legacy colour channels hold a double

**What to build:** an author writing `rgba(...)` with a precise alpha, or any
`hsl()` / `hsla()` colour, gets the channel values the official compiler emits
rather than single-precision approximations of them.

Scoped to the legacy colour spaces so that the change stays reviewable: the
colour module and its two test files run to roughly six thousand lines between
them, which is why the modern spaces are ticket 05.

**Blocked by:** 03.

**Status:** done

- [x] The `rgba` alpha channel and every `hsl` / `hsla` channel hold a double
- [x] No narrowing cast remains on the path from a parsed colour token to
      emitted text for these spaces
- [x] Every changed expectation is confirmed against Babel and listed in the
      ticket's closing note
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and
      `pnpm test` pass

## Closing note

Delivered together with 05, in one commit
(`fix(stylexswc/css-parser): hold every colour channel at double width`). The
two tickets were split to keep the review small, but the seam between them is
one file and three shared helpers -- the optional-alpha parser is copied
verbatim into `Lch`, `Oklch`, and `Oklab` -- so splitting the commit would have
meant a middle state that narrowed an alpha the surrounding code no longer
narrowed anywhere else. Combined at the user's request.

**What Babel actually does with a colour, checked first.** The official
compiler does not re-serialize a colour through a type at all: it echoes the
authored text with whitespace normalized and leading zeros stripped, so
`rgba(255, 0, 0, 0.123456789012345)` comes out as
`rgba(255,0,0,.123456789012345)` -- all fifteen digits. That makes the target
unambiguous: any channel that cannot hold the authored digits is a divergence.
Confirmed by running `@stylexjs/babel-plugin@0.19.0` over eight colours across
all six spaces.

**The colour types are not on the transform emission path.** Nothing outside
`stylex-css-parser` constructs a `Rgba` or an `Oklch`; the shorthand expansion
path reads `cssparser` tokens directly and echoes them, the same way Babel
does. So there is no end-to-end seam to assert at, and the new tests sit at the
parser -- which is what the spec's testing section anticipated for the types
"the transform seam cannot reach cheaply".

**Expectations that moved: two, both approximations.**
`color_coverage_test.rs` compared the eight-digit-hex alpha accessor with
`abs() < 0.001`, because at single precision `0x78 / 255` could only be
compared approximately. At double precision it is the same quotient JavaScript
computes, so both assertions are now exact equalities against
`f64::from(byte) / 255.0`. No emitted-CSS expectation moved.

**Two divergences found and left alone, both out of this ticket's scope:**

1. `Lch` does not remember whether its lightness was written as a percentage,
   so `lch(52.2345% 72.2 56.2)` prints as `lch(52.2345 72.2 56.2)`. Babel
   echoes the `%`. This is a lost token, not a lost digit.
2. `rgba()` and `hsla()` refuse an alpha outside `0..=1` with their own
   hand-rolled range check, while the modern spaces accept one through the
   shared alpha parser. Both behaviours are now pinned so the inconsistency is
   visible rather than latent.
