# 05 — Modern colour channels hold a double

**What to build:** an author writing a colour in a precise space — `lch`,
`oklch`, `lab`, `oklab` — gets the channel values the official compiler emits.
These spaces are where single-precision rounding is most visible, because their
lightness and chroma channels carry small fractional values by design.

**Blocked by:** 04.

**Status:** done

- [~] Every `lch`, `oklch`, and `oklab` channel, and their optional alpha, hold
  a double. `lab` is not a space this crate has, so the box is partial by
  design rather than done -- see the closing note.
- [x] No narrowing cast remains on the path from a parsed colour token to
      emitted text for these spaces
- [x] No `f32` field remains anywhere in the crate's numeric CSS types
- [x] Every changed expectation is confirmed against Babel and listed in the
      ticket's closing note
- [x] `cargo test`, `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`,
      and
      `pnpm test` pass

## Closing note

Delivered together with 04, in one commit -- see 04's closing note for why the
two could not be split cleanly, and for the Babel comparison both rest on.

**`lab()` does not exist in this crate.** The ticket names four modern spaces;
the crate has three. `Lch`, `Oklch`, and `Oklab` are all there is, and
`lab(52.2345 40.1645 59.9971 / 0.5)` is refused outright rather than
mis-parsed. Adding the space is a feature, not a widening, so it was left
alone -- and the refusal is now pinned as a test, so the gap is recorded where
the next contributor will trip over it rather than inferred from an absence.

**No `f32` field remains in the crate.** The one surviving mention is
`value_parser.rs`'s `authored_number(.., fallback: f32)`, which names what
`cssparser` hands over -- the value being escaped from, not a field the crate
stores.

**Expectations that moved:** none beyond 04's two hex-alpha approximations. No
emitted-CSS expectation moved.

**Three behaviours pinned rather than changed,** because the widening had to
leave them alone and a silent change to any of them would now show up as a
failing test: `none` reads as zero on every channel; an out-of-range alpha is
carried through on these paths where `rgba()` refuses one; and a function
truncated at its closing paren is tolerated while one truncated before its last
channel is refused.
